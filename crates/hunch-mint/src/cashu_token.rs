//! Real Cashu (NUT-11) integration — proves Hunch outcome tokens are valid, spendable proofs.
//!
//! `token.rs` models the HIP-3 secret as hand-built JSON (dependency-light). This module uses
//! the actual `cashu` crate — the same code a CDK mint runs — to (a) build the NUT-11 secret for
//! an outcome lock and (b) prove that the oracle attestation yields a spend key that produces a
//! `verify_p2pk`-valid proof. If real Cashu accepts it here, a real mint accepts it in production.
//!
//! Scope: the core OUTCOME_MATCH path (lock to `L_X`, spend with `l_X = b + s_X`). The
//! refund/locktime branch (NUT-11 `Conditions`) is represented in [`crate::token`]'s wire secret
//! and is a straightforward follow-up on the same `SpendingConditions` API.

use std::str::FromStr;

use anyhow::{Context, Result};
use cashu::nuts::{Conditions, Proof, PublicKey, SigFlag, SpendingConditions};
use cashu::secret::Secret;
use cashu::Amount;

/// Builds the real Cashu NUT-11 secret for an outcome token locked to `lock_pubkey_hex` (`L_X`),
/// with no refund branch (core OUTCOME_MATCH only).
pub fn p2pk_secret(lock_pubkey_hex: &str) -> Result<Secret> {
    let pubkey = PublicKey::from_hex(lock_pubkey_hex).context("invalid lock pubkey hex")?;
    let conditions = SpendingConditions::new_p2pk(pubkey, None);
    let secret: Secret = conditions.try_into().context("building NUT-11 secret")?;
    Ok(secret)
}

/// Builds the full HIP-3 outcome-token secret: P2PK to `lock_pubkey_hex` (`L_X`) with a NUT-11
/// refund branch (`refund` key spendable after `refund_timeout`). This is the production form —
/// winners redeem via the outcome path immediately; losers / INVALID reclaim via the refund path
/// after the timeout. `refund_timeout` must be in the future (NUT-11 `Conditions::new` rejects a
/// past locktime).
pub fn outcome_secret(lock_pubkey_hex: &str, refund_pubkey_hex: &str, refund_timeout: u64) -> Result<Secret> {
    let lock = PublicKey::from_hex(lock_pubkey_hex).context("invalid lock pubkey hex")?;
    let refund = PublicKey::from_hex(refund_pubkey_hex).context("invalid refund pubkey hex")?;
    let conditions = Conditions::new(
        Some(refund_timeout),
        None,
        Some(vec![refund]),
        None,
        Some(SigFlag::SigInputs),
        None,
    )
    .context("building NUT-11 refund conditions")?;
    let secret: Secret = SpendingConditions::new_p2pk(lock, Some(conditions))
        .try_into()
        .context("building NUT-11 secret")?;
    Ok(secret)
}

/// Builds an unblinded Cashu proof for the outcome lock, as a mint would issue it.
///
/// `c` (the unblinded signature point) is a placeholder here — `verify_p2pk` checks only the
/// witness against the secret's `data`, not `c`. A live mint supplies the real blind signature.
pub fn p2pk_proof(amount_sat: u64, lock_pubkey_hex: &str) -> Result<Proof> {
    let secret = p2pk_secret(lock_pubkey_hex)?;
    let c = PublicKey::from_hex(lock_pubkey_hex)?; // any valid point; not checked by verify_p2pk
    let keyset_id = cashu::nuts::Id::from_str("009a1f293253e41e").context("keyset id")?;
    Ok(Proof::new(Amount::from(amount_sat), keyset_id, secret, c))
}

#[cfg(test)]
mod tests {
    use super::*;
    use cashu::nuts::SecretKey;
    use hunch_dlc::{outcome_lock_key, outcome_unlock_secret, sign_attestation_with_nonce};
    use hunch_protocol::outcome::Outcome;
    use secp256k1::{Keypair, PublicKey as SPublicKey, Secp256k1, SecretKey as SSecretKey};

    const ORACLE: &str = "5f80b1ac81a47b0e3ee7e3bd4e23c1f3a96a0b56cd96b3a5d99e3a7a76d8c3a0";
    const NONCE: &str = "a1b2c3d4e5f60718293a4b5c6d7e8f90112233445566778899aabbccddeeff00";
    const BETTOR: &str = "1111111111111111111111111111111111111111111111111111111111111111";

    fn xonly(secret_hex: &str) -> String {
        let sk = SSecretKey::from_slice(&hex::decode(secret_hex).unwrap()).unwrap();
        hex::encode(Keypair::from_secret_key(&Secp256k1::new(), &sk).x_only_public_key().0.serialize())
    }
    fn bettor_pub() -> String {
        let sk = SSecretKey::from_slice(&hex::decode(BETTOR).unwrap()).unwrap();
        hex::encode(SPublicKey::from_secret_key(&Secp256k1::new(), &sk).serialize())
    }
    fn market() -> String {
        format!("{}:30888:m", "aa".repeat(32))
    }
    fn lock(outcome: Outcome) -> String {
        outcome_lock_key(&bettor_pub(), &xonly(ORACLE), &xonly(NONCE), &market(), outcome).unwrap()
    }
    fn attest(outcome: Outcome) -> String {
        let o: [u8; 32] = hex::decode(ORACLE).unwrap().try_into().unwrap();
        let n: [u8; 32] = hex::decode(NONCE).unwrap().try_into().unwrap();
        sign_attestation_with_nonce(&o, &n, &market(), outcome).unwrap()
    }

    #[test]
    fn p2pk_secret_is_a_valid_nut11_p2pk_secret() {
        // It must round-trip into a cashu nut10 P2PK secret whose data is our lock key.
        let lock_yes = lock(Outcome::Yes);
        let secret = p2pk_secret(&lock_yes).unwrap();
        let nut10: cashu::nuts::nut10::Secret = secret.try_into().unwrap();
        assert_eq!(nut10.secret_data().data(), lock_yes);
    }

    #[test]
    fn yes_attestation_yields_a_verify_p2pk_valid_proof() {
        // The whole point: oracle attests YES → bettor derives l_YES → signs the real cashu
        // proof → cashu's own verify_p2pk accepts it. This is what a CDK mint runs.
        let mut proof = p2pk_proof(1000, &lock(Outcome::Yes)).unwrap();
        let l_yes = outcome_unlock_secret(BETTOR, &attest(Outcome::Yes)).unwrap();
        let sk = SecretKey::from_hex(&l_yes).unwrap();
        proof.sign_p2pk(sk).unwrap();
        proof.verify_p2pk().expect("YES token must verify under real cashu NUT-11");
    }

    #[test]
    fn yes_attestation_cannot_spend_the_no_token() {
        // l_YES must NOT satisfy the NO token's lock (L_NO = B + S_NO).
        let mut no_proof = p2pk_proof(1000, &lock(Outcome::No)).unwrap();
        let l_yes = outcome_unlock_secret(BETTOR, &attest(Outcome::Yes)).unwrap();
        let sk = SecretKey::from_hex(&l_yes).unwrap();
        no_proof.sign_p2pk(sk).unwrap();
        assert!(no_proof.verify_p2pk().is_err(), "YES key must not spend the NO token");
    }

    fn now() -> u64 {
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()
    }

    // --- Refund / INVALID branch (NUT-11 locktime + refund), against real cashu ---

    fn proof_with_secret(secret: Secret) -> Proof {
        let c = PublicKey::from_hex(&lock(Outcome::Yes)).unwrap();
        let id = cashu::nuts::Id::from_str("009a1f293253e41e").unwrap();
        Proof::new(Amount::from(1000u64), id, secret, c)
    }

    #[test]
    fn winner_redeems_via_outcome_path_despite_future_locktime() {
        // Full token: outcome lock + refund branch with a far-future timeout.
        let secret = outcome_secret(&lock(Outcome::Yes), &bettor_pub(), now() + 1_000_000).unwrap();
        let mut proof = proof_with_secret(secret);
        let l_yes = outcome_unlock_secret(BETTOR, &attest(Outcome::Yes)).unwrap();
        proof.sign_p2pk(SecretKey::from_hex(&l_yes).unwrap()).unwrap();
        proof.verify_p2pk().expect("winner spends the outcome path regardless of locktime");
    }

    #[test]
    fn refund_key_cannot_spend_before_locktime() {
        // Future locktime: the bettor's refund key alone must NOT spend yet (and it isn't the
        // outcome key either), so neither NUT-11 path is satisfied.
        let secret = outcome_secret(&lock(Outcome::Yes), &bettor_pub(), now() + 1_000_000).unwrap();
        let mut proof = proof_with_secret(secret);
        proof.sign_p2pk(SecretKey::from_hex(BETTOR).unwrap()).unwrap();
        assert!(proof.verify_p2pk().is_err(), "refund key must not spend before locktime");
    }

    #[test]
    fn refund_key_spends_after_locktime() {
        // Past locktime (built via struct literal to bypass new()'s future-locktime check):
        // the refund branch is active and the bettor's key reclaims the token (INVALID / silence).
        let lock_pk = PublicKey::from_hex(&lock(Outcome::Yes)).unwrap();
        let refund_pk = PublicKey::from_hex(&bettor_pub()).unwrap();
        let conditions = Conditions {
            locktime: Some(1),
            pubkeys: None,
            refund_keys: Some(vec![refund_pk]),
            num_sigs: None,
            sig_flag: SigFlag::SigInputs,
            num_sigs_refund: None,
        };
        let secret: Secret = SpendingConditions::new_p2pk(lock_pk, Some(conditions)).try_into().unwrap();
        let mut proof = proof_with_secret(secret);
        proof.sign_p2pk(SecretKey::from_hex(BETTOR).unwrap()).unwrap();
        proof.verify_p2pk().expect("refund key spends after locktime (HIP-2 refund / INVALID)");
    }
}
