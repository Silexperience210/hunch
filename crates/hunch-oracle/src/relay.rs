//! Minimal Nostr relay publisher.
//!
//! Connects to a relay over WebSocket, sends `["EVENT", <event>]` (NIP-01), and waits for the
//! relay's `["OK", <id>, <accepted>, <message>]` reply. No subscription/query support — the
//! oracle only ever publishes. Querying markets is the CLI/frontend's job.

use std::time::Duration;

use anyhow::{Context, Result};
use futures_util::{SinkExt, StreamExt};
use serde_json::{json, Value};
use tokio::time::timeout;
use tokio_tungstenite::tungstenite::Message;

/// Result of publishing one event to one relay.
#[derive(Debug, Clone)]
pub struct PublishOutcome {
    pub relay: String,
    pub accepted: bool,
    pub message: String,
}

/// Publishes `event` to a single relay and waits up to `wait` for the `OK` reply.
pub async fn publish(relay: &str, event: &Value, wait: Duration) -> Result<PublishOutcome> {
    let event_id = event
        .get("id")
        .and_then(Value::as_str)
        .context("event missing id")?
        .to_string();

    let (mut ws, _resp) = tokio_tungstenite::connect_async(relay)
        .await
        .with_context(|| format!("connecting to relay {relay}"))?;

    let payload = json!(["EVENT", event]).to_string();
    ws.send(Message::Text(payload.into()))
        .await
        .with_context(|| format!("sending EVENT to {relay}"))?;

    let relay_owned = relay.to_string();
    let waited = timeout(wait, async {
        while let Some(frame) = ws.next().await {
            let msg = frame.context("reading relay frame")?;
            if let Message::Text(txt) = msg {
                let v: Value = match serde_json::from_str(txt.as_str()) {
                    Ok(v) => v,
                    Err(_) => continue, // skip non-JSON noise
                };
                // ["OK", <event_id>, <bool>, <message>]
                if v.get(0).and_then(Value::as_str) == Some("OK")
                    && v.get(1).and_then(Value::as_str) == Some(event_id.as_str())
                {
                    return Ok::<PublishOutcome, anyhow::Error>(PublishOutcome {
                        relay: relay_owned.clone(),
                        accepted: v.get(2).and_then(Value::as_bool).unwrap_or(false),
                        message: v.get(3).and_then(Value::as_str).unwrap_or("").to_string(),
                    });
                }
                // ["NOTICE", <message>] — relay-level rejection without an OK.
                if v.get(0).and_then(Value::as_str) == Some("NOTICE") {
                    return Ok(PublishOutcome {
                        relay: relay_owned.clone(),
                        accepted: false,
                        message: format!("NOTICE: {}", v.get(1).and_then(Value::as_str).unwrap_or("")),
                    });
                }
            }
        }
        anyhow::bail!("relay {relay_owned} closed the connection before sending OK")
    })
    .await;

    let _ = ws.close(None).await;

    match waited {
        Ok(result) => result,
        Err(_) => anyhow::bail!("timed out after {:?} waiting for OK from {relay}", wait),
    }
}

/// Publishes to every relay, returning one result per relay. Network errors are captured
/// per-relay as `Err` so one dead relay never blocks the others.
pub async fn publish_all(relays: &[String], event: &Value, wait: Duration) -> Vec<(String, Result<PublishOutcome>)> {
    let mut out = Vec::with_capacity(relays.len());
    for relay in relays {
        let result = publish(relay, event, wait).await;
        out.push((relay.clone(), result));
    }
    out
}
