//! Hunch oracle (v1, single-key).
//!
//! Publishes NIP-88 oracle announces (kind 88) and attestations (kind 89) over Nostr, signing
//! HIP-2 outcome attestations with a single secp256k1 key via `hunch-protocol`'s
//! [`SingleKeySigner`](hunch_protocol::oracle::SingleKeySigner). FROST k-of-n threshold signing
//! (HIP-4) is deferred to Phase 4 and slots in behind the same `OracleSigner` trait without
//! changing the mint or DLC adapter.
//!
//! NIP-01 event signing + relay client live in the shared `hunch-nostr` crate.
//!
//! Layers:
//! - [`dlc`] — DLC oracle attestation (pre-committed nonce) via `ddk-dlc`.
//! - [`nonce_store`] — persistent per-market nonce store with reuse guard.
//! - [`service`] — the oracle identity wiring the protocol types to signed events.

pub mod dlc;
pub mod nonce_store;
pub mod service;

pub use service::OracleService;

use rand::RngCore;
use secp256k1::{Keypair, Secp256k1, SecretKey};

/// Generates a fresh 32-byte secp256k1 secret key, returned as `(secret_hex, xonly_pubkey_hex)`.
///
/// Uses the OS CSPRNG and rejection-samples until the bytes are a valid scalar.
pub fn generate_keypair() -> (String, String) {
    let secp = Secp256k1::new();
    loop {
        let mut bytes = [0u8; 32];
        rand::rngs::OsRng.fill_bytes(&mut bytes);
        if let Ok(sk) = SecretKey::from_slice(&bytes) {
            let kp = Keypair::from_secret_key(&secp, &sk);
            let (xonly, _) = kp.x_only_public_key();
            return (hex::encode(bytes), hex::encode(xonly.serialize()));
        }
    }
}

/// Generates a fresh x-only nonce pubkey for an announce, returned as `(secret_hex, pubkey_hex)`.
///
/// The oracle must persist the nonce secret to later bind it into an adapter signature
/// (deferred to SPIKE-02). See [`service::OracleService::build_announce_event`].
pub fn generate_nonce() -> (String, String) {
    generate_keypair()
}
