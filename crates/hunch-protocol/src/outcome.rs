//! DLC outcome enum (HIP-2).
//!
//! Per HIP-2 §Outcome Encoding: outcomes are encoded as case-sensitive UTF-8 strings,
//! exactly one of `YES`, `NO`, or `INVALID`. The mint applies the matching CET when
//! the oracle attests the corresponding outcome.

use serde::{Deserialize, Serialize};

use crate::error::ProtocolError;

/// DLC outcome (HIP-2).
///
/// `YES` and `NO` are the binary market outcomes. `INVALID` is the dispute-escape branch:
/// when the oracle attests INVALID, the mint receives the refund half of the funding output
/// and refunds bettors at entry price (HIP-3 §Refund Mechanics for INVALID Outcome).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Outcome {
    Yes,
    No,
    Invalid,
}

impl Outcome {
    /// Canonical string form per HIP-2 §Outcome Encoding.
    pub fn as_str(&self) -> &'static str {
        match self {
            Outcome::Yes => "YES",
            Outcome::No => "NO",
            Outcome::Invalid => "INVALID",
        }
    }

    /// All three outcomes, in CET-enumeration order.
    pub const ALL: [Outcome; 3] = [Outcome::Yes, Outcome::No, Outcome::Invalid];
}

impl std::fmt::Display for Outcome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::str::FromStr for Outcome {
    type Err = ProtocolError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "YES" => Ok(Outcome::Yes),
            "NO" => Ok(Outcome::No),
            "INVALID" => Ok(Outcome::Invalid),
            other => Err(ProtocolError::InvalidOutcome(other.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn outcome_strings_match_hip2() {
        // HIP-2 §Outcome Encoding mandates these exact strings.
        assert_eq!(Outcome::Yes.as_str(), "YES");
        assert_eq!(Outcome::No.as_str(), "NO");
        assert_eq!(Outcome::Invalid.as_str(), "INVALID");
    }

    #[test]
    fn outcome_parse_roundtrip() {
        for o in Outcome::ALL {
            assert_eq!(Outcome::from_str(o.as_str()).unwrap(), o);
        }
    }

    #[test]
    fn outcome_parse_case_sensitive() {
        // HIP-2 §Outcome Encoding: "case-sensitive. UTF-8."
        assert!(Outcome::from_str("yes").is_err());
        assert!(Outcome::from_str("Yes").is_err());
        assert!(Outcome::from_str("MAYBE").is_err());
    }

    #[test]
    fn outcome_serde_roundtrip() {
        for o in Outcome::ALL {
            let j = serde_json::to_string(&o).unwrap();
            let back: Outcome = serde_json::from_str(&j).unwrap();
            assert_eq!(back, o);
        }
        // Surface form is uppercase string.
        assert_eq!(serde_json::to_string(&Outcome::Yes).unwrap(), "\"YES\"");
    }
}
