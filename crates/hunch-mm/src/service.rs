//! HTTP `/buy` service — the mint-as-market-maker endpoint.
//!
//! - `GET /health` → liveness.
//! - `GET /pool/<market>` → live pool state (prices, depth, fee, rake).
//! - `POST /quote` → price a buy (`{market, side, shares|budget}`), fee included.
//! - `POST /buy` → issue `shares` outcome-locked tokens at the quoted odds. The body adds the
//!   bettor's `lock` (L_X hex) + `refund` (B hex) [+ `locktime`]; the MM mints the full payout from
//!   cdk-mintd (its reserve) and records the fill.
//!
//! Single-threaded request loop so the JSON pool ledger is never raced. CORS `*` so the browser can
//! call it. Issuance is the proven `hunch_mint::issue_locked` primitive.

use anyhow::{anyhow, Context, Result};
use serde_json::{json, Value};
use tiny_http::{Header, Method, Response, Server};

use hunch_mint::issue_locked;

use crate::pool::Side;
use crate::{Pool, PoolStore, Quote};

fn side_from(s: &str) -> Result<Side> {
    match s.to_uppercase().as_str() {
        "YES" => Ok(Side::Yes),
        "NO" => Ok(Side::No),
        other => Err(anyhow!("side must be YES or NO (got {other})")),
    }
}

/// Resolve a share count from `shares` or `budget` in the request body.
fn shares_from(pool: &Pool, side: Side, body: &Value) -> Result<f64> {
    if let Some(s) = body.get("shares").and_then(Value::as_f64) {
        return Ok(s);
    }
    if let Some(b) = body.get("budget").and_then(Value::as_f64) {
        return Ok(pool.shares_for_budget(side, b));
    }
    Err(anyhow!("provide `shares` or `budget`"))
}

fn quote_json(q: &Quote) -> Value {
    json!({
        "shares": q.shares, "fair": q.fair, "fee": q.fee, "cost": q.cost,
        "avgPrice": q.avg_price, "priceBefore": q.price_before, "priceAfter": q.price_after,
    })
}

fn default_locktime() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
        + 90 * 24 * 3600
}

/// Minimal percent-decode for path segments (market ids are hex/digits/`-` with `:` → `%3A`).
fn urldecode(s: &str) -> String {
    let b = s.as_bytes();
    let mut out = String::with_capacity(s.len());
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'%' && i + 2 < b.len() {
            if let Ok(c) = u8::from_str_radix(&s[i + 1..i + 3], 16) {
                out.push(c as char);
                i += 3;
                continue;
            }
        }
        out.push(b[i] as char);
        i += 1;
    }
    out
}

/// Run the service until killed.
pub fn serve(listen: &str, mint_url: &str, store_path: &str) -> Result<()> {
    let server = Server::http(listen).map_err(|e| anyhow!("bind {listen}: {e}"))?;
    eprintln!("hunch-mm serving on http://{listen}  (mint {mint_url}, store {store_path})");
    for mut req in server.incoming_requests() {
        let method = req.method().clone();
        let url = req.url().to_string();
        let mut body = String::new();
        let _ = req.as_reader().read_to_string(&mut body);
        let (code, payload) = match route(&method, &url, &body, mint_url, store_path) {
            Ok(v) => (200, v.to_string()),
            Err(e) => (400, json!({ "error": e.to_string() }).to_string()),
        };
        let resp = Response::from_string(payload)
            .with_status_code(code)
            .with_header(
                Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap(),
            )
            .with_header(
                Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap(),
            );
        let _ = req.respond(resp);
    }
    Ok(())
}

fn route(
    method: &Method,
    url: &str,
    body: &str,
    mint_url: &str,
    store_path: &str,
) -> Result<Value> {
    let path = url.split('?').next().unwrap_or(url);
    match (method, path) {
        (Method::Options, _) => Ok(json!({})), // CORS preflight
        (Method::Get, "/health") => Ok(json!({ "ok": true })),

        (Method::Get, p) if p.starts_with("/pool/") => {
            let market = urldecode(&p["/pool/".len()..]);
            let store = PoolStore::load(store_path)?;
            let pool = store.get(&market).context("no pool for that market")?;
            Ok(json!({
                "market": pool.market, "priceYes": pool.price_yes(), "priceNo": pool.price_no(),
                "b": pool.b, "feeBps": pool.fee_bps, "qYes": pool.q_yes, "qNo": pool.q_no,
                "realizedRake": pool.realized_rake, "collected": pool.collected,
            }))
        }

        (Method::Post, "/quote") => {
            let j: Value = serde_json::from_str(body).context("invalid JSON body")?;
            let market = j["market"].as_str().context("`market` required")?;
            let side = side_from(j["side"].as_str().context("`side` required")?)?;
            let store = PoolStore::load(store_path)?;
            let pool = store.get(market).context("no pool — seed it first")?;
            let shares = shares_from(pool, side, &j)?;
            Ok(quote_json(&pool.quote_buy(side, shares)))
        }

        (Method::Post, "/buy") => {
            let j: Value = serde_json::from_str(body).context("invalid JSON body")?;
            let market = j["market"]
                .as_str()
                .context("`market` required")?
                .to_string();
            let side = side_from(j["side"].as_str().context("`side` required")?)?;
            let lock = j["lock"].as_str().context("`lock` (L_X hex) required")?;
            let refund = j["refund"].as_str().context("`refund` (B hex) required")?;
            let locktime = j["locktime"].as_u64().unwrap_or_else(default_locktime);

            let mut store = PoolStore::load(store_path)?;
            // Round the payout to whole sats (token denominations are integers) and quote on it.
            let payout = {
                let pool = store.get(&market).context("no pool — seed it first")?;
                let shares = shares_from(pool, side, &j)?;
                shares.round().max(0.0) as u64
            };
            if payout == 0 {
                return Err(anyhow!("stake too small — payout rounds to 0 sat"));
            }
            let quote = store.get(&market).unwrap().quote_buy(side, payout as f64);

            // Payment leg: the bettor's `payment` (Cashu proofs) must cover the quoted cost. Claim it
            // into the MM reserve FIRST (the swap is the verification gate) — no payment, no tokens.
            let cost_sat = quote.cost.ceil() as u64;
            let reserve = hunch_mint::claim_payment_json(mint_url, &j["payment"], cost_sat)
                .context("claiming the bettor's payment")?;
            append_reserve(store_path, &reserve)?;

            // Now issue the full payout, P2PK-locked to the bettor's L_X (MM fronts payout − cost).
            let proofs = issue_locked(mint_url, payout, lock, refund, locktime)
                .context("issuing outcome tokens")?;

            // Record the fill only after issuance succeeds.
            store.apply_buy(&market, side, payout as f64)?;

            Ok(json!({
                "market": market, "side": side_label(side), "shares": payout,
                "cost": quote.cost, "fee": quote.fee, "proofs": proofs,
            }))
        }

        _ => Err(anyhow!("not found: {method:?} {path}")),
    }
}

fn side_label(s: Side) -> &'static str {
    match s {
        Side::Yes => "YES",
        Side::No => "NO",
    }
}

/// Append claimed payment proofs to the MM reserve file (`<store>.reserve.json`). This is the
/// operator's ecash float that backs winner payouts; persist it so nothing is lost on restart.
fn append_reserve(store_path: &str, proofs: &[Value]) -> Result<()> {
    let path = format!("{store_path}.reserve.json");
    let mut existing: Vec<Value> = std::fs::read_to_string(&path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default();
    existing.extend_from_slice(proofs);
    std::fs::write(&path, serde_json::to_string(&existing)?)
        .with_context(|| format!("writing reserve {path}"))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn seeded_store(tag: &str) -> String {
        let path =
            std::env::temp_dir().join(format!("hunch-mm-svc-{tag}-{}.json", std::process::id()));
        let p = path.to_string_lossy().to_string();
        let _ = std::fs::remove_file(&p);
        let mut s = PoolStore::load(&p).unwrap();
        s.seed("m:1", 10_000.0, 200, true).unwrap();
        p
    }

    #[test]
    fn quote_route_returns_cost_and_fee() {
        let store = seeded_store("quote");
        let body = r#"{"market":"m:1","side":"YES","budget":1000}"#;
        let v = route(&Method::Post, "/quote", body, "http://unused", &store).unwrap();
        assert!(v["cost"].as_f64().unwrap() > 0.0);
        assert!(v["fee"].as_f64().unwrap() > 0.0);
        assert!(v["shares"].as_f64().unwrap() > 1000.0); // price < 1 → payout > stake
        let _ = std::fs::remove_file(&store);
    }

    #[test]
    fn pool_route_and_error_paths() {
        let store = seeded_store("pool");
        let v = route(&Method::Get, "/pool/m:1", "", "http://unused", &store).unwrap();
        assert!((v["priceYes"].as_f64().unwrap() - 0.5).abs() < 1e-9);
        assert!(route(&Method::Get, "/pool/nope", "", "x", &store).is_err()); // unknown market
        assert!(route(
            &Method::Post,
            "/quote",
            r#"{"market":"m:1","side":"X"}"#,
            "x",
            &store
        )
        .is_err());
        assert!(route(&Method::Get, "/health", "", "x", &store).unwrap()["ok"] == true);
        let _ = std::fs::remove_file(&store);
    }

    #[test]
    fn buy_without_payment_is_rejected() {
        // No `payment` field → rejected before any mint call (lock/refund are valid dummy hex).
        let store = seeded_store("nopay");
        let lock = "02".to_string() + &"a".repeat(64);
        let body = format!(
            r#"{{"market":"m:1","side":"YES","budget":100,"lock":"{lock}","refund":"{lock}"}}"#
        );
        let err = route(&Method::Post, "/buy", &body, "http://127.0.0.1:1", &store).unwrap_err();
        assert!(err.to_string().to_lowercase().contains("payment"));
        let _ = std::fs::remove_file(&store);
    }

    // Full /buy with a real payment + issuance against a live cdk-mintd.
    #[test]
    #[ignore = "requires a running cdk-mintd at HUNCH_MINT_URL"]
    fn buy_route_e2e() {
        let url =
            std::env::var("HUNCH_MINT_URL").unwrap_or_else(|_| "http://127.0.0.1:8085".into());
        let store = seeded_store("buy-e2e");
        // A valid compressed pubkey for lock/refund (the mint's amount-1 key); redemption isn't
        // tested here — we only prove /buy claims payment and issues at odds.
        let keys: Value = reqwest::blocking::get(format!("{url}/v1/keys"))
            .unwrap()
            .json()
            .unwrap();
        let lock = keys["keysets"][0]["keys"]["1"]
            .as_str()
            .unwrap()
            .to_string();
        // Mint 200 sat of bearer ecash as the bettor's payment for a 100-sat budget.
        let payment = serde_json::to_value(hunch_mint::mint_bearer(&url, 200).unwrap()).unwrap();
        let body = json!({
            "market": "m:1", "side": "YES", "budget": 100,
            "lock": lock, "refund": lock, "payment": payment,
        })
        .to_string();
        let v = route(&Method::Post, "/buy", &body, &url, &store).unwrap();
        assert!(
            v["shares"].as_u64().unwrap() >= 100,
            "payout >= the 100-sat budget"
        );
        assert!(
            v["proofs"].as_array().unwrap().len() >= 2,
            "issued multi-denomination set"
        );
        let reserve = std::fs::read_to_string(format!("{store}.reserve.json")).unwrap();
        assert!(!serde_json::from_str::<Value>(&reserve)
            .unwrap()
            .as_array()
            .unwrap()
            .is_empty());
        println!(
            "buy-route OK: paid + issued {} sat payout via the service",
            v["shares"]
        );
        let _ = std::fs::remove_file(&store);
        let _ = std::fs::remove_file(format!("{store}.reserve.json"));
    }
}
