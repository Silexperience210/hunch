//! Persistent pool store — the market maker's ledger of per-market LMSR pools + realized rake.
//!
//! Format: a JSON object `{ "<market_id>": Pool }`, written atomically (temp file + rename).
//! Single-process use (the MM is one service).

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};

use crate::pool::Pool;

#[derive(Debug, Default)]
pub struct PoolStore {
    path: PathBuf,
    pools: BTreeMap<String, Pool>,
}

impl PoolStore {
    /// Loads the store, or returns an empty one if the file does not exist.
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        if !path.exists() {
            return Ok(PoolStore {
                path,
                pools: BTreeMap::new(),
            });
        }
        let raw = std::fs::read_to_string(&path)
            .with_context(|| format!("reading pool store {}", path.display()))?;
        let pools: BTreeMap<String, Pool> = serde_json::from_str(&raw)
            .with_context(|| format!("parsing pool store {}", path.display()))?;
        Ok(PoolStore { path, pools })
    }

    fn save(&self) -> Result<()> {
        let tmp = self.path.with_extension("tmp");
        std::fs::write(&tmp, serde_json::to_string_pretty(&self.pools)?)
            .with_context(|| format!("writing {}", tmp.display()))?;
        std::fs::rename(&tmp, &self.path)
            .with_context(|| format!("renaming into {}", self.path.display()))?;
        Ok(())
    }

    pub fn get(&self, market: &str) -> Option<&Pool> {
        self.pools.get(market)
    }

    pub fn all(&self) -> impl Iterator<Item = &Pool> {
        self.pools.values()
    }

    /// Seed a new pool. Fails if one already exists for `market` unless `force`.
    pub fn seed(&mut self, market: &str, b: f64, fee_bps: u32, force: bool) -> Result<()> {
        if self.pools.contains_key(market) && !force {
            bail!("pool already exists for {market} (use --force to reseed)");
        }
        self.pools
            .insert(market.to_string(), Pool::new(market, b, fee_bps));
        self.save()
    }

    /// Apply a buy to a market's pool and persist. Returns the realized fee (rake) for this fill.
    pub fn apply_buy(&mut self, market: &str, side: crate::pool::Side, shares: f64) -> Result<f64> {
        let pool = self
            .pools
            .get_mut(market)
            .with_context(|| format!("no pool for {market} (seed it first)"))?;
        let fee = pool.apply_buy(side, shares);
        self.save()?;
        Ok(fee)
    }

    /// Total realized rake across all pools (sats).
    pub fn total_rake(&self) -> f64 {
        self.pools.values().map(|p| p.realized_rake).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pool::Side;

    #[test]
    fn seed_buy_persist_roundtrip() {
        let dir = std::env::temp_dir();
        let path = dir.join(format!("hunch-mm-test-{}.json", std::process::id()));
        let _ = std::fs::remove_file(&path);

        let mut s = PoolStore::load(&path).unwrap();
        s.seed("m", 10_000.0, 200, false).unwrap();
        assert!(s.seed("m", 1.0, 0, false).is_err()); // no double seed
        let fee = s.apply_buy("m", Side::Yes, 100.0).unwrap();
        assert!(fee > 0.0);

        // reload from disk: rake + inventory persisted
        let s2 = PoolStore::load(&path).unwrap();
        let p = s2.get("m").unwrap();
        assert!((p.realized_rake - fee).abs() < 1e-9);
        assert!(p.q_yes > 0.0);
        assert!((s2.total_rake() - fee).abs() < 1e-9);

        let _ = std::fs::remove_file(&path);
    }
}
