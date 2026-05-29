//! Oracle types — HIP-1 kinds 88/89 + HIP-4 `OracleSigner` trait.
//!
//! `OracleSigner` is the abstraction across signing strategies:
//! - `SingleKeySigner` — v1 single-key Schnorr signing
//! - `FrostSigner` — Phase 4 multi-key FROST aggregate signing (impl lives in `hunch-oracle`)
//!
//! The mint and the DLC adapter (`hunch-mint`) consume signatures via this trait
//! and need not change when v2 swaps single-key for FROST.

use secp256k1::{schnorr::Signature, Keypair, PublicKey, Secp256k1, SecretKey, XOnlyPublicKey};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::error::ProtocolError;
use crate::event_kinds::{KIND_ORACLE_ANNOUNCE, KIND_ORACLE_ATTESTATION};
use crate::market::TagTuple;
use crate::outcome::Outcome;

/// Oracle announce content (NIP-88, HIP-1 kind 88).
///
/// The oracle publishes one announce per market it commits to attest, declaring its
/// announced public nonce R for the adapter-signature construction in HIP-2.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OracleAnnounce {
    /// Market identifier the oracle commits to (`<creator_pubkey>:30888:<d>`).
    pub market: String,
    /// Oracle's announced public nonce R for this market (hex, 32 bytes).
    ///
    /// Used by the DLC adapter (HIP-2) to construct the per-outcome adapter point.
    pub nonce_pubkey: String,
    /// Free-form content body (resolution rules summary, contact info, etc.).
    pub body: String,
}

impl OracleAnnounce {
    pub const KIND: u32 = KIND_ORACLE_ANNOUNCE;

    pub fn from_event(kind: u32, tags: &[TagTuple], content: &str) -> Result<Self, ProtocolError> {
        if kind != Self::KIND {
            return Err(ProtocolError::KindMismatch {
                expected: Self::KIND,
                actual: kind,
            });
        }
        let market = required(tags, "market")?.to_string();
        let nonce_pubkey = required(tags, "nonce")?.to_string();
        if hex::decode(&nonce_pubkey).map(|b| b.len()) != Ok(32) {
            return Err(ProtocolError::InvalidPubkey(nonce_pubkey));
        }
        Ok(OracleAnnounce {
            market,
            nonce_pubkey,
            body: content.to_string(),
        })
    }

    pub fn to_event_parts(&self) -> (Vec<TagTuple>, String) {
        (
            vec![
                vec!["market".into(), self.market.clone()],
                vec!["nonce".into(), self.nonce_pubkey.clone()],
            ],
            self.body.clone(),
        )
    }
}

/// Oracle attestation content (NIP-88, HIP-1 kind 89).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OracleAttestation {
    /// Market identifier this attestation resolves.
    pub market: String,
    /// Resolved outcome (HIP-2).
    pub outcome: Outcome,
    /// Schnorr signature over the canonical message (HIP-2 adapter construction), hex.
    pub signature_hex: String,
}

impl OracleAttestation {
    pub const KIND: u32 = KIND_ORACLE_ATTESTATION;

    /// Canonical message bytes signed for `(market, outcome)`.
    ///
    /// Format: `"hunch/oracle/v1\n<market>\n<outcome>"`. Stable across implementations.
    pub fn signing_message(market: &str, outcome: Outcome) -> Vec<u8> {
        let mut buf = Vec::with_capacity(64 + market.len());
        buf.extend_from_slice(b"hunch/oracle/v1\n");
        buf.extend_from_slice(market.as_bytes());
        buf.push(b'\n');
        buf.extend_from_slice(outcome.as_str().as_bytes());
        buf
    }

    pub fn from_event(kind: u32, tags: &[TagTuple], _content: &str) -> Result<Self, ProtocolError> {
        if kind != Self::KIND {
            return Err(ProtocolError::KindMismatch {
                expected: Self::KIND,
                actual: kind,
            });
        }
        let market = required(tags, "market")?.to_string();
        let outcome = required(tags, "outcome")?.parse()?;
        let signature_hex = required(tags, "sig")?.to_string();
        if hex::decode(&signature_hex).map(|b| b.len()) != Ok(64) {
            return Err(ProtocolError::MalformedTag {
                tag: "sig",
                detail: "Schnorr signature must be 64 bytes hex".into(),
            });
        }
        Ok(OracleAttestation {
            market,
            outcome,
            signature_hex,
        })
    }

    pub fn to_event_parts(&self) -> (Vec<TagTuple>, String) {
        (
            vec![
                vec!["market".into(), self.market.clone()],
                vec!["outcome".into(), self.outcome.as_str().into()],
                vec!["sig".into(), self.signature_hex.clone()],
            ],
            String::new(),
        )
    }

    /// Verify this attestation's signature against the given Schnorr pubkey.
    pub fn verify(&self, pubkey: &XOnlyPublicKey) -> Result<(), ProtocolError> {
        let secp = Secp256k1::verification_only();
        let sig_bytes = hex::decode(&self.signature_hex)?;
        let sig = Signature::from_slice(&sig_bytes)?;
        let digest = sha256_digest(&Self::signing_message(&self.market, self.outcome));
        secp.verify_schnorr(&sig, &digest, pubkey).map_err(Into::into)
    }
}

/// Oracle signing trait — uniform interface for single-key and FROST signers.
///
/// Implementations:
/// - [`SingleKeySigner`]: v1 single-secret-key Schnorr signing.
/// - `FrostSigner` (Phase 4, in `hunch-oracle`): k-of-n threshold Schnorr per HIP-4.
pub trait OracleSigner {
    /// Returns the oracle's public Schnorr key (x-only, 32 bytes).
    fn pubkey(&self) -> XOnlyPublicKey;

    /// Sign the canonical `(market, outcome)` message and produce the attestation.
    fn sign_attestation(&self, market: &str, outcome: Outcome) -> Result<OracleAttestation, ProtocolError>;
}

/// Single-key Schnorr signer for v1 oracles (HIP-4 §Parameters: implementations MAY
/// choose single-key for low-stakes markets).
pub struct SingleKeySigner {
    keypair: Keypair,
}

impl SingleKeySigner {
    pub fn new(secret_key: SecretKey) -> Self {
        let secp = Secp256k1::new();
        let keypair = Keypair::from_secret_key(&secp, &secret_key);
        SingleKeySigner { keypair }
    }

    pub fn from_secret_hex(hex_str: &str) -> Result<Self, ProtocolError> {
        let bytes = hex::decode(hex_str)?;
        if bytes.len() != 32 {
            return Err(ProtocolError::InvalidPubkey(hex_str.into()));
        }
        let sk = SecretKey::from_slice(&bytes)?;
        Ok(Self::new(sk))
    }

    pub fn pubkey_hex(&self) -> String {
        hex::encode(self.pubkey().serialize())
    }
}

impl OracleSigner for SingleKeySigner {
    fn pubkey(&self) -> XOnlyPublicKey {
        let pk = PublicKey::from_keypair(&self.keypair);
        XOnlyPublicKey::from(pk)
    }

    fn sign_attestation(&self, market: &str, outcome: Outcome) -> Result<OracleAttestation, ProtocolError> {
        let secp = Secp256k1::new();
        let digest = sha256_digest(&OracleAttestation::signing_message(market, outcome));
        let sig = secp.sign_schnorr_no_aux_rand(&digest, &self.keypair);
        Ok(OracleAttestation {
            market: market.into(),
            outcome,
            signature_hex: hex::encode(sig.as_ref()),
        })
    }
}

fn required<'a>(tags: &'a [TagTuple], key: &'static str) -> Result<&'a str, ProtocolError> {
    tags.iter()
        .find(|t| t.first().map(|k| k == key).unwrap_or(false))
        .and_then(|t| t.get(1).map(String::as_str))
        .ok_or(ProtocolError::MissingTag(key))
}

/// Local SHA-256 helper. Used to compress arbitrary-length signing messages into
/// the 32-byte digest BIP-340 / Schnorr signing expects.
fn sha256_digest(input: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(input);
    hasher.finalize().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn keypair_hex() -> &'static str {
        // Test-only, deterministic key. Not a real secret.
        "5f80b1ac81a47b0e3ee7e3bd4e23c1f3a96a0b56cd96b3a5d99e3a7a76d8c3a0"
    }

    #[test]
    fn signer_pubkey_is_32_bytes_hex() {
        let signer = SingleKeySigner::from_secret_hex(keypair_hex()).unwrap();
        let hex_str = signer.pubkey_hex();
        assert_eq!(hex_str.len(), 64);
        assert!(hex::decode(&hex_str).is_ok());
    }

    #[test]
    fn sign_and_verify_roundtrip() {
        let signer = SingleKeySigner::from_secret_hex(keypair_hex()).unwrap();
        let market = format!("{}:30888:btc-100k", "aa".repeat(32));
        for outcome in Outcome::ALL {
            let att = signer.sign_attestation(&market, outcome).unwrap();
            att.verify(&signer.pubkey()).unwrap();
        }
    }

    #[test]
    fn signature_does_not_verify_for_wrong_outcome() {
        let signer = SingleKeySigner::from_secret_hex(keypair_hex()).unwrap();
        let market = format!("{}:30888:btc-100k", "aa".repeat(32));
        let att = signer.sign_attestation(&market, Outcome::Yes).unwrap();
        // Tamper with outcome — verification must fail.
        let tampered = OracleAttestation {
            market: att.market.clone(),
            outcome: Outcome::No,
            signature_hex: att.signature_hex,
        };
        assert!(tampered.verify(&signer.pubkey()).is_err());
    }

    #[test]
    fn signing_message_format_is_stable() {
        let msg = OracleAttestation::signing_message("pk:30888:m", Outcome::Yes);
        assert_eq!(msg, b"hunch/oracle/v1\npk:30888:m\nYES");
    }

    #[test]
    fn announce_parse_and_roundtrip() {
        let tags = vec![
            vec!["market".into(), format!("{}:30888:btc-100k", "aa".repeat(32))],
            vec!["nonce".into(), "bb".repeat(32)],
        ];
        let a = OracleAnnounce::from_event(KIND_ORACLE_ANNOUNCE, &tags, "Resolves on CB feed").unwrap();
        let (out_tags, out_content) = a.to_event_parts();
        let a2 = OracleAnnounce::from_event(KIND_ORACLE_ANNOUNCE, &out_tags, &out_content).unwrap();
        assert_eq!(a, a2);
    }

    #[test]
    fn attestation_parse_rejects_bad_sig_len() {
        let tags = vec![
            vec!["market".into(), "m".into()],
            vec!["outcome".into(), "YES".into()],
            vec!["sig".into(), "ab".into()],
        ];
        assert!(OracleAttestation::from_event(KIND_ORACLE_ATTESTATION, &tags, "").is_err());
    }
}
