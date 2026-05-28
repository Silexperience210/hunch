# NUT-CTF Spike Workspace

**Spike:** SPIKE-01 + SPIKE-02
**Status:** SPIKE-01 outreach drafted (queued); SPIKE-02 implementation deferred to user opsec
**Date:** 2026-05-28

This directory contains the Phase 1 spike work for the NUT-CTF integration (PR #337):

- `PATH-A-VALIDATION.md` — SPIKE-01 deliverable: validation framework + 3 maintainer outreach drafts
- `outreach-log.md` — outreach status tracker

## SPIKE-02 Implementation Status

SPIKE-02 (NUT-CTF working prototype on Bitcoin signet) requires:

1. A working Cashu mint implementation supporting the (still-draft) NUT-CTF spec
2. Bitcoin signet (Mutinynet) wallet with funded test sats
3. A test oracle pubkey + signing setup
4. End-to-end test: create market → fund Lightning → bet → resolve → withdraw

**Why not auto-implemented:** The reference `crates/hunch-mint-spike` crate is queued for Phase 2 implementation but its end-to-end signet test cannot run autonomously in a Claude session because:

- Mutinynet funding requires CAPTCHA / human approval at the faucet
- The test oracle needs a real, persistent Nostr pubkey
- Live signet transactions consume real (test) network resources
- DLEQ proof verification on token mint receive requires running a full Cashu mint locally

**User action when ready:**

```bash
# 1. Generate test wallet + fund from Mutinynet faucet
# 2. Add hunch-mint-spike to Cargo workspace members
# 3. Implement test against cdk + rust-dlc (skeleton in HIP-3 §Reference Implementation)
# 4. Run end-to-end: cargo run --bin spike-nut-ctf --features signet
```

This is the work that gates HIP-3's Status: Draft → Status: Final transition (per HIPs-reviews.md "HIP-3 special gate").

## Falsification Conditions (deferred)

Per HIP-3 §Test Vectors, the SPIKE-02 prototype must demonstrate:

1. Mint issues blinded YES/NO conditional tokens against a signet DLC
2. Oracle attestation (kind:89) is verifiable against the token's stored oracle pubkey
3. Spending a winning token via OUTCOME_MATCH predicate succeeds
4. Spending a losing token via OUTCOME_MATCH predicate fails (cryptographic rejection)
5. REFUND_AFTER_TIMEOUT predicate succeeds after wallclock passes refund_timeout
6. INVALID outcome routes both YES and NO tokens to refund at entry price

These tests will land in `crates/hunch-mint-spike/tests/e2e_signet.rs`.
