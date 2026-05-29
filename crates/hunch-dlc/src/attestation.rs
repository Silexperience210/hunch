//! DLC oracle attestation — Schnorr signing with a pre-committed nonce.
//!
//! A DLC oracle does NOT sign with a fresh BIP-340 nonce: it commits to a nonce point
//! `R = k·G` at announce time, then signs the resolved outcome with that exact `k`. This is
//! what lets a counterparty (the mint) compute the per-outcome *signature point*
//! `S = R + e·P` in advance. When the oracle reveals the signature `(R, s)`, `s` is the
//! discrete log of `S` — the secret that unlocks the matching outcome's ecash (see
//! [`crate::conditional`]).
//!
//! We do NOT implement this signing flow ourselves (CLAUDE.md: "Never roll your own nonces /
//! signing flows"). We use `ddk-dlc`'s `secp_utils`, the maintained rust-dlc primitives, which
//! wrap libsecp256k1-zkp. This module only owns the byte-level bridge to `hunch-protocol`'s
//! types plus the canonical message (identical bytes to `hunch_protocol::oracle::OracleAttestation`).
//!
//! ## Nonce reuse is catastrophic
//!
//! Signing two different outcomes with the same nonce `k` reveals the oracle's secret key
//! (`x = (s1 - s2) / (e1 - e2)`). The caller MUST guarantee one attestation per announced
//! nonce — see `hunch-oracle`'s nonce store.

use anyhow::{Context, Result};
use ddk_dlc::secp256k1_zkp as zkp;
use ddk_dlc::secp_utils::{schnorrsig_compute_sig_point, schnorrsig_sign_with_nonce};
use hunch_protocol::oracle::OracleAttestation;
use hunch_protocol::outcome::Outcome;
use sha2::{Digest, Sha256};

/// The 32-byte digest the oracle signs for `(market, outcome)`.
///
/// Identical bytes to what `OracleAttestation::verify` checks: `sha256` of the canonical
/// `hunch/oracle/v1\n<market>\n<outcome>` message.
pub(crate) fn attestation_digest(market: &str, outcome: Outcome) -> [u8; 32] {
    let msg = OracleAttestation::signing_message(market, outcome);
    Sha256::digest(&msg).into()
}

/// Signs `(market, outcome)` with the pre-committed `nonce_secret`, returning the 64-byte
/// BIP-340 signature as hex. The signature is bound to `R = nonce_secret·G`.
pub fn sign_attestation_with_nonce(
    secret_bytes: &[u8; 32],
    nonce_secret: &[u8; 32],
    market: &str,
    outcome: Outcome,
) -> Result<String> {
    let secp = zkp::Secp256k1::new();
    let sk = zkp::SecretKey::from_slice(secret_bytes).context("oracle secret invalid for zkp")?;
    let keypair = zkp::Keypair::from_secret_key(&secp, &sk);
    let msg = zkp::Message::from_digest(attestation_digest(market, outcome));
    let sig = schnorrsig_sign_with_nonce(&secp, &msg, &keypair, nonce_secret);
    Ok(hex::encode(sig.as_ref()))
}

/// Computes the per-outcome signature point `S = R + e·P`, the public key whose secret the
/// oracle reveals when it attests `outcome`.
///
/// `oracle_pubkey` and `nonce_pubkey` are x-only hex (32 bytes). Returned as 33-byte
/// compressed-point hex.
pub fn signature_point(
    oracle_pubkey_xonly_hex: &str,
    nonce_pubkey_xonly_hex: &str,
    market: &str,
    outcome: Outcome,
) -> Result<String> {
    let secp = zkp::Secp256k1::new();
    let pubkey = zkp::XOnlyPublicKey::from_slice(&hex::decode(oracle_pubkey_xonly_hex)?)
        .context("oracle pubkey invalid")?;
    let nonce = zkp::XOnlyPublicKey::from_slice(&hex::decode(nonce_pubkey_xonly_hex)?)
        .context("nonce pubkey invalid")?;
    let msg = zkp::Message::from_digest(attestation_digest(market, outcome));
    let point = schnorrsig_compute_sig_point(&secp, &pubkey, &nonce, &msg)
        .map_err(|e| anyhow::anyhow!("compute sig point: {e}"))?;
    Ok(hex::encode(point.serialize()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use secp256k1::{Keypair as Kp, Secp256k1 as Secp, SecretKey as Sk};

    const ORACLE_SECRET: &str = "5f80b1ac81a47b0e3ee7e3bd4e23c1f3a96a0b56cd96b3a5d99e3a7a76d8c3a0";
    const NONCE_SECRET: &str = "a1b2c3d4e5f60718293a4b5c6d7e8f90112233445566778899aabbccddeeff00";

    pub(crate) fn xonly_hex(secret_hex: &str) -> String {
        let sk = Sk::from_slice(&hex::decode(secret_hex).unwrap()).unwrap();
        let kp = Kp::from_secret_key(&Secp::new(), &sk);
        hex::encode(kp.x_only_public_key().0.serialize())
    }

    #[test]
    fn attestation_with_nonce_verifies_under_protocol_verifier() {
        let secret: [u8; 32] = hex::decode(ORACLE_SECRET).unwrap().try_into().unwrap();
        let nonce: [u8; 32] = hex::decode(NONCE_SECRET).unwrap().try_into().unwrap();
        let market = format!("{}:30888:btc-100k", "aa".repeat(32));

        let sig_hex = sign_attestation_with_nonce(&secret, &nonce, &market, Outcome::Yes).unwrap();

        let att = OracleAttestation {
            market: market.clone(),
            outcome: Outcome::Yes,
            signature_hex: sig_hex,
        };
        let oracle_xonly =
            secp256k1::XOnlyPublicKey::from_slice(&hex::decode(xonly_hex(ORACLE_SECRET)).unwrap())
                .unwrap();
        att.verify(&oracle_xonly).unwrap();
    }

    #[test]
    fn signature_s_equals_sig_point_dlog() {
        let secret: [u8; 32] = hex::decode(ORACLE_SECRET).unwrap().try_into().unwrap();
        let nonce: [u8; 32] = hex::decode(NONCE_SECRET).unwrap().try_into().unwrap();
        let market = format!("{}:30888:btc-100k", "aa".repeat(32));

        let sig_hex = sign_attestation_with_nonce(&secret, &nonce, &market, Outcome::No).unwrap();
        let sig_bytes = hex::decode(&sig_hex).unwrap();
        let s_scalar: [u8; 32] = sig_bytes[32..64].try_into().unwrap();

        let secp = zkp::Secp256k1::new();
        let s_g =
            zkp::PublicKey::from_secret_key(&secp, &zkp::SecretKey::from_slice(&s_scalar).unwrap());

        let sig_point_hex = signature_point(
            &xonly_hex(ORACLE_SECRET),
            &xonly_hex(NONCE_SECRET),
            &market,
            Outcome::No,
        )
        .unwrap();

        assert_eq!(
            hex::encode(s_g.serialize()),
            sig_point_hex,
            "s·G must equal the sig point R + e·P"
        );
    }

    #[test]
    fn different_nonces_give_different_signatures() {
        let secret: [u8; 32] = hex::decode(ORACLE_SECRET).unwrap().try_into().unwrap();
        let n1: [u8; 32] = hex::decode(NONCE_SECRET).unwrap().try_into().unwrap();
        let mut n2 = n1;
        n2[0] ^= 0x01;
        let s1 = sign_attestation_with_nonce(&secret, &n1, "m", Outcome::Yes).unwrap();
        let s2 = sign_attestation_with_nonce(&secret, &n2, "m", Outcome::Yes).unwrap();
        assert_ne!(s1, s2);
    }
}
