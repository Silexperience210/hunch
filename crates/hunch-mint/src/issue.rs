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
use cashu::nuts::{BlindSignature, BlindedMessage, Proof, PublicKey};
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

    // 1) Active keyset: id + denomination→mint-pubkey map.
    let keys: Value = client.get(format!("{mint_url}/v1/keys")).send()?.json()?;
    let keyset = &keys["keysets"][0];
    let id = Id::from_str(keyset["id"].as_str().context("mint: no keyset id")?)?;
    let keymap: BTreeMap<u64, PublicKey> = keyset["keys"]
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
    let denoms: Vec<u64> = keymap.keys().copied().collect();
    let split = denominations(amount, &denoms);
    if split.iter().sum::<u64>() != amount {
        return Err(anyhow!(
            "mint denominations {denoms:?} can't represent {amount} sat exactly"
        ));
    }

    // 2) One blinded, P2PK-locked output per denomination.
    let mut outputs = Vec::with_capacity(split.len());
    let mut pending = Vec::with_capacity(split.len()); // (r, denom, secret)
    for d in &split {
        let secret = outcome_secret(lock_pubkey_hex, refund_pubkey_hex, refund_timeout)?;
        let secret_bytes: Vec<u8> = (&secret).into();
        let (blinded, r) = blind_message(&secret_bytes, None).map_err(|e| anyhow!("blind: {e}"))?;
        outputs.push(serde_json::to_value(BlindedMessage::new(
            Amount::from(*d),
            id,
            blinded,
        ))?);
        pending.push((r, *d, secret));
    }

    // 3) Mint quote for the full amount; wait until PAID (fakewallet auto-settles; a real LN backend
    //    needs the MM to pay this bolt11 from its reserve).
    let quote: Value = client
        .post(format!("{mint_url}/v1/mint/quote/bolt11"))
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
            .get(format!("{mint_url}/v1/mint/quote/bolt11/{quote_id}"))
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

    // 4) Mint, then unblind each signature into a spendable proof.
    let resp: Value = client
        .post(format!("{mint_url}/v1/mint/bolt11"))
        .json(&json!({ "quote": quote_id, "outputs": outputs }))
        .send()?
        .json()?;
    let sigs: Vec<BlindSignature> = serde_json::from_value(resp["signatures"].clone())
        .map_err(|_| anyhow!("mint returned no signatures: {resp}"))?;
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
