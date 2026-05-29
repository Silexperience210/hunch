//! Reputation events — HIP-1 kind 30891 + HIP-5 aggregation rules.
//!
//! HIP-5 specifies the event format; aggregation algorithms live in `apps/hunch-web`
//! (frontend) and `hunch-mint`/`hunch-oracle` for backend reputation surfaces. The
//! `score` field is constrained to [-100, +100] per HIP-5 §Score Semantics.

use serde::{Deserialize, Serialize};

use crate::error::ProtocolError;
use crate::event_kinds::KIND_REPUTATION;
use crate::market::TagTuple;

/// Reputation subject scope per HIP-5 §Scope-Specific Conventions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReputationScope {
    Oracle,
    Mint,
    MarketCreator,
    Bettor,
}

impl ReputationScope {
    pub fn as_str(&self) -> &'static str {
        match self {
            ReputationScope::Oracle => "oracle",
            ReputationScope::Mint => "mint",
            ReputationScope::MarketCreator => "market_creator",
            ReputationScope::Bettor => "bettor",
        }
    }
}

impl std::str::FromStr for ReputationScope {
    type Err = ProtocolError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "oracle" => Ok(ReputationScope::Oracle),
            "mint" => Ok(ReputationScope::Mint),
            "market_creator" => Ok(ReputationScope::MarketCreator),
            "bettor" => Ok(ReputationScope::Bettor),
            other => Err(ProtocolError::MalformedTag {
                tag: "scope",
                detail: format!("unknown scope {other}"),
            }),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Reputation {
    /// Reputation event identifier (`d` tag).
    pub d: String,
    /// Target pubkey being scored (hex, 32 bytes).
    pub target_pubkey: String,
    pub scope: ReputationScope,
    /// Score in range [-100, +100] (HIP-5 §Score Semantics).
    pub score: i16,
    /// Optional attester-claimed stake-in-the-truth (sat amount; 0 = no bond).
    pub weight: Option<u64>,
    /// Optional evidence URL (verifiable doc, archived page, Nostr event ID).
    pub evidence: Option<String>,
    /// Optional market identifier if the attestation is scoped to a single market.
    pub market: Option<String>,
    /// Optional methodology hint.
    pub method: Option<String>,
    /// Free-form JSON body from the source event content.
    pub body: String,
}

impl Reputation {
    pub const KIND: u32 = KIND_REPUTATION;

    pub fn from_event(kind: u32, tags: &[TagTuple], content: &str) -> Result<Self, ProtocolError> {
        if kind != Self::KIND {
            return Err(ProtocolError::KindMismatch {
                expected: Self::KIND,
                actual: kind,
            });
        }
        let d = required(tags, "d")?.to_string();
        let target_pubkey = required(tags, "p")?.to_string();
        if hex::decode(&target_pubkey).map(|b| b.len()) != Ok(32) {
            return Err(ProtocolError::InvalidPubkey(target_pubkey));
        }
        let scope = required(tags, "scope")?.parse()?;
        let score: i16 = required(tags, "score")?.parse().map_err(|e| ProtocolError::MalformedTag {
            tag: "score",
            detail: format!("i16 parse: {e}"),
        })?;
        if !(-100..=100).contains(&score) {
            return Err(ProtocolError::MalformedTag {
                tag: "score",
                detail: "score must be in [-100, +100] per HIP-5".into(),
            });
        }
        let weight = optional(tags, "weight").map(|s| s.parse::<u64>()).transpose().map_err(|e| {
            ProtocolError::MalformedTag {
                tag: "weight",
                detail: format!("u64 parse: {e}"),
            }
        })?;
        let evidence = optional(tags, "evidence").map(str::to_string);
        let market = optional(tags, "market").map(str::to_string);
        let method = optional(tags, "method").map(str::to_string);

        Ok(Reputation {
            d,
            target_pubkey,
            scope,
            score,
            weight,
            evidence,
            market,
            method,
            body: content.to_string(),
        })
    }

    pub fn to_event_parts(&self) -> (Vec<TagTuple>, String) {
        let mut tags = vec![
            vec!["d".into(), self.d.clone()],
            vec!["p".into(), self.target_pubkey.clone()],
            vec!["scope".into(), self.scope.as_str().into()],
            vec!["score".into(), self.score.to_string()],
        ];
        if let Some(weight) = self.weight {
            tags.push(vec!["weight".into(), weight.to_string()]);
        }
        if let Some(evidence) = &self.evidence {
            tags.push(vec!["evidence".into(), evidence.clone()]);
        }
        if let Some(market) = &self.market {
            tags.push(vec!["market".into(), market.clone()]);
        }
        if let Some(method) = &self.method {
            tags.push(vec!["method".into(), method.clone()]);
        }
        (tags, self.body.clone())
    }
}

fn required<'a>(tags: &'a [TagTuple], key: &'static str) -> Result<&'a str, ProtocolError> {
    tags.iter()
        .find(|t| t.first().map(|k| k == key).unwrap_or(false))
        .and_then(|t| t.get(1).map(String::as_str))
        .ok_or(ProtocolError::MissingTag(key))
}

fn optional<'a>(tags: &'a [TagTuple], key: &'static str) -> Option<&'a str> {
    tags.iter()
        .find(|t| t.first().map(|k| k == key).unwrap_or(false))
        .and_then(|t| t.get(1).map(String::as_str))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_roundtrip_and_score_bounds() {
        let tags = vec![
            vec!["d".into(), "rep-001".into()],
            vec!["p".into(), "cc".repeat(32)],
            vec!["scope".into(), "oracle".into()],
            vec!["score".into(), "75".into()],
            vec!["weight".into(), "100000".into()],
        ];
        let r = Reputation::from_event(KIND_REPUTATION, &tags, "body").unwrap();
        assert_eq!(r.scope, ReputationScope::Oracle);
        assert_eq!(r.score, 75);
        assert_eq!(r.weight, Some(100_000));

        let (out_tags, out_content) = r.to_event_parts();
        let r2 = Reputation::from_event(KIND_REPUTATION, &out_tags, &out_content).unwrap();
        assert_eq!(r, r2);
    }

    #[test]
    fn reject_score_out_of_range() {
        let tags = vec![
            vec!["d".into(), "rep-002".into()],
            vec!["p".into(), "dd".repeat(32)],
            vec!["scope".into(), "mint".into()],
            vec!["score".into(), "150".into()], // out of bounds
        ];
        let err = Reputation::from_event(KIND_REPUTATION, &tags, "").unwrap_err();
        assert!(matches!(err, ProtocolError::MalformedTag { tag: "score", .. }));
    }

    #[test]
    fn reject_unknown_scope() {
        let tags = vec![
            vec!["d".into(), "rep-003".into()],
            vec!["p".into(), "ee".repeat(32)],
            vec!["scope".into(), "miner".into()],
            vec!["score".into(), "10".into()],
        ];
        assert!(Reputation::from_event(KIND_REPUTATION, &tags, "").is_err());
    }
}
