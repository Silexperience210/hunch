//! Relay engine — validation, storage, and filter matching. Pure and synchronous so it can be
//! unit-tested without a network. The async server in `main.rs` wraps this in shared state.
//!
//! What makes this a *Hunch* relay (not just any Nostr relay): every event is signature-checked
//! (relays must not serve forged events), and Hunch-reserved kinds are validated against
//! `hunch-protocol` — a malformed market/order/attestation is rejected, not stored.

use std::collections::{BTreeMap, HashMap};

use hunch_nostr::{event_tags, verify_event};
use hunch_protocol::event_kinds::{
    KIND_MARKET, KIND_ORACLE_ANNOUNCE, KIND_ORACLE_ATTESTATION, KIND_ORDER,
};
use hunch_protocol::market::Market;
use hunch_protocol::oracle::{OracleAnnounce, OracleAttestation};
use hunch_protocol::order::Order;
use serde_json::Value;

/// Outcome of ingesting an event, mapped by the server to `["OK", id, accepted, message]`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IngestResult {
    pub accepted: bool,
    pub message: String,
}

impl IngestResult {
    fn ok(msg: &str) -> Self {
        IngestResult {
            accepted: true,
            message: msg.into(),
        }
    }
    fn reject(msg: String) -> Self {
        IngestResult {
            accepted: false,
            message: msg,
        }
    }
}

#[derive(Default)]
pub struct Relay {
    /// All stored events, keyed by event id.
    by_id: BTreeMap<String, Value>,
    /// Replaceable index: (pubkey, kind, d_tag) -> current event id.
    replaceable: HashMap<(String, u32, String), String>,
}

impl Relay {
    pub fn new() -> Self {
        Self::default()
    }

    /// Number of stored events.
    pub fn len(&self) -> usize {
        self.by_id.len()
    }

    pub fn is_empty(&self) -> bool {
        self.by_id.is_empty()
    }

    /// Validates and (if appropriate) stores an event. Returns whether to ACK it and the message.
    ///
    /// Order of checks: signature → Hunch-kind protocol validation → replaceable/ephemeral
    /// storage rules.
    pub fn ingest(&mut self, ev: &Value) -> IngestResult {
        if !verify_event(ev) {
            return IngestResult::reject("invalid: signature or id verification failed".into());
        }
        let (Some(id), Some(pubkey), Some(kind), Some(created_at)) = (
            ev.get("id").and_then(Value::as_str),
            ev.get("pubkey").and_then(Value::as_str),
            ev.get("kind").and_then(Value::as_u64).map(|k| k as u32),
            ev.get("created_at").and_then(Value::as_i64),
        ) else {
            return IngestResult::reject("invalid: missing id/pubkey/kind/created_at".into());
        };

        if let Err(reason) = validate_hunch_kind(kind, ev) {
            return IngestResult::reject(format!("invalid: {reason}"));
        }

        // Ephemeral (20000..30000): never stored, but a valid ephemeral event is ACKed (the
        // server still broadcasts it to live subscriptions).
        if (20000..30000).contains(&kind) {
            return IngestResult::ok("ephemeral: not stored");
        }

        match replace_key(kind, pubkey, ev) {
            Some(key) => {
                if let Some(old_id) = self.replaceable.get(&key) {
                    let old_created = self
                        .by_id
                        .get(old_id)
                        .and_then(|e| e.get("created_at"))
                        .and_then(Value::as_i64)
                        .unwrap_or(i64::MIN);
                    if old_created >= created_at {
                        return IngestResult::ok("have newer or equal replaceable event");
                    }
                    let old_id = old_id.clone();
                    self.by_id.remove(&old_id);
                }
                self.by_id.insert(id.to_string(), ev.clone());
                self.replaceable.insert(key, id.to_string());
                IngestResult::ok("")
            }
            None => {
                if self.by_id.contains_key(id) {
                    return IngestResult::ok("duplicate: already have this event");
                }
                self.by_id.insert(id.to_string(), ev.clone());
                IngestResult::ok("")
            }
        }
    }

    /// Returns stored events matching `filter`, newest first, honoring `limit`.
    pub fn query(&self, filter: &Value) -> Vec<Value> {
        let mut matched: Vec<&Value> = self
            .by_id
            .values()
            .filter(|ev| matches_filter(ev, filter))
            .collect();
        matched.sort_by_key(|ev| {
            std::cmp::Reverse(ev.get("created_at").and_then(Value::as_i64).unwrap_or(0))
        });
        if let Some(limit) = filter.get("limit").and_then(Value::as_u64) {
            matched.truncate(limit as usize);
        }
        matched.into_iter().cloned().collect()
    }
}

/// Computes the replaceable storage key, or `None` for regular (stored-by-id) events.
fn replace_key(kind: u32, pubkey: &str, ev: &Value) -> Option<(String, u32, String)> {
    if (30000..40000).contains(&kind) {
        // Parameterized-replaceable: keyed by the `d` tag (default empty).
        let d = first_tag_value(ev, "d").unwrap_or_default();
        Some((pubkey.to_string(), kind, d))
    } else if (10000..20000).contains(&kind) || kind == 0 || kind == 3 {
        Some((pubkey.to_string(), kind, String::new()))
    } else {
        None
    }
}

/// Validates Hunch-reserved kinds against the protocol. Non-Hunch kinds pass (signature already
/// checked by the caller). Returns `Err(reason)` if a Hunch event is malformed.
fn validate_hunch_kind(kind: u32, ev: &Value) -> Result<(), String> {
    let tags = event_tags(ev);
    let content = ev.get("content").and_then(Value::as_str).unwrap_or("");
    let result = match kind {
        KIND_MARKET => Market::from_event(kind, &tags, content).map(|_| ()),
        KIND_ORDER => Order::from_event(kind, &tags).map(|_| ()),
        KIND_ORACLE_ANNOUNCE => OracleAnnounce::from_event(kind, &tags, content).map(|_| ()),
        KIND_ORACLE_ATTESTATION => OracleAttestation::from_event(kind, &tags, content).map(|_| ()),
        _ => return Ok(()),
    };
    result.map_err(|e| e.to_string())
}

fn first_tag_value(ev: &Value, name: &str) -> Option<String> {
    event_tags(ev)
        .into_iter()
        .find(|t| t.first().map(|k| k == name).unwrap_or(false))
        .and_then(|t| t.get(1).cloned())
}

/// NIP-01 filter matching: ids, authors, kinds, since, until, single-letter `#x` tag filters, limit.
/// Public so the server can match live events against active subscriptions.
pub fn matches_filter(ev: &Value, filter: &Value) -> bool {
    let get_arr = |key: &str| filter.get(key).and_then(Value::as_array);

    if let Some(ids) = get_arr("ids") {
        let id = ev.get("id").and_then(Value::as_str).unwrap_or("");
        if !ids.iter().any(|v| v.as_str() == Some(id)) {
            return false;
        }
    }
    if let Some(authors) = get_arr("authors") {
        let pk = ev.get("pubkey").and_then(Value::as_str).unwrap_or("");
        if !authors.iter().any(|v| v.as_str() == Some(pk)) {
            return false;
        }
    }
    if let Some(kinds) = get_arr("kinds") {
        let k = ev.get("kind").and_then(Value::as_u64).unwrap_or(u64::MAX);
        if !kinds.iter().any(|v| v.as_u64() == Some(k)) {
            return false;
        }
    }
    if let Some(since) = filter.get("since").and_then(Value::as_i64) {
        if ev.get("created_at").and_then(Value::as_i64).unwrap_or(0) < since {
            return false;
        }
    }
    if let Some(until) = filter.get("until").and_then(Value::as_i64) {
        if ev.get("created_at").and_then(Value::as_i64).unwrap_or(0) > until {
            return false;
        }
    }
    // Tag filters: any key shaped like "#x" (single-letter) constrains tag values.
    if let Some(obj) = filter.as_object() {
        let tags = event_tags(ev);
        for (key, vals) in obj {
            let Some(letter) = key.strip_prefix('#') else {
                continue;
            };
            let Some(wanted) = vals.as_array() else {
                continue;
            };
            let present = tags.iter().any(|t| {
                t.first().map(|k| k == letter).unwrap_or(false)
                    && t.get(1)
                        .map(|v| wanted.iter().any(|w| w.as_str() == Some(v.as_str())))
                        .unwrap_or(false)
            });
            if !present {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use hunch_nostr::build_signed_event;
    use hunch_protocol::event_kinds::KIND_ORDER;
    use secp256k1::{Keypair, Secp256k1, SecretKey};
    use serde_json::json;

    fn kp(seed: u8) -> Keypair {
        let sk = SecretKey::from_slice(&[seed.max(1); 32]).unwrap();
        Keypair::from_secret_key(&Secp256k1::new(), &sk)
    }

    fn signed(
        kp: &Keypair,
        kind: u32,
        tags: Vec<Vec<String>>,
        content: &str,
        created_at: i64,
    ) -> Value {
        build_signed_event(
            &Secp256k1::new(),
            kp,
            kind,
            tags,
            content.into(),
            created_at,
        )
    }

    fn valid_order_tags(market: &str) -> Vec<Vec<String>> {
        vec![
            vec!["market".into(), market.into()],
            vec!["side".into(), "YES".into()],
            vec!["amount".into(), "10000".into()],
            vec!["price".into(), "73".into()],
            vec!["kind".into(), "bid".into()],
            vec!["expires".into(), "1900000000".into()],
            vec!["d".into(), market.into()],
        ]
    }

    #[test]
    fn rejects_forged_signature() {
        let mut relay = Relay::new();
        let mut ev = signed(&kp(1), 1, vec![], "x", 1);
        ev["sig"] = json!("00".repeat(64));
        assert!(!relay.ingest(&ev).accepted);
        assert!(relay.is_empty());
    }

    #[test]
    fn rejects_malformed_hunch_order() {
        let mut relay = Relay::new();
        // kind 38888 with no order tags → protocol rejects.
        let ev = signed(
            &kp(1),
            KIND_ORDER,
            vec![vec!["d".into(), "x".into()]],
            "",
            1,
        );
        let r = relay.ingest(&ev);
        assert!(!r.accepted, "{r:?}");
        assert!(r.message.contains("invalid"));
    }

    #[test]
    fn accepts_and_stores_valid_order() {
        let mut relay = Relay::new();
        let market = format!("{}:30888:m", "aa".repeat(32));
        let ev = signed(&kp(2), KIND_ORDER, valid_order_tags(&market), "", 100);
        assert!(relay.ingest(&ev).accepted);
        assert_eq!(relay.len(), 1);
    }

    #[test]
    fn parameterized_replaceable_keeps_newest_per_d() {
        let mut relay = Relay::new();
        let market = format!("{}:30888:m", "aa".repeat(32));
        let old = signed(&kp(3), KIND_ORDER, valid_order_tags(&market), "", 100);
        let new = signed(&kp(3), KIND_ORDER, valid_order_tags(&market), "", 200);
        assert!(relay.ingest(&old).accepted);
        assert!(relay.ingest(&new).accepted);
        assert_eq!(
            relay.len(),
            1,
            "newer order should replace older for same (pubkey,kind,d)"
        );
        // An older event arriving late does not displace the newer one.
        let older = signed(&kp(3), KIND_ORDER, valid_order_tags(&market), "", 50);
        let r = relay.ingest(&older);
        assert!(r.accepted);
        assert_eq!(relay.len(), 1);
    }

    #[test]
    fn query_by_kind_and_d_tag_with_limit() {
        let mut relay = Relay::new();
        let m1 = format!("{}:30888:m1", "aa".repeat(32));
        let m2 = format!("{}:30888:m2", "aa".repeat(32));
        relay.ingest(&signed(&kp(4), KIND_ORDER, valid_order_tags(&m1), "", 100));
        relay.ingest(&signed(&kp(5), KIND_ORDER, valid_order_tags(&m2), "", 200));

        let all = relay.query(&json!({ "kinds": [KIND_ORDER] }));
        assert_eq!(all.len(), 2);
        // newest first
        assert_eq!(all[0]["created_at"].as_i64(), Some(200));

        let only_m1 = relay.query(&json!({ "kinds": [KIND_ORDER], "#d": [m1] }));
        assert_eq!(only_m1.len(), 1);

        let limited = relay.query(&json!({ "kinds": [KIND_ORDER], "limit": 1 }));
        assert_eq!(limited.len(), 1);
        assert_eq!(limited[0]["created_at"].as_i64(), Some(200));
    }

    #[test]
    fn non_hunch_kinds_pass_when_signed() {
        let mut relay = Relay::new();
        let ev = signed(&kp(6), 1, vec![], "gm", 1);
        assert!(relay.ingest(&ev).accepted);
        assert_eq!(relay.len(), 1);
    }
}
