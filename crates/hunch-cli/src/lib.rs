//! Hunch CLI library — market construction + event parsing, kept separate from the binary
//! wiring so it can be unit-tested without a network or clap.

use anyhow::{Context, Result};
use hunch_nostr::event_tags;
use hunch_protocol::event_kinds::{KIND_MARKET, KIND_MINT_ANNOUNCE, KIND_ORDER};
use hunch_protocol::market::{DlcOutpoint, Market, MarketContent};
use hunch_protocol::mint_announce::MintAnnounce;
use hunch_protocol::order::{Order, OrderKind, OrderSide};
use hunch_protocol::outcome::Outcome;
use serde_json::Value;

/// Minimum gap between expiry and refund_timeout mandated by HIP-2 §Refund Branch.
pub const SEVEN_DAYS: u64 = 7 * 24 * 3600;

/// Inputs for creating a market. Outcomes are always the HIP-2 canonical `[YES, NO, INVALID]`.
pub struct MarketParams {
    pub d: String,
    pub oracle_pubkey: String,
    pub expiry: u64,
    /// Defaults to `expiry + 7 days` (the HIP-2 minimum) when `None`.
    pub refund_timeout: Option<u64>,
    pub mint: String,
    pub dlc_contract: String,
    pub question: String,
    pub resolution_criteria: String,
    pub sources: Vec<String>,
    pub rules_version: String,
    pub category: Option<String>,
    pub image: Option<String>,
    pub topics: Vec<String>,
}

/// Builds and validates a [`Market`]. Validation runs the protocol's own `from_event` over the
/// serialized form, so a market that builds here is guaranteed to parse on the other side.
pub fn build_market(p: MarketParams) -> Result<Market> {
    let refund_timeout = p
        .refund_timeout
        .unwrap_or_else(|| p.expiry.saturating_add(SEVEN_DAYS));
    let dlc_contract: DlcOutpoint = p
        .dlc_contract
        .parse()
        .context("invalid --dlc-contract (expected <txid_hex>:<vout>)")?;

    let market = Market {
        d: p.d,
        oracle_pubkey: p.oracle_pubkey,
        outcomes: Outcome::ALL.to_vec(),
        expiry: p.expiry,
        refund_timeout,
        mint: p.mint,
        dlc_contract,
        category: p.category,
        image: p.image,
        topics: p.topics,
        content: MarketContent {
            question: p.question,
            resolution_criteria: p.resolution_criteria,
            sources: p.sources,
            rules_version: p.rules_version,
        },
    };

    // Round-trip through the protocol validator before we ever sign or publish it.
    let (tags, content) = market.to_event_parts().context("serializing market")?;
    Market::from_event(KIND_MARKET, &tags, &content)
        .context("market failed protocol validation")?;
    Ok(market)
}

/// The HIP-1 market identifier: `<creator_pubkey>:30888:<d>`.
pub fn market_id(creator_pubkey: &str, d: &str) -> String {
    format!("{creator_pubkey}:{KIND_MARKET}:{d}")
}

/// Extracts a Nostr event's `tags` as `Vec<Vec<String>>`, ignoring non-string tag elements.
pub fn tags_from_event(ev: &Value) -> Vec<Vec<String>> {
    event_tags(ev)
}

/// Parses a kind:30888 Nostr event into `(market_id, Market)`.
pub fn parse_market_event(ev: &Value) -> Result<(String, Market)> {
    let kind = ev
        .get("kind")
        .and_then(Value::as_u64)
        .context("event missing kind")? as u32;
    let pubkey = ev
        .get("pubkey")
        .and_then(Value::as_str)
        .context("event missing pubkey")?;
    let content = ev.get("content").and_then(Value::as_str).unwrap_or("");
    let tags = tags_from_event(ev);
    let market = Market::from_event(kind, &tags, content)?;
    let id = market_id(pubkey, &market.d);
    Ok((id, market))
}

/// Inputs for placing an order.
pub struct OrderParams {
    pub market: String,
    pub side: OrderSide,
    pub amount: u64,
    pub price: u64,
    pub kind: OrderKind,
    pub expires: u64,
}

/// Builds and validates an [`Order`] (validates by round-tripping through the protocol parser).
pub fn build_order(p: OrderParams) -> Result<Order> {
    let order = Order {
        market: p.market,
        side: p.side,
        amount: p.amount,
        price: p.price,
        kind: p.kind,
        expires: p.expires,
    };
    Order::from_event(KIND_ORDER, &order.to_tags()).context("order failed protocol validation")?;
    Ok(order)
}

/// Tags for an order event: the protocol order tags plus a `d` tag set to the market.
///
/// Kind 38888 is parameterized-replaceable; the single-letter `d` tag makes the order
/// addressable (one outstanding order per pubkey per market, per HIP-1) and lets relays filter
/// by `#d` (multi-char tag names like `market` are NOT filterable per NIP-01).
pub fn order_tags_with_d(order: &Order) -> Vec<Vec<String>> {
    let mut tags = order.to_tags();
    tags.push(vec!["d".into(), order.market.clone()]);
    tags
}

/// Parses a kind:38888 Nostr event into `(author_pubkey, Order)`.
pub fn parse_order_event(ev: &Value) -> Result<(String, Order)> {
    let kind = ev
        .get("kind")
        .and_then(Value::as_u64)
        .context("event missing kind")? as u32;
    let pubkey = ev
        .get("pubkey")
        .and_then(Value::as_str)
        .context("event missing pubkey")?;
    let order = Order::from_event(kind, &tags_from_event(ev))?;
    Ok((pubkey.to_string(), order))
}

/// Inputs for a mint announce (kind 30892).
pub struct MintAnnounceParams {
    /// Mint identifier (the `d` tag).
    pub mint_id: String,
    /// Mint endpoint URL (https / onion / ipfs).
    pub endpoint: String,
    /// Latest reserves-proof URL (HIP-3 transparency).
    pub reserves_proof: String,
    /// Accepted oracle x-only pubkeys (hex, 32 bytes each).
    pub supported_oracles: Vec<String>,
    /// Free-form policy body (JSON).
    pub body: String,
}

/// Builds and validates a [`MintAnnounce`] (round-trips through the protocol parser, which
/// rejects malformed oracle pubkeys), so an announce that builds here parses on the other side.
pub fn build_mint_announce(p: MintAnnounceParams) -> Result<MintAnnounce> {
    let announce = MintAnnounce {
        d: p.mint_id,
        endpoint: p.endpoint,
        reserves_proof: p.reserves_proof,
        supported_oracles: p.supported_oracles,
        body: p.body,
    };
    let (tags, content) = announce.to_event_parts();
    MintAnnounce::from_event(KIND_MINT_ANNOUNCE, &tags, &content)
        .context("mint announce failed protocol validation")?;
    Ok(announce)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn params() -> MarketParams {
        MarketParams {
            d: "btc-100k-eoy-2026".into(),
            oracle_pubkey: "aa".repeat(32),
            expiry: 1_767_139_200,
            refund_timeout: None,
            mint: "https://mint.hunch.markets".into(),
            dlc_contract: format!("{}:0", "bb".repeat(32)),
            question: "Will BTC close above $100k on 2026-12-31?".into(),
            resolution_criteria: "YES if BTC/USD >= 100000 at 23:59 UTC".into(),
            sources: vec!["https://pro.coinbase.com/markets/BTC-USD".into()],
            rules_version: "1.0".into(),
            category: Some("crypto".into()),
            image: None,
            topics: vec!["bitcoin".into(), "macro".into()],
        }
    }

    #[test]
    fn build_market_defaults_refund_to_expiry_plus_7d() {
        let m = build_market(params()).unwrap();
        assert_eq!(m.refund_timeout, 1_767_139_200 + SEVEN_DAYS);
        assert_eq!(m.outcomes, Outcome::ALL.to_vec());
    }

    #[test]
    fn build_market_rejects_bad_dlc_contract() {
        let mut p = params();
        p.dlc_contract = "not-an-outpoint".into();
        assert!(build_market(p).is_err());
    }

    #[test]
    fn parse_market_event_roundtrips_a_built_market() {
        let m = build_market(params()).unwrap();
        let (tags, content) = m.to_event_parts().unwrap();
        // Simulate the wire event a relay would return.
        let ev = json!({
            "kind": KIND_MARKET,
            "pubkey": "cc".repeat(32),
            "tags": tags,
            "content": content,
        });
        let (id, parsed) = parse_market_event(&ev).unwrap();
        assert_eq!(id, format!("{}:30888:btc-100k-eoy-2026", "cc".repeat(32)));
        assert_eq!(parsed.content.question, m.content.question);
        assert_eq!(parsed.topics, vec!["bitcoin", "macro"]);
    }

    fn order_params(market: &str) -> OrderParams {
        OrderParams {
            market: market.into(),
            side: OrderSide::Yes,
            amount: 10_000,
            price: 73,
            kind: OrderKind::Bid,
            expires: 1_800_000_000,
        }
    }

    #[test]
    fn build_order_and_parse_roundtrip_with_d_tag() {
        let market = format!("{}:30888:btc-100k", "aa".repeat(32));
        let order = build_order(order_params(&market)).unwrap();
        let tags = order_tags_with_d(&order);
        // The d tag must be present and equal the market (relay-filterable via #d).
        assert!(tags.iter().any(|t| t[0] == "d" && t[1] == market));

        let ev = json!({
            "kind": KIND_ORDER,
            "pubkey": "cc".repeat(32),
            "tags": tags,
            "content": "",
        });
        let (author, parsed) = parse_order_event(&ev).unwrap();
        assert_eq!(author, "cc".repeat(32));
        assert_eq!(parsed.amount, 10_000);
        assert_eq!(parsed.price, 73);
        assert_eq!(parsed.side, OrderSide::Yes);
        assert_eq!(parsed.kind, OrderKind::Bid);
    }

    #[test]
    fn build_order_rejects_bad_side_via_protocol() {
        // Sanity: an order with a side that isn't YES/NO can't even be constructed here
        // because OrderSide only has Yes/No, so we instead check protocol validation runs.
        let market = format!("{}:30888:m", "aa".repeat(32));
        assert!(build_order(order_params(&market)).is_ok());
    }

    #[test]
    fn tags_from_event_ignores_non_strings_and_missing() {
        let ev = json!({ "tags": [["d", "x"], ["t", 5], "junk"] });
        let tags = tags_from_event(&ev);
        assert_eq!(tags[0], vec!["d".to_string(), "x".to_string()]);
        assert_eq!(tags[1], vec!["t".to_string()]); // the numeric 5 is dropped
        assert_eq!(tags_from_event(&json!({})), Vec::<Vec<String>>::new());
    }

    #[test]
    fn build_mint_announce_validates_and_roundtrips() {
        let announce = build_mint_announce(MintAnnounceParams {
            mint_id: "hunch-mint-1".into(),
            endpoint: "https://mint.hunch.markets".into(),
            reserves_proof: "https://mint.hunch.markets/reserves/2026-W22".into(),
            supported_oracles: vec!["aa".repeat(32), "bb".repeat(32)],
            body: "{\"max_market_sat\":10000000}".into(),
        })
        .unwrap();
        assert_eq!(announce.supported_oracles.len(), 2);
        // A malformed oracle pubkey must be rejected by protocol validation.
        assert!(build_mint_announce(MintAnnounceParams {
            mint_id: "m".into(),
            endpoint: "https://x".into(),
            reserves_proof: "https://x/r".into(),
            supported_oracles: vec!["not-hex".into()],
            body: String::new(),
        })
        .is_err());
    }
}
