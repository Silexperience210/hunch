# HIP-1: Nostr Event Kinds

```
HIP:      1
Title:    Nostr Event Kinds for Markets, Orders, Disputes, Reputation, Oracle
Authors:  Silex <silex@hunch.markets>
Status:   Draft
Type:     Standards Track
Created:  2026-05-28
License:  MIT
Requires: HIP-0
```

## Abstract

HIP-1 defines the Nostr event kinds used by Hunch for market metadata, order placement, dispute records, reputation attestations, and oracle announce/attestation events. Hunch reserves five kinds in the parameterized-replaceable range (30000–39999) plus one ephemeral kind (38888) and depends on NIP-88 (PR #1681) for oracle kinds 88 and 89. HIP-1 specifies the JSON payload schema, tag conventions, and validation rules for each kind. Conforming Hunch relays accept these kinds; non-Hunch relays may accept or filter them at operator discretion.

## Motivation

Hunch is discovery-, attestation-, and reputation-routed through Nostr. Defining the event schemas as a stable specification means:

- Frontends can be implemented against the spec without coordinating with any single backend
- Relays can index and filter by kind without parsing protocol-specific tags
- Cross-implementation interop is possible without registry coordination
- Specifications evolve via HIP amendments, not silent backend changes

## Specification

### Reserved Kinds

| Kind   | Class                   | Name              | Purpose                                                                 |
| ------ | ----------------------- | ----------------- | ----------------------------------------------------------------------- |
| 88     | Regular                 | Oracle Announce   | Oracle declares intent to attest a market outcome (see NIP-88)         |
| 89     | Regular                 | Oracle Attestation| Oracle Schnorr signature over the market outcome (see NIP-88)          |
| 30888  | Parameterized Replaceable | Market          | Market metadata: question, outcome enum, oracle pubkey, refund timeout |
| 38888  | Parameterized Replaceable | Order           | Bet order: market reference, side YES/NO, amount, price                |
| 30890  | Parameterized Replaceable | Dispute         | Dispute filing: market reference, claim, evidence URL                  |
| 30891  | Parameterized Replaceable | Reputation      | Reputation attestation: target pubkey, scope, score                    |
| 30892  | Parameterized Replaceable | Mint Announce   | Mint identity, supported markets, reserves proof URL                   |

Kinds 88 and 89 follow NIP-88. Hunch does not redefine them; it imports them by reference. If NIP-88 PR #1681 changes its kind numbers before merge, HIP-1 will issue a corrigendum.

### Kind 30888 — Market

A parameterized-replaceable event identifying a single market. The `d` tag is the market's canonical identifier under the creator's pubkey.

**Required tags:**
- `d` — market identifier (max 64 chars, slug or UUID)
- `oracle` — oracle pubkey (hex, 32 bytes)
- `outcomes` — comma-separated outcome labels (e.g., `YES,NO,INVALID`)
- `expiry` — UNIX timestamp when the market closes to betting
- `refund_timeout` — UNIX timestamp after which refund branch is claimable if oracle silent
- `mint` — mint pubkey or URL backing the DLC contract
- `dlc_contract` — Bitcoin txid:vout of the DLC funding output

**Optional tags:**
- `category` — high-level grouping (`politics`, `sports`, `tech`, `weather`, etc.)
- `image` — preview image URL
- `t` — topic tags (multiple allowed)

**Content:**

A JSON object describing the question:
```json
{
  "question": "Will event X happen by date Y?",
  "resolution_criteria": "The oracle attests YES if [...] and NO if [...]. INVALID if [...]",
  "sources": ["https://primary-source.example", "https://backup-source.example"],
  "rules_version": "1.0"
}
```

`resolution_criteria` is the contract: the oracle's signing rule. Bettors are responsible for reading it before betting.

### Kind 38888 — Order

A parameterized-replaceable event (kind range 30000–39999) posted to Tier 2 P2P matching relays. The `d` tag makes the order addressable, so a pubkey keeps **one outstanding order per market** (re-posting replaces the prior order) and relays can filter by `#d`.

> **Corrigendum:** earlier drafts of this table labeled kind 38888 "Ephemeral". That was a documentation error — 38888 is in the 30000–39999 parameterized-replaceable range per NIP-01. The reference implementation (`hunch-protocol`, `hunch-relay`) treats it as parameterized-replaceable.

**Required tags:**
- `d` — set to the `market` identifier (addressability / one order per pubkey per market)
- `market` — market identifier (`<pubkey>:30888:<d>`)
- `side` — `YES` or `NO`
- `amount` — token amount (sat)
- `price` — bid/ask price in sat per token
- `kind` — `bid` or `ask`
- `expires` — UNIX timestamp order expires

**Content:** Empty.

### Kind 30890 — Dispute

Filed by a bettor or third party challenging an oracle attestation.

**Required tags:**
- `d` — dispute identifier
- `market` — market identifier being disputed
- `attestation` — event ID of the disputed `kind:89` attestation
- `claim` — short claim (`oracle_misread`, `source_unavailable`, `INVALID_should_have_been`, etc.)

**Content:** Free-form evidence including URLs, screenshots, alternative-source references.

Disputes are advisory at the protocol level. Mints, frontends, and reputation aggregators may use disputes as input to reputation events (HIP-5). Disputes do not reverse on-chain DLC settlement.

### Kind 30891 — Reputation

Attestations about an actor's prior conduct. Subject of HIP-5.

**Required tags:**
- `d` — reputation event ID
- `p` — target pubkey
- `scope` — `oracle`, `mint`, `market_creator`, or `bettor`
- `score` — integer in range [-100, +100]

**Content:** JSON with evidence references and methodology. See HIP-5.

### Kind 30892 — Mint Announce

Mint operator publishes mint identity and current state.

**Required tags:**
- `d` — mint identifier
- `endpoint` — mint URL (HTTPS, onion, IPFS gateway acceptable)
- `reserves_proof` — URL to latest weekly reserves proof
- `supported_oracles` — comma-separated oracle pubkeys

**Content:** JSON describing mint policy (max market size, supported asset, age of mint, geographical jurisdiction if disclosed, etc.).

### Validation

Conforming relays MUST:
1. Verify Schnorr signature on every event before storing
2. Reject events with malformed required tags (missing `d`, malformed pubkey, missing `oracle`)
3. Apply replaceable-event rules to kinds 30888, 30890, 30891, 30892
4. Apply ephemeral-event rules to kind 38888

Conforming clients SHOULD:
1. Verify `dlc_contract` references a real on-chain output via a Bitcoin node or Esplora endpoint
2. Verify `oracle` pubkey has a corresponding `kind:88` announce event before showing the market
3. Verify `mint` identifier has a corresponding `kind:30892` announce event

## Backwards Compatibility

Kinds 88 and 89 are imported from NIP-88 (currently in draft status as PR #1681). If PR #1681 changes its kind allocation before merging, HIP-1 will issue a versioned corrigendum and dual-kind events for a deprecation window of 6 months. The reference implementation tracks NIP-88 stability and will not transition Hunch to "Final" status until NIP-88 itself is merged or has reached stable draft status.

Kinds 30888, 38888, 30890, 30891, 30892 are not registered against the NIPs registry as of HIP-1 publication. The Hunch reference implementation opens a NIPs-repo PR requesting kind reservations concurrent with this HIP. If collisions emerge before reservation merges, Hunch will renumber.

## Test Vectors

```json
// kind:30888 market event (example)
{
  "kind": 30888,
  "pubkey": "<creator pubkey hex>",
  "created_at": 1748390400,
  "tags": [
    ["d", "btc-100k-eoy-2026"],
    ["oracle", "<oracle pubkey hex>"],
    ["outcomes", "YES,NO,INVALID"],
    ["expiry", "1767139200"],
    ["refund_timeout", "1769817600"],
    ["mint", "<mint pubkey hex>"],
    ["dlc_contract", "<funding txid>:0"],
    ["category", "crypto"]
  ],
  "content": "{\"question\":\"Will BTC close above $100k on 2026-12-31?\",\"resolution_criteria\":\"YES if BTC/USD spot on Coinbase Pro at 23:59 UTC >= 100000.00; NO if < 100000.00; INVALID if Coinbase Pro feed is down for >2 hours during the resolution window.\",\"sources\":[\"https://pro.coinbase.com/markets/BTC-USD\"],\"rules_version\":\"1.0\"}",
  "sig": "<schnorr sig hex>"
}
```

Additional test vectors land in `crates/hunch-protocol/tests/` in Phase 2.

## Reference Implementation

Reference Rust types in `crates/hunch-protocol` (Phase 2). Reference TypeScript types and validators in `apps/hunch-web/lib/protocol/` (Phase 2).

## References

1. nostr-protocol/nips, README — Kind registry. https://github.com/nostr-protocol/nips/blob/master/README.md
2. nostr-protocol/nips#1681 — NIP-88 (oracle event kinds, draft). https://github.com/nostr-protocol/nips/pull/1681
3. nostr-protocol/nips, NIP-01 — Basic protocol flow. https://nips.nostr.com/01
4. nostr-protocol/nips, NIP-09 — Event deletion (relevant for dispute lifecycle). https://nips.nostr.com/09
5. nostr-protocol/nips, NIP-23 — Long-form content (HIP publication channel). https://nips.nostr.com/23
6. nostr-protocol/nips, NIP-65 — Outbox model (recommended for Hunch frontends). https://nips.nostr.com/65
7. HIP-0 — Protocol overview. [`./HIP-0.md`](./HIP-0.md)
8. HIP-3 — Cashu NUT-CTF integration (consumes kinds 38888 + 30892). [`./HIP-3.md`](./HIP-3.md)
9. HIP-5 — Reputation event format (extends kind 30891). [`./HIP-5.md`](./HIP-5.md)

---

*HIP-1 — Nostr event kinds.*
