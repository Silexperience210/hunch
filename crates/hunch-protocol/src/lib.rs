//! Hunch protocol — shared types.
//!
//! Implements HIP-1 (Nostr event kinds), HIP-2 (DLC outcomes), HIP-4 (FROST signer trait),
//! HIP-5 (reputation event types). See `docs/HIP-*.md` for the protocol specifications.
//!
//! Phase 2 deliverable. Phase 1 (`bcf24cb`..`249c68c`) shipped this crate as an empty stub.

pub mod error;
pub mod event_kinds;
pub mod market;
pub mod order;
pub mod dispute;
pub mod reputation;
pub mod mint_announce;
pub mod outcome;
pub mod oracle;

pub use error::ProtocolError;
pub use event_kinds::*;
pub use market::{Market, MarketContent};
pub use order::{Order, OrderSide, OrderKind};
pub use dispute::Dispute;
pub use reputation::{Reputation, ReputationScope};
pub use mint_announce::MintAnnounce;
pub use outcome::Outcome;
pub use oracle::{OracleSigner, OracleAttestation, OracleAnnounce, SingleKeySigner};
