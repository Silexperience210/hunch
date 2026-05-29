//! Minimal Nostr relay client — publish (NIP-01 EVENT) and query (REQ/EVENT/EOSE).
//!
//! No subscription persistence: `query` opens a one-shot subscription, drains stored events
//! until the relay sends EOSE (or the timeout fires), then closes. Enough for the oracle
//! (publish only) and the CLI (publish markets / list markets).

use std::collections::BTreeMap;
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
                    Err(_) => continue,
                };
                if v.get(0).and_then(Value::as_str) == Some("OK")
                    && v.get(1).and_then(Value::as_str) == Some(event_id.as_str())
                {
                    return Ok::<PublishOutcome, anyhow::Error>(PublishOutcome {
                        relay: relay_owned.clone(),
                        accepted: v.get(2).and_then(Value::as_bool).unwrap_or(false),
                        message: v.get(3).and_then(Value::as_str).unwrap_or("").to_string(),
                    });
                }
                if v.get(0).and_then(Value::as_str) == Some("NOTICE") {
                    return Ok(PublishOutcome {
                        relay: relay_owned.clone(),
                        accepted: false,
                        message: format!(
                            "NOTICE: {}",
                            v.get(1).and_then(Value::as_str).unwrap_or("")
                        ),
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

/// Publishes to every relay, one result per relay. Per-relay errors are captured as `Err`.
pub async fn publish_all(
    relays: &[String],
    event: &Value,
    wait: Duration,
) -> Vec<(String, Result<PublishOutcome>)> {
    let mut out = Vec::with_capacity(relays.len());
    for relay in relays {
        out.push((relay.clone(), publish(relay, event, wait).await));
    }
    out
}

/// Queries a single relay with a NIP-01 `filter`, returning the stored events.
///
/// Opens a one-shot subscription, collects `["EVENT", sub, ev]` frames until `["EOSE", sub]`
/// or `wait` elapses, then sends `["CLOSE", sub]`. The timeout is a soft stop: relays that
/// never send EOSE still return whatever arrived before `wait`.
pub async fn query(relay: &str, filter: Value, wait: Duration) -> Result<Vec<Value>> {
    let (mut ws, _resp) = tokio_tungstenite::connect_async(relay)
        .await
        .with_context(|| format!("connecting to relay {relay}"))?;

    let sub_id = "hunch-q";
    ws.send(Message::Text(
        json!(["REQ", sub_id, filter]).to_string().into(),
    ))
    .await
    .with_context(|| format!("sending REQ to {relay}"))?;

    let mut events = Vec::new();
    let _ = timeout(wait, async {
        while let Some(frame) = ws.next().await {
            let msg = match frame {
                Ok(m) => m,
                Err(_) => break,
            };
            if let Message::Text(txt) = msg {
                let v: Value = match serde_json::from_str(txt.as_str()) {
                    Ok(v) => v,
                    Err(_) => continue,
                };
                match v.get(0).and_then(Value::as_str) {
                    Some("EVENT") if v.get(1).and_then(Value::as_str) == Some(sub_id) => {
                        if let Some(ev) = v.get(2) {
                            events.push(ev.clone());
                        }
                    }
                    Some("EOSE") if v.get(1).and_then(Value::as_str) == Some(sub_id) => break,
                    _ => {}
                }
            }
        }
    })
    .await;

    let _ = ws
        .send(Message::Text(json!(["CLOSE", sub_id]).to_string().into()))
        .await;
    let _ = ws.close(None).await;
    Ok(events)
}

/// Queries every relay and merges results, de-duplicating by event `id` (last write wins).
pub async fn query_all(relays: &[String], filter: Value, wait: Duration) -> Vec<Value> {
    let mut by_id: BTreeMap<String, Value> = BTreeMap::new();
    for relay in relays {
        if let Ok(events) = query(relay, filter.clone(), wait).await {
            for ev in events {
                if let Some(id) = ev.get("id").and_then(Value::as_str) {
                    by_id.insert(id.to_string(), ev);
                }
            }
        }
    }
    by_id.into_values().collect()
}
