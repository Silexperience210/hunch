//! Hunch Nostr client primitives.
//!
//! Shared between `hunch-oracle`, `hunch-cli`, and any other Hunch service that talks to a
//! relay. Deliberately tiny and SDK-free — NIP-01 event id + BIP-340 signing on workspace
//! `secp256k1`/`sha2`/`serde_json`, plus a minimal WebSocket publish/query client. Keeps the
//! critical path small and auditable (CLAUDE.md — "treat Rust services like Bitcoin Core").

pub mod event;
pub mod relay;

pub use event::{build_signed_event, event_id, event_tags, verify_event, Tag};
pub use relay::{publish, publish_all, query, query_all, PublishOutcome};
