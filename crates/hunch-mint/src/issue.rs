//! Issue-at-odds — the market maker's `/buy` primitive.
//!
//! Mints `amount` sat of outcome tokens against a live cdk-mintd, each NUT-11 P2PK-locked to the
//! bettor's `L_X = B + S_X` with a refund branch back to `B` after `refund_timeout`. The MM pays the
//! mint quote from its own reserve (fronting the full payout `shares`), so a winner can later redeem
//! the full `amount`, not just what they paid. We talk raw HTTP with `reqwest` + `cashu` types (no
//! cdk wallet) so the blinded outputs carry *our* P2PK lock, not a wallet-managed key.
//!
//! `denominations` (the amount split) is pure + unit-tested; `issue_locked` needs a running mint and
//! is exercised by the `buy_e2e` harness.

use anyhow::{anyhow, Context, Result};
use cashu::dhke::{blind_message, unblind_message};
use cashu::nuts::{BlindSignature, BlindedMessage, Proof, PublicKey, SecretKey};
use cashu::secret::Secret;
use cashu::{Amount, Id};
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::str::FromStr;

use crate::cashu_token::outcome_secret;

/// Greedy split of `amount` into the largest available denominations (Cashu keysets are powers of 2
/// and always include 1, so any integer splits exactly). Pure + tested.
pub fn denominations(amount: u64, available: &[u64]) -> Vec<u64> {
    let mut denoms: Vec<u64> = available.iter().copied().filter(|d| *d > 0).collect();
    denoms.sort_unstable_by(|a, b| b.cmp(a)); // descending
    let mut out = Vec::new();
    let mut rem = amount;
    for d in denoms {
        while rem >= d {
            out.push(d);
            rem -= d;
        }
    }
    out
}

/// Active keyset: id + denomination→mint-pubkey map.
fn keyset(client: &reqwest::blocking::Client, url: &str) -> Result<(Id, BTreeMap<u64, PublicKey>)> {
    let keys: Value = client.get(format!("{url}/v1/keys")).send()?.json()?;
    let keyset = &keys["keysets"][0];
    let id = Id::from_str(keyset["id"].as_str().context("mint: no keyset id")?)?;
    let keymap = keyset["keys"]
        .as_object()
        .context("mint: no keyset keys")?
        .iter()
        .filter_map(|(k, v)| {
            Some((
                k.parse::<u64>().ok()?,
                PublicKey::from_hex(v.as_str()?).ok()?,
            ))
        })
        .collect();
    Ok((id, keymap))
}

/// Split `amount` into the mint's denominations, erroring if it can't be represented exactly.
fn split_for(keymap: &BTreeMap<u64, PublicKey>, amount: u64) -> Result<Vec<u64>> {
    let denoms: Vec<u64> = keymap.keys().copied().collect();
    let split = denominations(amount, &denoms);
    if split.iter().sum::<u64>() != amount {
        return Err(anyhow!(
            "mint denominations {denoms:?} can't represent {amount} sat exactly"
        ));
    }
    Ok(split)
}

/// Quote + (poll PAID) + mint the prepared `(secret, denom)` outputs into proofs. The MM funds the
/// bolt11 quote from its reserve (fakewallet auto-settles in tests).
fn quote_and_mint(
    client: &reqwest::blocking::Client,
    url: &str,
    id: Id,
    keymap: &BTreeMap<u64, PublicKey>,
    prepared: Vec<(Secret, u64)>,
) -> Result<Vec<Proof>> {
    let amount: u64 = prepared.iter().map(|(_, d)| *d).sum();
    let mut outputs = Vec::with_capacity(prepared.len());
    let mut pending = Vec::with_capacity(prepared.len()); // (r, denom, secret)
    for (secret, d) in prepared {
        let secret_bytes: Vec<u8> = (&secret).into();
        let (blinded, r) = blind_message(&secret_bytes, None).map_err(|e| anyhow!("blind: {e}"))?;
        outputs.push(serde_json::to_value(BlindedMessage::new(
            Amount::from(d),
            id,
            blinded,
        ))?);
        pending.push((r, d, secret));
    }

    let quote: Value = client
        .post(format!("{url}/v1/mint/quote/bolt11"))
        .json(&json!({ "amount": amount, "unit": "sat" }))
        .send()?
        .json()?;
    let quote_id = quote["quote"]
        .as_str()
        .context("mint: no quote id")?
        .to_string();
    let mut paid = false;
    for _ in 0..60 {
        let status: Value = client
            .get(format!("{url}/v1/mint/quote/bolt11/{quote_id}"))
            .send()?
            .json()?;
        if status["state"].as_str() == Some("PAID") {
            paid = true;
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(250));
    }
    if !paid {
        return Err(anyhow!(
            "mint quote {quote_id} not paid — fund the MM reserve / pay the invoice"
        ));
    }

    let resp: Value = client
        .post(format!("{url}/v1/mint/bolt11"))
        .json(&json!({ "quote": quote_id, "outputs": outputs }))
        .send()?
        .json()?;
    let sigs: Vec<BlindSignature> = serde_json::from_value(resp["signatures"].clone())
        .map_err(|_| anyhow!("mint returned no signatures: {resp}"))?;
    unblind_all(sigs, pending, id, keymap)
}

/// Unblind mint/swap signatures (paired with their blinding `(r, denom, secret)`) into proofs.
fn unblind_all(
    sigs: Vec<BlindSignature>,
    pending: Vec<(SecretKey, u64, Secret)>,
    id: Id,
    keymap: &BTreeMap<u64, PublicKey>,
) -> Result<Vec<Proof>> {
    if sigs.len() != pending.len() {
        return Err(anyhow!(
            "mint returned {} signatures for {} outputs",
            sigs.len(),
            pending.len()
        ));
    }
    let mut proofs = Vec::with_capacity(sigs.len());
    for (sig, (r, d, secret)) in sigs.into_iter().zip(pending) {
        let mint_key = keymap
            .get(&d)
            .context("mint: missing key for denomination")?;
        let c = unblind_message(&sig.c, &r, mint_key).map_err(|e| anyhow!("unblind: {e}"))?;
        proofs.push(Proof::new(Amount::from(d), id, secret, c));
    }
    Ok(proofs)
}

/// Mint `amount` sat of proofs P2PK-locked to `lock_pubkey_hex` (L_X), reclaimable by
/// `refund_pubkey_hex` (B) after `refund_timeout`. The MM funds the quote from its reserve.
pub fn issue_locked(
    mint_url: &str,
    amount: u64,
    lock_pubkey_hex: &str,
    refund_pubkey_hex: &str,
    refund_timeout: u64,
) -> Result<Vec<Proof>> {
    if amount == 0 {
        return Ok(Vec::new());
    }
    let client = reqwest::blocking::Client::new();
    let (id, keymap) = keyset(&client, mint_url)?;
    let mut prepared = Vec::new();
    for d in split_for(&keymap, amount)? {
        prepared.push((
            outcome_secret(lock_pubkey_hex, refund_pubkey_hex, refund_timeout)?,
            d,
        ));
    }
    quote_and_mint(&client, mint_url, id, &keymap, prepared)
}

/// Mint `amount` sat of plain bearer proofs (no lock) — to fund the MM reserve, or in tests.
pub fn mint_bearer(mint_url: &str, amount: u64) -> Result<Vec<Proof>> {
    if amount == 0 {
        return Ok(Vec::new());
    }
    let client = reqwest::blocking::Client::new();
    let (id, keymap) = keyset(&client, mint_url)?;
    let prepared = split_for(&keymap, amount)?
        .into_iter()
        .map(|d| (Secret::generate(), d))
        .collect();
    quote_and_mint(&client, mint_url, id, &keymap, prepared)
}

/// Claim payment `proofs` into the MM reserve by swapping them for fresh bearer proofs at the mint.
/// Returns the fresh MM-owned proofs (their sum is the amount paid). Fails if the mint rejects the
/// inputs (invalid / already spent / bad signature) — this swap IS the payment-verification gate.
pub fn claim_proofs(mint_url: &str, proofs: Vec<Proof>) -> Result<Vec<Proof>> {
    if proofs.is_empty() {
        return Err(anyhow!("empty payment"));
    }
    let client = reqwest::blocking::Client::new();
    let (id, keymap) = keyset(&client, mint_url)?;
    let total: u64 = proofs.iter().map(|p| u64::from(p.amount)).sum();

    let mut outputs = Vec::new();
    let mut pending = Vec::new();
    for d in split_for(&keymap, total)? {
        let secret = Secret::generate();
        let secret_bytes: Vec<u8> = (&secret).into();
        let (blinded, r) = blind_message(&secret_bytes, None).map_err(|e| anyhow!("blind: {e}"))?;
        outputs.push(serde_json::to_value(BlindedMessage::new(
            Amount::from(d),
            id,
            blinded,
        ))?);
        pending.push((r, d, secret));
    }
    let mut inputs = Vec::with_capacity(proofs.len());
    for p in &proofs {
        inputs.push(serde_json::to_value(p)?);
    }
    let resp: Value = client
        .post(format!("{mint_url}/v1/swap"))
        .json(&json!({ "inputs": inputs, "outputs": outputs }))
        .send()?
        .json()?;
    let sigs: Vec<BlindSignature> = serde_json::from_value(resp["signatures"].clone())
        .map_err(|_| anyhow!("mint rejected the payment: {resp}"))?;
    unblind_all(sigs, pending, id, &keymap)
}

/// Payment leg for `/buy`: parse `payment` (a JSON array of Cashu proofs), require it covers
/// `min_amount` sat, and claim it into the MM reserve. Returns the fresh reserve proofs as JSON so
/// the caller can persist them without depending on `cashu` types.
pub fn claim_payment_json(mint_url: &str, payment: &Value, min_amount: u64) -> Result<Vec<Value>> {
    let proofs: Vec<Proof> = serde_json::from_value(payment.clone())
        .map_err(|e| anyhow!("invalid payment proofs: {e}"))?;
    let declared: u64 = proofs.iter().map(|p| u64::from(p.amount)).sum();
    if declared < min_amount {
        return Err(anyhow!(
            "underpaid: payment {declared} sat < cost {min_amount} sat"
        ));
    }
    let reserve = claim_proofs(mint_url, proofs)?;
    reserve
        .iter()
        .map(|p| serde_json::to_value(p).map_err(Into::into))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn denominations_split_exactly() {
        let avail = [1, 2, 4, 8, 16, 32, 64];
        for amount in [1u64, 3, 7, 100, 127, 1000] {
            let split = denominations(amount, &avail);
            assert_eq!(split.iter().sum::<u64>(), amount, "amount {amount}");
            assert!(split.iter().all(|d| avail.contains(d)));
        }
        assert!(denominations(0, &avail).is_empty());
    }

    #[test]
    fn denominations_uses_largest_first() {
        let split = denominations(7, &[1, 2, 4]);
        assert_eq!(split, vec![4, 2, 1]); // greedy descending
    }
}
