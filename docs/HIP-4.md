# HIP-4: Multi-Oracle FROST Protocol

```
HIP:      4
Title:    Multi-Oracle FROST k-of-n Threshold Schnorr Protocol
Authors:  Silex <silex@hunch.markets>
Status:   Draft
Type:     Standards Track
Created:  2026-05-28
License:  MIT
Requires: HIP-0, HIP-1, HIP-2
```

## Abstract

HIP-4 specifies the threshold-Schnorr scheme used by Hunch markets that elect a k-of-n oracle quorum instead of a single oracle. The scheme is FROST as standardized in RFC 9591 over secp256k1 with BIP-340 tagged hashing, using the `frost-secp256k1-tr` reference implementation (Zcash Foundation) v2.2 or later. The output of a successful FROST signing ceremony is a single Schnorr signature indistinguishable from a single-signer signature; the DLC adapter (HIP-2) consumes it identically. HIP-4 covers Distributed Key Generation (DKG), signing ceremony, key rotation, and operational practices for 3-of-5 reference deployments.

## Motivation

A single-oracle market concentrates trust: the oracle either signs honestly, signs maliciously, or stays silent. Honest single oracles are sufficient for most markets, but high-stakes markets benefit from multi-oracle quorums where no single operator can attest unilaterally and no single failure makes the market unresolvable. FROST gives the cryptographic primitive: a threshold scheme whose output is a Schnorr signature compatible with BIP-340 verification, so the DLC layer (HIP-2) needs no changes.

Reasons to choose FROST over alternatives:

- **MuSig2** requires all participants to be online for every signing — no fault tolerance during attestation.
- **ECDSA threshold schemes** are more complex and require non-standard verification.
- **FROST** standardized as RFC 9591 (2024), with mature Rust reference (`frost-secp256k1-tr`), Schnorr verification identical to single-signer, and tolerance for k-1 silent or compromised participants.

## Specification

### Parameters

A FROST quorum is parameterized by `(n, k)` where `n` is the total number of participants and `k` is the threshold (minimum signers required). HIP-4 RECOMMENDS:

- `n = 5, k = 3` for high-stakes markets (default in Phase 1 reference deployment)
- `n = 3, k = 2` for medium-stakes
- `n = 7, k = 4` for very-high-stakes

Implementations MAY choose other parameters within FROST's safety bounds (k <= n, k >= 1).

### Distributed Key Generation (DKG)

HIP-4 uses Pedersen DKG with the Trail of Bits February 2024 fix applied at the application layer. Specifically:

> Each participant MUST verify that every other participant's published coefficient commitment vector has length exactly `k` before accepting it. If any vector has length != k, the participant MUST abort the ceremony.

The DKG flow:

1. **Round 1 — Commitment broadcast.** Each participant i generates a random degree-(k-1) polynomial f_i and broadcasts the commitment vector `[g^c_i_0, g^c_i_1, ..., g^c_i_(k-1)]`. Application-layer check: each received vector has length == k.
2. **Round 2 — Share distribution.** Each participant i sends share `f_i(j)` to each other participant j over an encrypted channel (NIP-44 gift-wrapped Nostr DMs are the reference channel).
3. **Round 3 — Verification.** Each participant verifies received shares against the published commitments. If any share fails verification, the participant publishes a complaint event identifying the malformed share.
4. **Output.** Each participant computes their secret share `s_i = sum_j f_j(i)` and the public group key `Y = sum_j g^c_j_0`. The public group key is the aggregate oracle pubkey published in HIP-1 kind:88 oracle-announce.

### Signing Ceremony

Per FROST RFC 9591:

1. **Round 1 — Nonce commitment.** k participants each generate (d_i, e_i) nonce pair and broadcast (D_i, E_i) = (g^d_i, g^e_i).
2. **Round 2 — Signature share.** Each participant computes binding factor ρ_i and signature share z_i = d_i + e_i * ρ_i + λ_i * s_i * c (where c is the challenge hash). Each broadcasts z_i.
3. **Aggregation.** The coordinator (or any participant) sums z_i and (R = sum of D_i * E_i^ρ_i) to form the final Schnorr signature (R, z).

The coordinator role is rotatable — any participant or external party can aggregate. Failed aggregations are detectable (final signature fails BIP-340 verify), at which point the participants identify and exclude the misbehaving share-holder and re-run.

### Coordination Channel

HIP-4 specifies Nostr NIP-44 gift-wrapped direct messages as the reference coordination channel between FROST participants. This choice keeps the trust assumptions consistent (Nostr is already the discovery channel) and means no additional infrastructure (no Signal account, no shared backend). Participants list each other's pubkeys in a private coordination event posted only between quorum members.

Implementations MAY use alternative channels (Matrix, encrypted email, Signal) as long as they provide authenticated, encrypted, ordered message delivery between participants.

### Key Rotation

Long-running oracle quorums SHOULD rotate keys periodically. HIP-4 RECOMMENDS rotation every 90 days or after any participant change. Rotation is a fresh DKG ceremony producing a new aggregate pubkey; the old aggregate pubkey is decommissioned and announced as rotated in a follow-up kind:88 event.

Rotations MUST overlap with outstanding markets: the old key continues attesting markets opened before rotation; the new key attests markets opened after. A market's `oracle` tag (HIP-1 kind:30888) pins the key version for that market's lifetime.

### Reset Procedure

If a quorum is compromised (e.g., k participants are coerced or compromised), the surviving honest participants MUST abort the current quorum, publish a public abort event, and initialize a fresh DKG. Markets that depended on the compromised quorum's pending attestations resolve to INVALID via the HIP-2 INVALID outcome.

### Participant Selection

Participants in a 3-of-5 FROST quorum SHOULD be operationally independent: different operators, different jurisdictions, different hardware. Co-located participants reduce the quorum's effective `k` below the nominal value.

## Backwards Compatibility

HIP-4 does not modify HIP-1 or HIP-2 — the quorum's aggregate Schnorr signature is consumed identically by the DLC adapter. Markets that choose single-oracle attestation interact with HIP-2 unchanged.

If BIP-445 (the in-progress BIP standardizing FROST for Bitcoin) lands with conventions that differ from RFC 9591 in ways relevant to Hunch (tagging conventions, nonce derivation, etc.), HIP-4 will issue a corrigendum aligning to BIP-445.

## Test Vectors

Test vectors for the 3-of-5 reference DKG and signing ceremony land in `crates/hunch-oracle-spike/tests/frost_dkg_3of5.rs` during Phase 1 SPIKE-03. Vectors verify:

- DKG produces a group pubkey reproducibly from the same seed
- Trail of Bits length check rejects malformed coefficient vectors
- 3-of-5 quorum can sign with any 3 of the 5 participants
- The final signature passes BIP-340 verify against the group pubkey

## Reference Implementation

- Phase 1 prototype: `crates/hunch-oracle-spike` (SPIKE-03 deliverable)
- Phase 2 production: `crates/hunch-oracle`
- Library: [`frost-secp256k1-tr`](https://crates.io/crates/frost-secp256k1-tr) v2.2+

## Operational Playbook (Reference 3-of-5 Deployment)

1. **Participant onboarding**: 5 operators each generate a long-term identity keypair, exchange Nostr pubkeys, establish NIP-44 gift-wrap channels.
2. **DKG ceremony**: scheduled time, all 5 participants online, runs in ~2 hours (3 rounds with verification pauses). Output: aggregate pubkey, individual secret shares.
3. **Hardware**: secret shares stored in air-gapped hardware (ColdCard, hardware security module, or paper-backup with multi-factor). Never in cloud.
4. **Signing ceremony**: when a market needs attestation, the coordinating participant publishes a `kind:88` announce, gathers signature shares from k participants (default 3) over 24-72 hours, aggregates, publishes `kind:89` attestation.
5. **Rotation**: every 90 days OR after any participant changes; fresh DKG, new aggregate pubkey, announce as rotated.

A reference playbook with concrete CLI commands lives in `docs/playbooks/FROST-DKG-3of5.md` (Plan 03 SPIKE-03 deliverable).

## References

1. RFC 9591 — Flexible Round-Optimized Schnorr Threshold (FROST) Signing Protocol. https://datatracker.ietf.org/doc/rfc9591/
2. BIP-445 (draft) — FROST signatures for Bitcoin. https://github.com/bitcoin/bips (search for BIP-445)
3. BIP-340 — Schnorr Signatures for secp256k1. https://github.com/bitcoin/bips/blob/master/bip-0340.mediawiki
4. ZcashFoundation/frost — Reference FROST implementation in Rust. https://github.com/ZcashFoundation/frost
5. crates.io: frost-secp256k1-tr — Taproot-compatible FROST crate. https://crates.io/crates/frost-secp256k1-tr
6. Trail of Bits, *Breaking the Shared Key in Threshold Signature Schemes* (Feb 2024) — DKG coefficient length disclosure. https://blog.trailofbits.com/2024/02/20/breaking-the-shared-key-in-threshold-signature-schemes/
7. nostr-protocol/nips, NIP-44 — Encrypted direct messages (FROST coordination channel). https://nips.nostr.com/44
8. HIP-1 — Nostr event kinds (oracle announce/attest). [`./HIP-1.md`](./HIP-1.md)
9. HIP-2 — DLC contract structure (consumes the aggregated FROST signature). [`./HIP-2.md`](./HIP-2.md)

---

*HIP-4 — Multi-oracle FROST.*
