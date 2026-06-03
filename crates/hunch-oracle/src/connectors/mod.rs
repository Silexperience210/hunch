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

pub mod http;
pub mod llm;
pub mod onchain;
pub mod price;
pub mod weather;

/// What a connector decided, plus human-readable evidence (observed value + source) for
/// transparency — logged by the oracle and available to surface to bettors.
pub struct Resolution {
    pub outcome: Outcome,
    pub evidence: String,
}

/// Comparison operator shared by the numeric connectors, parsed from the usual symbols.
#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
pub enum Op {
    #[serde(rename = ">=")]
    Gte,
    #[serde(rename = ">")]
    Gt,
    #[serde(rename = "<=")]
    Lte,
    #[serde(rename = "<")]
    Lt,
    #[serde(rename = "==", alias = "=")]
    Eq,
}

impl Op {
    /// Evaluates `value <op> threshold`.
    pub fn eval(self, value: f64, threshold: f64) -> bool {
        match self {
            Op::Gte => value >= threshold,
            Op::Gt => value > threshold,
            Op::Lte => value <= threshold,
            Op::Lt => value < threshold,
            Op::Eq => (value - threshold).abs() < f64::EPSILON,
        }
    }

    /// YES if the comparison holds, else NO.
    pub fn decide(self, value: f64, threshold: f64) -> Outcome {
        if self.eval(value, threshold) {
            Outcome::Yes
        } else {
            Outcome::No
        }
    }
}

/// A market's machine-readable resolution rule, tagged by `connector`. Extend by adding a variant
/// and a module.
#[derive(Debug, Deserialize)]
#[serde(tag = "connector", rename_all = "snake_case")]
pub enum ResolutionSpec {
    /// Compare an asset's price to a threshold (Coinbase + Kraken median).
    Price(price::PriceSpec),
    /// Compare a daily weather metric to a threshold (open-meteo).
    Weather(weather::WeatherSpec),
    /// Compare a Bitcoin on-chain metric to a threshold (mempool.space).
    Onchain(onchain::OnchainSpec),
    /// Generic: fetch any JSON URL, extract a numeric field by path, compare.
    Http(http::HttpSpec),
    /// Natural-language: ask an LLM (oracle-operator endpoint via env) for a strict YES/NO/INVALID.
    Llm(llm::LlmSpec),
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
            ResolutionSpec::Weather(w) => weather::resolve(w).await,
            ResolutionSpec::Onchain(o) => onchain::resolve(o).await,
            ResolutionSpec::Http(h) => http::resolve(h).await,
            ResolutionSpec::Llm(l) => llm::resolve(l).await,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn op_eval_all() {
        assert!(Op::Gte.eval(100.0, 100.0));
        assert!(!Op::Gt.eval(100.0, 100.0));
        assert!(Op::Lte.eval(100.0, 100.0));
        assert!(Op::Lt.eval(99.0, 100.0));
        assert!(Op::Eq.eval(7.0, 7.0));
        assert!(!Op::Eq.eval(7.0, 8.0));
    }

    #[test]
    fn op_decide() {
        assert_eq!(Op::Gte.decide(101.0, 100.0).as_str(), "YES");
        assert_eq!(Op::Gte.decide(99.0, 100.0).as_str(), "NO");
    }

    #[test]
    fn parses_each_connector() {
        assert!(matches!(
            ResolutionSpec::from_json(
                r#"{"connector":"price","asset":"BTC","op":">=","threshold":100000}"#
            )
            .unwrap(),
            ResolutionSpec::Price(_)
        ));
        assert!(matches!(
            ResolutionSpec::from_json(r#"{"connector":"weather","lat":48.8,"lon":2.3,"date":"2026-06-10","op":">","threshold":0}"#).unwrap(),
            ResolutionSpec::Weather(_)
        ));
        assert!(matches!(
            ResolutionSpec::from_json(
                r#"{"connector":"onchain","metric":"block_height","op":">=","threshold":900000}"#
            )
            .unwrap(),
            ResolutionSpec::Onchain(_)
        ));
        assert!(matches!(
            ResolutionSpec::from_json(
                r#"{"connector":"http","url":"https://x","path":["a"],"op":">","threshold":1}"#
            )
            .unwrap(),
            ResolutionSpec::Http(_)
        ));
        assert!(matches!(
            ResolutionSpec::from_json(r#"{"connector":"llm","question":"Did X happen?"}"#).unwrap(),
            ResolutionSpec::Llm(_)
        ));
    }

    #[test]
    fn rejects_unknown_connector() {
        assert!(ResolutionSpec::from_json(r#"{"connector":"tarot"}"#).is_err());
    }
}
