//! LLM connector — resolves natural-language yes/no markets by asking a language model.
//!
//! For questions that aren't a clean numeric threshold ("Did team X win?", "Was event Y announced
//! before date Z?"), the oracle asks an OpenAI-compatible chat model and parses a strict verdict.
//!
//! The endpoint, API key and model are **oracle-operator config via env** (`HUNCH_LLM_URL`,
//! `HUNCH_LLM_KEY`, `HUNCH_LLM_MODEL`) — never in the spec/event. So the API key is never published
//! on Nostr, and different oracles can answer the same spec with different models (multi-oracle, the
//! market lists which oracle it trusts). The model's reasoning lands in the evidence string for the
//! audit trail; bettors who distrust an LLM verdict use the dispute path (kind 30890).
//!
//! The parsing ([`parse_decision`], [`build_prompt`]) is pure and unit-tested; the HTTP call is a
//! thin layer exercised only at runtime.

use anyhow::{anyhow, Context, Result};
use hunch_protocol::outcome::Outcome;
use serde::Deserialize;
use std::str::FromStr;

use super::Resolution;

/// "Resolve this yes/no question by asking an LLM." Endpoints/keys/models are env, not part of the
/// spec — only a *provider label* may appear, so different oracles can answer with different models.
#[derive(Debug, Deserialize)]
pub struct LlmSpec {
    /// The yes/no question to decide.
    pub question: String,
    /// Optional extra resolution criteria / context handed to the model.
    #[serde(default)]
    pub criteria: String,
    /// Which configured provider(s) answer. `None`/empty → the default env. A label like `"kimi"` or
    /// `"claude"` → that provider's env. A comma list (`"kimi,claude"`) or `"consensus"`/`"all"`
    /// (which expands `HUNCH_LLM_PROVIDERS`) → query each and require unanimity, else INVALID.
    #[serde(default)]
    pub provider: Option<String>,
}

/// The exact prompt sent to the model — public so the wording is auditable.
pub fn build_prompt(spec: &LlmSpec) -> String {
    let mut p = format!(
        "You are a prediction-market oracle. Decide the following yes/no question strictly and \
         impartially, only from verifiable facts.\n\nQuestion: {}\n",
        spec.question.trim()
    );
    if !spec.criteria.trim().is_empty() {
        p.push_str(&format!("Resolution criteria: {}\n", spec.criteria.trim()));
    }
    p.push_str(
        "\nReply with ONLY a JSON object and nothing else: \
         {\"outcome\":\"YES\"|\"NO\"|\"INVALID\",\"reasoning\":\"<one sentence citing the deciding fact>\"}. \
         Use INVALID only if the question is genuinely unanswerable or ambiguous.",
    );
    p
}

#[derive(Deserialize)]
struct Decision {
    outcome: String,
    #[serde(default)]
    reasoning: String,
}

/// Parse the model's reply into `(outcome, reasoning)`. Tries strict JSON first (even if wrapped in
/// prose or code fences), then falls back to the first standalone YES/NO/INVALID token. Pure + tested.
pub fn parse_decision(text: &str) -> Result<(Outcome, String)> {
    if let Some(obj) = extract_json_object(text) {
        if let Ok(d) = serde_json::from_str::<Decision>(&obj) {
            if let Ok(o) = Outcome::from_str(d.outcome.trim().to_uppercase().as_str()) {
                let reasoning = if d.reasoning.trim().is_empty() {
                    "(no reasoning given)".to_string()
                } else {
                    d.reasoning.trim().to_string()
                };
                return Ok((o, reasoning));
            }
        }
    }
    if let Some(o) = scan_outcome(text) {
        return Ok((o, text.trim().to_string()));
    }
    Err(anyhow!(
        "LLM reply has no parseable outcome: {:?}",
        truncate(text, 200)
    ))
}

/// First standalone YES/NO/INVALID token (case-insensitive), so we don't match substrings of words.
fn scan_outcome(text: &str) -> Option<Outcome> {
    for tok in text.split(|c: char| !c.is_ascii_alphabetic()) {
        match tok.to_uppercase().as_str() {
            "YES" => return Some(Outcome::Yes),
            "NO" => return Some(Outcome::No),
            "INVALID" => return Some(Outcome::Invalid),
            _ => {}
        }
    }
    None
}

/// Extract the first balanced `{...}` object (string-aware), so a JSON verdict wrapped in prose or
/// ```json fences is still recovered.
fn extract_json_object(text: &str) -> Option<String> {
    let start = text.find('{')?;
    let bytes = text.as_bytes();
    let (mut depth, mut in_str, mut esc) = (0i32, false, false);
    for (offset, &c) in bytes.iter().enumerate().skip(start) {
        if in_str {
            if esc {
                esc = false;
            } else if c == b'\\' {
                esc = true;
            } else if c == b'"' {
                in_str = false;
            }
        } else {
            match c {
                b'"' => in_str = true,
                b'{' => depth += 1,
                b'}' => {
                    depth -= 1;
                    if depth == 0 {
                        return Some(text[start..=offset].to_string());
                    }
                }
                _ => {}
            }
        }
    }
    None
}

fn truncate(s: &str, n: usize) -> String {
    let t = s.trim();
    if t.chars().count() <= n {
        t.to_string()
    } else {
        format!("{}…", t.chars().take(n).collect::<String>())
    }
}

/// A resolved provider config: where to POST, the bearer key, and the model id.
struct Provider {
    name: String,
    url: String,
    key: String,
    model: String,
}

impl Provider {
    fn label(&self) -> &str {
        if self.name.is_empty() {
            "default"
        } else {
            &self.name
        }
    }
}

/// Provider names to query. `None`/empty → `[""]` (the default env). `"consensus"`/`"all"` expand the
/// `configured` list (from `HUNCH_LLM_PROVIDERS`). Anything else is treated as a comma list. Pure.
pub fn provider_names(spec_provider: Option<&str>, configured: &str) -> Vec<String> {
    match spec_provider.map(str::trim) {
        None | Some("") => vec![String::new()],
        Some("consensus") | Some("all") => split_csv(configured),
        Some(list) => split_csv(list),
    }
}

fn split_csv(s: &str) -> Vec<String> {
    let v: Vec<String> = s
        .split(',')
        .map(|x| x.trim().to_string())
        .filter(|x| !x.is_empty())
        .collect();
    if v.is_empty() {
        vec![String::new()]
    } else {
        v
    }
}

/// Reads a provider's config from env. Default provider (`""`) uses `HUNCH_LLM_{URL,KEY,MODEL}`; a
/// named provider `kimi` uses `HUNCH_LLM_KIMI_{URL,KEY,MODEL}`. Key is optional (local models).
fn provider_env(name: &str) -> Result<Provider> {
    let prefix = if name.is_empty() {
        "HUNCH_LLM".to_string()
    } else {
        format!(
            "HUNCH_LLM_{}",
            name.to_uppercase()
                .replace(|c: char| !c.is_ascii_alphanumeric(), "_")
        )
    };
    let label = if name.is_empty() { "default" } else { name };
    let url = std::env::var(format!("{prefix}_URL")).with_context(|| {
        format!("{prefix}_URL not set (OpenAI-compatible /chat/completions endpoint for '{label}')")
    })?;
    let key = std::env::var(format!("{prefix}_KEY")).unwrap_or_default();
    let model =
        std::env::var(format!("{prefix}_MODEL")).unwrap_or_else(|_| "gpt-4o-mini".to_string());
    Ok(Provider {
        name: name.to_string(),
        url,
        key,
        model,
    })
}

/// Combine per-provider verdicts: unanimous → that outcome; any disagreement (or none) → INVALID.
/// This is the ensemble guard — a single model can't unilaterally settle a consensus market. Pure.
pub fn combine(verdicts: &[(String, Outcome)]) -> Resolution {
    let detail = verdicts
        .iter()
        .map(|(n, o)| format!("{n}={}", o.as_str()))
        .collect::<Vec<_>>()
        .join(", ");
    let first = verdicts.first().map(|(_, o)| *o);
    let unanimous = first.is_some_and(|f| verdicts.iter().all(|(_, o)| *o == f));
    let outcome = if unanimous {
        first.unwrap()
    } else {
        Outcome::Invalid
    };
    Resolution {
        outcome,
        evidence: format!("llm consensus [{detail}] => {}", outcome.as_str()),
    }
}

/// One provider's verdict for the spec.
async fn resolve_with(p: &Provider, spec: &LlmSpec) -> Result<Resolution> {
    let body = serde_json::json!({
        "model": p.model,
        "temperature": 0,
        "messages": [{ "role": "user", "content": build_prompt(spec) }],
    });
    let client = reqwest::Client::builder()
        .user_agent("hunch-oracle")
        .build()?;
    let mut req = client.post(&p.url).json(&body);
    if !p.key.is_empty() {
        req = req.bearer_auth(&p.key);
    }
    let resp: serde_json::Value = req.send().await?.error_for_status()?.json().await?;
    let content = resp["choices"][0]["message"]["content"]
        .as_str()
        .context("LLM response missing choices[0].message.content")?;
    let (outcome, reasoning) = parse_decision(content)?;
    let evidence = format!(
        "llm[{}/{}] \"{}\" => {} — {}",
        p.label(),
        p.model,
        truncate(&spec.question, 80),
        outcome.as_str(),
        reasoning,
    );
    Ok(Resolution { outcome, evidence })
}

/// Asks the configured LLM(s) and parses the verdict. One provider → its answer; several → consensus.
pub async fn resolve(spec: &LlmSpec) -> Result<Resolution> {
    let configured = std::env::var("HUNCH_LLM_PROVIDERS").unwrap_or_default();
    let names = provider_names(spec.provider.as_deref(), &configured);
    if names.len() == 1 {
        let p = provider_env(&names[0])?;
        return resolve_with(&p, spec).await;
    }
    let mut verdicts = Vec::new();
    for name in &names {
        let p = provider_env(name)?;
        let r = resolve_with(&p, spec).await?;
        verdicts.push((p.label().to_string(), r.outcome));
    }
    Ok(combine(&verdicts))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn spec() -> LlmSpec {
        LlmSpec {
            question: "Did it happen?".into(),
            criteria: "by 2026-12-31 UTC".into(),
            provider: None,
        }
    }

    #[test]
    fn parses_clean_json() {
        let (o, r) = parse_decision(r#"{"outcome":"YES","reasoning":"it was confirmed"}"#).unwrap();
        assert_eq!(o, Outcome::Yes);
        assert_eq!(r, "it was confirmed");
    }

    #[test]
    fn parses_json_wrapped_in_prose_and_fences() {
        let text =
            "Sure!\n```json\n{ \"outcome\": \"no\", \"reasoning\": \"never occurred\" }\n```\n";
        let (o, r) = parse_decision(text).unwrap();
        assert_eq!(o, Outcome::No);
        assert_eq!(r, "never occurred");
    }

    #[test]
    fn parses_invalid_outcome() {
        let (o, _) = parse_decision(r#"{"outcome":"INVALID","reasoning":"ambiguous"}"#).unwrap();
        assert_eq!(o, Outcome::Invalid);
    }

    #[test]
    fn falls_back_to_first_token() {
        let (o, _) = parse_decision("After review, the answer is NO.").unwrap();
        assert_eq!(o, Outcome::No);
        // Substrings of words must not match (NONETHELESS contains "no" but not as a token).
        let (o2, _) = parse_decision("Nonetheless it is a YES overall.").unwrap();
        assert_eq!(o2, Outcome::Yes);
    }

    #[test]
    fn errors_without_any_outcome() {
        assert!(parse_decision("I cannot tell you that.").is_err());
    }

    #[test]
    fn prompt_includes_question_and_criteria() {
        let p = build_prompt(&spec());
        assert!(p.contains("Did it happen?"));
        assert!(p.contains("by 2026-12-31 UTC"));
        assert!(p.contains("INVALID"));
    }

    #[test]
    fn provider_names_routing() {
        assert_eq!(provider_names(None, ""), vec![String::new()]);
        assert_eq!(provider_names(Some(""), ""), vec![String::new()]);
        assert_eq!(provider_names(Some("kimi"), ""), vec!["kimi"]);
        assert_eq!(
            provider_names(Some("kimi, claude"), ""),
            vec!["kimi", "claude"]
        );
        assert_eq!(
            provider_names(Some("consensus"), "kimi,claude"),
            vec!["kimi", "claude"]
        );
        assert_eq!(
            provider_names(Some("all"), "kimi,claude"),
            vec!["kimi", "claude"]
        );
    }

    #[test]
    fn combine_unanimous_vs_disagree() {
        let yes = |n: &str| (n.to_string(), Outcome::Yes);
        // both agree → that outcome
        assert_eq!(combine(&[yes("kimi"), yes("claude")]).outcome, Outcome::Yes);
        // disagreement → INVALID (no single model can settle a consensus market)
        let mixed = [
            ("kimi".to_string(), Outcome::Yes),
            ("claude".to_string(), Outcome::No),
        ];
        assert_eq!(combine(&mixed).outcome, Outcome::Invalid);
        // single verdict carries
        assert_eq!(combine(&[yes("kimi")]).outcome, Outcome::Yes);
        // empty → INVALID
        assert_eq!(combine(&[]).outcome, Outcome::Invalid);
    }
}
