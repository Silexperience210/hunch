//! Hunch DLC primitives — shared by the oracle (signs) and the mint (locks/redeems).
//!
//! - [`attestation`] — DLC oracle Schnorr signing with a pre-committed nonce + the per-outcome
//!   signature point `S_X = R + e_X·P`.
//! - [`conditional`] — outcome-conditional ecash locks: a token locked to `L = B + S_X` is
//!   spendable only once the oracle reveals `s_X = dlog(S_X)` (the attestation), implementing
//!   HIP-3's OUTCOME_MATCH predicate on top of stable NUT-11 P2PK (no NUT-CTF dependency).
//!
//! Crypto is never hand-rolled (CLAUDE.md): signing/sig-point come from `ddk-dlc` (rust-dlc)
//! over libsecp256k1-zkp; the conditional locks are plain secp256k1 point/scalar addition.

pub mod attestation;
pub mod conditional;

pub use attestation::{sign_attestation_with_nonce, signature_point};
pub use conditional::{outcome_lock_key, outcome_unlock_secret, verify_unlock};
