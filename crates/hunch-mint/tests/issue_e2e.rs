//! End-to-end test of the MM `/buy` issuance primitive (`issue_locked`) against a RUNNING cdk-mintd.
//!
//! Ignored by default; needs a live mint (see e2e_mint.rs for setup). It proves the "issue at odds"
//! core: the MM mints an arbitrary-amount, multi-denomination set of outcome tokens P2PK-locked to
//! the bettor's L_YES, and the whole set redeems with the YES attestation key (while the YES key is
//! rejected on a NO-locked token).
//!
//!   HUNCH_MINT_URL=http://127.0.0.1:8085 cargo test -p hunch-mint --test issue_e2e -- --ignored --nocapture

use cashu::dhke::blind_message;
use cashu::nuts::{BlindedMessage, Proof, SecretKey};
use cashu::secret::Secret;
use cashu::{Amount, Id};
use hunch_dlc::{outcome_lock_key, outcome_unlock_secret, sign_attestation_with_nonce};
use hunch_mint::issue_locked;
use hunch_protocol::outcome::Outcome;
use secp256k1::{Keypair, PublicKey as SPublicKey, Secp256k1, SecretKey as SSecretKey};
use serde_json::{json, Value};
use std::str::FromStr;

const ORACLE: &str = "5f80b1ac81a47b0e3ee7e3bd4e23c1f3a96a0b56cd96b3a5d99e3a7a76d8c3a0";
const NONCE: &str = "a1b2c3d4e5f60718293a4b5c6d7e8f90112233445566778899aabbccddeeff00";
const BETTOR: &str = "1111111111111111111111111111111111111111111111111111111111111111";

fn mint_url() -> String {
    std::env::var("HUNCH_MINT_URL").unwrap_or_else(|_| "http://127.0.0.1:8085".to_string())
}
fn xonly(secret_hex: &str) -> String {
    let sk = SSecretKey::from_slice(&hex::decode(secret_hex).unwrap()).unwrap();
    hex::encode(
        Keypair::from_secret_key(&Secp256k1::new(), &sk)
            .x_only_public_key()
            .0
            .serialize(),
    )
}
fn bettor_pub() -> String {
    let sk = SSecretKey::from_slice(&hex::decode(BETTOR).unwrap()).unwrap();
    hex::encode(SPublicKey::from_secret_key(&Secp256k1::new(), &sk).serialize())
}
fn market() -> String {
    format!("{}:30888:m", "aa".repeat(32))
}
fn lock(outcome: Outcome) -> String {
    outcome_lock_key(
        &bettor_pub(),
        &xonly(ORACLE),
        &xonly(NONCE),
        &market(),
        outcome,
    )
    .unwrap()
}
fn attest_yes() -> String {
    let o: [u8; 32] = hex::decode(ORACLE).unwrap().try_into().unwrap();
    let n: [u8; 32] = hex::decode(NONCE).unwrap().try_into().unwrap();
    sign_attestation_with_nonce(&o, &n, &market(), Outcome::Yes).unwrap()
}
fn keyset_id(client: &reqwest::blocking::Client, url: &str) -> Id {
    let keys: Value = client
        .get(format!("{url}/v1/keys"))
        .send()
        .unwrap()
        .json()
        .unwrap();
    Id::from_str(keys["keysets"][0]["id"].as_str().unwrap()).unwrap()
}

/// Redeems a whole locked set in one NUT-03 swap signed with `key_hex`; returns whether accepted.
fn redeem_set(
    client: &reqwest::blocking::Client,
    url: &str,
    id: Id,
    proofs: Vec<Proof>,
    key_hex: &str,
    total: u64,
) -> bool {
    let mut inputs = Vec::new();
    for mut p in proofs {
        p.sign_p2pk(SecretKey::from_hex(key_hex).unwrap()).unwrap();
        inputs.push(serde_json::to_value(&p).unwrap());
    }
    // Outputs must also be valid (power-of-2) denominations summing to `total` — the mint has no
    // single key for e.g. 100. Split it the same way issuance does.
    let denoms = [
        1u64, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768, 65536,
    ];
    let mut outputs = Vec::new();
    for d in hunch_mint::denominations(total, &denoms) {
        let fresh = Secret::generate();
        let (fb, _r) = blind_message(&Vec::<u8>::from(&fresh), None).unwrap();
        outputs.push(serde_json::to_value(BlindedMessage::new(Amount::from(d), id, fb)).unwrap());
    }
    let resp = client
        .post(format!("{url}/v1/swap"))
        .json(&json!({ "inputs": inputs, "outputs": outputs }))
        .send()
        .unwrap();
    let ok = resp.status().is_success();
    ok && resp
        .json::<Value>()
        .ok()
        .and_then(|b| {
            b.get("signatures")
                .and_then(|s| s.as_array().map(|a| !a.is_empty()))
        })
        .unwrap_or(false)
}

#[test]
#[ignore = "requires a running cdk-mintd at HUNCH_MINT_URL"]
fn e2e_issue_locked_set_redeems() {
    let url = mint_url();
    let client = reqwest::blocking::Client::new();
    let id = keyset_id(&client, &url);
    let refund_timeout = 1_900_000_000;
    let payout = 100u64; // multi-denomination: 64 + 32 + 4

    // The MM issues 100 sat of YES-locked tokens (it fronts the full payout from its reserve).
    let yes_set = issue_locked(
        &url,
        payout,
        &lock(Outcome::Yes),
        &bettor_pub(),
        refund_timeout,
    )
    .unwrap();
    let total: u64 = yes_set.iter().map(|p| u64::from(p.amount)).sum();
    assert_eq!(total, payout, "issued set must sum to the payout");
    assert!(
        yes_set.len() >= 2,
        "100 sat should span multiple denominations"
    );

    // The whole set redeems with the YES attestation key.
    let l_yes = outcome_unlock_secret(BETTOR, &attest_yes()).unwrap();
    assert!(
        redeem_set(&client, &url, id, yes_set, &l_yes, payout),
        "mint must accept the full YES-locked set after the YES attestation"
    );

    // A NO-locked set signed with the YES key must be rejected.
    let no_set = issue_locked(
        &url,
        payout,
        &lock(Outcome::No),
        &bettor_pub(),
        refund_timeout,
    )
    .unwrap();
    assert!(
        !redeem_set(&client, &url, id, no_set, &l_yes, payout),
        "mint must reject the NO-locked set signed with the YES key"
    );

    println!("issue-at-odds OK: {payout}-sat YES set redeemed, NO set rejected by {url}");
}
