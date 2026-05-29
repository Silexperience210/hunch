//! `hunch-matcher` — read a market's orders from relays and print proposed matches.
//!
//! Stateless and advisory: it suggests compatible bid/ask and complementary pairs. It does not
//! settle, custody, or publish anything — the matched parties execute through the mint (HIP-3).

use std::time::Duration;

use anyhow::Result;
use clap::Parser;
use hunch_matcher::{book_orders_from_events, match_book, Match};
use hunch_nostr::query_all;
use hunch_protocol::event_kinds::KIND_ORDER;
use serde_json::json;

#[derive(Parser)]
#[command(name = "hunch-matcher", version, about = "Hunch Tier-2 P2P order matcher (advisory)")]
struct Cli {
    /// Market id to match: `<creator_pubkey>:30888:<d>`.
    #[arg(long)]
    market: String,
    /// Relay URL (repeatable). Or set HUNCH_RELAYS (comma-separated).
    #[arg(long = "relay")]
    relays: Vec<String>,
    /// Per-token settlement payout in sat (market/mint parameter, HIP-3). Used for complementary matching.
    #[arg(long, default_value_t = 100)]
    face_value: u64,
    /// Maximum orders to request per relay.
    #[arg(long, default_value_t = 500)]
    limit: u64,
    /// Seconds to wait per relay.
    #[arg(long, default_value_t = 10)]
    timeout: u64,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let relays = resolve_relays(&cli.relays)?;

    let filter = json!({ "kinds": [KIND_ORDER], "#d": [cli.market], "limit": cli.limit });
    let events = query_all(&relays, filter, Duration::from_secs(cli.timeout)).await;
    let orders = book_orders_from_events(&events, &cli.market);

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let matches = match_book(&orders, cli.face_value, now);

    println!("Market   {}", cli.market);
    println!("Orders   {} live (face value {} sat/token)\n", orders.len(), cli.face_value);
    if matches.is_empty() {
        println!("No compatible matches.");
        return Ok(());
    }
    for m in &matches {
        match m {
            Match::Direct { side, amount, price, buyer, seller, .. } => println!(
                "DIRECT       {:<3} {:>8} tokens @ {:>4} sat   buyer {}… ← seller {}…",
                side.as_str(), amount, price, short(buyer), short(seller)
            ),
            Match::Complementary { amount, price_yes, price_no, yes_buyer, no_buyer, .. } => println!(
                "COMPLEMENT   {:>8} pairs  YES@{} + NO@{} = {}   yes {}…  no {}…",
                amount, price_yes, price_no, price_yes + price_no, short(yes_buyer), short(no_buyer)
            ),
        }
    }
    eprintln!("\n{} proposed match(es)", matches.len());
    Ok(())
}

fn resolve_relays(relays: &[String]) -> Result<Vec<String>> {
    if !relays.is_empty() {
        return Ok(relays.to_vec());
    }
    if let Ok(env) = std::env::var("HUNCH_RELAYS") {
        let list: Vec<String> =
            env.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
        if !list.is_empty() {
            return Ok(list);
        }
    }
    anyhow::bail!("no relays: pass --relay <wss://...> (repeatable) or set HUNCH_RELAYS")
}

fn short(hex_str: &str) -> &str {
    &hex_str[..hex_str.len().min(12)]
}
