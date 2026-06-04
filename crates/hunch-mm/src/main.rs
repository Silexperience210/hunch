//! `hunch-mm` CLI — operate the mint-as-market-maker ledger.
//!
//! - `seed`  — create/seed a market's LMSR pool (depth `b`, maker fee in bps).
//! - `quote` — price a buy (by shares or by sat budget), fee included.
//! - `buy`   — record a fill: move inventory, accrue the maker rake.
//! - `rake`  — show realized rake (per market or total).
//! - `list`  — list pools with live prices + rake.
//!
//! State lives in a JSON pool store (`--store`, default `hunch-mm-pools.json`).

use anyhow::{Context, Result};
use clap::{Args, Parser, Subcommand, ValueEnum};
use hunch_mm::{pool::Side, PoolStore};

#[derive(Parser)]
#[command(
    name = "hunch-mm",
    version,
    about = "Hunch mint-as-market-maker (LMSR + maker fee)"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Clone, Copy, ValueEnum)]
enum SideArg {
    Yes,
    No,
}
impl From<SideArg> for Side {
    fn from(s: SideArg) -> Self {
        match s {
            SideArg::Yes => Side::Yes,
            SideArg::No => Side::No,
        }
    }
}

#[derive(Args)]
struct StoreArg {
    /// Path to the pool ledger (JSON).
    #[arg(long, default_value = "hunch-mm-pools.json")]
    store: String,
}

#[derive(Subcommand)]
enum Command {
    /// Create/seed a market's LMSR pool.
    Seed {
        #[command(flatten)]
        store: StoreArg,
        #[arg(long)]
        market: String,
        /// Liquidity depth b (sats). Worst-case subsidy = b·ln2.
        #[arg(long)]
        depth: f64,
        /// Maker fee in basis points (100 = 1%).
        #[arg(long, default_value_t = 200)]
        fee_bps: u32,
        /// Overwrite an existing pool.
        #[arg(long)]
        force: bool,
    },
    /// Quote a buy (by --shares or by --budget sat), maker fee included.
    Quote {
        #[command(flatten)]
        store: StoreArg,
        #[arg(long)]
        market: String,
        #[arg(long, value_enum)]
        side: SideArg,
        /// Shares (sats of payout) to buy.
        #[arg(long, group = "size")]
        shares: Option<f64>,
        /// Sat budget to spend (fee included); the quote inverts it to shares.
        #[arg(long, group = "size")]
        budget: Option<f64>,
    },
    /// Record a fill: move inventory and accrue the maker rake.
    Buy {
        #[command(flatten)]
        store: StoreArg,
        #[arg(long)]
        market: String,
        #[arg(long, value_enum)]
        side: SideArg,
        #[arg(long, group = "size")]
        shares: Option<f64>,
        #[arg(long, group = "size")]
        budget: Option<f64>,
    },
    /// Settle a market: show payout owed to winners and the MM's realized profit (the rake).
    Settle {
        #[command(flatten)]
        store: StoreArg,
        #[arg(long)]
        market: String,
        /// Winning outcome: YES, NO, or INVALID (refund all).
        #[arg(long)]
        outcome: String,
    },
    /// Show realized rake (one market, or total across all).
    Rake {
        #[command(flatten)]
        store: StoreArg,
        #[arg(long)]
        market: Option<String>,
    },
    /// List pools with live prices and realized rake.
    List {
        #[command(flatten)]
        store: StoreArg,
    },
}

fn main() -> Result<()> {
    match Cli::parse().command {
        Command::Seed {
            store,
            market,
            depth,
            fee_bps,
            force,
        } => {
            let mut s = PoolStore::load(&store.store)?;
            s.seed(&market, depth, fee_bps, force)?;
            println!(
                "seeded {market}: depth {depth} sat, fee {fee_bps} bps, max subsidy {:.0} sat",
                depth * std::f64::consts::LN_2
            );
        }
        Command::Quote {
            store,
            market,
            side,
            shares,
            budget,
        } => {
            let s = PoolStore::load(&store.store)?;
            let pool = s.get(&market).context("no pool — seed it first")?;
            let side: Side = side.into();
            let shares = resolve_shares(pool, side, shares, budget)?;
            let q = pool.quote_buy(side, shares);
            println!(
                "{market} {side:?}: buy {:.0} shares → pay {:.2} sat (fair {:.2} + fee {:.2}), \
                 avg {:.4}, price {:.4}→{:.4}",
                q.shares, q.cost, q.fair, q.fee, q.avg_price, q.price_before, q.price_after
            );
        }
        Command::Buy {
            store,
            market,
            side,
            shares,
            budget,
        } => {
            let mut s = PoolStore::load(&store.store)?;
            let side: Side = side.into();
            let shares = {
                let pool = s.get(&market).context("no pool — seed it first")?;
                resolve_shares(pool, side, shares, budget)?
            };
            let fee = s.apply_buy(&market, side, shares)?;
            let pool = s.get(&market).unwrap();
            println!(
                "filled {market} {side:?} {:.0} shares: rake +{:.2} sat (market total {:.2}), \
                 new YES price {:.4}",
                shares,
                fee,
                pool.realized_rake,
                pool.price_yes()
            );
        }
        Command::Settle {
            store,
            market,
            outcome,
        } => {
            let s = PoolStore::load(&store.store)?;
            let pool = s.get(&market).context("no pool — seed it first")?;
            let winner = match outcome.to_uppercase().as_str() {
                "YES" => Some(Side::Yes),
                "NO" => Some(Side::No),
                "INVALID" => None,
                other => anyhow::bail!("outcome must be YES, NO or INVALID (got {other})"),
            };
            let r = pool.settle(winner);
            let label = winner
                .map(|w| format!("{w:?}"))
                .unwrap_or_else(|| "INVALID".into());
            println!(
                "settle {market} = {label}: took in {:.2} sat, owe winners {:.2} sat → \
                 MM P&L {:+.2} sat (rake {:.2} sat, max subsidy {:.0} sat)",
                r.total_in,
                r.payout,
                r.mm_pnl,
                r.rake,
                pool.max_subsidy()
            );
        }
        Command::Rake { store, market } => {
            let s = PoolStore::load(&store.store)?;
            match market {
                Some(m) => {
                    let pool = s.get(&m).context("no such pool")?;
                    println!("{m}: {:.2} sat realized rake", pool.realized_rake);
                }
                None => println!("total realized rake: {:.2} sat", s.total_rake()),
            }
        }
        Command::List { store } => {
            let s = PoolStore::load(&store.store)?;
            let mut any = false;
            for p in s.all() {
                any = true;
                println!(
                    "{}  YES {:.1}% / NO {:.1}%  depth {:.0}  fee {}bps  rake {:.2} sat",
                    p.market,
                    p.price_yes() * 100.0,
                    p.price_no() * 100.0,
                    p.b,
                    p.fee_bps,
                    p.realized_rake
                );
            }
            if !any {
                println!("(no pools)");
            }
        }
    }
    Ok(())
}

/// Resolve the share count from either `--shares` or `--budget` (exactly one is required).
fn resolve_shares(
    pool: &hunch_mm::Pool,
    side: Side,
    shares: Option<f64>,
    budget: Option<f64>,
) -> Result<f64> {
    match (shares, budget) {
        (Some(s), None) => Ok(s),
        (None, Some(b)) => Ok(pool.shares_for_budget(side, b)),
        _ => anyhow::bail!("pass exactly one of --shares or --budget"),
    }
}
