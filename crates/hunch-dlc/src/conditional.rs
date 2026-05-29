//! Outcome-conditional ecash locks — HIP-3 OUTCOME_MATCH on stable NUT-11 P2PK.
//!
//! ## Why this exists (SPIKE-02 resolution)
//!
//! HIP-3 originally targeted NUT-CTF (cashubtc/nuts#337) for outcome-conditional tokens, but
//! that PR is still draft/WIP and unstable. We don't need it. The DLC oracle already gives us,
//! for each outcome X, a public point `S_X = R + e_X·P` whose discrete log `s_X` is revealed
//! *only* when the oracle attests X (the kind:89 attestation signature; see [`crate::attestation`]).
//!
//! So we lock a bettor's outcome token to `L = B + S_X`, where `B` is the bettor's pubkey. The
//! token is a vanilla NUT-11 P2PK proof — the mint enforces only "valid Schnorr signature under
//! `L`", knowing nothing about DLCs. The spend secret is `l = b + s_X`:
//!
//! - The bettor always knows `b`.
//! - `s_X` is unknown to everyone until the oracle attests X.
//! - Therefore the token is spendable iff (holder knows `b`) AND (oracle attested X).
//!
//! Tokens for the non-winning outcomes (`L = B + S_NO`, `B + S_INVALID`) never become spendable
//! because the oracle reveals exactly one `s_X` (enforced by the nonce-reuse guard). The HIP-2
//! refund branch is handled by NUT-11's `locktime` + `refund` tags (a mint concern, see
//! `hunch-mint`). No custom mint NUT is required.

use anyhow::{Context, Result};
use ddk_dlc::secp256k1_zkp as zkp;
use hunch_protocol::outcome::Outcome;

use crate::attestation::signature_point;

/// Computes the outcome lock key `L = B + S_X` as a 33-byte compressed pubkey hex.
///
/// `bettor_pubkey_compressed_hex` is the bettor's 33-byte compressed secp256k1 pubkey (NUT-11
/// locks to compressed keys). `oracle`/`nonce` are x-only hex.
pub fn outcome_lock_key(
    bettor_pubkey_compressed_hex: &str,
    oracle_pubkey_xonly_hex: &str,
    nonce_pubkey_xonly_hex: &str,
    market: &str,
    outcome: Outcome,
) -> Result<String> {
    let s_x_hex = signature_point(oracle_pubkey_xonly_hex, nonce_pubkey_xonly_hex, market, outcome)?;
    let s_x = zkp::PublicKey::from_slice(&hex::decode(s_x_hex)?).context("sig point invalid")?;
    let bettor = zkp::PublicKey::from_slice(
        &hex::decode(bettor_pubkey_compressed_hex).context("bettor pubkey hex")?,
    )
    .context("bettor pubkey must be a 33-byte compressed secp256k1 point")?;
    let lock = bettor.combine(&s_x).map_err(|e| anyhow::anyhow!("combining B + S_X: {e}"))?;
    Ok(hex::encode(lock.serialize()))
}

/// Derives the spend secret `l = b + s_X` from the bettor's secret and the oracle's attestation
/// signature. `s_X` is the scalar half (last 32 bytes) of the 64-byte BIP-340 signature.
pub fn outcome_unlock_secret(bettor_secret_hex: &str, attestation_signature_hex: &str) -> Result<String> {
    let sig = hex::decode(attestation_signature_hex.trim()).context("attestation sig hex")?;
    if sig.len() != 64 {
        anyhow::bail!("attestation signature must be 64 bytes, got {}", sig.len());
    }
    let s_x: [u8; 32] = sig[32..64].try_into().expect("32-byte slice");

    let bettor = zkp::SecretKey::from_slice(
        &hex::decode(bettor_secret_hex.trim()).context("bettor secret hex")?,
    )
    .context("bettor secret invalid")?;
    let tweak = zkp::Scalar::from_be_bytes(s_x).context("s_X is not a valid scalar")?;
    let spend = bettor.add_tweak(&tweak).map_err(|e| anyhow::anyhow!("b + s_X: {e}"))?;
    Ok(hex::encode(spend.secret_bytes()))
}

/// Returns true iff `spend_secret·G == lock_key`, i.e. the secret can sign for the lock.
pub fn verify_unlock(spend_secret_hex: &str, lock_key_compressed_hex: &str) -> Result<bool> {
    let secp = zkp::Secp256k1::new();
    let sk = zkp::SecretKey::from_slice(&hex::decode(spend_secret_hex.trim())?).context("spend secret invalid")?;
    let pk = zkp::PublicKey::from_secret_key(&secp, &sk);
    Ok(hex::encode(pk.serialize()) == lock_key_compressed_hex.trim())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::attestation::sign_attestation_with_nonce;

    // Deterministic test material.
    const ORACLE: &str = "5f80b1ac81a47b0e3ee7e3bd4e23c1f3a96a0b56cd96b3a5d99e3a7a76d8c3a0";
    const NONCE: &str = "a1b2c3d4e5f60718293a4b5c6d7e8f90112233445566778899aabbccddeeff00";
    const BETTOR: &str = "1111111111111111111111111111111111111111111111111111111111111111";

    fn xonly(secret_hex: &str) -> String {
        let secp = zkp::Secp256k1::new();
        let sk = zkp::SecretKey::from_slice(&hex::decode(secret_hex).unwrap()).unwrap();
        let kp = zkp::Keypair::from_secret_key(&secp, &sk);
        hex::encode(kp.x_only_public_key().0.serialize())
    }

    fn bettor_compressed() -> String {
        let secp = zkp::Secp256k1::new();
        let sk = zkp::SecretKey::from_slice(&hex::decode(BETTOR).unwrap()).unwrap();
        hex::encode(zkp::PublicKey::from_secret_key(&secp, &sk).serialize())
    }

    #[test]
    fn winning_outcome_attestation_unlocks_its_token() {
        let market = format!("{}:30888:m", "aa".repeat(32));
        let lock_yes =
            outcome_lock_key(&bettor_compressed(), &xonly(ORACLE), &xonly(NONCE), &market, Outcome::Yes).unwrap();

        // Oracle attests YES → reveals s_YES.
        let oracle_secret: [u8; 32] = hex::decode(ORACLE).unwrap().try_into().unwrap();
        let nonce_secret: [u8; 32] = hex::decode(NONCE).unwrap().try_into().unwrap();
        let sig = sign_attestation_with_nonce(&oracle_secret, &nonce_secret, &market, Outcome::Yes).unwrap();

        let spend = outcome_unlock_secret(BETTOR, &sig).unwrap();
        assert!(verify_unlock(&spend, &lock_yes).unwrap(), "YES attestation must unlock the YES token");
    }

    #[test]
    fn winning_attestation_does_not_unlock_other_outcome_tokens() {
        let market = format!("{}:30888:m", "aa".repeat(32));
        let lock_no =
            outcome_lock_key(&bettor_compressed(), &xonly(ORACLE), &xonly(NONCE), &market, Outcome::No).unwrap();

        // Oracle attests YES; the revealed s_YES must NOT unlock the NO token.
        let oracle_secret: [u8; 32] = hex::decode(ORACLE).unwrap().try_into().unwrap();
        let nonce_secret: [u8; 32] = hex::decode(NONCE).unwrap().try_into().unwrap();
        let sig_yes = sign_attestation_with_nonce(&oracle_secret, &nonce_secret, &market, Outcome::Yes).unwrap();

        let spend = outcome_unlock_secret(BETTOR, &sig_yes).unwrap();
        assert!(!verify_unlock(&spend, &lock_no).unwrap(), "YES attestation must NOT unlock the NO token");
    }

    #[test]
    fn lock_key_is_33_byte_compressed() {
        let market = format!("{}:30888:m", "aa".repeat(32));
        let lock =
            outcome_lock_key(&bettor_compressed(), &xonly(ORACLE), &xonly(NONCE), &market, Outcome::Yes).unwrap();
        assert_eq!(hex::decode(&lock).unwrap().len(), 33);
    }

    #[test]
    fn unlock_rejects_bad_signature_length() {
        assert!(outcome_unlock_secret(BETTOR, "abcd").is_err());
    }
}
