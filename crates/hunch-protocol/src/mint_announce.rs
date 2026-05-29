//! Mint announce events — HIP-1 kind 30892 (parameterized replaceable).

use serde::{Deserialize, Serialize};

use crate::error::ProtocolError;
use crate::event_kinds::KIND_MINT_ANNOUNCE;
use crate::market::TagTuple;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MintAnnounce {
    /// Mint identifier (`d` tag).
    pub d: String,
    /// Mint endpoint URL (HTTPS, onion, or IPFS gateway).
    pub endpoint: String,
    /// Latest reserves-proof URL.
    pub reserves_proof: String,
    /// Pubkeys of oracles this mint accepts (comma-separated hex list, parsed here).
    pub supported_oracles: Vec<String>,
    /// Free-form JSON body describing mint policy (max market size, supported asset, etc.).
    pub body: String,
}

impl MintAnnounce {
    pub const KIND: u32 = KIND_MINT_ANNOUNCE;

    pub fn from_event(kind: u32, tags: &[TagTuple], content: &str) -> Result<Self, ProtocolError> {
        if kind != Self::KIND {
            return Err(ProtocolError::KindMismatch {
                expected: Self::KIND,
                actual: kind,
            });
        }
        let d = required(tags, "d")?.to_string();
        let endpoint = required(tags, "endpoint")?.to_string();
        let reserves_proof = required(tags, "reserves_proof")?.to_string();
        let supported_oracles_raw = required(tags, "supported_oracles")?;
        let supported_oracles: Vec<String> = supported_oracles_raw
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        // Validate every entry is 64 hex chars (32 bytes).
        for p in &supported_oracles {
            if hex::decode(p).map(|b| b.len()) != Ok(32) {
                return Err(ProtocolError::InvalidPubkey(p.clone()));
            }
        }
        Ok(MintAnnounce {
            d,
            endpoint,
            reserves_proof,
            supported_oracles,
            body: content.to_string(),
        })
    }

    pub fn to_event_parts(&self) -> (Vec<TagTuple>, String) {
        (
            vec![
                vec!["d".into(), self.d.clone()],
                vec!["endpoint".into(), self.endpoint.clone()],
                vec!["reserves_proof".into(), self.reserves_proof.clone()],
                vec!["supported_oracles".into(), self.supported_oracles.join(",")],
            ],
            self.body.clone(),
        )
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

    #[test]
    fn parse_and_roundtrip() {
        let tags = vec![
            vec!["d".into(), "hunch-mint-1".into()],
            vec!["endpoint".into(), "https://mint.hunch.markets".into()],
            vec![
                "reserves_proof".into(),
                "https://mint.hunch.markets/reserves/2026-W22".into(),
            ],
            vec![
                "supported_oracles".into(),
                format!("{},{}", "aa".repeat(32), "bb".repeat(32)),
            ],
        ];
        let m =
            MintAnnounce::from_event(KIND_MINT_ANNOUNCE, &tags, "{\"max_market_sat\":10000000}")
                .unwrap();
        assert_eq!(m.supported_oracles.len(), 2);
        let (out_tags, out_content) = m.to_event_parts();
        let m2 = MintAnnounce::from_event(KIND_MINT_ANNOUNCE, &out_tags, &out_content).unwrap();
        assert_eq!(m, m2);
    }

    #[test]
    fn reject_malformed_oracle_pubkey_list() {
        let tags = vec![
            vec!["d".into(), "hunch-mint-1".into()],
            vec!["endpoint".into(), "https://mint.hunch.markets".into()],
            vec![
                "reserves_proof".into(),
                "https://mint.hunch.markets/reserves/2026-W22".into(),
            ],
            vec!["supported_oracles".into(), "not-hex".into()],
        ];
        assert!(MintAnnounce::from_event(KIND_MINT_ANNOUNCE, &tags, "").is_err());
    }
}
