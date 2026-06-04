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

            // Issue the full payout, P2PK-locked to the bettor's L_X (MM fronts it from reserve).
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
