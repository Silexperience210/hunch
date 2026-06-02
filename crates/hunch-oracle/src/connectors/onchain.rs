//! On-chain connector — resolves Bitcoin on-chain markets via the free mempool.space API.
//! Supported metrics: `block_height`, `mempool_count`, `fee_fastest` (sat/vB). Decision uses the
//! shared [`Op`](super::Op).

use anyhow::{anyhow, Context, Result};
use serde::Deserialize;

use super::{Op, Resolution};

/// "Resolve YES if the on-chain `metric` `op` `threshold`."
#[derive(Debug, Deserialize)]
pub struct OnchainSpec {
    /// `block_height` | `mempool_count` | `fee_fastest`.
    pub metric: String,
    pub op: Op,
    pub threshold: f64,
    /// mempool.space-compatible API base (override for a self-hosted instance / Tor).
    #[serde(default = "default_base")]
    pub base_url: String,
}

fn default_base() -> String {
    "https://mempool.space/api".to_string()
}

pub async fn resolve(spec: &OnchainSpec) -> Result<Resolution> {
    let client = reqwest::Client::builder()
        .user_agent("hunch-oracle")
        .build()?;
    let base = spec.base_url.trim_end_matches('/');
    let value = match spec.metric.as_str() {
        "block_height" => client
            .get(format!("{base}/blocks/tip/height"))
            .send()
            .await?
            .text()
            .await?
            .trim()
            .parse::<f64>()
            .context("mempool: block height not a number")?,
        "mempool_count" => {
            let v: serde_json::Value = client
                .get(format!("{base}/mempool"))
                .send()
                .await?
                .json()
                .await?;
            v["count"].as_f64().context("mempool: missing count")?
        }
        "fee_fastest" => {
            let v: serde_json::Value = client
                .get(format!("{base}/v1/fees/recommended"))
                .send()
                .await?
                .json()
                .await?;
            v["fastestFee"]
                .as_f64()
                .context("mempool: missing fastestFee")?
        }
        other => return Err(anyhow!("unknown onchain metric: {other}")),
    };
    let outcome = spec.op.decide(value, spec.threshold);
    let evidence = format!(
        "onchain {}={} {:?} {} => {}",
        spec.metric,
        value,
        spec.op,
        spec.threshold,
        outcome.as_str(),
    );
    Ok(Resolution { outcome, evidence })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_with_default_base() {
        let spec: OnchainSpec =
            serde_json::from_str(r#"{"metric":"block_height","op":">=","threshold":900000}"#)
                .unwrap();
        assert_eq!(spec.metric, "block_height");
        assert!(spec.base_url.contains("mempool.space"));
    }
}
