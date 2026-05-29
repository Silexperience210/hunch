//! Persistent nonce store with a hard nonce-reuse guard.
//!
//! A DLC oracle commits to one nonce `R = k·G` per market at announce time and must sign
//! exactly one outcome with it. Signing two *different* outcomes under the same nonce leaks
//! the oracle's secret key. This store is the single source of truth that makes that
//! impossible: it persists each market's nonce and the outcome (if any) it has already been
//! used for, and refuses to hand out a nonce for a second, conflicting attestation.
//!
//! Format: a JSON object `{ "<market_id>": { secret, pubkey, attested } }`, written atomically
//! (temp file + rename). Single-process use; the oracle is one daemon.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::generate_nonce;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NonceEntry {
    /// Nonce secret `k` (32-byte hex). Required to sign; never published.
    pub secret: String,
    /// Announced nonce point `R` x-only (32-byte hex). Published in the announce.
    pub pubkey: String,
    /// Outcome this nonce has already attested, if any. Locks the nonce to one outcome.
    pub attested: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct NonceStore {
    #[serde(skip)]
    path: PathBuf,
    #[serde(flatten)]
    entries: BTreeMap<String, NonceEntry>,
}

impl NonceStore {
    /// Loads the store from `path`, or returns an empty store if the file does not exist.
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        if !path.exists() {
            return Ok(NonceStore {
                path,
                entries: BTreeMap::new(),
            });
        }
        let raw = std::fs::read_to_string(&path)
            .with_context(|| format!("reading nonce store {}", path.display()))?;
        let entries: BTreeMap<String, NonceEntry> = serde_json::from_str(&raw)
            .with_context(|| format!("parsing nonce store {}", path.display()))?;
        Ok(NonceStore { path, entries })
    }

    /// Returns the existing nonce for `market`, or generates, persists, and returns a new one.
    /// Idempotent: announcing the same market twice reuses the same `R`.
    pub fn get_or_create(&mut self, market: &str) -> Result<NonceEntry> {
        if let Some(entry) = self.entries.get(market) {
            return Ok(entry.clone());
        }
        let (secret, pubkey) = generate_nonce();
        let entry = NonceEntry {
            secret,
            pubkey,
            attested: None,
        };
        self.entries.insert(market.to_string(), entry.clone());
        self.save()?;
        Ok(entry)
    }

    /// Returns the nonce to sign `market`/`outcome` with, enforcing the reuse guard.
    ///
    /// - No nonce for `market` → error (the oracle must `announce` before it can `attest`).
    /// - Already attested a *different* outcome → hard error (signing would leak the key).
    /// - Already attested the *same* outcome → allowed (idempotent re-publish, identical sig).
    pub fn nonce_for_attest(&self, market: &str, outcome: &str) -> Result<NonceEntry> {
        let entry = self.entries.get(market).with_context(|| {
            format!("no announced nonce for market {market}; run `announce` first")
        })?;
        if let Some(prev) = &entry.attested {
            if prev != outcome {
                anyhow::bail!(
                    "REFUSING to reuse nonce: market {market} already attested {prev}, cannot also attest {outcome} \
                     (signing two outcomes under one nonce leaks the oracle key)"
                );
            }
        }
        Ok(entry.clone())
    }

    /// Records that `market` has attested `outcome`, locking its nonce. Persists immediately.
    pub fn commit_attest(&mut self, market: &str, outcome: &str) -> Result<()> {
        let entry = self
            .entries
            .get_mut(market)
            .with_context(|| format!("no nonce entry for market {market}"))?;
        entry.attested = Some(outcome.to_string());
        self.save()
    }

    /// Writes the store atomically (temp file + rename).
    fn save(&self) -> Result<()> {
        let json = serde_json::to_string_pretty(&self.entries)?;
        let tmp = self.path.with_extension("json.tmp");
        std::fs::write(&tmp, json).with_context(|| format!("writing {}", tmp.display()))?;
        std::fs::rename(&tmp, &self.path)
            .with_context(|| format!("renaming into {}", self.path.display()))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_path(tag: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "hunch-nonce-test-{tag}-{}.json",
            std::process::id()
        ))
    }

    #[test]
    fn get_or_create_is_idempotent_and_persists() {
        let path = temp_path("idem");
        let _ = std::fs::remove_file(&path);
        let mut store = NonceStore::load(&path).unwrap();
        let a = store.get_or_create("m1").unwrap();
        let b = store.get_or_create("m1").unwrap();
        assert_eq!(a.pubkey, b.pubkey);
        assert_eq!(a.secret, b.secret);

        // Reload from disk: same nonce survives.
        let reloaded = NonceStore::load(&path).unwrap();
        let c = reloaded.nonce_for_attest("m1", "YES").unwrap();
        assert_eq!(c.pubkey, a.pubkey);
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn refuses_second_outcome_under_same_nonce() {
        let path = temp_path("reuse");
        let _ = std::fs::remove_file(&path);
        let mut store = NonceStore::load(&path).unwrap();
        store.get_or_create("m1").unwrap();
        store.nonce_for_attest("m1", "YES").unwrap();
        store.commit_attest("m1", "YES").unwrap();

        // Same outcome: idempotent, allowed.
        assert!(store.nonce_for_attest("m1", "YES").is_ok());
        // Different outcome: hard refusal.
        assert!(store.nonce_for_attest("m1", "NO").is_err());
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn attest_without_announce_errors() {
        let path = temp_path("noann");
        let _ = std::fs::remove_file(&path);
        let store = NonceStore::load(&path).unwrap();
        assert!(store.nonce_for_attest("ghost", "YES").is_err());
        let _ = std::fs::remove_file(&path);
    }
}
