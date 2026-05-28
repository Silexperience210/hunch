# HIP-3: Cashu NUT-CTF Integration for Hunch

```
HIP:      3
Title:    Cashu NUT-CTF Integration for Hunch Markets
Authors:  Silex <silex@hunch.markets>
Status:   Draft
Type:     Standards Track
Created:  2026-05-28
License:  MIT
Requires: HIP-0, HIP-1, HIP-2
```

> **Status note:** HIP-3 is in DRAFT and will remain so until Phase 1 SPIKE-02 (NUT-CTF signet prototype) successfully demonstrates the full flow on Bitcoin signet AND the upstream Cashu NUT-CTF proposal (cashubtc/nuts#337) stabilizes. Transition to FINAL requires both gates.

## Abstract

HIP-3 specifies how a Hunch mint extends the Cashu Conditional Token Framework ([cashubtc/nuts#337](https://github.com/cashubtc/nuts/pull/337)) to issue conditional tokens representing YES, NO, or refund claims against an underlying DLC contract (HIP-2). The mint accepts Lightning deposits, issues blinded conditional tokens that are spendable only when the associated oracle attestation matches the token's outcome, and redeems winning tokens for Lightning withdrawals after DLC settlement. The integration relies on NUT-CTF's outcome-conditional spend predicates rather than the bilateral NUT-DLC pattern of the obsolete cashubtc/nuts#128.

## Motivation

A single DLC supports two counterparties. A prediction market needs many bettors. HIP-3 solves this by making the mint the bilateral DLC counterparty (HIP-2) and issuing many-to-many tokens to bettors that resolve in lockstep with the DLC. The NUT-CTF framework provides the cryptographic primitive: blinded tokens whose unblinding requires a specific oracle attestation.

The earlier proposal (cashubtc/nuts#128 by conduition) was a bilateral NUT-DLC primitive. It was closed by Cashu maintainers on 2025-05-20 with the comment "Closing as there is no active work. Please reopen if work continues." NUT-CTF (cashubtc/nuts#337 by joemphilips, opened 2026-02-07) is its architectural successor and is the chosen direction for Hunch (see CONTEXT.md decision D-01).

## Specification

### Token Lifecycle

A bettor's interaction with a Hunch mint is a five-step lifecycle:

1. **Deposit**: bettor pays a Lightning invoice issued by the mint for amount A sat.
2. **Mint**: bettor sends blinded outputs and receives signed conditional tokens. Each token carries a spend predicate: "spendable only if oracle attestation matches outcome X". The bettor chooses YES tokens or NO tokens at mint time.
3. **Hold / trade**: bettor holds the tokens until settlement, or trades them through the mint's Tier 1 orderbook (HIP-1 kind:38888 ephemeral orders) or via Tier 2 P2P matching.
4. **Settle**: after market expiry, the oracle publishes a kind:89 attestation (HIP-1). The mint observes the attestation, settles the underlying DLC (HIP-2), and unlocks redemption for the winning side.
5. **Redeem / refund**: bettor swaps winning tokens for fresh tokens that are immediately Lightning-redeemable. Losing tokens become unspendable. Under INVALID outcome, all bettors receive refund tokens redeemable at entry price.

### Token Format

A Hunch NUT-CTF token extends the canonical Cashu token (NUT-00) with conditional-spend metadata. Per NUT-CTF (PR #337), the token blob includes:

```
token = {
  amount: <integer sat>,
  C: <blind-signed commitment, 33 bytes>,
  conditions: {
    market: "<pubkey>:30888:<d>",        // HIP-1 market identifier
    outcome: "YES" | "NO" | "REFUND",
    oracle_pubkey: <hex 32 bytes>,
    spend_predicate: "OUTCOME_MATCH" | "REFUND_AFTER_TIMEOUT"
  },
  proof: <NUT-12 DLEQ proof, mandatory>
}
```

The mint signs the token using the standard Cashu blind-signature scheme (NUT-01 / NUT-02), but the keyset is parameterized by `(market_id, outcome, oracle_pubkey)`. The mint maintains separate keysets per market.

NUT-12 DLEQ proofs are mandatory (CLAUDE.md "Do" list); bettors verify the proof on every token receive to detect mint cheating.

### Spend Predicate: OUTCOME_MATCH

To spend a YES token, the bettor presents the token plus a witness containing the oracle's published kind:89 attestation (event ID + Schnorr signature). The mint verifies:

1. The attestation's `pubkey` field matches `conditions.oracle_pubkey` in the token
2. The attestation's content payload outcome matches `conditions.outcome` in the token
3. The Schnorr signature over the attestation is valid against `conditions.oracle_pubkey`
4. The attestation's `market` tag matches `conditions.market` in the token

If all four checks pass, the mint accepts the spend and issues redemption tokens (either Lightning-redeemable proofs or fresh standard Cashu proofs).

### Spend Predicate: REFUND_AFTER_TIMEOUT

For tokens minted with `spend_predicate: REFUND_AFTER_TIMEOUT`, the bettor may unilaterally spend the token after wallclock passes the market's `refund_timeout` (HIP-1 kind:30888). The mint verifies only the timeout and accepts the spend, releasing refund proofs.

This predicate exists because the oracle may go silent. Bettors deserve a recovery path independent of mint cooperation; the timeout-gated refund predicate is enforced by the underlying DLC refund branch (HIP-2 §Refund Branch).

### Refund Mechanics for INVALID Outcome

When the oracle attests INVALID (HIP-2 §INVALID Outcome Semantics), the mint receives the refund branch of the DLC. The mint then accepts spends of both YES and NO tokens at their entry mint price. Bettors swap their YES/NO tokens for refund tokens redeemable at face value via Lightning. There is no winner; there is no loser.

### Mint State Machine

```
[Market open]    → mint accepts deposits, issues YES/NO tokens
[Market expiry]  → mint freezes issuance; existing tokens remain in circulation
[Oracle signs]   → mint settles DLC, unlocks spending of matching outcome tokens
[Refund timeout] → mint accepts unilateral REFUND_AFTER_TIMEOUT spends from any token holder
```

The mint MUST publish a `kind:30892` mint-announce event (HIP-1) updated whenever it transitions state.

### Reserves Disclosure

Per CLAUDE.md "Engineering Principles", the mint MUST publish weekly reserves proofs documenting:
- Total YES tokens outstanding per market
- Total NO tokens outstanding per market
- Bitcoin held in DLC funding outputs (txid:vout references)
- Bitcoin held in Lightning channels for redemption

The reserves proof URL is published as the `reserves_proof` tag in the mint's `kind:30892` event.

## Backwards Compatibility

HIP-3 builds on Cashu NUT-CTF as proposed in cashubtc/nuts#337. NUT-CTF is itself a draft and may evolve. HIP-3 tracks NUT-CTF stability:

- If NUT-CTF merges as PR #337 as currently scoped, HIP-3 transitions to Final after Hunch's SPIKE-02 demonstrates a working signet prototype.
- If NUT-CTF re-scopes significantly, HIP-3 issues a corrigendum reflecting the upstream changes.
- If NUT-CTF stalls (similar to PR #128's fate), Hunch may revisit Path B (PR #128 fork) or Path C (custodial-promise fallback) — both currently de-scoped per CONTEXT.md decision D-01.

Earlier draft references in `.planning/research/*.md` to PR #128 are OBSOLETE; see those files' Corrigendum sections (Plan 01 Task 1).

## Test Vectors

Test vectors for the NUT-CTF integration land in `crates/hunch-mint-spike/tests/e2e_signet.rs` during Phase 1 SPIKE-02. The full Phase 2 implementation in `crates/hunch-mint/tests/` will add reference token-format vectors against the canonical NUT-CTF spec.

## Reference Implementation

- Phase 1 prototype: `crates/hunch-mint-spike` (SPIKE-02 deliverable, throwaway)
- Phase 2 production: `crates/hunch-mint`
- Library: [cdk (cashu dev kit)](https://github.com/cashubtc/cdk) — Hunch will track CDK's NUT-CTF support as it lands

## References

1. cashubtc/nuts#337 — NUTs for Prediction Markets (Conditional Token Framework). https://github.com/cashubtc/nuts/pull/337
2. cashubtc/nuts#128 — Bilateral NUT-DLC (CLOSED 2025-05-20, historical reference only). https://github.com/cashubtc/nuts/pull/128
3. cashubtc/nuts, NUT-00 — Notation and conventions. https://github.com/cashubtc/nuts/blob/main/00.md
4. cashubtc/nuts, NUT-01 — Mint public keys. https://github.com/cashubtc/nuts/blob/main/01.md
5. cashubtc/nuts, NUT-02 — Keysets. https://github.com/cashubtc/nuts/blob/main/02.md
6. cashubtc/nuts, NUT-12 — Offline ecash via DLEQ proofs (mandatory in Hunch). https://github.com/cashubtc/nuts/blob/main/12.md
7. cashubtc/cdk — Reference Cashu mint implementation in Rust. https://github.com/cashubtc/cdk
8. HIP-1 — Nostr event kinds. [`./HIP-1.md`](./HIP-1.md)
9. HIP-2 — DLC contract structure. [`./HIP-2.md`](./HIP-2.md)

---

*HIP-3 — Cashu NUT-CTF integration. STATUS: DRAFT.*
