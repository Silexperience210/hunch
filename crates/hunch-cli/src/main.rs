//! `hunch` — create and list prediction markets over Nostr.
//!
//! Subcommands:
//! - `keygen`        — generate a creator identity key.
//! - `market create` — build, sign (kind 30888), and publish a market.
//! - `market list`   — query relays for markets and print them.
//!
//! Secret resolution: `--secret <hex>`, `--secret-file <path>`, env `HUNCH_SECRET`.
//! Relays: repeated `--relay <wss://...>`, or env `HUNCH_RELAYS` (comma-separated).

use std::time::Duration;

use anyhow::{Context, Result};
use clap::{Args, Parser, Subcommand};
use hunch_cli::{
    build_market, build_order, market_id, order_tags_with_d, parse_market_event, parse_order_event,
    MarketParams, OrderParams,
};
use hunch_nostr::{build_signed_event, query_all, relay, verify_event};
use hunch_protocol::event_kinds::{KIND_MARKET, KIND_ORDER};
use hunch_protocol::order::{OrderKind, OrderSide};
use secp256k1::{Keypair, Secp256k1, SecretKey};
use serde_json::{json, Value};

#[derive(Parser)]
#[command(
    name = "hunch",
    version,
    about = "Hunch — permissionless prediction markets on Bitcoin"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
#[allow(clippy::large_enum_variant)] // clap subcommand enum; boxing would fight the derive
enum Command {
    /// Generate a new creator identity key (does not touch the network).
    Keygen,
    /// Create or list markets.
    Market {
        #[command(subcommand)]
        cmd: MarketCmd,
    },
    /// Place or list orders on a market.
    Order {
        #[command(subcommand)]
        cmd: OrderCmd,
    },
}

#[derive(Subcommand)]
enum OrderCmd {
    /// Sign (kind 38888) and publish an order on a market.
    Place {
        #[command(flatten)]
        key: KeyArgs,
        #[command(flatten)]
        net: NetArgs,
        /// Market id: `<creator_pubkey>:30888:<d>`.
        #[arg(long)]
        market: String,
        /// Side: YES or NO.
        #[arg(long)]
        side: OrderSide,
        /// Token amount in sat.
        #[arg(long)]
        amount: u64,
        /// Price in sat per token.
        #[arg(long)]
        price: u64,
        /// Order kind: bid or ask.
        #[arg(long)]
        kind: OrderKind,
        /// Expiry (unix seconds).
        #[arg(long)]
        expires: u64,
    },
    /// Query relays for a market's orders (kind 38888) and print the book.
    List {
        #[command(flatten)]
        net: NetArgs,
        /// Market id to list orders for.
        #[arg(long)]
        market: String,
        /// Maximum orders to request per relay.
        #[arg(long, default_value_t = 200)]
        limit: u64,
    },
}

#[derive(Subcommand)]
#[allow(clippy::large_enum_variant)] // clap subcommand enum; boxing would fight the derive
enum MarketCmd {
    /// Build, sign (kind 30888), and publish a market.
    Create {
        #[command(flatten)]
        key: KeyArgs,
        #[command(flatten)]
        net: NetArgs,
        /// Short market slug (the `d` tag), e.g. `btc-100k-eoy-2026`.
        #[arg(long)]
        slug: String,
        /// Oracle x-only public key (hex, 32 bytes).
        #[arg(long)]
        oracle: String,
        /// Betting close time (unix seconds).
        #[arg(long)]
        expiry: u64,
        /// Refund-claimable time (unix seconds). Defaults to expiry + 7 days.
        #[arg(long)]
        refund_timeout: Option<u64>,
        /// Mint backing the DLC (URL or pubkey hex).
        #[arg(long)]
        mint: String,
        /// On-chain DLC funding output: `<txid_hex>:<vout>`.
        #[arg(long)]
        dlc_contract: String,
        /// The market question.
        #[arg(long)]
        question: String,
        /// Resolution criteria (how the oracle decides the outcome).
        #[arg(long, default_value = "")]
        resolution: String,
        /// Source URL backing resolution (repeatable).
        #[arg(long = "source")]
        sources: Vec<String>,
        /// Rules version string.
        #[arg(long, default_value = "1.0")]
        rules_version: String,
        /// Optional category.
        #[arg(long)]
        category: Option<String>,
        /// Optional preview image URL.
        #[arg(long)]
        image: Option<String>,
        /// Topic tag (repeatable).
        #[arg(long = "topic")]
        topics: Vec<String>,
    },
    /// Query relays for markets (kind 30888) and print them.
    List {
        #[command(flatten)]
        net: NetArgs,
        /// Maximum markets to request per relay.
        #[arg(long, default_value_t = 200)]
        limit: u64,
    },
}

#[derive(Args)]
struct KeyArgs {
    /// Creator secret key (32-byte hex). Prefer --secret-file or HUNCH_SECRET for opsec.
    #[arg(long, env = "HUNCH_SECRET", hide_env_values = true)]
    secret: Option<String>,
    /// Path to a file containing the creator secret key (hex).
    #[arg(long)]
    secret_file: Option<String>,
}

#[derive(Args)]
struct NetArgs {
    /// Relay URL (repeatable). Or set HUNCH_RELAYS (comma-separated).
    #[arg(long = "relay")]
    relays: Vec<String>,
    /// Build/print without touching the network (create only).
    #[arg(long)]
    dry_run: bool,
    /// Seconds to wait per relay.
    #[arg(long, default_value_t = 10)]
    timeout: u64,
}

impl KeyArgs {
    fn keypair(&self) -> Result<Keypair> {
        let hexkey = if let Some(s) = &self.secret {
            s.trim().to_string()
        } else if let Some(path) = &self.secret_file {
            std::fs::read_to_string(path)
                .with_context(|| format!("reading secret file {path}"))?
                .trim()
                .to_string()
        } else {
            anyhow::bail!("no secret key: pass --secret, --secret-file, or set HUNCH_SECRET");
        };
        let bytes = hex::decode(&hexkey).context("secret key is not valid hex")?;
        let sk =
            SecretKey::from_slice(&bytes).context("secret key is not a valid secp256k1 scalar")?;
        Ok(Keypair::from_secret_key(&Secp256k1::new(), &sk))
    }
}

impl NetArgs {
    fn relay_list(&self) -> Result<Vec<String>> {
        if !self.relays.is_empty() {
            return Ok(self.relays.clone());
        }
        if let Ok(env) = std::env::var("HUNCH_RELAYS") {
            let list: Vec<String> = env
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            if !list.is_empty() {
                return Ok(list);
            }
        }
        anyhow::bail!("no relays: pass --relay <wss://...> (repeatable) or set HUNCH_RELAYS")
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    match Cli::parse().command {
        Command::Keygen => keygen(),
        Command::Market { cmd } => match cmd {
            MarketCmd::Create {
                key,
                net,
                slug,
                oracle,
                expiry,
                refund_timeout,
                mint,
                dlc_contract,
                question,
                resolution,
                sources,
                rules_version,
                category,
                image,
                topics,
            } => {
                let keypair = key.keypair()?;
                let creator_pubkey = hex::encode(keypair.x_only_public_key().0.serialize());

                let market = build_market(MarketParams {
                    d: slug.clone(),
                    oracle_pubkey: oracle,
                    expiry,
                    refund_timeout,
                    mint,
                    dlc_contract,
                    question,
                    resolution_criteria: resolution,
                    sources,
                    rules_version,
                    category,
                    image,
                    topics,
                })?;
                let (tags, content) = market.to_event_parts()?;
                let event = build_signed_event(
                    &Secp256k1::new(),
                    &keypair,
                    KIND_MARKET,
                    tags,
                    content,
                    now(),
                );

                eprintln!("market id: {}", market_id(&creator_pubkey, &slug));
                broadcast(&net, &event, "market").await?;
            }
            MarketCmd::List { net, limit } => {
                let relays = net.relay_list()?;
                let filter = json!({ "kinds": [KIND_MARKET], "limit": limit });
                let events = query_all(&relays, filter, Duration::from_secs(net.timeout)).await;
                print_markets(events);
            }
        },
        Command::Order { cmd } => match cmd {
            OrderCmd::Place {
                key,
                net,
                market,
                side,
                amount,
                price,
                kind,
                expires,
            } => {
                let keypair = key.keypair()?;
                let order = build_order(OrderParams {
                    market,
                    side,
                    amount,
                    price,
                    kind,
                    expires,
                })?;
                let tags = order_tags_with_d(&order);
                let event = build_signed_event(
                    &Secp256k1::new(),
                    &keypair,
                    KIND_ORDER,
                    tags,
                    String::new(),
                    now(),
                );
                eprintln!(
                    "order: {} {} {} sat @ {} sat/token on {}",
                    order.kind.as_str(),
                    order.side.as_str(),
                    order.amount,
                    order.price,
                    order.market
                );
                broadcast(&net, &event, "order").await?;
            }
            OrderCmd::List { net, market, limit } => {
                let relays = net.relay_list()?;
                // `d` == market, and `d` is single-letter so relays can filter it (#d).
                let filter = json!({ "kinds": [KIND_ORDER], "#d": [market], "limit": limit });
                let events = query_all(&relays, filter, Duration::from_secs(net.timeout)).await;
                print_orders(events, &market);
            }
        },
    }
    Ok(())
}

/// Prints the event, then either stops (`--dry-run`) or publishes to all relays.
async fn broadcast(net: &NetArgs, event: &Value, what: &str) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(event)?);
    if net.dry_run {
        eprintln!("(dry-run: not published)");
        return Ok(());
    }
    let relays = net.relay_list()?;
    let results = relay::publish_all(&relays, event, Duration::from_secs(net.timeout)).await;
    let mut accepted = 0usize;
    for (url, res) in &results {
        match res {
            Ok(o) if o.accepted => {
                accepted += 1;
                eprintln!("✔ {url}: accepted {}", o.message);
            }
            Ok(o) => eprintln!("✗ {url}: rejected {}", o.message),
            Err(e) => eprintln!("✗ {url}: {e:#}"),
        }
    }
    eprintln!("published to {accepted}/{} relays", relays.len());
    if accepted == 0 {
        anyhow::bail!("no relay accepted the {what}");
    }
    Ok(())
}

fn keygen() {
    let secp = Secp256k1::new();
    let (sk, _pk) = secp.generate_keypair(&mut secp256k1::rand::rngs::OsRng);
    let keypair = Keypair::from_secret_key(&secp, &sk);
    eprintln!("⚠  SAVE THIS SECRET KEY OFFLINE. Anyone with it can post markets as you.");
    println!("secret: {}", hex::encode(sk.secret_bytes()));
    println!(
        "pubkey: {}",
        hex::encode(keypair.x_only_public_key().0.serialize())
    );
}

fn print_markets(events: Vec<Value>) {
    let mut shown = 0usize;
    let mut skipped = 0usize;
    let mut forged = 0usize;
    for ev in &events {
        // Relays are untrusted: drop any event whose id/signature doesn't check out.
        if !verify_event(ev) {
            forged += 1;
            continue;
        }
        match parse_market_event(ev) {
            Ok((id, m)) => {
                shown += 1;
                println!("\n● {}", m.content.question);
                println!("  id:      {id}");
                println!(
                    "  oracle:  {}…  expiry: {}  refund: {}",
                    short(&m.oracle_pubkey),
                    m.expiry,
                    m.refund_timeout
                );
                println!("  mint:    {}", m.mint);
                if !m.topics.is_empty() {
                    println!("  topics:  {}", m.topics.join(", "));
                }
            }
            Err(_) => skipped += 1,
        }
    }
    eprintln!(
        "\n{shown} market(s){}{}",
        count_note("unparseable", skipped),
        count_note("forged", forged)
    );
}

fn print_orders(events: Vec<Value>, market: &str) {
    let mut orders: Vec<(String, hunch_protocol::order::Order)> = Vec::new();
    let mut forged = 0usize;
    for ev in &events {
        if !verify_event(ev) {
            forged += 1;
            continue;
        }
        if let Ok((author, order)) = parse_order_event(ev) {
            if order.market == market {
                orders.push((author, order));
            }
        }
    }
    // Order book: best price first within each side (bids high→low, asks low→high is conventional,
    // but we keep it simple and sort all by price descending).
    orders.sort_by_key(|b| std::cmp::Reverse(b.1.price));
    println!("Order book for {market}\n");
    println!(
        "{:<5} {:<4} {:>12} {:>10}  author",
        "SIDE", "KIND", "AMOUNT(sat)", "PRICE"
    );
    for (author, o) in &orders {
        println!(
            "{:<5} {:<4} {:>12} {:>10}  {}…",
            o.side.as_str(),
            o.kind.as_str(),
            o.amount,
            o.price,
            short(author)
        );
    }
    eprintln!(
        "\n{} order(s){}",
        orders.len(),
        count_note("forged", forged)
    );
}

fn count_note(label: &str, n: usize) -> String {
    if n > 0 {
        format!(", {n} {label} skipped")
    } else {
        String::new()
    }
}

fn short(hex_str: &str) -> &str {
    &hex_str[..hex_str.len().min(12)]
}

fn now() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system clock before unix epoch")
        .as_secs() as i64
}
