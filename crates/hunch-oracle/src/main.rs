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
use hunch_oracle::{generate_keypair, nonce_store::NonceStore, relay, OracleService};
use hunch_protocol::outcome::Outcome;

#[derive(Parser)]
#[command(name = "hunch-oracle", version, about = "Hunch single-key oracle daemon (NIP-88 over Nostr)")]
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
            let list: Vec<String> = env.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
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
            eprintln!("⚠  SAVE THIS SECRET KEY OFFLINE. Anyone with it controls this oracle identity.");
            eprintln!("⚠  Do not commit it, paste it, or store it unencrypted.");
            println!("secret: {secret}");
            println!("pubkey: {pubkey}");
        }
        Command::Pubkey(key) => {
            let oracle = key.oracle()?;
            println!("{}", oracle.pubkey_hex());
        }
        Command::Announce { key, net, market, body } => {
            let oracle = key.oracle()?;
            // The oracle owns its nonce: generate + persist R for this market (idempotent).
            let mut store = NonceStore::load(&net.nonce_store)?;
            let nonce = store.get_or_create(&market)?;
            eprintln!("announced nonce R: {}", nonce.pubkey);
            let created_at = now();
            let event = oracle.build_announce_event(&market, &nonce.pubkey, &body, created_at)?;
            broadcast(&net, &event).await?;
        }
        Command::Attest { key, net, market, outcome } => {
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
