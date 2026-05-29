//! Hunch mint — outcome-conditional ecash on stable Cashu primitives.
//!
//! # SPIKE-02 resolution: no NUT-CTF needed
//!
//! HIP-3 originally targeted NUT-CTF (cashubtc/nuts#337) for conditional tokens. That PR is
//! still draft/WIP, so building on it blocked the mint. We don't need it: a Hunch outcome token
//! is just a **NUT-11 P2PK** proof locked to `L = B + S_X`, where `B` is the bettor's pubkey and
//! `S_X = R + e_X·P` is the oracle's signature point for outcome X. The mint verifies only a
//! Schnorr signature under `L` — it needs no DLC awareness. The spend key `b + s_X` exists only
//! after the oracle attests X (revealing `s_X`), so the token is spendable iff the outcome
//! occurred. See [`token`] and `hunch_dlc::conditional`.
//!
//! This unblocks the mint on **stable, shipping Cashu** (NUT-11 is final and implemented in CDK).
//!
//! ## What this crate models today
//!
//! The conditional-token construction: deriving each outcome's NUT-11 secret (lock + refund +
//! locktime) and redeeming a winning token from an oracle attestation. This is the part that was
//! blocked. The remaining mint engineering is NOT blocked and is deliberately out of scope here:
//!
//! - **Issuance/blind signatures**: standard NUT-00/02/03 via CDK (`cdk` crate) — unmodified.
//! - **Lightning deposit/withdraw**: LDK Node — standard.
//! - **Collateral / DLC funding tx (HIP-2)**: the mint funds the bilateral DLC; settlement is
//!   on-chain. The conditional tokens above gate redemption against the oracle attestation.
//!
//! None of those require a protocol change, which is the whole point of the NUT-11 pivot.

pub mod cashu_token;
pub mod token;

pub use token::{build_outcome_token, redeem_spend_secret, verify_token_unlock, OutcomeToken};
