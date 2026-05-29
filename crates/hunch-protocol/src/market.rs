//! Market metadata — HIP-1 kind 30888 (parameterized replaceable).
//!
//! A market event references an oracle pubkey, an outcome enumeration, expiry + refund
//! timeouts, the mint backing the DLC, and the funding output (txid:vout) on Bitcoin.
//! The event's `content` field carries the human-readable question + resolution criteria;
//! the tags carry the protocol-readable metadata.

use serde::{Deserialize, Serialize};

use crate::error::ProtocolError;
use crate::event_kinds::KIND_MARKET;
use crate::outcome::Outcome;

/// On-chain DLC funding output reference: `<txid_hex>:<vout>`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DlcOutpoint {
    pub txid: String,
    pub vout: u32,
}

impl std::fmt::Display for DlcOutpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.txid, self.vout)
    }
}

impl std::str::FromStr for DlcOutpoint {
    type Err = ProtocolError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (txid, vout) = s.split_once(':').ok_or_else(|| ProtocolError::MalformedTag {
            tag: "dlc_contract",
            detail: format!("expected `<txid>:<vout>`, got {s:?}"),
        })?;
        if txid.len() != 64 {
            return Err(ProtocolError::MalformedTag {
                tag: "dlc_contract",
                detail: format!("txid must be 64 hex chars, got {}", txid.len()),
            });
        }
        // Validate hex.
        hex::decode(txid).map_err(|e| ProtocolError::MalformedTag {
            tag: "dlc_contract",
            detail: format!("txid hex decode: {e}"),
        })?;
        let vout: u32 = vout.parse().map_err(|e| ProtocolError::MalformedTag {
            tag: "dlc_contract",
            detail: format!("vout parse: {e}"),
        })?;
        Ok(DlcOutpoint { txid: txid.to_string(), vout })
    }
}

/// Market metadata extracted from the tags of a kind:30888 event (HIP-1).
///
/// The originating Nostr event is signed by `creator_pubkey` and carries this
/// data in its tags + content body. `MarketContent` is the parsed content body.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Market {
    /// Market identifier (`d` tag, max 64 chars).
    pub d: String,
    /// Oracle public key (hex, 32 bytes).
    pub oracle_pubkey: String,
    /// Outcome enumeration — MUST be exactly [Yes, No, Invalid] per HIP-2.
    pub outcomes: Vec<Outcome>,
    /// UNIX timestamp when betting closes.
    pub expiry: u64,
    /// UNIX timestamp after which the refund branch is claimable.
    pub refund_timeout: u64,
    /// Mint identifier (URL or pubkey hex).
    pub mint: String,
    /// On-chain DLC funding output reference.
    pub dlc_contract: DlcOutpoint,
    /// Optional high-level category.
    pub category: Option<String>,
    /// Optional preview image URL.
    pub image: Option<String>,
    /// Free-form topic tags (multiple allowed; `t` tags in source event).
    pub topics: Vec<String>,
    /// Parsed content body.
    pub content: MarketContent,
}

/// Human-readable market description (the `content` field of a kind:30888 event).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketContent {
    pub question: String,
    pub resolution_criteria: String,
    #[serde(default)]
    pub sources: Vec<String>,
    pub rules_version: String,
}

/// Tag tuple. Nostr tags are arrays of strings; in `nostr-sdk` this is `Vec<String>`.
/// We keep a minimal local representation so this crate has zero `nostr-sdk` dependency.
pub type TagTuple = Vec<String>;

impl Market {
    /// Expected kind for a Market event.
    pub const KIND: u32 = KIND_MARKET;

    /// Parse a Market from a kind:30888 event's `tags` array + `content` string.
    ///
    /// `kind` must equal [`Market::KIND`] (30888). The content body is parsed as JSON
    /// into [`MarketContent`].
    pub fn from_event(kind: u32, tags: &[TagTuple], content: &str) -> Result<Self, ProtocolError> {
        if kind != Self::KIND {
            return Err(ProtocolError::KindMismatch {
                expected: Self::KIND,
                actual: kind,
            });
        }
        let content: MarketContent = serde_json::from_str(content)?;

        let d = required_tag(tags, "d")?.to_string();
        let oracle_pubkey = required_tag(tags, "oracle")?.to_string();
        if hex::decode(&oracle_pubkey).map(|b| b.len()) != Ok(32) {
            return Err(ProtocolError::InvalidPubkey(oracle_pubkey));
        }
        let outcomes_raw = required_tag(tags, "outcomes")?.to_string();
        let outcomes: Vec<Outcome> = outcomes_raw
            .split(',')
            .map(str::trim)
            .map(|s| s.parse::<Outcome>())
            .collect::<Result<Vec<_>, _>>()?;
        // HIP-2 mandates exactly the three outcomes in the canonical order.
        if outcomes != Outcome::ALL.to_vec() {
            return Err(ProtocolError::MalformedTag {
                tag: "outcomes",
                detail: format!(
                    "HIP-2 mandates YES,NO,INVALID; got {outcomes_raw:?}"
                ),
            });
        }
        let expiry: u64 = required_tag(tags, "expiry")?.parse().map_err(|e| {
            ProtocolError::MalformedTag {
                tag: "expiry",
                detail: format!("u64 parse: {e}"),
            }
        })?;
        let refund_timeout: u64 = required_tag(tags, "refund_timeout")?.parse().map_err(|e| {
            ProtocolError::MalformedTag {
                tag: "refund_timeout",
                detail: format!("u64 parse: {e}"),
            }
        })?;
        // HIP-2 §Refund Branch: refund_timeout MUST be at least 7 days after expiry.
        const SEVEN_DAYS: u64 = 7 * 24 * 3600;
        if refund_timeout < expiry.saturating_add(SEVEN_DAYS) {
            return Err(ProtocolError::MalformedTag {
                tag: "refund_timeout",
                detail: "refund_timeout must be at least 7 days after expiry (HIP-2)".into(),
            });
        }

        let mint = required_tag(tags, "mint")?.to_string();
        let dlc_contract: DlcOutpoint = required_tag(tags, "dlc_contract")?.parse()?;

        let category = optional_tag(tags, "category").map(str::to_string);
        let image = optional_tag(tags, "image").map(str::to_string);
        let topics: Vec<String> = tags
            .iter()
            .filter(|t| t.first().map(|k| k == "t").unwrap_or(false))
            .filter_map(|t| t.get(1).cloned())
            .collect();

        Ok(Market {
            d,
            oracle_pubkey,
            outcomes,
            expiry,
            refund_timeout,
            mint,
            dlc_contract,
            category,
            image,
            topics,
            content,
        })
    }

    /// Serialize this market into `(tags, content)` ready for embedding in a Nostr event.
    pub fn to_event_parts(&self) -> Result<(Vec<TagTuple>, String), ProtocolError> {
        let mut tags = vec![
            vec!["d".into(), self.d.clone()],
            vec!["oracle".into(), self.oracle_pubkey.clone()],
            vec![
                "outcomes".into(),
                self.outcomes
                    .iter()
                    .map(Outcome::as_str)
                    .collect::<Vec<_>>()
                    .join(","),
            ],
            vec!["expiry".into(), self.expiry.to_string()],
            vec!["refund_timeout".into(), self.refund_timeout.to_string()],
            vec!["mint".into(), self.mint.clone()],
            vec!["dlc_contract".into(), self.dlc_contract.to_string()],
        ];
        if let Some(category) = &self.category {
            tags.push(vec!["category".into(), category.clone()]);
        }
        if let Some(image) = &self.image {
            tags.push(vec!["image".into(), image.clone()]);
        }
        for topic in &self.topics {
            tags.push(vec!["t".into(), topic.clone()]);
        }
        let content = serde_json::to_string(&self.content)?;
        Ok((tags, content))
    }
}

fn required_tag<'a>(tags: &'a [TagTuple], key: &'static str) -> Result<&'a str, ProtocolError> {
    tags.iter()
        .find(|t| t.first().map(|k| k == key).unwrap_or(false))
        .and_then(|t| t.get(1).map(String::as_str))
        .ok_or(ProtocolError::MissingTag(key))
}

fn optional_tag<'a>(tags: &'a [TagTuple], key: &'static str) -> Option<&'a str> {
    tags.iter()
        .find(|t| t.first().map(|k| k == key).unwrap_or(false))
        .and_then(|t| t.get(1).map(String::as_str))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_tags() -> Vec<TagTuple> {
        vec![
            vec!["d".into(), "btc-100k-eoy-2026".into()],
            vec![
                "oracle".into(),
                "aa".repeat(32),
            ],
            vec!["outcomes".into(), "YES,NO,INVALID".into()],
            vec!["expiry".into(), "1767139200".into()],
            vec!["refund_timeout".into(), "1769817600".into()],
            vec!["mint".into(), "https://mint.hunch.markets".into()],
            vec![
                "dlc_contract".into(),
                format!("{}:0", "bb".repeat(32)),
            ],
            vec!["category".into(), "crypto".into()],
            vec!["t".into(), "bitcoin".into()],
            vec!["t".into(), "macro".into()],
        ]
    }

    fn sample_content_json() -> String {
        serde_json::to_string(&MarketContent {
            question: "Will BTC close above $100k on 2026-12-31?".into(),
            resolution_criteria: "YES if BTC/USD spot on Coinbase Pro at 23:59 UTC >= 100000.00; NO if < 100000.00; INVALID if Coinbase Pro feed is down for >2 hours during the resolution window.".into(),
            sources: vec!["https://pro.coinbase.com/markets/BTC-USD".into()],
            rules_version: "1.0".into(),
        })
        .unwrap()
    }

    #[test]
    fn parse_valid_market_event() {
        let tags = sample_tags();
        let content = sample_content_json();
        let m = Market::from_event(KIND_MARKET, &tags, &content).unwrap();
        assert_eq!(m.d, "btc-100k-eoy-2026");
        assert_eq!(m.outcomes, vec![Outcome::Yes, Outcome::No, Outcome::Invalid]);
        assert_eq!(m.expiry, 1767139200);
        assert_eq!(m.refund_timeout, 1769817600);
        assert_eq!(m.mint, "https://mint.hunch.markets");
        assert_eq!(m.dlc_contract.vout, 0);
        assert_eq!(m.category.as_deref(), Some("crypto"));
        assert_eq!(m.topics, vec!["bitcoin", "macro"]);
        assert_eq!(m.content.rules_version, "1.0");
    }

    #[test]
    fn reject_wrong_kind() {
        let tags = sample_tags();
        let content = sample_content_json();
        let err = Market::from_event(38888, &tags, &content).unwrap_err();
        assert!(matches!(err, ProtocolError::KindMismatch { expected: 30888, actual: 38888 }));
    }

    #[test]
    fn reject_missing_d() {
        let mut tags = sample_tags();
        tags.retain(|t| t.first().map(|k| k != "d").unwrap_or(true));
        let err = Market::from_event(KIND_MARKET, &tags, &sample_content_json()).unwrap_err();
        assert!(matches!(err, ProtocolError::MissingTag("d")));
    }

    #[test]
    fn reject_outcomes_not_canonical_per_hip2() {
        // HIP-2 mandates exactly YES,NO,INVALID — variant orderings rejected.
        let mut tags = sample_tags();
        for t in tags.iter_mut() {
            if t.first().map(|k| k == "outcomes").unwrap_or(false) {
                t[1] = "YES,NO".into();
            }
        }
        let err = Market::from_event(KIND_MARKET, &tags, &sample_content_json()).unwrap_err();
        assert!(matches!(err, ProtocolError::MalformedTag { tag: "outcomes", .. }));
    }

    #[test]
    fn reject_refund_timeout_too_close_per_hip2() {
        // HIP-2 §Refund Branch: refund_timeout must be at least 7 days after expiry.
        let mut tags = sample_tags();
        for t in tags.iter_mut() {
            if t.first().map(|k| k == "refund_timeout").unwrap_or(false) {
                t[1] = "1767139300".into(); // 100 seconds after expiry — invalid
            }
        }
        let err = Market::from_event(KIND_MARKET, &tags, &sample_content_json()).unwrap_err();
        assert!(matches!(err, ProtocolError::MalformedTag { tag: "refund_timeout", .. }));
    }

    #[test]
    fn reject_oracle_not_32_bytes() {
        let mut tags = sample_tags();
        for t in tags.iter_mut() {
            if t.first().map(|k| k == "oracle").unwrap_or(false) {
                t[1] = "deadbeef".into(); // 4 bytes, not 32
            }
        }
        let err = Market::from_event(KIND_MARKET, &tags, &sample_content_json()).unwrap_err();
        assert!(matches!(err, ProtocolError::InvalidPubkey(_)));
    }

    #[test]
    fn roundtrip_event_parts() {
        let tags = sample_tags();
        let content = sample_content_json();
        let m = Market::from_event(KIND_MARKET, &tags, &content).unwrap();
        let (out_tags, out_content) = m.to_event_parts().unwrap();
        let m2 = Market::from_event(KIND_MARKET, &out_tags, &out_content).unwrap();
        assert_eq!(m, m2);
    }
}
