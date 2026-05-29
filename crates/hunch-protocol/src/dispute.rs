//! Dispute events — HIP-1 kind 30890 (parameterized replaceable).

use serde::{Deserialize, Serialize};

use crate::error::ProtocolError;
use crate::event_kinds::KIND_DISPUTE;
use crate::market::TagTuple;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Dispute {
    /// Dispute identifier (`d` tag).
    pub d: String,
    /// Market reference (`<creator_pubkey>:30888:<d>`).
    pub market: String,
    /// Event ID of the disputed kind:89 attestation.
    pub attestation: String,
    /// Short claim category (`oracle_misread`, `source_unavailable`, etc.).
    pub claim: String,
    /// Free-form evidence body from the source event's content field.
    pub evidence: String,
}

impl Dispute {
    pub const KIND: u32 = KIND_DISPUTE;

    pub fn from_event(kind: u32, tags: &[TagTuple], content: &str) -> Result<Self, ProtocolError> {
        if kind != Self::KIND {
            return Err(ProtocolError::KindMismatch {
                expected: Self::KIND,
                actual: kind,
            });
        }
        Ok(Dispute {
            d: required(tags, "d")?.to_string(),
            market: required(tags, "market")?.to_string(),
            attestation: required(tags, "attestation")?.to_string(),
            claim: required(tags, "claim")?.to_string(),
            evidence: content.to_string(),
        })
    }

    pub fn to_event_parts(&self) -> (Vec<TagTuple>, String) {
        (
            vec![
                vec!["d".into(), self.d.clone()],
                vec!["market".into(), self.market.clone()],
                vec!["attestation".into(), self.attestation.clone()],
                vec!["claim".into(), self.claim.clone()],
            ],
            self.evidence.clone(),
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
            vec!["d".into(), "d-001".into()],
            vec!["market".into(), format!("{}:30888:btc-100k", "aa".repeat(32))],
            vec!["attestation".into(), "ev-deadbeef".into()],
            vec!["claim".into(), "oracle_misread".into()],
        ];
        let d = Dispute::from_event(KIND_DISPUTE, &tags, "evidence body").unwrap();
        let (out_tags, out_content) = d.to_event_parts();
        let d2 = Dispute::from_event(KIND_DISPUTE, &out_tags, &out_content).unwrap();
        assert_eq!(d, d2);
    }
}
