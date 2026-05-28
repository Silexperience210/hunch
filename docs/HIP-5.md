# HIP-5: Reputation Event Format and Aggregation

```
HIP:      5
Title:    Reputation Event Format and Aggregation Rules
Authors:  Silex <silex@hunch.markets>
Status:   Draft
Type:     Standards Track
Created:  2026-05-28
License:  MIT
Requires: HIP-0, HIP-1
```

## Abstract

HIP-5 specifies the reputation event format (`kind:30891` from HIP-1) and the aggregation rules that turn raw attestations into per-actor reputation scores. Reputation in Hunch is a signal layer: it does not gate access to the protocol, but it surfaces actor history so bettors can make informed choices about which oracles, mints, and market creators to engage. Frontends and indexers aggregate reputation events into displayed scores; the protocol does not centralize aggregation.

## Motivation

Multi-oracle, multi-mint, permissionless markets need a way for participants to surface track records without introducing a central rating authority. Reputation as a Nostr event set lets:

- Anyone attest to any actor's behavior
- Frontends choose their own aggregation methodology
- Reputation be portable across frontends without lock-in
- Bad actors be flagged without being silenced at the protocol level

The risk is that reputation events themselves become spam or coordinated attacks. HIP-5 specifies the data format; recommended aggregation rules (in §Aggregation) mitigate spam via attester-weight scaling and provenance scoring.

## Specification

### Event Schema

A reputation event is `kind:30891` (HIP-1).

**Required tags:**
- `d` — reputation event identifier (string, max 64 chars)
- `p` — target pubkey being scored (hex, 32 bytes)
- `scope` — one of `oracle`, `mint`, `market_creator`, `bettor`
- `score` — signed integer in range [-100, +100]
- `weight` — optional integer indicating attester's claimed stake-in-the-truth (sat amount or 0; see §Aggregation)

**Optional tags:**
- `evidence` — URL to an evidence document (commit hash, archived web page, Nostr event ID)
- `market` — market identifier the attestation refers to, if scoped to a single market (HIP-1 `<pubkey>:30888:<d>` format)
- `method` — short string indicating the attester's methodology (`personal_experience`, `automated_dispute_count`, `manual_review`, `aggregate_of_prior_reputation`)

**Content:** Free-form JSON describing the attestation:

```json
{
  "score": 75,
  "summary": "Resolved 47 markets correctly out of 48; one INVALID disputed and upheld.",
  "evidence_refs": ["<event_id1>", "<event_id2>"],
  "methodology": "Auto-tallied via personal market history; one manual review for disputed market 0x...",
  "attester_disclosures": ["I bet on the disputed market and lost; no financial interest in the outcome of this attestation."]
}
```

### Score Semantics

The integer `score` range [-100, +100] is interpreted as:

| Range          | Meaning                                      |
| -------------- | -------------------------------------------- |
| +90 to +100    | Excellent — long-term verified track record |
| +60 to +89     | Good — consistent, no major issues          |
| +30 to +59     | Mixed — some issues, some good outcomes     |
| -29 to +29     | Neutral / insufficient data                  |
| -30 to -59     | Concerning — repeated issues                |
| -60 to -89     | Bad — pattern of misbehavior                 |
| -90 to -100    | Severe — confirmed malicious behavior        |

Frontends MAY map score ranges to UI affordances (badges, color codes, sort-order downweighting), but MUST display the raw score on demand.

### Subject Disclosure

Reputation events are signed by the attester's pubkey. The attester's identity is on the chain of accountability: an attester who issues many low-evidence accusations builds their own negative reputation.

Self-attestation (an actor signing reputation about themselves) is allowed but SHOULD be downweighted by aggregators to near-zero. The attester's pubkey is in the event's `pubkey` field; the subject's pubkey is in the `p` tag.

### Aggregation Rules

Aggregators (frontends, indexers) compute an actor's effective reputation from the set of `kind:30891` events targeting that actor. RECOMMENDED aggregation algorithm:

```
For each reputation event e about actor A:
  attester_rep = current_reputation(e.pubkey, scope=any)  // recursive — bootstrap to 0
  age_decay = exp(-(now - e.created_at) / 180_days)
  evidence_weight = if e.evidence URL is verifiable, 1.0 else 0.5
  staked_weight = log(1 + e.weight_tag) / log(1 + max_observed_weight)
  contribution = e.score * attester_rep * age_decay * evidence_weight * staked_weight

effective_reputation = clamp(sum(contributions) / count_meaningful_attesters, -100, +100)
```

Key invariants:

- Recursive: an attester's own reputation weights their attestations. Sybil attesters with no prior reputation contribute near-zero.
- Age decay: 180-day half-life ensures stale reputation can be overridden by recent evidence.
- Evidence weighted: attestations with verifiable evidence URLs count more than free-form claims.
- Self-attestation: implementations MUST set `attester_rep` to 0 when `e.pubkey == e.tags.p` (subject self-attesting).

Aggregators MAY choose alternative algorithms. The protocol does not enforce one. Frontends SHOULD disclose their aggregation methodology to bettors.

### Anti-Spam

To resist coordinated reputation attacks:

1. **Stake bonding (optional)**: attesters MAY bond Bitcoin via Lightning escrow alongside their reputation attestation. The bond is forfeit if the attestation is later proven false. Frontends weight bonded attestations higher.
2. **Source diversity**: frontends MAY require attestations from at least N independent pubkeys before surfacing a score change.
3. **Mint cross-attest**: mints holding bettor deposits in a market MAY publish a `kind:30891` attestation summarizing market outcomes; mint-attested reputation has high inherent provenance.
4. **Dispute integration**: a `kind:30890` dispute upheld by a quorum of bettors becomes a feeding-stock for a `kind:30891` reputation event targeting the oracle.

### Reputation Lifecycle

Reputation events are parameterized replaceable — an attester may replace their own attestation over time. The latest attestation per `(pubkey, d)` is canonical; earlier versions are dropped per Nostr replaceable-event rules.

Attesters who change their assessment SHOULD reference the prior assessment in the new event's content to maintain provenance.

### Scope-Specific Conventions

**Oracle reputation** (`scope: oracle`): attestations focus on attestation accuracy, attestation latency, dispute history, and quorum participation rate.

**Mint reputation** (`scope: mint`): attestations focus on reserves transparency, DLC settlement reliability, refund punctuality, fee disclosure, and KYC stance.

**Market creator reputation** (`scope: market_creator`): attestations focus on resolution-criteria clarity, ambiguity rate, oracle-pairing quality.

**Bettor reputation** (`scope: bettor`): generally unused at protocol level (bettor anonymity is a feature). Frontends MAY use bettor reputation for Tier 2 P2P matching trust signals; if so, they MUST allow bettors to operate under disposable per-market keys to preserve anonymity by default.

## Backwards Compatibility

HIP-5 is a fresh kind allocation. Earlier drafts of HIP-5 (if any) are not in production; this is the initial public version.

If the aggregation algorithm needs to evolve (e.g., due to attack patterns observed in production), HIP-5 can issue versioned algorithms (e.g., `aggregation_version: 2`) that frontends opt into.

## Test Vectors

Test vectors for parsing `kind:30891` events and computing aggregate scores land in `crates/hunch-protocol/tests/reputation/` in Phase 2.

## Reference Implementation

`crates/hunch-protocol` (Phase 2): canonical types and verification.
`apps/hunch-web/lib/reputation/` (Phase 2): a reference aggregator implementing the algorithm in §Aggregation.

## References

1. HIP-1 — Nostr event kinds (kind 30891 definition). [`./HIP-1.md`](./HIP-1.md)
2. nostr-protocol/nips, NIP-32 — Labelling (relevant for evidence tagging). https://nips.nostr.com/32
3. Adler, R., *Reputation Systems* — survey of recursive reputation algorithms (academic, not Nostr-specific).
4. Augur whitepaper, §Reputation — historical context for token-based reputation pitfalls. https://augur.net (note: Hunch deliberately does NOT use a reputation token; the lesson is that token-based reputation creates a financial backdoor to governance)
5. HIP-0 — Protocol overview. [`./HIP-0.md`](./HIP-0.md)

---

*HIP-5 — Reputation event format.*
