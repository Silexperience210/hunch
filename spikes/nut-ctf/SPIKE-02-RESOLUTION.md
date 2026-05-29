# SPIKE-02 Resolution — Outcome-Conditional Tokens Without NUT-CTF

```
Spike:    SPIKE-02
Title:    Conditional-token mechanism for the Hunch mint
Status:   RESOLVED (pivot)
Date:     2026-05-29
Author:   Silex <silex@hunch.markets>
```

## Problem

HIP-3 specified the Hunch mint on top of **NUT-CTF** (cashubtc/nuts#337), the proposed Cashu
Conditional Token Framework. As of 2026-05, PR #337 is still **draft/WIP** ("NUTs for prediction
markets"). Its predecessor for DLC settlement, PR #128, was **closed** (2025-05-20). Building the
mint on an unstable, unmerged spec blocked `hunch-mint` indefinitely — the single remaining
blocker in the Phase 2 backend.

## Resolution

**We do not need NUT-CTF.** Outcome-conditional tokens can be built on **NUT-11 P2PK**, which is
final and shipping (implemented in CDK / `cashu` crate). The DLC conditionality is encoded
entirely in the *lock key*, so the mint stays a vanilla NUT-11 mint with no custom NUT.

### Mechanism

The DLC oracle already provides, for each outcome `X`, a public point

```
S_X = R + e_X · P
```

where `P` is the oracle pubkey, `R` its announced nonce, and `e_X` the BIP-340 challenge for the
canonical `(market, X)` message. The oracle's kind:89 attestation for `X` reveals
`s_X = dlog(S_X) = k + e_X · x` — and **only** for the single outcome it attests.

A bettor's outcome-`X` token is a NUT-11 P2PK proof locked to:

```
L = B + S_X          (B = bettor pubkey)
```

To spend it the holder needs a Schnorr signature under `L`, i.e. the secret

```
l = b + s_X
```

- The bettor always knows `b`.
- `s_X` is unknown to everyone until the oracle attests `X`.
- ⇒ the token is spendable **iff** the holder is the bettor **and** outcome `X` occurred.

Tokens for the losing outcomes (`B + S_NO`, `B + S_INVALID`) never become spendable, because the
oracle reveals exactly one `s_X`. This is enforced by the oracle's **nonce-reuse guard**
(`hunch-oracle`'s nonce store): signing two outcomes under one nonce `R` would leak the oracle
key, so the daemon refuses it.

### Refund / INVALID (HIP-2)

Handled by NUT-11's own `locktime` + `refund` tags: each token carries `["locktime", <refund_timeout>]`
and `["refund", <bettor_pubkey>]`. Before the timeout only the outcome branch can spend; after it
(no attestation, or INVALID), the bettor reclaims. No custom logic.

### Why this is safe / not custom crypto

- Signing and the sig-point come from `ddk-dlc` (maintained rust-dlc) over libsecp256k1-zkp.
- The lock/unlock are plain secp256k1 point/scalar addition (`B + S_X`, `b + s_X`).
- The mint enforces only NUT-11 — audited, shipping code.

## What was built and proven

- `hunch-dlc` (new shared crate): `attestation` (sign-with-nonce, `signature_point`) +
  `conditional` (`outcome_lock_key`, `outcome_unlock_secret`, `verify_unlock`).
- `hunch-mint`: `build_outcome_token` (NUT-11 secret with lock + refund + locktime),
  `redeem_spend_secret`, `verify_token_unlock`.
- Tests (all green) prove, against the **real** oracle attestation path:
  - `s_X · G == S_X` (attestation reveals the lock's discrete log);
  - a YES attestation **unlocks** the YES token and **does not** unlock the NO token;
  - a wrong bettor secret cannot redeem;
  - the NUT-11 secret carries the correct lock, refund, and locktime.

## Out of scope (NOT blocked)

These are standard engineering on stable Cashu/Lightning and need no protocol change:

- Blind-signature issuance (NUT-00/02/03) via CDK.
- Lightning deposit/withdraw via LDK Node.
- DLC funding tx / on-chain settlement (HIP-2) — the mint funds the bilateral DLC; the
  conditional tokens above gate redemption against the attestation.

## HIP-3 action

Update HIP-3: replace the NUT-CTF dependency with the **NUT-11-P2PK-to-signature-point**
construction above. The NUT-CTF gate is removed from HIP-3's Draft→Final criteria; remaining
gates are the signet end-to-end demo (issuance + redemption) and external review.
