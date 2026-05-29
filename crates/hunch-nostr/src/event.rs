//! NIP-01 Nostr event construction + signing.
//!
//! We build and sign Nostr events directly from workspace primitives
//! (`secp256k1` + `sha2` + `serde_json`) rather than pulling a Nostr SDK. The only
//! protocol-sensitive part is the canonical serialization for the event id; everything
//! else is BIP-340 Schnorr over that id, reusing the same secp256k1 the rest of the
//! workspace uses.
//!
//! ## NIP-01 id computation
//!
//! `id = sha256(utf8(json([0, pubkey, created_at, kind, tags, content])))`, where the JSON
//! is the compact form with no insignificant whitespace and the six mandated string escapes
//! (`\"`, `\\`, `\n`, `\r`, `\t`, `\b`, `\f`). `serde_json::to_string` produces exactly this
//! form: no whitespace, those escapes for control characters, raw UTF-8 otherwise. The
//! signature is a BIP-340 Schnorr signature over the 32-byte id.

use secp256k1::{Keypair, Secp256k1, Signing};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

/// A Nostr tag: an ordered list of strings, first element is the tag name (`["e", "<id>"]`).
pub type Tag = Vec<String>;

/// Computes the NIP-01 event id (the 32-byte sha256 of the canonical serialization).
pub fn event_id(pubkey_hex: &str, created_at: i64, kind: u32, tags: &[Tag], content: &str) -> [u8; 32] {
    // Per NIP-01 the id preimage is a JSON array, NOT an object: [0, pubkey, created_at, kind, tags, content].
    let preimage = json!([0, pubkey_hex, created_at, kind, tags, content]);
    let serialized = serde_json::to_string(&preimage)
        .expect("serializing a JSON array of strings/ints never fails");
    let mut hasher = Sha256::new();
    hasher.update(serialized.as_bytes());
    hasher.finalize().into()
}

/// Builds a fully signed NIP-01 event as a JSON object ready to wrap in `["EVENT", ...]`.
///
/// The event is signed with a deterministic BIP-340 nonce (`sign_schnorr_no_aux_rand`), so a
/// given `(key, kind, tags, content, created_at)` always yields the same id and signature —
/// useful for reproducible tests and idempotent re-publishing.
pub fn build_signed_event<C: Signing>(
    secp: &Secp256k1<C>,
    keypair: &Keypair,
    kind: u32,
    tags: Vec<Tag>,
    content: String,
    created_at: i64,
) -> Value {
    let (xonly, _parity) = keypair.x_only_public_key();
    let pubkey_hex = hex::encode(xonly.serialize());
    let id = event_id(&pubkey_hex, created_at, kind, &tags, &content);
    let sig = secp.sign_schnorr_no_aux_rand(&id, keypair);
    json!({
        "id": hex::encode(id),
        "pubkey": pubkey_hex,
        "created_at": created_at,
        "kind": kind,
        "tags": tags,
        "content": content,
        "sig": hex::encode(sig.as_ref()),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use secp256k1::{schnorr::Signature, SecretKey, XOnlyPublicKey};

    fn test_keypair() -> Keypair {
        // Deterministic test-only key. Not a real secret.
        let sk = SecretKey::from_slice(
            &hex::decode("5f80b1ac81a47b0e3ee7e3bd4e23c1f3a96a0b56cd96b3a5d99e3a7a76d8c3a0").unwrap(),
        )
        .unwrap();
        Keypair::from_secret_key(&Secp256k1::new(), &sk)
    }

    #[test]
    fn id_matches_recomputation_and_sig_verifies() {
        let secp = Secp256k1::new();
        let kp = test_keypair();
        let tags = vec![vec!["market".to_string(), "aa:30888:m".to_string()]];
        let event = build_signed_event(&secp, &kp, 89, tags.clone(), "body".into(), 1_700_000_000);

        // Recompute id from the emitted fields and confirm it matches.
        let pubkey_hex = event["pubkey"].as_str().unwrap();
        let recomputed = event_id(pubkey_hex, 1_700_000_000, 89, &tags, "body");
        assert_eq!(event["id"].as_str().unwrap(), hex::encode(recomputed));

        // The signature must verify against the event id under the event pubkey.
        let sig = Signature::from_slice(&hex::decode(event["sig"].as_str().unwrap()).unwrap()).unwrap();
        let xonly = XOnlyPublicKey::from_slice(&hex::decode(pubkey_hex).unwrap()).unwrap();
        secp.verify_schnorr(&sig, &recomputed, &xonly).unwrap();
    }

    #[test]
    fn signing_is_deterministic() {
        let secp = Secp256k1::new();
        let kp = test_keypair();
        let a = build_signed_event(&secp, &kp, 88, vec![], "x".into(), 42);
        let b = build_signed_event(&secp, &kp, 88, vec![], "x".into(), 42);
        assert_eq!(a, b);
    }

    #[test]
    fn canonical_serialization_has_no_whitespace_and_escapes_control_chars() {
        // content with a newline + quote must be escaped per NIP-01; no spaces in the array.
        let id = event_id("ab".repeat(32).as_str(), 1, 1, &[], "line1\n\"q\"");
        let preimage = json!([0, "ab".repeat(32), 1, 1, Vec::<Tag>::new(), "line1\n\"q\""]);
        let s = serde_json::to_string(&preimage).unwrap();
        assert!(!s.contains(' '), "canonical form must not contain spaces: {s}");
        assert!(s.contains("\\n"), "newline must be escaped");
        assert!(s.contains("\\\""), "quote must be escaped");
        assert_eq!(id, event_id("ab".repeat(32).as_str(), 1, 1, &[], "line1\n\"q\""));
    }
}
