//! Hunch protocol — shared types.
//!
//! Implements HIP-1 (Nostr event kinds), HIP-2 (DLC outcomes), HIP-4 (FROST signer trait),
//! HIP-5 (reputation event types). See `docs/HIP-*.md` for the protocol specifications.
//!
//! Phase 2 deliverable. Phase 1 (`bcf24cb`..`249c68c`) shipped this crate as an empty stub.

pub mod dispute;
pub mod error;
pub mod event_kinds;
pub mod market;
pub mod mint_announce;
pub mod oracle;
pub mod order;
pub mod outcome;
pub mod reputation;

pub use dispute::Dispute;
pub use error::ProtocolError;
pub use event_kinds::*;
pub use market::{Market, MarketContent};
pub use mint_announce::MintAnnounce;
pub use oracle::{OracleAnnounce, OracleAttestation, OracleSigner, SingleKeySigner};
pub use order::{Order, OrderKind, OrderSide};
pub use outcome::Outcome;
pub use reputation::{Reputation, ReputationScope};
