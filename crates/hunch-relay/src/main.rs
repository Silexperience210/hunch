//! `hunch-relay` — async WebSocket Nostr relay (NIP-01) with Hunch-kind validation.
//!
//! In-memory store shared across connections behind a Mutex; newly accepted events are
//! broadcast so live subscriptions get real-time pushes after their initial EOSE.
//!
//! Supported client messages: `["EVENT", ev]`, `["REQ", sub, filter...]`, `["CLOSE", sub]`.
//! Server replies: `["OK", id, bool, msg]`, `["EVENT", sub, ev]`, `["EOSE", sub]`, `["CLOSED", sub, msg]`.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use anyhow::{Context, Result};
use clap::Parser;
use futures_util::stream::SplitSink;
use futures_util::{SinkExt, StreamExt};
use hunch_relay::{matches_filter, Relay};
use serde_json::{json, Value};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;

#[derive(Parser)]
#[command(name = "hunch-relay", version, about = "Hunch Nostr relay with HIP-1 kind validation")]
struct Cli {
    /// Address to listen on (ws://).
    #[arg(long, env = "HUNCH_RELAY_LISTEN", default_value = "127.0.0.1:8080")]
    listen: String,
    /// Broadcast channel capacity for live subscriptions.
    #[arg(long, default_value_t = 1024)]
    broadcast_capacity: usize,
}

type Tx = broadcast::Sender<Arc<Value>>;
type Sink = SplitSink<WebSocketStream<TcpStream>, Message>;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let relay = Arc::new(Mutex::new(Relay::new()));
    let (tx, _rx) = broadcast::channel::<Arc<Value>>(cli.broadcast_capacity);

    let listener = TcpListener::bind(&cli.listen).await.with_context(|| format!("binding {}", cli.listen))?;
    eprintln!("hunch-relay listening on ws://{}", cli.listen);

    loop {
        let (stream, peer) = match listener.accept().await {
            Ok(pair) => pair,
            Err(e) => {
                eprintln!("accept error: {e}");
                continue;
            }
        };
        let relay = Arc::clone(&relay);
        let tx = tx.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_conn(stream, relay, tx).await {
                eprintln!("connection {peer} ended: {e:#}");
            }
        });
    }
}

async fn handle_conn(stream: TcpStream, relay: Arc<Mutex<Relay>>, tx: Tx) -> Result<()> {
    let ws = tokio_tungstenite::accept_async(stream).await.context("websocket handshake")?;
    let (mut write, mut read) = ws.split();
    let mut rx = tx.subscribe();
    // Active subscriptions on this connection: sub_id -> its list of filters.
    let mut subs: HashMap<String, Vec<Value>> = HashMap::new();

    loop {
        tokio::select! {
            incoming = read.next() => {
                match incoming {
                    Some(Ok(Message::Text(txt))) => {
                        handle_client_message(txt.as_str(), &relay, &tx, &mut write, &mut subs).await?;
                    }
                    Some(Ok(Message::Ping(p))) => write.send(Message::Pong(p)).await?,
                    Some(Ok(Message::Close(_))) | None => break,
                    Some(Ok(_)) => {} // ignore binary/pong
                    Some(Err(e)) => return Err(e).context("reading client frame"),
                }
            }
            live = rx.recv() => {
                match live {
                    Ok(ev) => {
                        for (sub_id, filters) in &subs {
                            if filters.iter().any(|f| matches_filter(&ev, f)) {
                                send(&mut write, &json!(["EVENT", sub_id, &*ev])).await?;
                            }
                        }
                    }
                    Err(broadcast::error::RecvError::Lagged(_)) => {} // dropped some live events; clients re-query
                    Err(broadcast::error::RecvError::Closed) => break,
                }
            }
        }
    }
    Ok(())
}

async fn handle_client_message(
    txt: &str,
    relay: &Arc<Mutex<Relay>>,
    tx: &Tx,
    write: &mut Sink,
    subs: &mut HashMap<String, Vec<Value>>,
) -> Result<()> {
    let msg: Value = match serde_json::from_str(txt) {
        Ok(v) => v,
        Err(_) => return send(write, &json!(["NOTICE", "invalid: not JSON"])).await,
    };
    match msg.get(0).and_then(Value::as_str) {
        Some("EVENT") => {
            let Some(ev) = msg.get(1).cloned() else {
                return send(write, &json!(["NOTICE", "invalid: EVENT missing event"])).await;
            };
            let id = ev.get("id").and_then(Value::as_str).unwrap_or("").to_string();
            let result = relay.lock().expect("relay mutex poisoned").ingest(&ev);
            send(write, &json!(["OK", id, result.accepted, result.message])).await?;
            if result.accepted {
                // Best-effort live fan-out; if no receivers, send returns Err which we ignore.
                let _ = tx.send(Arc::new(ev));
            }
        }
        Some("REQ") => {
            let Some(sub_id) = msg.get(1).and_then(Value::as_str) else {
                return send(write, &json!(["NOTICE", "invalid: REQ missing subscription id"])).await;
            };
            let filters: Vec<Value> = msg.as_array().map(|a| a.iter().skip(2).cloned().collect()).unwrap_or_default();
            // Snapshot stored matches while holding the lock, then release before awaiting sends.
            let stored: Vec<Value> = {
                let relay = relay.lock().expect("relay mutex poisoned");
                let mut out: Vec<Value> = Vec::new();
                let mut seen = std::collections::HashSet::new();
                for f in &filters {
                    for ev in relay.query(f) {
                        if let Some(id) = ev.get("id").and_then(Value::as_str) {
                            if seen.insert(id.to_string()) {
                                out.push(ev);
                            }
                        }
                    }
                }
                out
            };
            for ev in &stored {
                send(write, &json!(["EVENT", sub_id, ev])).await?;
            }
            send(write, &json!(["EOSE", sub_id])).await?;
            subs.insert(sub_id.to_string(), filters);
        }
        Some("CLOSE") => {
            if let Some(sub_id) = msg.get(1).and_then(Value::as_str) {
                subs.remove(sub_id);
                send(write, &json!(["CLOSED", sub_id, ""])).await?;
            }
        }
        _ => return send(write, &json!(["NOTICE", "invalid: unknown message type"])).await,
    }
    Ok(())
}

async fn send(write: &mut Sink, value: &Value) -> Result<()> {
    write.send(Message::Text(value.to_string().into())).await.context("sending to client")
}
