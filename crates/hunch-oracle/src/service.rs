//! Oracle service — single-key (v1) oracle identity that signs NIP-88 announce/attestation
//! events and produces HIP-2 outcome attestations.
//!
//! The same secp256k1 key is the oracle's Nostr identity (signs the kind 88/89 events) and
//! its HIP-2 attestation key (the inner Schnorr signature over the canonical
//! `hunch/oracle/v1` message, produced by [`SingleKeySigner`]). FROST k-of-n threshold
//! signing (HIP-4) is deferred to Phase 4 and slots in behind the same `OracleSigner` trait.

use anyhow::{Context, Result};
use hunch_protocol::oracle::{OracleAnnounce, OracleAttestation, OracleSigner, SingleKeySigner};
use hunch_protocol::outcome::Outcome;
use secp256k1::{All, Keypair, Secp256k1, SecretKey};
use serde_json::Value;

use crate::event::{build_signed_event, Tag};

/// A v1 single-key Hunch oracle.
pub struct OracleService {
    signer: SingleKeySigner,
    keypair: Keypair,
    secp: Secp256k1<All>,
}

impl OracleService {
    /// Builds an oracle from a 32-byte secret key in hex.
    pub fn from_secret_hex(secret_hex: &str) -> Result<Self> {
        let secret_hex = secret_hex.trim();
        let bytes = hex::decode(secret_hex).context("oracle secret key is not valid hex")?;
        if bytes.len() != 32 {
            anyhow::bail!("oracle secret key must be 32 bytes ({} hex chars), got {}", 64, secret_hex.len());
        }
        let sk = SecretKey::from_slice(&bytes).context("oracle secret key is not a valid secp256k1 scalar")?;
        let secp = Secp256k1::new();
        let keypair = Keypair::from_secret_key(&secp, &sk);
        let signer = SingleKeySigner::new(sk);
        Ok(OracleService { signer, keypair, secp })
    }

    /// The oracle's x-only public key as hex (its Nostr pubkey and HIP-2 attestation key).
    pub fn pubkey_hex(&self) -> String {
        let (xonly, _) = self.keypair.x_only_public_key();
        hex::encode(xonly.serialize())
    }

    /// Builds a signed NIP-88 announce event (kind 88) committing to attest `market`.
    ///
    /// `nonce_pubkey` is the oracle's announced public nonce R for this market (hex, 32 bytes).
    ///
    /// NOTE (v1 limitation): binding the announced R into the attestation as a DLC adapter
    /// signature (so `announced R == attestation R`) is deferred to the mint/DLC integration
    /// (SPIKE-02). The current attestation uses a BIP-340 deterministic nonce, so the announce
    /// is a well-formed commitment but not yet adapter-bound. This is documented loudly per
    /// the cypherpunk "document centralized/incomplete trust assumptions" rule.
    pub fn build_announce_event(
        &self,
        market: &str,
        nonce_pubkey: &str,
        body: &str,
        created_at: i64,
    ) -> Result<Value> {
        let announce = OracleAnnounce {
            market: market.to_string(),
            nonce_pubkey: nonce_pubkey.to_string(),
            body: body.to_string(),
        };
        // Re-validate via the protocol parser so we never publish a malformed announce.
        let (tags, content) = announce.to_event_parts();
        OracleAnnounce::from_event(OracleAnnounce::KIND, &tags, &content)
            .context("constructed announce failed protocol validation")?;
        Ok(self.sign(OracleAnnounce::KIND, tags, content, created_at))
    }

    /// Signs the outcome and builds a signed NIP-88 attestation event (kind 89).
    ///
    /// Returns both the wire event and the parsed [`OracleAttestation`] so callers can log /
    /// verify the inner HIP-2 signature before publishing.
    pub fn build_attestation_event(
        &self,
        market: &str,
        outcome: Outcome,
        created_at: i64,
    ) -> Result<(Value, OracleAttestation)> {
        let attestation = self
            .signer
            .sign_attestation(market, outcome)
            .context("signing attestation")?;
        // Defense in depth: verify our own signature before broadcasting it.
        attestation
            .verify(&self.signer.pubkey())
            .context("self-verification of attestation signature failed")?;
        let (tags, content) = attestation.to_event_parts();
        let event = self.sign(OracleAttestation::KIND, tags, content, created_at);
        Ok((event, attestation))
    }

    fn sign(&self, kind: u32, tags: Vec<Tag>, content: String, created_at: i64) -> Value {
        build_signed_event(&self.secp, &self.keypair, kind, tags, content, created_at)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_SECRET: &str = "5f80b1ac81a47b0e3ee7e3bd4e23c1f3a96a0b56cd96b3a5d99e3a7a76d8c3a0";

    fn market_id() -> String {
        format!("{}:30888:btc-100k", "aa".repeat(32))
    }

    #[test]
    fn pubkey_is_32_byte_hex() {
        let oracle = OracleService::from_secret_hex(TEST_SECRET).unwrap();
        assert_eq!(oracle.pubkey_hex().len(), 64);
    }

    #[test]
    fn rejects_bad_secret() {
        assert!(OracleService::from_secret_hex("zz").is_err());
        assert!(OracleService::from_secret_hex(&"00".repeat(31)).is_err());
    }

    #[test]
    fn attestation_event_is_well_formed_and_self_verifies() {
        let oracle = OracleService::from_secret_hex(TEST_SECRET).unwrap();
        let (event, att) = oracle
            .build_attestation_event(&market_id(), Outcome::Yes, 1_700_000_000)
            .unwrap();

        assert_eq!(event["kind"], 89);
        assert_eq!(event["pubkey"].as_str().unwrap(), oracle.pubkey_hex());
        // The inner HIP-2 attestation verifies under the oracle pubkey.
        att.verify(&oracle.signer.pubkey()).unwrap();
        // The wire event carries the outcome + sig tags.
        let tags = event["tags"].as_array().unwrap();
        assert!(tags.iter().any(|t| t[0] == "outcome" && t[1] == "YES"));
    }

    #[test]
    fn announce_event_is_well_formed() {
        let oracle = OracleService::from_secret_hex(TEST_SECRET).unwrap();
        let event = oracle
            .build_announce_event(&market_id(), &"bb".repeat(32), "Resolves on CB feed", 1_700_000_000)
            .unwrap();
        assert_eq!(event["kind"], 88);
        let tags = event["tags"].as_array().unwrap();
        assert!(tags.iter().any(|t| t[0] == "market"));
        assert!(tags.iter().any(|t| t[0] == "nonce"));
    }

    #[test]
    fn announce_rejects_bad_nonce_length() {
        let oracle = OracleService::from_secret_hex(TEST_SECRET).unwrap();
        assert!(oracle
            .build_announce_event(&market_id(), "bb", "body", 1)
            .is_err());
    }
}
