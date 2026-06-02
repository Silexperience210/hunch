//! `hunch-oracle` — single-key oracle daemon CLI.
//!
//! Subcommands:
//! - `keygen`   — generate a new oracle secret key (print + opsec warning).
//! - `pubkey`   — print the oracle's x-only public key.
//! - `announce` — publish a NIP-88 announce (kind 88) committing to attest a market.
//! - `attest`   — sign an outcome and publish a NIP-88 attestation (kind 89).
//!
//! Secret resolution (in order): `--secret <hex>`, `--secret-file <path>`, env `HUNCH_ORACLE_SECRET`.
//! Relays: repeated `--relay <wss://...>`, or env `HUNCH_ORACLE_RELAYS` (comma-separated).

use std::time::Duration;

use anyhow::{Context, Result};
use clap::{Args, Parser, Subcommand};
use hunch_nostr::{query_all, relay, verify_event};
use hunch_oracle::{
    connectors::ResolutionSpec, generate_keypair, nonce_store::NonceStore, OracleService,
};
use hunch_protocol::market::Market;
use hunch_protocol::outcome::Outcome;
use serde_json::{json, Value};

#[derive(Parser)]
#[command(
    name = "hunch-oracle",
    version,
    about = "Hunch single-key oracle daemon (NIP-88 over Nostr)"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Generate a new oracle secret key (does not touch the network).
    Keygen,
    /// Print the oracle's x-only public key (hex).
    Pubkey(KeyArgs),
    /// Publish a NIP-88 announce (kind 88) committing to attest a market.
    Announce {
        #[command(flatten)]
        key: KeyArgs,
        #[command(flatten)]
        net: NetArgs,
        /// Market identifier: `<creator_pubkey>:30888:<d>`.
        #[arg(long)]
        market: String,
        /// Free-form announce body (resolution rules summary, contact, etc.).
        #[arg(long, default_value = "")]
        body: String,
    },
    /// Sign an outcome and publish a NIP-88 attestation (kind 89).
    Attest {
        #[command(flatten)]
        key: KeyArgs,
        #[command(flatten)]
        net: NetArgs,
        /// Market identifier: `<creator_pubkey>:30888:<d>`.
        #[arg(long)]
        market: String,
        /// Resolved outcome: YES, NO, or INVALID.
        #[arg(long)]
        outcome: Outcome,
    },
    /// Resolve a market automatically via a connector spec, then publish the attestation.
    ///
    /// The spec is JSON (inline or `@path`) tagged by `connector`, e.g.:
    ///   {"connector":"price","asset":"BTC","quote":"USD","op":">=","threshold":100000}
    Resolve {
        #[command(flatten)]
        key: KeyArgs,
        #[command(flatten)]
        net: NetArgs,
        /// Market identifier: `<creator_pubkey>:30888:<d>`.
        #[arg(long)]
        market: String,
        /// Resolution spec as JSON, or `@path` to a JSON file.
        #[arg(long)]
        spec: String,
    },
    /// Daemon: query markets this oracle is assigned to, announce new ones, and auto-resolve
    /// expired ones that carry a `resolution_spec`. Run once, or loop with `--interval`.
    Tick {
        #[command(flatten)]
        key: KeyArgs,
        #[command(flatten)]
        net: NetArgs,
        /// Max markets to pull per relay query.
        #[arg(long, default_value_t = 500)]
        limit: u64,
        /// If set, loop forever, sleeping this many seconds between passes (else one pass).
        #[arg(long)]
        interval: Option<u64>,
    },
}

#[derive(Args)]
struct KeyArgs {
    /// Oracle secret key (32-byte hex). Prefer --secret-file or HUNCH_ORACLE_SECRET for opsec.
    #[arg(long, env = "HUNCH_ORACLE_SECRET", hide_env_values = true)]
    secret: Option<String>,
    /// Path to a file containing the oracle secret key (hex).
    #[arg(long)]
    secret_file: Option<String>,
}

#[derive(Args)]
struct NetArgs {
    /// Relay URL to publish to (repeatable). Or set HUNCH_ORACLE_RELAYS (comma-separated).
    #[arg(long = "relay")]
    relays: Vec<String>,
    /// Build and print the event without publishing.
    #[arg(long)]
    dry_run: bool,
    /// Seconds to wait for each relay's OK reply.
    #[arg(long, default_value_t = 10)]
    timeout: u64,
    /// Path to the nonce store (persists announced nonces + enforces the reuse guard).
    #[arg(long, default_value = "hunch-oracle-nonces.json")]
    nonce_store: String,
}

impl KeyArgs {
    fn resolve_secret(&self) -> Result<String> {
        if let Some(s) = &self.secret {
            return Ok(s.trim().to_string());
        }
        if let Some(path) = &self.secret_file {
            let contents = std::fs::read_to_string(path)
                .with_context(|| format!("reading secret file {path}"))?;
            return Ok(contents.trim().to_string());
        }
        anyhow::bail!("no secret key: pass --secret, --secret-file, or set HUNCH_ORACLE_SECRET")
    }

    fn oracle(&self) -> Result<OracleService> {
        OracleService::from_secret_hex(&self.resolve_secret()?)
    }
}

impl NetArgs {
    fn relay_list(&self) -> Result<Vec<String>> {
        if !self.relays.is_empty() {
            return Ok(self.relays.clone());
        }
        if let Ok(env) = std::env::var("HUNCH_ORACLE_RELAYS") {
            let list: Vec<String> = env
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            if !list.is_empty() {
                return Ok(list);
            }
        }
        anyhow::bail!("no relays: pass --relay <wss://...> (repeatable) or set HUNCH_ORACLE_RELAYS")
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Keygen => {
            let (secret, pubkey) = generate_keypair();
            eprintln!(
                "⚠  SAVE THIS SECRET KEY OFFLINE. Anyone with it controls this oracle identity."
            );
            eprintln!("⚠  Do not commit it, paste it, or store it unencrypted.");
            println!("secret: {secret}");
            println!("pubkey: {pubkey}");
        }
        Command::Pubkey(key) => {
            let oracle = key.oracle()?;
            println!("{}", oracle.pubkey_hex());
        }
        Command::Announce {
            key,
            net,
            market,
            body,
        } => {
            let oracle = key.oracle()?;
            // The oracle owns its nonce: generate + persist R for this market (idempotent).
            let mut store = NonceStore::load(&net.nonce_store)?;
            let nonce = store.get_or_create(&market)?;
            eprintln!("announced nonce R: {}", nonce.pubkey);
            let created_at = now();
            let event = oracle.build_announce_event(&market, &nonce.pubkey, &body, created_at)?;
            broadcast(&net, &event).await?;
        }
        Command::Attest {
            key,
            net,
            market,
            outcome,
        } => {
            let oracle = key.oracle()?;
            // Load the nonce committed at announce time; the store refuses a conflicting reuse.
            let mut store = NonceStore::load(&net.nonce_store)?;
            let nonce = store.nonce_for_attest(&market, outcome.as_str())?;
            let created_at = now();
            let (event, attestation) =
                oracle.build_attestation_event(&market, outcome, &nonce.secret, created_at)?;
            // Lock the nonce to this outcome BEFORE publishing, so a later attest can never
            // sign a different outcome under the same R (which would leak the oracle key).
            store.commit_attest(&market, outcome.as_str())?;
            eprintln!(
                "attestation: market={} outcome={} sig={}",
                attestation.market, attestation.outcome, attestation.signature_hex
            );
            eprintln!("nonce R {} now locked to {}", nonce.pubkey, outcome);
            broadcast(&net, &event).await?;
        }
        Command::Resolve {
            key,
            net,
            market,
            spec,
        } => {
            let oracle = key.oracle()?;
            // Spec is inline JSON or @path to a file.
            let spec_json = match spec.strip_prefix('@') {
                Some(path) => std::fs::read_to_string(path)
                    .with_context(|| format!("reading spec file {path}"))?,
                None => spec,
            };
            let spec = ResolutionSpec::from_json(&spec_json).context("parsing resolution spec")?;
            // Fetch the data and decide — auto-resolution.
            let resolution = spec.resolve().await.context("connector resolution")?;
            let outcome = resolution.outcome;
            eprintln!("resolved {outcome}: {}", resolution.evidence);
            // Same attest flow as `Attest`, but the outcome came from the connector.
            let mut store = NonceStore::load(&net.nonce_store)?;
            let nonce = store.nonce_for_attest(&market, outcome.as_str())?;
            let created_at = now();
            let (event, attestation) =
                oracle.build_attestation_event(&market, outcome, &nonce.secret, created_at)?;
            store.commit_attest(&market, outcome.as_str())?;
            eprintln!(
                "attestation: market={} outcome={} sig={}",
                attestation.market, attestation.outcome, attestation.signature_hex
            );
            broadcast(&net, &event).await?;
        }
        Command::Tick {
            key,
            net,
            limit,
            interval,
        } => {
            let oracle = key.oracle()?;
            loop {
                if let Err(e) = tick_once(&oracle, &net, limit).await {
                    eprintln!("tick error: {e:#}");
                }
                match interval {
                    Some(secs) if secs > 0 => {
                        tokio::time::sleep(Duration::from_secs(secs)).await;
                    }
                    _ => break,
                }
            }
        }
    }
    Ok(())
}

/// Extracts `(market_id, Market)` from a kind:30888 Nostr event Value, or `None` if it doesn't parse.
fn event_to_market(ev: &Value) -> Option<(String, Market)> {
    let kind = ev.get("kind")?.as_u64()? as u32;
    let creator = ev.get("pubkey")?.as_str()?;
    let content = ev.get("content")?.as_str()?;
    let tags: Vec<Vec<String>> = ev
        .get("tags")?
        .as_array()?
        .iter()
        .map(|t| {
            t.as_array()
                .map(|a| {
                    a.iter()
                        .filter_map(|x| x.as_str().map(String::from))
                        .collect()
                })
                .unwrap_or_default()
        })
        .collect();
    let market = Market::from_event(kind, &tags, content).ok()?;
    let id = format!("{creator}:{}:{}", Market::KIND, market.d);
    Some((id, market))
}

/// One daemon pass: announce this oracle's un-announced markets (so bettors can lock), then
/// auto-resolve any that are past expiry and carry a `resolution_spec`.
async fn tick_once(oracle: &OracleService, net: &NetArgs, limit: u64) -> Result<()> {
    let relays = net.relay_list()?;
    let me = oracle.pubkey_hex();
    let filter = json!({ "kinds": [Market::KIND], "limit": limit });
    let events = query_all(&relays, filter, Duration::from_secs(net.timeout)).await;
    let now = now();
    let mut store = NonceStore::load(&net.nonce_store)?;

    for ev in &events {
        if !verify_event(ev) {
            continue;
        }
        let Some((id, market)) = event_to_market(ev) else {
            continue;
        };
        if market.oracle_pubkey != me {
            continue; // not our market
        }

        // 1) Announce early (idempotent) so bettors can lock to the committed nonce R.
        if !store.is_announced(&id) {
            let nonce = store.get_or_create(&id)?;
            let event = oracle.build_announce_event(&id, &nonce.pubkey, "auto-resolved", now)?;
            eprintln!("announce {id} (R={})", nonce.pubkey);
            let _ = broadcast(net, &event).await;
        }

        // 2) Auto-resolve once expired, if a spec is present and we haven't attested yet.
        if now as u64 >= market.expiry
            && store.attested_outcome(&id).is_none()
            && market.resolution_spec.is_some()
        {
            let spec_json = market.resolution_spec.as_deref().unwrap_or_default();
            let spec = match ResolutionSpec::from_json(spec_json) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("skip {id}: bad resolution_spec: {e:#}");
                    continue;
                }
            };
            match spec.resolve().await {
                Ok(res) => {
                    let nonce = store.nonce_for_attest(&id, res.outcome.as_str())?;
                    let (event, _att) =
                        oracle.build_attestation_event(&id, res.outcome, &nonce.secret, now)?;
                    store.commit_attest(&id, res.outcome.as_str())?;
                    eprintln!("resolve {id} -> {} ({})", res.outcome, res.evidence);
                    let _ = broadcast(net, &event).await;
                }
                Err(e) => eprintln!("resolve {id} failed (will retry next tick): {e:#}"),
            }
        }
    }
    Ok(())
}

/// Current unix time in seconds, for `created_at`.
fn now() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system clock before unix epoch")
        .as_secs() as i64
}

/// Prints the event, then either stops (`--dry-run`) or publishes to all relays.
async fn broadcast(net: &NetArgs, event: &serde_json::Value) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(event)?);
    if net.dry_run {
        eprintln!("(dry-run: not published)");
        return Ok(());
    }
    let relays = net.relay_list()?;
    let results = relay::publish_all(&relays, event, Duration::from_secs(net.timeout)).await;
    let mut accepted = 0usize;
    for (relay_url, result) in &results {
        match result {
            Ok(outcome) if outcome.accepted => {
                accepted += 1;
                eprintln!("✔ {relay_url}: accepted {}", outcome.message);
            }
            Ok(outcome) => eprintln!("✗ {relay_url}: rejected {}", outcome.message),
            Err(e) => eprintln!("✗ {relay_url}: {e:#}"),
        }
    }
    eprintln!("published to {accepted}/{} relays", relays.len());
    if accepted == 0 {
        anyhow::bail!("no relay accepted the event");
    }
    Ok(())
}
