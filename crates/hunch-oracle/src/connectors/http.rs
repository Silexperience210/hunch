//! Generic HTTP/JSON connector — fetch any JSON URL, extract a numeric field by path, compare to a
//! threshold. Covers sports scores, indices, custom feeds — anything a JSON API exposes. The path
//! walk ([`extract`]) is pure and unit-tested; trust is the oracle operator's choice of URL, and the
//! URL + observed value land in the evidence string.

use anyhow::{anyhow, Result};
use serde::Deserialize;

use super::{Op, Resolution};

/// "Resolve YES if the number at `path` in the JSON at `url` `op` `threshold`."
#[derive(Debug, Deserialize)]
pub struct HttpSpec {
    pub url: String,
    /// Path of object keys / array indices (indices as numeric strings, e.g. `["data","0","score"]`).
    pub path: Vec<String>,
    pub op: Op,
    pub threshold: f64,
}

/// Walks a JSON value by a path of object keys / array indices to a number. Accepts a numeric value
/// or a numeric string. Pure + tested.
pub fn extract(v: &serde_json::Value, path: &[String]) -> Option<f64> {
    let mut cur = v;
    for seg in path {
        cur = match seg.parse::<usize>() {
            Ok(i) => cur.get(i)?,
            Err(_) => cur.get(seg)?,
        };
    }
    cur.as_f64()
        .or_else(|| cur.as_str().and_then(|s| s.parse().ok()))
}

pub async fn resolve(spec: &HttpSpec) -> Result<Resolution> {
    let client = reqwest::Client::builder()
        .user_agent("hunch-oracle")
        .build()?;
    let v: serde_json::Value = client.get(&spec.url).send().await?.json().await?;
    let value = extract(&v, &spec.path)
        .ok_or_else(|| anyhow!("http: no number at path {:?}", spec.path))?;
    let outcome = spec.op.decide(value, spec.threshold);
    let evidence = format!(
        "http {} path={:?}={} {:?} {} => {}",
        spec.url,
        spec.path,
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
    use serde_json::json;

    #[test]
    fn extract_nested_and_index() {
        let v = json!({"data": {"items": [{"v": 5}, {"v": 9}]}});
        let path = ["data", "items", "1", "v"].map(String::from);
        assert_eq!(extract(&v, &path), Some(9.0));
    }

    #[test]
    fn extract_string_number_and_missing() {
        let v = json!({"price": "42.5"});
        assert_eq!(extract(&v, &["price".to_string()]), Some(42.5));
        assert_eq!(extract(&v, &["nope".to_string()]), None);
    }
}
