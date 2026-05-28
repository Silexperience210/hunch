# HIP-0: Hunch Protocol Overview

```
HIP:      0
Title:    Hunch Protocol Overview & Cypherpunk Manifesto
Authors:  Silex <silex@hunch.markets>
Status:   Draft
Type:     Informational
Created:  2026-05-28
License:  MIT
```

## Abstract

Hunch is a permissionless, Bitcoin-native prediction market protocol. It combines Discreet Log Contracts (DLC) for settlement, Cashu mints with Conditional Token Framework (NUT-CTF, cashubtc/nuts#337) for multi-bettor liquidity, Lightning for deposits and withdrawals, and Nostr for market discovery, oracle attestations, and reputation. The protocol has no governance token, no operator-controlled liquidity, and no canonical frontend. Anyone can host a mint, run an oracle, deploy a frontend, or implement the protocol from these specifications.

HIP-0 introduces the protocol's identity, principles, components, and the relationship of subsequent HIPs (HIP-1 through HIP-5) that define the technical surface.

## Motivation

Existing prediction markets are either (a) centralized exchanges that custody user funds and demand identity disclosure, or (b) blockchain protocols whose native asset, settlement layer, and governance model are entangled. Both shapes concentrate trust in a single operator: a database, a foundation, a token committee.

Hunch sidesteps the trust concentration by:

- Settling on the most decentralized, censorship-resistant timechain (Bitcoin) using DLC, where neither operator nor counterparty can spend funds outside the contracted outcomes.
- Routing liquidity through Cashu mints, which never know who is betting and which can be replaced market-by-market — the mint is a counterparty for the duration of a market, not a permanent custodian.
- Publishing market metadata, oracle announcements, oracle attestations, and reputation on Nostr — an open relay network with no central registry.
- Refusing to issue a project token. The unit of account is Bitcoin (or its Cashu-issued representation). Speculators cannot capture protocol governance because there is none.

This document is the entry point. Subsequent HIPs define the technical surface required for independent implementations.

## Cypherpunk Manifesto

The protocol's principles, stated plainly:

1. **Trust the math, not the operator.** Every cryptographic primitive is open. Every settlement is verifiable on-chain. Every oracle attestation is a Schnorr signature against a public key. The protocol does not require trusting the maintainer, the mint operator, or the oracle operator beyond the math they sign.
2. **No identity, no custody.** Hunch does not collect identity. Nostr public keys are the only identifier. Mints hold reserves for as short as practicable and emit blinded tokens that cannot be linked to a bettor. Custody is bounded by the math, not by goodwill.
3. **Permissionless market creation.** Anyone with a Nostr pubkey may post a market about any verifiable question. The protocol provides no mechanism to censor a market at the protocol level. Frontends and indexers may curate; the protocol stays neutral.
4. **No protocol token.** Bitcoin is the unit of account. There is no Hunch token, no governance vote, no founder allocation. Protocol upgrades happen through HIPs and independent implementations choosing to adopt them.
5. **Open source, MIT.** Every line is forkable. The license never changes. Contributor copyright is not collected; contributors retain their own rights and grant the same MIT terms.
6. **Multi-frontend, multi-mint, multi-oracle.** No single deployment is canonical. The protocol is what survives when any single operator disappears.
7. **Cypherpunk by default.** Tor hidden services and IPFS pins are first-class distribution channels, not afterthoughts. The clearnet website is a convenience.
8. **Pseudonymity is normal.** Contributors are encouraged to operate under pseudonym. Maintainers do not collect contributor real names. Doxxing — of maintainers or contributors — is prohibited in community spaces.

The full manifesto lives at [`docs/MANIFESTO.md`](./MANIFESTO.md).

## Specification

Hunch is the union of five technical surfaces. Each surface has its own HIP:

| HIP   | Title                              | Defines                                                                  |
| ----- | ---------------------------------- | ------------------------------------------------------------------------ |
| HIP-1 | Nostr Event Kinds                  | Event schemas for market, order, dispute, reputation, oracle announce/attest |
| HIP-2 | DLC Contract Structure             | Outcome enumeration (YES/NO/INVALID), refund timeout, multi-oracle compatibility |
| HIP-3 | Cashu NUT-CTF Integration          | How a Hunch mint extends Cashu's Conditional Token Framework (PR #337)   |
| HIP-4 | Multi-Oracle FROST Protocol        | Threshold Schnorr (RFC 9591) for k-of-n oracle attestation                |
| HIP-5 | Reputation Event Format            | Aggregation rules and weighting for oracle / market / mint reputation     |

Implementations conform to Hunch by implementing a coherent subset of these specs. For example, a minimal frontend implements HIP-1 (read-only), HIP-5 (display), and the relevant subset of HIP-2/HIP-3 to render bet flows; it need not implement HIP-4 directly because oracle signing happens in oracle software, not frontend.

### Component Roles

```
   ┌─────────┐   markets/orders/disputes   ┌──────────┐
   │  User   │ ◄────────── Nostr ──────────│ Frontend │
   │ (npub)  │                             │  (Tier 1) │
   └─────────┘                             └────┬─────┘
        │                                       │
        │  Lightning deposits                   │  optional Tier 2 P2P
        ▼                                       ▼
   ┌─────────────┐  DLC contract  ┌─────────────────────┐
   │  Cashu mint │ ◄────────────► │ Bitcoin (DLC chain) │
   │ (NUT-CTF)   │                │      settlement      │
   └─────┬───────┘                └──────────────────────┘
         │  oracle pub-event (NIP-88)
         │  oracle attestation
         ▼
   ┌──────────┐    FROST (HIP-4)   ┌──────────────┐
   │  Oracle  │ ◄───────────────► │ k-of-n quorum │
   │  (HIP-1) │                   └──────────────┘
   └──────────┘
```

### Trust Model

| Component   | Trust required                                                  | What replaces trust                                                    |
| ----------- | --------------------------------------------------------------- | ---------------------------------------------------------------------- |
| Bettor      | Math only                                                       | DLC contract enforces payout; Cashu blind signature preserves privacy |
| Mint        | Operator does not rug during market lifetime                    | Public reserve proofs (weekly), DLC on-chain settlement, market-scoped exposure |
| Oracle      | Operator signs correctly per their attestation policy           | Multi-oracle FROST (HIP-4), reputation events (HIP-5), market creator chooses oracle |
| Relay       | Routes events; cannot forge them                                | Schnorr signature in every Nostr event                                  |
| Frontend    | Renders honestly                                                | Source open; multiple implementations; protocol-direct CLI option       |
| Maintainer  | Cannot upgrade protocol unilaterally                            | Independent implementations; HIP process; pseudonymous contributors    |

### Settlement Lifecycle

1. Market creator posts a `kind:30888` market event (HIP-1) referencing an oracle pubkey, an outcome enumeration, and a refund timeout.
2. Mint creates a DLC contract (HIP-2) backing the market, posts it on Bitcoin.
3. Bettors deposit via Lightning; the mint emits NUT-CTF conditional tokens (HIP-3) representing YES or NO claims.
4. Bettors may split, merge, or transfer tokens via Tier 1 mint orderbook or Tier 2 P2P Nostr matching.
5. At expiry, the oracle publishes a `kind:89` attestation (HIP-1) signed under its announced pubkey.
6. The mint resolves the DLC outcome on-chain, redeeming winning tokens for Bitcoin via Lightning; losing tokens become unspendable.
7. If the oracle attests INVALID (HIP-2), the refund branch executes and bettors recover their stakes.
8. If the oracle never signs and the refund timeout elapses, the refund branch executes.

This flow is verifiable end-to-end. Each step references a public key, a public event, or a public Bitcoin transaction.

## Backwards Compatibility

HIP-0 has no backwards-compatibility surface; it introduces the protocol. Subsequent HIPs document their own compatibility with prior drafts of themselves and with external specs (NIP-88, NIP-23, cashubtc/nuts#337).

## Test Vectors

No test vectors at the HIP-0 level. See HIP-1 through HIP-5 for the technical test surfaces.

## Reference Implementation

The reference implementation lives at `https://github.com/Silexperience210/hunch` under MIT license. The reference implementation is one of many — alternative implementations are encouraged.

## References

1. cashubtc/nuts#337 — NUTs for Prediction Markets (Conditional Token Framework). https://github.com/cashubtc/nuts/pull/337
2. nostr-protocol/nips#1681 — NIP-88 oracle event kinds (draft). https://github.com/nostr-protocol/nips/pull/1681
3. nostr-protocol/nips, NIP-23 — Long-form Content. https://nips.nostr.com/23
4. discreetlogcontracts/dlcspecs — DLC specifications. https://github.com/discreetlogcontracts/dlcspecs
5. RFC 9591 — Flexible Round-Optimized Schnorr Threshold (FROST). https://datatracker.ietf.org/doc/rfc9591/
6. ZcashFoundation/frost — Reference FROST implementation. https://github.com/ZcashFoundation/frost
7. Hughes, E. *A Cypherpunk's Manifesto* (1993). https://www.activism.net/cypherpunk/manifesto.html

---

*HIP-0 — Trust the math.*
