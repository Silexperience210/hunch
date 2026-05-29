//! Order events — HIP-1 kind 38888 (ephemeral).
//!
//! Orders are posted to Tier 2 P2P matching relays and not stored beyond relay TTL.
//! The market reference uses the parameterized-replaceable format
//! `<creator_pubkey>:30888:<d>`.

use serde::{Deserialize, Serialize};

use crate::error::ProtocolError;
use crate::event_kinds::KIND_ORDER;
use crate::market::TagTuple;
use crate::outcome::Outcome;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum OrderSide {
    Yes,
    No,
}

impl OrderSide {
    pub fn as_str(&self) -> &'static str {
        match self {
            OrderSide::Yes => "YES",
            OrderSide::No => "NO",
        }
    }

    /// Translate the order side to the corresponding DLC outcome.
    pub fn to_outcome(&self) -> Outcome {
        match self {
            OrderSide::Yes => Outcome::Yes,
            OrderSide::No => Outcome::No,
        }
    }
}

impl std::str::FromStr for OrderSide {
    type Err = ProtocolError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "YES" => Ok(OrderSide::Yes),
            "NO" => Ok(OrderSide::No),
            other => Err(ProtocolError::InvalidOutcome(other.into())),
        }
    }
}

/// Bid (buy YES/NO tokens) or ask (sell YES/NO tokens).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderKind {
    Bid,
    Ask,
}

impl OrderKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            OrderKind::Bid => "bid",
            OrderKind::Ask => "ask",
        }
    }
}

impl std::str::FromStr for OrderKind {
    type Err = ProtocolError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bid" => Ok(OrderKind::Bid),
            "ask" => Ok(OrderKind::Ask),
            other => Err(ProtocolError::MalformedTag {
                tag: "kind",
                detail: format!("expected bid or ask, got {other}"),
            }),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Order {
    /// Market reference (`<creator_pubkey>:30888:<d>`).
    pub market: String,
    pub side: OrderSide,
    /// Token amount in sat.
    pub amount: u64,
    /// Price in sat per token.
    pub price: u64,
    pub kind: OrderKind,
    /// UNIX timestamp the order expires (must be > now).
    pub expires: u64,
}

impl Order {
    pub const KIND: u32 = KIND_ORDER;

    pub fn from_event(kind: u32, tags: &[TagTuple]) -> Result<Self, ProtocolError> {
        if kind != Self::KIND {
            return Err(ProtocolError::KindMismatch {
                expected: Self::KIND,
                actual: kind,
            });
        }
        let market = required(tags, "market")?.to_string();
        let side = required(tags, "side")?.parse()?;
        let amount =
            required(tags, "amount")?
                .parse::<u64>()
                .map_err(|e| ProtocolError::MalformedTag {
                    tag: "amount",
                    detail: format!("u64 parse: {e}"),
                })?;
        let price =
            required(tags, "price")?
                .parse::<u64>()
                .map_err(|e| ProtocolError::MalformedTag {
                    tag: "price",
                    detail: format!("u64 parse: {e}"),
                })?;
        let kind = required(tags, "kind")?.parse()?;
        let expires =
            required(tags, "expires")?
                .parse::<u64>()
                .map_err(|e| ProtocolError::MalformedTag {
                    tag: "expires",
                    detail: format!("u64 parse: {e}"),
                })?;
        Ok(Order {
            market,
            side,
            amount,
            price,
            kind,
            expires,
        })
    }

    pub fn to_tags(&self) -> Vec<TagTuple> {
        vec![
            vec!["market".into(), self.market.clone()],
            vec!["side".into(), self.side.as_str().into()],
            vec!["amount".into(), self.amount.to_string()],
            vec!["price".into(), self.price.to_string()],
            vec!["kind".into(), self.kind.as_str().into()],
            vec!["expires".into(), self.expires.to_string()],
        ]
    }
}

fn required<'a>(tags: &'a [TagTuple], key: &'static str) -> Result<&'a str, ProtocolError> {
    tags.iter()
        .find(|t| t.first().map(|k| k == key).unwrap_or(false))
        .and_then(|t| t.get(1).map(String::as_str))
        .ok_or(ProtocolError::MissingTag(key))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Vec<TagTuple> {
        vec![
            vec![
                "market".into(),
                format!("{}:30888:btc-100k", "aa".repeat(32)),
            ],
            vec!["side".into(), "YES".into()],
            vec!["amount".into(), "10000".into()],
            vec!["price".into(), "73".into()],
            vec!["kind".into(), "bid".into()],
            vec!["expires".into(), "1800000000".into()],
        ]
    }

    #[test]
    fn parse_order() {
        let o = Order::from_event(KIND_ORDER, &sample()).unwrap();
        assert_eq!(o.side, OrderSide::Yes);
        assert_eq!(o.amount, 10_000);
        assert_eq!(o.price, 73);
        assert_eq!(o.kind, OrderKind::Bid);
        assert_eq!(o.side.to_outcome(), Outcome::Yes);
    }

    #[test]
    fn reject_unknown_kind_value() {
        let mut tags = sample();
        for t in tags.iter_mut() {
            if t.first().map(|k| k == "kind").unwrap_or(false) {
                t[1] = "limit".into();
            }
        }
        assert!(Order::from_event(KIND_ORDER, &tags).is_err());
    }

    #[test]
    fn roundtrip() {
        let o = Order::from_event(KIND_ORDER, &sample()).unwrap();
        let o2 = Order::from_event(KIND_ORDER, &o.to_tags()).unwrap();
        assert_eq!(o, o2);
    }
}
