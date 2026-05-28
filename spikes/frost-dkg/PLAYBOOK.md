# SPIKE-03: FROST 3-of-5 DKG Ceremony Playbook

**Spike:** SPIKE-03
**Status:** Playbook written; live signet ceremony deferred to user (requires 5 independent participants + persistent keys)
**Date:** 2026-05-28
**Requirement:** SPIKE-03 — FROST k-of-n DKG ceremony tested on signet with 3-of-5 setup; document playbook (init / DKG / multi-round signing / reset-rotate)

## Goal

Document the operational procedure for running a 3-of-5 FROST Distributed Key Generation (DKG) ceremony on Bitcoin signet using `frost-secp256k1-tr` v2.2+, including the Trail of Bits Feb 2024 length-check fix applied at the application layer.

The playbook must be precise enough for an external operator to follow without intervention by the Hunch maintainer, since pseudonymity and decentralization rules require multi-jurisdictional independent participants.

## Prerequisites

1. **5 independent participants.** Different operators, different hardware, different jurisdictions. Each participant has:
   - A long-term identity Nostr keypair (different from their FROST participant key)
   - A NIP-44 gift-wrap-capable Nostr client
   - Hardware-backed storage for their FROST secret share (Ledger / Trezor / air-gapped device / paper backup)
   - Rust toolchain installed for ceremony execution
2. **Coordinated time window.** Ceremony runs in ~2 hours synchronous; schedule via Nostr DM or external channel.
3. **Trail of Bits length-check awareness.** Each participant must verify other participants' coefficient commitment vectors have length exactly `k = 3` before accepting.

## Ceremony Phases

### Phase 0 — Identity Exchange (out-of-band)

Each participant publishes their long-term identity Nostr npub to a shared coordination event. Confirm npub authenticity via independent channels (Signal voice call, in-person, prior trust). This is the trust root for the ceremony.

### Phase 1 — DKG Round 1: Commitment Broadcast

Each participant `i` (1..5):

1. Generate a random degree-(k-1) = degree-2 polynomial `f_i(x) = a_i_0 + a_i_1*x + a_i_2*x^2 mod n`.
2. Compute the commitment vector `C_i = [g^a_i_0, g^a_i_1, g^a_i_2]` (length must be exactly 3).
3. Broadcast `C_i` via NIP-44 gift-wrapped DM to each of the other 4 participants.

**Application-layer length check (TOB Feb 2024 fix):**

```rust
for received_C_j in received_commitments {
    if received_C_j.len() != 3 {
        return Err(DKGError::MalformedCommitmentVector { participant: j, expected: 3, got: received_C_j.len() });
    }
}
```

If ANY received `C_j` has length != 3, abort ceremony and publicly identify the malformed participant.

### Phase 2 — DKG Round 2: Share Distribution

Each participant `i`:

1. For each other participant `j`, compute `share_ij = f_i(j) mod n`.
2. Send `share_ij` to `j` via NIP-44 gift-wrapped DM (NOT broadcast).

Each participant `j` receives 4 shares (one from each other participant) and verifies each share `share_ij` against the public commitment `C_i`:

```rust
// Verification: g^share_ij == prod_k(C_i[k]^(j^k)) for k in 0..3
let expected = C_i[0] * C_i[1].mul(j) * C_i[2].mul(j.pow(2));
if g.mul(share_ij) != expected {
    return Err(DKGError::ShareVerificationFailed { from: i, to: j });
}
```

If verification fails, publish a complaint event identifying the malformed share-sender. Other participants verify the complaint and decide whether to abort or proceed without the malicious participant (depending on whether honest participants remain ≥ k).

### Phase 3 — DKG Round 3: Group Key Derivation

Each participant `i` computes their secret share:

```
s_i = sum_j(share_ji) mod n     // sum of shares received from all participants including self
```

Each participant computes the group public key:

```
Y = prod_j(C_j[0])     // product of constant-term commitments from all participants
```

All 5 participants compare their computed `Y` via Nostr broadcast; if any participant computes a different `Y`, ceremony aborts and is re-run.

### Phase 4 — Ceremony Output

After Phase 3 success:

- Aggregate public key `Y` is announced as a `kind:88` oracle-announce event (HIP-1)
- Each participant stores `s_i` in their hardware-backed storage; share never leaves the device
- Coordination event is closed; ceremony transcript (anonymized: round numbers + commitment hashes only, no shares) is published as `kind:1` for ceremony provenance

## Signing Ceremony (post-DKG)

When the quorum needs to attest a market outcome:

### Round 1 — Nonce Commitment

`k = 3` participants (any 3 of the 5) each:

1. Generate random nonce pair `(d_i, e_i)`.
2. Compute commitments `(D_i, E_i) = (g^d_i, g^e_i)`.
3. Broadcast `(D_i, E_i)` to the other 2 signers via NIP-44 DM.

### Round 2 — Signature Share

Each participant `i`:

1. Compute binding factor `ρ_i = H("FROST/rho" || i || msg || B)` where `B` is the concatenation of all `(D_j, E_j)` for signers.
2. Compute group commitment `R = sum_j(D_j + ρ_j * E_j)`.
3. Compute challenge `c = H_tag("BIP0340/challenge", R || Y || msg)`.
4. Compute Lagrange coefficient `λ_i = prod_j(j / (j - i))` for `j` in signer set, `j != i`.
5. Compute signature share `z_i = d_i + ρ_i * e_i + λ_i * s_i * c mod n`.
6. Broadcast `z_i` to coordinator (any signer can be coordinator).

### Round 3 — Aggregation

Coordinator (or any signer):

```
z = sum_i(z_i) mod n
signature = (R, z)
```

Verify via standard BIP-340:

```rust
assert!(schnorr::verify(Y, msg, signature));
```

If verification fails, identify misbehaving signer (re-run with different signer subset) and exclude them from future ceremonies.

The verified signature is published as a `kind:89` oracle-attestation event (HIP-1) and consumed by the DLC adapter (HIP-2).

## Reset / Rotate Procedure

Every 90 days OR when quorum membership changes:

1. Announce rotation intent via `kind:1` event signed by the current group key, including the rotation date and rationale.
2. Run a fresh DKG ceremony (Phases 0-4 above) with the new participant set.
3. Publish a `kind:88` event announcing the new aggregate public key as a successor to the old one (link via tag `rotated_from: <old_npub>`).
4. The old group key continues attesting markets opened before rotation (HIP-1 §Key Rotation). The new key attests markets opened after.
5. Decommissioned old shares are not deleted (in case of late-arriving attestations needed) but are tagged `decommissioned: <date>` in the participant's hardware storage.

## Abort / Re-run Procedure

If DKG aborts at any phase:

1. Identify the cause (malformed commitment? failed share verification? group key mismatch?).
2. If the cause is a single misbehaving participant, publish a public abort event identifying them and run a fresh DKG with the remaining 4 participants (now 3-of-4 instead of 3-of-5, or recruit a replacement).
3. If the cause is a network / protocol error (NIP-44 channel failure, clock skew), retry after a brief cool-off.
4. Document each ceremony attempt in `spikes/frost-dkg/ceremony-transcripts/` (creation date + outcome).

## Falsification Conditions

Per CONTEXT.md decision D-08 (FROST `frost-secp256k1-tr` v2.2+ locked) and RESEARCH §4:

- **VALIDATED:** DKG produces a verifiable aggregate Schnorr-compatible group pubkey; 3-of-5 signing produces a signature that passes BIP-340 verification against the aggregate pubkey; ceremony reproducible from seeded test vectors.
- **FALSIFIED:** Any deviation from BIP-340 signature compatibility (would break HIP-2 DLC adapter); inability to verify the TOB Feb 2024 length check; any non-Schnorr output (would force HIP-2 re-spec).

## Reference Implementation

`crates/hunch-oracle-spike/tests/frost_dkg_3of5.rs` — Phase 2 deliverable, automated 3-of-5 DKG + signing test using `frost-secp256k1-tr` v2.2+

`crates/hunch-oracle` — Phase 2 production implementation

## Live Ceremony Status

The actual 5-participant ceremony on signet is queued for Phase 2 when the operator recruits the 5 independent participants. Phase 1 deliverable is the playbook + the test-vector-based unit tests in `frost_dkg_3of5.rs` (which can be run autonomously without 5 humans).

## References

1. RFC 9591 — FROST. https://datatracker.ietf.org/doc/rfc9591/
2. Trail of Bits Feb 2024 disclosure. https://blog.trailofbits.com/2024/02/20/breaking-the-shared-key-in-threshold-signature-schemes/
3. ZcashFoundation/frost. https://github.com/ZcashFoundation/frost
4. crates.io: frost-secp256k1-tr. https://crates.io/crates/frost-secp256k1-tr
5. BIP-340. https://github.com/bitcoin/bips/blob/master/bip-0340.mediawiki
6. NIP-44. https://nips.nostr.com/44
7. HIP-4 — Multi-oracle FROST. [`../docs/HIP-4.md`](../docs/HIP-4.md)
