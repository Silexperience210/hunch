//! Oracle resolution connectors — turn a machine-readable resolution spec into a signed-attestable
//! outcome, so an oracle can resolve markets automatically and transparently instead of by hand.
//!
//! A spec is JSON tagged by `connector`; the engine fetches the relevant data, decides
//! YES / NO / INVALID, and returns a human-readable `evidence` string for the audit trail. Adding a
//! connector = one enum variant + one module. Pure decision logic is unit-tested; the network
//! fetches are thin and not covered by offline tests.

use anyhow::Result;
use hunch_protocol::outcome::Outcome;
use serde::Deserialize;

pub mod price;

/// What a connector decided, plus human-readable evidence (observed value + source) for
/// transparency — logged by the oracle and available to surface to bettors.
pub struct Resolution {
    pub outcome: Outcome,
    pub evidence: String,
}

/// A market's machine-readable resolution rule, tagged by `connector`. Extend by adding a variant
/// and a module (e.g. `weather`, `sport`, `onchain`).
#[derive(Debug, Deserialize)]
#[serde(tag = "connector", rename_all = "snake_case")]
pub enum ResolutionSpec {
    /// Compare an asset's price to a threshold (e.g. "BTC/USD >= 100000").
    Price(price::PriceSpec),
}

impl ResolutionSpec {
    /// Parses a spec from a JSON string (inline or file contents).
    pub fn from_json(s: &str) -> Result<Self> {
        Ok(serde_json::from_str(s.trim())?)
    }

    /// Fetches the relevant data and decides the outcome.
    pub async fn resolve(&self) -> Result<Resolution> {
        match self {
            ResolutionSpec::Price(p) => price::resolve(p).await,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_price_spec() {
        let spec = ResolutionSpec::from_json(
            r#"{"connector":"price","asset":"BTC","quote":"USD","op":">=","threshold":100000}"#,
        )
        .unwrap();
        match spec {
            ResolutionSpec::Price(p) => {
                assert_eq!(p.asset, "BTC");
                assert_eq!(p.threshold, 100000.0);
            }
        }
    }

    #[test]
    fn rejects_unknown_connector() {
        assert!(ResolutionSpec::from_json(r#"{"connector":"tarot"}"#).is_err());
    }
}
