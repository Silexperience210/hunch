//! End-to-end test against a RUNNING cdk-mintd (NUT-04 mint + NUT-03 swap over HTTP).
//!
//! Ignored by default — it needs a live mint. Start one (no Lightning required):
//!
//! ```sh
//! cargo install cdk-mintd
//! cp crates/hunch-mint/cdk-mintd.example.toml ~/.cdk-mintd/config.toml   # fakewallet backend
//! cdk-mintd &
//! HUNCH_MINT_URL=http://127.0.0.1:8085 cargo test -p hunch-mint --test e2e_mint -- --ignored --nocapture
//! ```
//!
//! It proves the whole Hunch conditional-token flow against a real mint server:
//!   1. mint a 1-sat token whose NUT-11 secret is locked to L_YES = B + S_YES,
//!   2. the oracle attests YES → bettor derives l_YES = b + s_YES,
//!   3. the mint's /v1/swap ACCEPTS the token signed with l_YES (redemption), and
//!   4. REJECTS the NO-locked token signed with l_YES (wrong outcome).
//!
//! We talk raw HTTP with `reqwest` + `cashu` types (no cdk wallet) for full control over the
//! blinded outputs (their secret is our P2PK lock, not a wallet-managed key).

use cashu::dhke::{blind_message, unblind_message};
use cashu::nuts::{BlindSignature, BlindedMessage, Proof, PublicKey, SecretKey};
use cashu::secret::Secret;
use cashu::{Amount, Id};
use hunch_dlc::{outcome_lock_key, outcome_unlock_secret, sign_attestation_with_nonce};
use hunch_mint::cashu_token::p2pk_secret;
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
fn attest_yes() -> String {
    let o: [u8; 32] = hex::decode(ORACLE).unwrap().try_into().unwrap();
    let n: [u8; 32] = hex::decode(NONCE).unwrap().try_into().unwrap();
    sign_attestation_with_nonce(&o, &n, &market(), Outcome::Yes).unwrap()
}

/// Reads the mint's active keyset id and its amount-1 public key from /v1/keys.
fn active_keyset(client: &reqwest::blocking::Client, url: &str) -> (Id, PublicKey) {
    let keys: Value = client.get(format!("{url}/v1/keys")).send().unwrap().json().unwrap();
    let keyset = &keys["keysets"][0];
    let id = Id::from_str(keyset["id"].as_str().unwrap()).unwrap();
    let pk_hex = keyset["keys"]["1"].as_str().expect("mint must offer the 1-sat denomination");
    (id, PublicKey::from_hex(pk_hex).unwrap())
}

/// Mints a 1-sat proof whose NUT-11 secret is `secret`, against the live mint.
fn mint_one_sat(client: &reqwest::blocking::Client, url: &str, keyset_id: Id, mint_key: &PublicKey, secret: Secret) -> Proof {
    let secret_bytes: Vec<u8> = (&secret).into();
    let (blinded, r) = blind_message(&secret_bytes, None).unwrap();
    let bm = BlindedMessage::new(Amount::from(1u64), keyset_id, blinded);

    let quote: Value = client
        .post(format!("{url}/v1/mint/quote/bolt11"))
        .json(&json!({ "amount": 1, "unit": "sat" }))
        .send().unwrap().json().unwrap();
    let quote_id = quote["quote"].as_str().expect("quote id");

    let resp: Value = client
        .post(format!("{url}/v1/mint/bolt11"))
        .json(&json!({ "quote": quote_id, "outputs": [serde_json::to_value(&bm).unwrap()] }))
        .send().unwrap().json().unwrap();
    let sigs: Vec<BlindSignature> = serde_json::from_value(resp["signatures"].clone())
        .unwrap_or_else(|_| panic!("mint did not return signatures: {resp}"));
    let c = unblind_message(&sigs[0].c, &r, mint_key).unwrap();
    Proof::new(Amount::from(1u64), keyset_id, secret, c)
}

/// Attempts to redeem `proof` via /v1/swap, returning whether the mint accepted it.
fn try_swap(client: &reqwest::blocking::Client, url: &str, keyset_id: Id, proof: Proof) -> bool {
    let fresh = Secret::generate();
    let (fb, _r) = blind_message(&Vec::<u8>::from(&fresh), None).unwrap();
    let out = BlindedMessage::new(Amount::from(1u64), keyset_id, fb);
    let resp = client
        .post(format!("{url}/v1/swap"))
        .json(&json!({ "inputs": [serde_json::to_value(&proof).unwrap()], "outputs": [serde_json::to_value(&out).unwrap()] }))
        .send().unwrap();
    let status = resp.status();
    let body: Value = resp.json().unwrap_or(Value::Null);
    // Accepted iff the mint returns blind signatures for the outputs.
    status.is_success() && body.get("signatures").and_then(Value::as_array).map(|a| !a.is_empty()).unwrap_or(false)
}

#[test]
#[ignore = "requires a running cdk-mintd at HUNCH_MINT_URL"]
fn e2e_mint_lock_and_redeem_against_live_mint() {
    let url = mint_url();
    let client = reqwest::blocking::Client::new();
    let (keyset_id, mint_key) = active_keyset(&client, &url);

    // 1-2) Mint a YES-locked token, then redeem it with the YES attestation key.
    let yes_secret = p2pk_secret(&lock(Outcome::Yes)).unwrap();
    let mut yes_proof = mint_one_sat(&client, &url, keyset_id, &mint_key, yes_secret);
    let l_yes = outcome_unlock_secret(BETTOR, &attest_yes()).unwrap();
    yes_proof.sign_p2pk(SecretKey::from_hex(&l_yes).unwrap()).unwrap();
    assert!(try_swap(&client, &url, keyset_id, yes_proof), "mint must accept the YES token after the YES attestation");

    // 3) A NO-locked token signed with l_YES must be REJECTED by the mint server.
    let no_secret = p2pk_secret(&lock(Outcome::No)).unwrap();
    let mut no_proof = mint_one_sat(&client, &url, keyset_id, &mint_key, no_secret);
    no_proof.sign_p2pk(SecretKey::from_hex(&l_yes).unwrap()).unwrap();
    assert!(!try_swap(&client, &url, keyset_id, no_proof), "mint must reject the NO token signed with the YES key");

    println!("e2e OK: YES token redeemed, NO token rejected by {url}");
}
