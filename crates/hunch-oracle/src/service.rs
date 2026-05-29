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

use crate::dlc;
use hunch_nostr::event::{build_signed_event, Tag};

/// A v1 single-key Hunch oracle.
pub struct OracleService {
    signer: SingleKeySigner,
    keypair: Keypair,
    secret_bytes: [u8; 32],
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
        let secret_bytes: [u8; 32] = bytes.clone().try_into().expect("length checked above");
        let sk = SecretKey::from_slice(&bytes).context("oracle secret key is not a valid secp256k1 scalar")?;
        let secp = Secp256k1::new();
        let keypair = Keypair::from_secret_key(&secp, &sk);
        let signer = SingleKeySigner::new(sk);
        Ok(OracleService { signer, keypair, secret_bytes, secp })
    }

    /// The oracle's x-only public key as hex (its Nostr pubkey and HIP-2 attestation key).
    pub fn pubkey_hex(&self) -> String {
        let (xonly, _) = self.keypair.x_only_public_key();
        hex::encode(xonly.serialize())
    }

    /// Builds a signed NIP-88 announce event (kind 88) committing to attest `market`.
    ///
    /// `nonce_pubkey` is the oracle's announced public nonce R for this market (hex, 32 bytes).
    /// This R is binding: [`build_attestation_event`](Self::build_attestation_event) signs the
    /// outcome with the matching nonce secret, so the mint can compute the per-outcome
    /// signature point `R + e·P` in advance and adaptor-encrypt the CET to it.
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

    /// Signs the outcome with the pre-committed `nonce_secret_hex` (the secret behind the
    /// announced R) and builds a signed NIP-88 attestation event (kind 89).
    ///
    /// The signature is a DLC oracle attestation (BIP-340 with the announced nonce), produced
    /// by `ddk-dlc` — not custom crypto. Returns both the wire event and the parsed
    /// [`OracleAttestation`] so callers can log / verify before publishing.
    ///
    /// SAFETY: the caller MUST guarantee `nonce_secret_hex` is used for exactly one outcome on
    /// this market (see [`crate::nonce_store`]). Reuse across outcomes leaks the oracle key.
    pub fn build_attestation_event(
        &self,
        market: &str,
        outcome: Outcome,
        nonce_secret_hex: &str,
        created_at: i64,
    ) -> Result<(Value, OracleAttestation)> {
        let nonce_bytes: [u8; 32] = hex::decode(nonce_secret_hex.trim())
            .context("nonce secret is not valid hex")?
            .try_into()
            .map_err(|_| anyhow::anyhow!("nonce secret must be 32 bytes"))?;
        let signature_hex =
            dlc::sign_attestation_with_nonce(&self.secret_bytes, &nonce_bytes, market, outcome)
                .context("DLC attestation signing")?;
        let attestation = OracleAttestation {
            market: market.to_string(),
            outcome,
            signature_hex,
        };
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

    const TEST_NONCE: &str = "a1b2c3d4e5f60718293a4b5c6d7e8f90112233445566778899aabbccddeeff00";

    #[test]
    fn attestation_event_is_well_formed_and_self_verifies() {
        let oracle = OracleService::from_secret_hex(TEST_SECRET).unwrap();
        let (event, att) = oracle
            .build_attestation_event(&market_id(), Outcome::Yes, TEST_NONCE, 1_700_000_000)
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
