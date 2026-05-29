//! Hunch Nostr relay.
//!
//! A relay that verifies event signatures and validates Hunch-reserved kinds (HIP-1) against
//! `hunch-protocol` before storing them. The [`engine`] module is the pure, testable core
//! (validation + storage + filter matching); the binary wraps it in an async WebSocket server.

pub mod engine;

pub use engine::{matches_filter, IngestResult, Relay};
