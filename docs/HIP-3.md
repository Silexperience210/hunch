# HIP-3: Conditional Tokens for Hunch (NUT-11 P2PK to Oracle Signature Point)

```
HIP:      3
Title:    Conditional Tokens for Hunch Markets via NUT-11 P2PK
Authors:  Silex <silex@hunch.markets>
Status:   Draft
Type:     Standards Track
Created:  2026-05-28
Updated:  2026-05-29
License:  MIT
Requires: HIP-0, HIP-1, HIP-2
```

> **Status note:** HIP-3 is in DRAFT. It originally targeted Cashu NUT-CTF (cashubtc/nuts#337); SPIKE-02 (2026-05-29) resolved that NUT-CTF is **not required** — outcome-conditional tokens are built on the **final, shipping NUT-11 P2PK** spec instead (see `spikes/nut-ctf/SPIKE-02-RESOLUTION.md`). Transition to FINAL now requires only: (1) a signet end-to-end demo (issuance + redemption), and (2) external review. The NUT-CTF stabilization gate has been removed.

## Abstract

HIP-3 specifies how a Hunch mint issues conditional ecash representing YES or NO claims against an underlying DLC contract (HIP-2). A Hunch outcome token is an ordinary **NUT-11 Pay-to-Pubkey (P2PK)** Cashu proof, locked to a key derived from the market's oracle. The token becomes spendable only once the oracle publishes the matching outcome attestation (HIP-1 kind:89). The mint enforces only NUT-11 — it needs no DLC awareness and no protocol extension. Lightning deposits/withdrawals and blind-signature issuance use unmodified Cashu (NUT-00/01/02/03). The DLC conditionality is encoded entirely in the P2PK lock key.

## Motivation

A single DLC supports two counterparties; a prediction market needs many bettors. HIP-3 makes the mint the bilateral DLC counterparty (HIP-2) and issues many-to-many tokens to bettors that resolve in lockstep with the DLC.

The earlier bilateral primitive (cashubtc/nuts#128 by conduition) was **closed** by Cashu maintainers on 2025-05-20. Its successor, NUT-CTF (cashubtc/nuts#337, "NUTs for prediction markets"), remains a **draft/WIP** and is not implemented in any shipping mint. Depending on it blocked the Hunch mint.

**SPIKE-02 found that no conditional-token NUT is needed.** The DLC oracle already publishes, for each outcome `X`, an implicit public key `S_X` whose secret is revealed exactly when (and only when) the oracle attests `X`. Locking a token to `S_X` (combined with the bettor's key) yields outcome-conditional spendability using only NUT-11, which is final and implemented in CDK.

## Specification

### Oracle Signature Point

Per HIP-2 / HIP-4, a Hunch oracle commits to a nonce `R = k·G` for a market (announced, HIP-1 kind:88) and attests an outcome by publishing a BIP-340 signature bound to that nonce (kind:89). For oracle key `P = x·G` and the canonical message `m_X = "hunch/oracle/v1\n<market>\n<X>"`, define the per-outcome **signature point**:

```
S_X = R + e_X · P,   where  e_X = BIP340_challenge( R_x || P_x || sha256(m_X) )
```

The oracle's attestation for `X` is the pair `(R, s_X)` with `s_X = k + e_X · x`. Thus:

```
s_X · G = S_X
```

`s_X` is computable by anyone **after** the attestation, and by **no one before** it. Because the oracle attests exactly one outcome per market — enforced by a nonce-reuse guard, since signing two outcomes under one `R` leaks `x` — at most one `s_X` is ever revealed.

### Token Lock

A bettor with key pair `(b, B = b·G)` holding an outcome-`X` token receives a NUT-11 P2PK proof locked to:

```
L_X = B + S_X        (33-byte compressed secp256k1 point)
```

The NUT-11 secret is the well-known form:

```json
["P2PK", {
  "nonce": "<random hex>",
  "data": "<L_X compressed hex>",
  "tags": [
    ["refund", "<B compressed hex>"],
    ["locktime", "<refund_timeout unix>"],
    ["sigflag", "SIG_INPUTS"]
  ]
}]
```

`data` (`L_X`) is the outcome branch; `refund` + `locktime` are the HIP-2 refund branch. NUT-12 DLEQ proofs are mandatory (CLAUDE.md "Do" list); bettors verify them on every receive to detect mint cheating.

### Token Lifecycle

1. **Deposit**: bettor pays a Lightning invoice issued by the mint for amount A sat.
2. **Mint**: bettor sends blinded outputs (NUT-00/03) and receives signed proofs whose secret is the NUT-11 lock above, with `data = L_X` for the chosen side. The mint signs with its standard keyset — no per-market keyset is required.
3. **Hold / trade**: bettor holds until settlement, or trades through the Tier 1 mint orderbook (HIP-1 kind:38888) or Tier 2 P2P matching (hunch-matcher).
4. **Settle**: after expiry the oracle publishes the kind:89 attestation. Anyone can now compute `s_X`.
5. **Redeem / refund**: the winning-side holder derives `l_X = b + s_X` and signs the spend (OUTCOME_MATCH). Losing-side tokens never unlock via the outcome branch; holders recover via the refund branch after `refund_timeout`.

### Spend Predicate: OUTCOME_MATCH

To spend an outcome-`X` token after attestation, the holder derives the spend key

```
l_X = b + s_X
```

where `s_X` is the scalar half (last 32 bytes) of the oracle's 64-byte kind:89 signature, and signs the NUT-11 secret with `l_X`. Because `l_X · G = B + S_X = L_X`, the signature verifies under `data`. The mint performs only the standard NUT-11 check (valid Schnorr signature under `data`); it does not parse attestations or markets. All conditionality lives in `L_X`.

This binds spending to BOTH the bettor (`b` required) AND the outcome (`s_X` required), so only the genuine winning-side holder can redeem.

### Spend Predicate: REFUND_AFTER_TIMEOUT

Per NUT-11, once wallclock passes `locktime`, the `refund` key may spend the proof. The bettor's `B` is the refund key, so after `refund_timeout` the bettor unilaterally reclaims the token regardless of any attestation. This is the recovery path when the oracle goes silent, enforced by the underlying DLC refund branch (HIP-2 §Refund Branch) — no mint cooperation beyond honoring NUT-11.

### Refund Mechanics for INVALID Outcome

When the oracle attests INVALID (HIP-2 §INVALID Outcome Semantics), it reveals `s_INVALID`, never `s_YES` or `s_NO`. Therefore neither YES nor NO tokens unlock via OUTCOME_MATCH; both recover through the refund branch after `refund_timeout`. The mint, having received the DLC's INVALID/refund branch on-chain (HIP-2 `CET_INVALID`), honors these refund spends at entry price. There is no winner and no loser. `refund_timeout` MUST therefore be set so bettors are not stranded waiting (HIP-2 §Refund Branch mandates ≥ 7 days after expiry).

### Mint State Machine

```
[Market open]    → mint accepts deposits, issues YES/NO P2PK tokens
[Market expiry]  → mint freezes issuance; existing tokens remain in circulation
[Oracle signs]   → s_X becomes public; winning-side tokens are spendable via OUTCOME_MATCH
[Refund timeout] → losing/all holders reclaim via the NUT-11 refund branch
```

The mint MUST publish a `kind:30892` mint-announce event (HIP-1), updated on state transitions.

### Reserves Disclosure

Per CLAUDE.md "Engineering Principles", the mint MUST publish weekly reserves proofs documenting:
- Total outstanding outcome tokens per market (YES and NO).
- Bitcoin held in DLC funding outputs (txid:vout references).
- Bitcoin held in Lightning channels for redemption.

The reserves proof URL is published as the `reserves_proof` tag in the mint's `kind:30892` event.

## Backwards Compatibility

HIP-3 now depends only on **final** Cashu NUTs (NUT-00/01/02/03/11/12). It does not depend on any draft NUT. If a future NUT-CTF (#337) stabilizes and offers concrete advantages (e.g. native multi-outcome predicates), Hunch MAY adopt it as an optional alternative; it is not on the critical path. References to PR #128 and to NUT-CTF as a *dependency* in earlier `.planning/research/*.md` are OBSOLETE.

## Reference Implementation

- `crates/hunch-dlc` — `attestation` (sign-with-nonce, `signature_point`) and `conditional`
  (`outcome_lock_key` = `B + S_X`, `outcome_unlock_secret` = `b + s_X`, `verify_unlock`).
- `crates/hunch-mint` — `token.rs`: `build_outcome_token` (NUT-11 secret with lock + refund +
  locktime), `redeem_spend_secret`, `verify_token_unlock`.
- Library: [cdk (cashu dev kit)](https://github.com/cashubtc/cdk) — standard NUT-11 mint/wallet.

## Test Vectors

Implemented as crate tests (proven against the real oracle attestation path):
- `crates/hunch-dlc/src/conditional.rs` — a YES attestation unlocks the YES token and does NOT
  unlock the NO token; `s_X·G == S_X`.
- `crates/hunch-mint/src/token.rs` — a YES attestation redeems the YES token only; a wrong bettor
  secret cannot redeem; the NUT-11 secret carries the correct lock/refund/locktime.

A signet end-to-end demo (Lightning deposit → mint → attest → redeem) is the remaining Draft→Final gate.

## References

1. cashubtc/nuts, NUT-11 — Pay-to-Pubkey (P2PK). https://github.com/cashubtc/nuts/blob/main/11.md
2. cashubtc/nuts, NUT-00 — Notation and conventions. https://github.com/cashubtc/nuts/blob/main/00.md
3. cashubtc/nuts, NUT-01 — Mint public keys. https://github.com/cashubtc/nuts/blob/main/01.md
4. cashubtc/nuts, NUT-02 — Keysets. https://github.com/cashubtc/nuts/blob/main/02.md
5. cashubtc/nuts, NUT-03 — Swap. https://github.com/cashubtc/nuts/blob/main/03.md
6. cashubtc/nuts, NUT-12 — Offline ecash via DLEQ proofs (mandatory in Hunch). https://github.com/cashubtc/nuts/blob/main/12.md
7. cashubtc/nuts#337 — NUTs for Prediction Markets (DRAFT; no longer a Hunch dependency). https://github.com/cashubtc/nuts/pull/337
8. cashubtc/nuts#128 — Bilateral NUT-DLC (CLOSED 2025-05-20, historical). https://github.com/cashubtc/nuts/pull/128
9. cashubtc/cdk — Reference Cashu implementation in Rust. https://github.com/cashubtc/cdk
10. HIP-1 — Nostr event kinds. [`./HIP-1.md`](./HIP-1.md)
11. HIP-2 — DLC contract structure. [`./HIP-2.md`](./HIP-2.md)
12. HIP-4 — Oracle signing (single-key + FROST). [`./HIP-4.md`](./HIP-4.md)

---

*HIP-3 — Conditional tokens via NUT-11 P2PK to the oracle signature point. STATUS: DRAFT.*
