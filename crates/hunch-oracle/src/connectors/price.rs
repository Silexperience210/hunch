//! Price connector — resolves "asset price vs threshold" markets from public spot-price APIs.
//!
//! Aggregates several sources (default Coinbase + Kraken) by **median** so a single feed being
//! wrong or manipulated can't flip the outcome, then compares to the threshold. The decision logic
//! ([`decide`], [`median`]) is pure and unit-tested; the HTTP fetch is a thin layer exercised only
//! at runtime. The comparison [`Op`](super::Op) is shared across connectors.

use anyhow::{anyhow, Context, Result};
use hunch_protocol::outcome::Outcome;
use serde::Deserialize;

use super::{Op, Resolution};

/// "Resolve YES if `asset/quote` `op` `threshold`" — e.g. BTC/USD >= 100000.
#[derive(Debug, Deserialize)]
pub struct PriceSpec {
    /// Base asset, e.g. "BTC".
    pub asset: String,
    /// Quote currency, e.g. "USD".
    #[serde(default = "default_quote")]
    pub quote: String,
    /// Comparison operator.
    pub op: Op,
    /// Threshold the aggregated price is compared against.
    pub threshold: f64,
    /// Price sources to aggregate (default: coinbase + kraken).
    #[serde(default = "default_sources")]
    pub sources: Vec<String>,
}

fn default_quote() -> String {
    "USD".to_string()
}
fn default_sources() -> Vec<String> {
    vec!["coinbase".to_string(), "kraken".to_string()]
}

/// Median of a slice (mean of the two middle values for even length). `None` if empty.
pub fn median(values: &[f64]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }
    let mut v = values.to_vec();
    v.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let n = v.len();
    Some(if n % 2 == 1 {
        v[n / 2]
    } else {
        (v[n / 2 - 1] + v[n / 2]) / 2.0
    })
}

/// Pure decision: given the aggregated price, YES if the comparison holds, else NO.
/// (INVALID is reserved for the case where no source could be reached — handled in [`resolve`].)
pub fn decide(spec: &PriceSpec, aggregated: f64) -> Outcome {
    spec.op.decide(aggregated, spec.threshold)
}

/// Fetches each source, takes the median, and decides. Errors only if no source returned a price.
pub async fn resolve(spec: &PriceSpec) -> Result<Resolution> {
    let mut prices = Vec::new();
    let mut details = Vec::new();
    for source in &spec.sources {
        match fetch_price(source, &spec.asset, &spec.quote).await {
            Ok(p) => {
                prices.push(p);
                details.push(format!("{source}={p}"));
            }
            Err(e) => details.push(format!("{source}=ERR({e})")),
        }
    }
    let agg = median(&prices)
        .ok_or_else(|| anyhow!("no price source returned a value: [{}]", details.join(", ")))?;
    let outcome = decide(spec, agg);
    let evidence = format!(
        "{}/{} median={} {:?} {} => {} [{}]",
        spec.asset,
        spec.quote,
        agg,
        spec.op,
        spec.threshold,
        outcome.as_str(),
        details.join(", "),
    );
    Ok(Resolution { outcome, evidence })
}

/// Fetches a spot price from a named public source. Open-source + multi-source = auditable.
async fn fetch_price(source: &str, asset: &str, quote: &str) -> Result<f64> {
    let client = reqwest::Client::builder()
        .user_agent("hunch-oracle")
        .build()?;
    match source {
        "coinbase" => {
            let url = format!("https://api.coinbase.com/v2/prices/{asset}-{quote}/spot");
            let v: serde_json::Value = client.get(url).send().await?.json().await?;
            v["data"]["amount"]
                .as_str()
                .context("coinbase: missing data.amount")?
                .parse()
                .context("coinbase: amount not a number")
        }
        "kraken" => {
            // Kraken denotes BTC as XBT.
            let kasset = if asset.eq_ignore_ascii_case("BTC") {
                "XBT"
            } else {
                asset
            };
            let pair = format!("{kasset}{quote}");
            let url = format!("https://api.kraken.com/0/public/Ticker?pair={pair}");
            let v: serde_json::Value = client.get(url).send().await?.json().await?;
            let result = v["result"].as_object().context("kraken: missing result")?;
            let (_, tick) = result.iter().next().context("kraken: empty result")?;
            tick["c"][0]
                .as_str()
                .context("kraken: missing last price")?
                .parse()
                .context("kraken: price not a number")
        }
        other => Err(anyhow!("unknown price source: {other}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn median_odd_even_empty() {
        assert_eq!(median(&[3.0, 1.0, 2.0]), Some(2.0));
        assert_eq!(median(&[1.0, 2.0, 3.0, 4.0]), Some(2.5));
        assert_eq!(median(&[42.0]), Some(42.0));
        assert_eq!(median(&[]), None);
    }

    #[test]
    fn decide_yes_no() {
        let spec = PriceSpec {
            asset: "BTC".into(),
            quote: "USD".into(),
            op: Op::Gte,
            threshold: 100_000.0,
            sources: vec![],
        };
        assert_eq!(decide(&spec, 100_001.0).as_str(), "YES");
        assert_eq!(decide(&spec, 100_000.0).as_str(), "YES");
        assert_eq!(decide(&spec, 99_999.0).as_str(), "NO");
    }

    #[test]
    fn op_parses_from_symbols() {
        let spec: PriceSpec = serde_json::from_str(
            r#"{"asset":"BTC","op":"<","threshold":50000,"sources":["coinbase"]}"#,
        )
        .unwrap();
        assert_eq!(spec.op, Op::Lt);
        assert_eq!(spec.quote, "USD"); // default applied
    }
}
