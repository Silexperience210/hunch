# HIP-2: DLC Contract Structure

```
HIP:      2
Title:    DLC Contract Structure (Binary YES/NO/INVALID, Refund Timeout, Multi-Oracle Ready)
Authors:  Silex <silex@hunch.markets>
Status:   Draft
Type:     Standards Track
Created:  2026-05-28
License:  MIT
Requires: HIP-0, HIP-1
```

## Abstract

HIP-2 specifies how Hunch markets construct Discreet Log Contracts (DLCs) on Bitcoin. Each market resolves to one of three outcomes: YES, NO, or INVALID. The contract enumerates three Contract Execution Transactions (CETs), one per outcome, plus a refund transaction with absolute timelock. The contract is constructed against a single oracle by default and is FROST-multi-oracle ready (HIP-4) by reserving the oracle aggregation pubkey field. The mint is the bilateral DLC counterparty at the Bitcoin layer; bettors transact with the mint through Cashu NUT-CTF tokens (HIP-3) rather than directly with the DLC.

## Motivation

Bilateral DLCs as defined in [discreetlogcontracts/dlcspecs](https://github.com/discreetlogcontracts/dlcspecs) require both counterparties to be online for contract construction. Prediction markets need many bettors, not one. HIP-2 resolves the tension by making the mint the bilateral DLC counterparty (signing once with the market creator at market open), then routing many-to-many betting through Cashu blind signatures on the mint side. The on-chain footprint is a single DLC per market regardless of bet count.

The choice of three outcomes (YES, NO, INVALID) lets oracles signal "the market cannot be resolved as specified" without forcing a coin toss. INVALID branches refund both sides.

## Specification

### Contract Anatomy

A Hunch DLC consists of:

1. **Funding transaction** — Bitcoin transaction creating a 2-of-2 multisig output between mint and market creator. Funded by both counterparties.
2. **Three Contract Execution Transactions (CETs)** — pre-signed at contract creation, each spending the funding output:
   - `CET_YES` — pays the entire pot to the mint's YES-resolution address. Released by the mint applying an oracle attestation of YES (HIP-1 kind:89 with outcome="YES").
   - `CET_NO` — pays the entire pot to the mint's NO-resolution address. Released by the mint applying an oracle attestation of NO.
   - `CET_INVALID` — splits the pot 50/50 between mint and market creator's refund addresses. Released by the mint applying an oracle attestation of INVALID.
3. **Refund transaction** — pays the funding amount back to both counterparties pro-rata. Absolute timelock at `refund_timeout` (HIP-1 kind:30888 tag). Released by either counterparty unilaterally if no oracle attestation arrives.

### Funding Output Structure

```
Output script: 2-of-2 multisig (mint_pubkey, creator_pubkey)
Amount: market_size_sat (mint contributes 1/2, creator deposits the other 1/2 from initial seed liquidity)
```

The mint and the market creator each contribute half of the initial liquidity. As bets arrive through Cashu deposits, the mint's effective exposure rebalances; the DLC itself does not need to be reopened. The mint internally tracks per-bettor positions via the NUT-CTF token state.

### Adapter Signatures and Oracle Schnorr

The mint and market creator pre-sign each CET using BIP-340 Schnorr adapter signatures. The adapter point is derived from the oracle's announced public nonce R for the market (HIP-1 kind:88) combined with each outcome's tag:

```
For outcome o in {YES, NO, INVALID}:
  s_adapter_o = sign_adapter(funding_input, CET_o, R + hash(o) * G)
```

When the oracle attests outcome `o` by publishing the Schnorr signature `(R, s_o)` (HIP-1 kind:89), either counterparty can complete the adapter:

```
s_complete_o = s_adapter_o + s_o
```

And broadcast `CET_o` to the chain.

### INVALID Outcome Semantics

The INVALID outcome is reserved for cases where the resolution criteria stated in the market's `content.resolution_criteria` (HIP-1) cannot be evaluated. Examples:
- Primary source unavailable during resolution window
- Oracle observes ambiguity in the question text
- Real-world event becomes impossible to verify (data corruption, source shutdown)

When the oracle signs INVALID, `CET_INVALID` redistributes the pot 50/50 between mint and creator. The mint then uses its returned half to refund bettors at their entry price (no winners, no losers); the creator's half compensates them for the market hosting opportunity cost. Per-bettor refunds happen at the NUT-CTF layer (HIP-3 §Refund Mechanics).

### Refund Branch (Oracle Silence)

If the oracle never publishes a `kind:89` attestation and the wallclock passes the market's `refund_timeout` (HIP-1 kind:30888 tag), either the mint or the creator may broadcast the refund transaction. The refund transaction is a CSV-locked sweep spending the funding output back to both counterparties. The refund timeout MUST be at least 7 days after market `expiry` to give the oracle reasonable signing latency.

### Multi-Oracle Readiness

When the market uses a FROST k-of-n oracle quorum (HIP-4), the construction is unchanged at the DLC layer: the FROST quorum produces a single aggregated Schnorr signature, which the adapter signatures consume identically. The `oracle` tag in HIP-1 kind:30888 then refers to the FROST aggregate pubkey, not an individual oracle.

### Outcome Encoding

Outcomes are encoded as ASCII strings tagged into the adapter point. The strings MUST be exactly one of `YES`, `NO`, `INVALID`. Case-sensitive. UTF-8.

## Backwards Compatibility

HIP-2 follows the structure of [dlcspecs](https://github.com/discreetlogcontracts/dlcspecs) for funding, CETs, and adapter signatures. It adds the INVALID outcome convention, which is a Hunch-specific extension of the binary-outcome pattern documented in dlcspecs §"Enumerated outcomes". Implementations conforming to dlcspecs binary contracts can extend to Hunch by adding the third outcome and refund-timeout convention.

If the dlcspecs canonical convention later standardizes a different INVALID encoding (e.g., a specific outcome string or a separate signaling channel), HIP-2 will issue a corrigendum aligning to canon.

## Test Vectors

Test vectors for the three CETs against a synthetic oracle pubkey + nonce land in `crates/hunch-protocol/tests/dlc/` in Phase 2. Phase 1 SPIKE-02 (NUT-CTF signet prototype) demonstrates the YES/NO/INVALID flow end-to-end on Bitcoin signet.

## Reference Implementation

`crates/hunch-mint` will use [`rust-dlc`](https://github.com/p2pderivatives/rust-dlc) for CET construction, adapter signing, and funding-tx serialization. The mint code lives in Phase 2 (HIP-3); the spike crate `hunch-mint-spike` (Plan 03) is the working Phase 1 prototype.

## References

1. discreetlogcontracts/dlcspecs — DLC specifications. https://github.com/discreetlogcontracts/dlcspecs
2. dlcspecs, Funding-Transaction.md — Funding output construction. https://github.com/discreetlogcontracts/dlcspecs/blob/master/Funding-Transaction.md
3. dlcspecs, Protocol.md — Adapter signature construction. https://github.com/discreetlogcontracts/dlcspecs/blob/master/Protocol.md
4. BIP-340 — Schnorr Signatures. https://github.com/bitcoin/bips/blob/master/bip-0340.mediawiki
5. p2pderivatives/rust-dlc — Reference Rust DLC implementation. https://github.com/p2pderivatives/rust-dlc
6. bennyhodl/dlcdevkit (DDK) — DLC application toolkit. https://github.com/bennyhodl/dlcdevkit
7. HIP-1 — Nostr event kinds (defines oracle announce/attest kinds 88/89). [`./HIP-1.md`](./HIP-1.md)
8. HIP-3 — Cashu NUT-CTF integration (translates DLC outcomes to bettor refunds). [`./HIP-3.md`](./HIP-3.md)
9. HIP-4 — Multi-oracle FROST (aggregates k-of-n into a single oracle Schnorr signature). [`./HIP-4.md`](./HIP-4.md)

---

*HIP-2 — DLC contract structure.*
