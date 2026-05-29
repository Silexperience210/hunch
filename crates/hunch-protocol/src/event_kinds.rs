//! Hunch Nostr event kind constants (HIP-1).
//!
//! Kinds 88 + 89 are imported from NIP-88 (PR #1681). Kinds 30888 + 38888 + 30890..30892
//! are reserved by Hunch (HIP-1 §Reserved Kinds). Kind 30023 is NIP-23 long-form, used
//! for publishing the HIPs themselves.

/// NIP-88 oracle announce — oracle declares intent to attest a market outcome
pub const KIND_ORACLE_ANNOUNCE: u32 = 88;

/// NIP-88 oracle attestation — oracle Schnorr signature over the resolved outcome
pub const KIND_ORACLE_ATTESTATION: u32 = 89;

/// NIP-23 long-form content — used by Hunch to publish HIPs (kind 30023)
pub const KIND_LONG_FORM: u32 = 30023;

/// Hunch HIP-1 — market metadata (parameterized replaceable)
pub const KIND_MARKET: u32 = 30888;

/// Hunch HIP-1 — order placement (parameterized replaceable per Nostr range; one
/// outstanding bid/ask per pubkey per market via the `d` tag).
///
/// **Doc note:** HIP-1.md currently labels kind 38888 as "Ephemeral" — that's a
/// documentation bug (38888 is in the 30000-39999 parameterized-replaceable range).
/// HIP-1 corrigendum tracked; the spec-true class is parameterized-replaceable.
pub const KIND_ORDER: u32 = 38888;

/// Hunch HIP-1 — dispute (parameterized replaceable)
pub const KIND_DISPUTE: u32 = 30890;

/// Hunch HIP-1 — reputation attestation (parameterized replaceable)
pub const KIND_REPUTATION: u32 = 30891;

/// Hunch HIP-1 — mint announce (parameterized replaceable)
pub const KIND_MINT_ANNOUNCE: u32 = 30892;

/// All Hunch-reserved kinds (excluding the NIP-88 and NIP-23 dependencies).
pub const HUNCH_RESERVED_KINDS: &[u32] = &[
    KIND_MARKET,
    KIND_ORDER,
    KIND_DISPUTE,
    KIND_REPUTATION,
    KIND_MINT_ANNOUNCE,
];

/// Returns true if `kind` is a parameterized replaceable Nostr kind (30000..40000).
pub fn is_parameterized_replaceable(kind: u32) -> bool {
    (30000..40000).contains(&kind)
}

/// Returns true if `kind` is an ephemeral Nostr kind (20000..30000).
pub fn is_ephemeral(kind: u32) -> bool {
    (20000..30000).contains(&kind)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hunch_kinds_have_expected_classes() {
        // Per Nostr NIP-01 ranges: 30000-39999 = parameterized replaceable.
        assert!(is_parameterized_replaceable(KIND_MARKET));
        assert!(is_parameterized_replaceable(KIND_ORDER));
        assert!(is_parameterized_replaceable(KIND_DISPUTE));
        assert!(is_parameterized_replaceable(KIND_REPUTATION));
        assert!(is_parameterized_replaceable(KIND_MINT_ANNOUNCE));
        assert!(is_parameterized_replaceable(KIND_LONG_FORM));

        // NIP-88 dependencies are regular kinds (< 10000).
        assert!(!is_parameterized_replaceable(KIND_ORACLE_ANNOUNCE));
        assert!(!is_ephemeral(KIND_ORACLE_ANNOUNCE));
        assert!(!is_parameterized_replaceable(KIND_ORACLE_ATTESTATION));
    }

    #[test]
    fn reserved_kinds_list_is_complete() {
        assert_eq!(HUNCH_RESERVED_KINDS.len(), 5);
        assert!(HUNCH_RESERVED_KINDS.contains(&KIND_MARKET));
        assert!(HUNCH_RESERVED_KINDS.contains(&KIND_ORDER));
        assert!(HUNCH_RESERVED_KINDS.contains(&KIND_DISPUTE));
        assert!(HUNCH_RESERVED_KINDS.contains(&KIND_REPUTATION));
        assert!(HUNCH_RESERVED_KINDS.contains(&KIND_MINT_ANNOUNCE));
    }
}
