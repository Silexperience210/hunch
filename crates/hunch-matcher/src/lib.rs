//! Hunch Tier-2 P2P order matcher.
//!
//! Reads signed kind:38888 orders for a market and proposes compatible matches. There is no
//! central matching engine and no settlement here — the matcher only *suggests* pairs; parties
//! execute through the mint (HIP-3). The [`engine`] module is the pure, testable core.

pub mod engine;

pub use engine::{match_book, BookOrder, Match};

use hunch_nostr::{event_tags, verify_event};
use hunch_protocol::order::Order;
use serde_json::Value;

/// Parses signed kind:38888 events into [`BookOrder`]s for `market`.
///
/// Drops events that fail signature verification (relays are untrusted), fail to parse as an
/// order, or reference a different market.
pub fn book_orders_from_events(events: &[Value], market: &str) -> Vec<BookOrder> {
    events
        .iter()
        .filter(|ev| verify_event(ev))
        .filter_map(|ev| {
            let author = ev.get("pubkey").and_then(Value::as_str)?.to_string();
            let event_id = ev.get("id").and_then(Value::as_str)?.to_string();
            let kind = ev.get("kind").and_then(Value::as_u64)? as u32;
            let order = Order::from_event(kind, &event_tags(ev)).ok()?;
            if order.market != market {
                return None;
            }
            Some(BookOrder {
                author,
                event_id,
                order,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use hunch_nostr::build_signed_event;
    use hunch_protocol::event_kinds::KIND_ORDER;
    use secp256k1::{Keypair, Secp256k1, SecretKey};
    use serde_json::json;

    fn order_event(seed: u8, market: &str, side: &str, kind: &str, price: u64) -> Value {
        let sk = SecretKey::from_slice(&[seed.max(1); 32]).unwrap();
        let kp = Keypair::from_secret_key(&Secp256k1::new(), &sk);
        let tags = vec![
            vec!["market".into(), market.into()],
            vec!["side".into(), side.into()],
            vec!["amount".into(), "1000".into()],
            vec!["price".into(), price.to_string()],
            vec!["kind".into(), kind.into()],
            vec!["expires".into(), "1900000000".into()],
            vec!["d".into(), market.into()],
        ];
        build_signed_event(&Secp256k1::new(), &kp, KIND_ORDER, tags, String::new(), 1)
    }

    #[test]
    fn parses_valid_orders_drops_forged_and_other_markets() {
        let market = format!("{}:30888:m", "aa".repeat(32));
        let other = format!("{}:30888:other", "aa".repeat(32));
        let good = order_event(1, &market, "YES", "bid", 70);
        let other_market = order_event(2, &other, "YES", "bid", 70);
        let mut forged = order_event(3, &market, "NO", "ask", 30);
        forged["sig"] = json!("00".repeat(64));

        let parsed = book_orders_from_events(&[good, other_market, forged], &market);
        assert_eq!(
            parsed.len(),
            1,
            "only the genuine order for this market survives"
        );
        assert_eq!(parsed[0].order.price, 70);
    }
}
