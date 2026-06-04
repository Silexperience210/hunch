//! `hunch-mm` — the mint-as-market-maker engine.
//!
//! An LMSR market maker that lets the mint be the counterparty for instant 1-click bets: it prices
//! YES/NO via the Logarithmic Market Scoring Rule, charges a maker fee (the operator's rake), and
//! keeps a persistent ledger of per-market inventory + realized rake. The directional subsidy is
//! bounded by `b·ln2` per market; the fee is margin on top.
//!
//! The math mirrors `apps/hunch-web/lib/{lmsr,amm}.ts` so the browser quote and the mint ledger
//! agree. Settlement (the MM issuing tokens against its reserve and paying winners) layers on top of
//! this ledger.

pub mod pool;
pub mod store;

pub use pool::{Pool, Quote, Side};
pub use store::PoolStore;
