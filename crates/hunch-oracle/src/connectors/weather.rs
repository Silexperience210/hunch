//! Weather connector — resolves "daily weather metric vs threshold" markets via the free,
//! key-less open-meteo API. E.g. "did it rain in Paris on 2026-06-10?" =
//! `precipitation_sum > 0` at that lat/lon/date. Decision uses the shared [`Op`](super::Op).

use anyhow::{Context, Result};
use serde::Deserialize;

use super::{Op, Resolution};

/// "Resolve YES if the daily `metric` at (`lat`,`lon`) on `date` `op` `threshold`."
#[derive(Debug, Deserialize)]
pub struct WeatherSpec {
    pub lat: f64,
    pub lon: f64,
    /// ISO date `YYYY-MM-DD` (UTC).
    pub date: String,
    /// open-meteo daily metric, e.g. `precipitation_sum`, `temperature_2m_max`.
    #[serde(default = "default_metric")]
    pub metric: String,
    pub op: Op,
    pub threshold: f64,
}

fn default_metric() -> String {
    "precipitation_sum".to_string()
}

pub async fn resolve(spec: &WeatherSpec) -> Result<Resolution> {
    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&daily={}&start_date={}&end_date={}&timezone=UTC",
        spec.lat, spec.lon, spec.metric, spec.date, spec.date
    );
    let client = reqwest::Client::builder()
        .user_agent("hunch-oracle")
        .build()?;
    let v: serde_json::Value = client.get(url).send().await?.json().await?;
    let value = v["daily"][spec.metric.as_str()][0]
        .as_f64()
        .context("open-meteo: no value for that metric/date")?;
    let outcome = spec.op.decide(value, spec.threshold);
    let evidence = format!(
        "weather {} @({},{}) {}={} {:?} {} => {}",
        spec.date,
        spec.lat,
        spec.lon,
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
    fn defaults_to_precipitation() {
        let spec: WeatherSpec = serde_json::from_str(
            r#"{"lat":48.85,"lon":2.35,"date":"2026-06-10","op":">","threshold":0}"#,
        )
        .unwrap();
        assert_eq!(spec.metric, "precipitation_sum");
        assert_eq!(spec.op, Op::Gt);
    }
}
