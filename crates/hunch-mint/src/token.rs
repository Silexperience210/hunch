//! HIP-3 outcome tokens as NUT-11 P2PK secrets.
//!
//! Each outcome token is a NUT-11 well-known secret `["P2PK", {nonce, data, tags}]` where:
//! - `data` = the outcome lock key `L = B + S_X` (spendable only after the oracle attests X),
//! - a `refund` tag = the bettor's pubkey, and a `locktime` tag = the HIP-2 refund timeout, so
//!   that under INVALID / no-attestation the bettor reclaims after the timeout (NUT-11 refund
//!   branch — this is how HIP-2's refund mechanics map onto stable Cashu).
//!
//! The mint enforces only NUT-11; all DLC conditionality is in `data`. See [`crate`] docs.

use anyhow::{Context, Result};
use hunch_dlc::{outcome_lock_key, outcome_unlock_secret, verify_unlock};
use hunch_protocol::outcome::Outcome;
use serde_json::{json, Value};

/// A derived outcome token: its lock key and the NUT-11 secret string to be minted.
#[derive(Debug, Clone)]
pub struct OutcomeToken {
    pub outcome: Outcome,
    /// `L = B + S_X`, 33-byte compressed pubkey hex (the NUT-11 `data`).
    pub lock_pubkey: String,
    /// The full NUT-11 P2PK secret JSON string.
    pub secret: String,
}

/// Builds the NUT-11 outcome token for `outcome` on `market`.
///
/// - `bettor_pubkey` / `refund_pubkey`: 33-byte compressed secp256k1 pubkeys (the latter usually
///   equals the former — the bettor reclaims on refund).
/// - `oracle_xonly` / `nonce_xonly`: the market's oracle key and announced nonce (x-only hex).
/// - `refund_timeout`: unix seconds; before it, only the outcome branch can spend; after it, the
///   refund key can.
/// - `secret_nonce_hex`: the NUT-11 secret's random nonce (uniqueness), not the DLC nonce.
#[allow(clippy::too_many_arguments)]
pub fn build_outcome_token(
    bettor_pubkey: &str,
    refund_pubkey: &str,
    oracle_xonly: &str,
    nonce_xonly: &str,
    market: &str,
    outcome: Outcome,
    refund_timeout: u64,
    secret_nonce_hex: &str,
) -> Result<OutcomeToken> {
    let lock_pubkey = outcome_lock_key(bettor_pubkey, oracle_xonly, nonce_xonly, market, outcome)
        .context("deriving outcome lock key")?;

    let secret = json!([
        "P2PK",
        {
            "nonce": secret_nonce_hex,
            "data": lock_pubkey,
            "tags": [
                ["refund", refund_pubkey],
                ["locktime", refund_timeout.to_string()],
                ["sigflag", "SIG_INPUTS"]
            ]
        }
    ])
    .to_string();

    Ok(OutcomeToken {
        outcome,
        lock_pubkey,
        secret,
    })
}

/// Derives the spend secret `b + s_X` that unlocks the winning token, from the bettor's secret
/// and the oracle's attestation signature (kind:89). Thin wrapper over `hunch_dlc`.
pub fn redeem_spend_secret(
    bettor_secret_hex: &str,
    attestation_signature_hex: &str,
) -> Result<String> {
    outcome_unlock_secret(bettor_secret_hex, attestation_signature_hex)
}

/// Returns true iff `spend_secret` can sign for the token's NUT-11 `data` lock key — i.e. this
/// secret would satisfy the mint's NUT-11 check for this token.
pub fn verify_token_unlock(token_secret_json: &str, spend_secret_hex: &str) -> Result<bool> {
    let parsed: Value = serde_json::from_str(token_secret_json).context("parsing NUT-11 secret")?;
    let data = parsed
        .get(1)
        .and_then(|o| o.get("data"))
        .and_then(Value::as_str)
        .context("secret missing data (lock key)")?;
    verify_unlock(spend_secret_hex, data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use hunch_dlc::sign_attestation_with_nonce;
    use secp256k1::{Keypair, PublicKey, Secp256k1, SecretKey};

    const ORACLE: &str = "5f80b1ac81a47b0e3ee7e3bd4e23c1f3a96a0b56cd96b3a5d99e3a7a76d8c3a0";
    const NONCE: &str = "a1b2c3d4e5f60718293a4b5c6d7e8f90112233445566778899aabbccddeeff00";
    const BETTOR: &str = "1111111111111111111111111111111111111111111111111111111111111111";

    fn xonly(secret_hex: &str) -> String {
        let sk = SecretKey::from_slice(&hex::decode(secret_hex).unwrap()).unwrap();
        let kp = Keypair::from_secret_key(&Secp256k1::new(), &sk);
        hex::encode(kp.x_only_public_key().0.serialize())
    }

    fn bettor_pub() -> String {
        let sk = SecretKey::from_slice(&hex::decode(BETTOR).unwrap()).unwrap();
        hex::encode(PublicKey::from_secret_key(&Secp256k1::new(), &sk).serialize())
    }

    fn market() -> String {
        format!("{}:30888:m", "aa".repeat(32))
    }

    fn token(outcome: Outcome) -> OutcomeToken {
        build_outcome_token(
            &bettor_pub(),
            &bettor_pub(),
            &xonly(ORACLE),
            &xonly(NONCE),
            &market(),
            outcome,
            1_800_000_000,
            &"ab".repeat(16),
        )
        .unwrap()
    }

    fn attest(outcome: Outcome) -> String {
        let oracle: [u8; 32] = hex::decode(ORACLE).unwrap().try_into().unwrap();
        let nonce: [u8; 32] = hex::decode(NONCE).unwrap().try_into().unwrap();
        sign_attestation_with_nonce(&oracle, &nonce, &market(), outcome).unwrap()
    }

    #[test]
    fn secret_carries_lock_refund_and_locktime() {
        let t = token(Outcome::Yes);
        let v: Value = serde_json::from_str(&t.secret).unwrap();
        assert_eq!(v[0], "P2PK");
        assert_eq!(v[1]["data"], t.lock_pubkey);
        let tags = v[1]["tags"].as_array().unwrap();
        assert!(tags
            .iter()
            .any(|tg| tg[0] == "refund" && tg[1] == bettor_pub()));
        assert!(tags
            .iter()
            .any(|tg| tg[0] == "locktime" && tg[1] == "1800000000"));
    }

    #[test]
    fn winning_attestation_redeems_matching_token_only() {
        let yes = token(Outcome::Yes);
        let no = token(Outcome::No);

        // Oracle attests YES.
        let spend = redeem_spend_secret(BETTOR, &attest(Outcome::Yes)).unwrap();

        assert!(
            verify_token_unlock(&yes.secret, &spend).unwrap(),
            "YES token must redeem"
        );
        assert!(
            !verify_token_unlock(&no.secret, &spend).unwrap(),
            "NO token must NOT redeem"
        );
    }

    #[test]
    fn wrong_bettor_secret_cannot_redeem() {
        let yes = token(Outcome::Yes);
        // A different bettor's secret yields a spend key that doesn't match this token's lock.
        let other = "2222222222222222222222222222222222222222222222222222222222222222";
        let spend = redeem_spend_secret(other, &attest(Outcome::Yes)).unwrap();
        assert!(!verify_token_unlock(&yes.secret, &spend).unwrap());
    }
}
