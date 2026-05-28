# HIP External Reviewer Attestation Tracker

**Purpose:** Phase 1 ROADMAP Success Criterion #1 requires each HIP to have at least one external Bitcoin / Cashu / DLC contributor review. This file tracks outreach status and recorded attestations per HIP.

**Created:** 2026-05-28
**Reviewer targets:** see RESEARCH.md §10 Q4 for shortlist; outreach sent via Nostr DM under project pseudonym key

## HIP-0 — Protocol Overview & Manifesto

| Reviewer (Nostr npub or GitHub handle) | Channel    | Date sent | Status        | Attestation event ID |
| -------------------------------------- | ---------- | --------- | ------------- | -------------------- |
| (queued) — Calle (cashubtc)            | Nostr DM   | PENDING   | queued        | —                    |
| (queued) — conduition                  | Nostr DM   | PENDING   | queued        | —                    |
| (queued) — Tadge Dryja (DLC inventor)  | GitHub     | PENDING   | queued        | —                    |

## HIP-1 — Nostr Event Kinds

| Reviewer (Nostr npub or GitHub handle) | Channel    | Date sent | Status        | Attestation event ID |
| -------------------------------------- | ---------- | --------- | ------------- | -------------------- |
| (queued) — fiatjaf (Nostr originator)  | Nostr DM   | PENDING   | queued        | —                    |
| (queued) — benthecarman (NIP-88 author)| Nostr DM   | PENDING   | queued        | —                    |

## HIP-2 — DLC Contract Structure

| Reviewer (Nostr npub or GitHub handle) | Channel    | Date sent | Status        | Attestation event ID |
| -------------------------------------- | ---------- | --------- | ------------- | -------------------- |
| (queued) — Crypto Garage rust-dlc team | GitHub     | PENDING   | queued        | —                    |
| (queued) — Benny (DDK author)          | Nostr DM   | PENDING   | queued        | —                    |

## HIP-3 — Cashu NUT-CTF Integration

| Reviewer (Nostr npub or GitHub handle) | Channel    | Date sent | Status        | Attestation event ID |
| -------------------------------------- | ---------- | --------- | ------------- | -------------------- |
| (queued) — joemphilips (PR #337)       | GitHub PR  | PENDING   | queued        | —                    |
| (queued) — Calle (cashubtc maintainer) | Nostr DM   | PENDING   | queued        | —                    |
| (queued) — thesimplekid (maintainer)   | Nostr DM   | PENDING   | queued        | —                    |

## HIP-4 — Multi-Oracle FROST

| Reviewer (Nostr npub or GitHub handle) | Channel    | Date sent | Status        | Attestation event ID |
| -------------------------------------- | ---------- | --------- | ------------- | -------------------- |
| (queued) — Zcash Foundation FROST team | GitHub     | PENDING   | queued        | —                    |
| (queued) — Trail of Bits crypto team   | Email      | PENDING   | queued        | —                    |
| (queued) — Jonas Nick (BIP-340 author) | Nostr DM   | PENDING   | queued        | —                    |

## HIP-5 — Reputation Event Format

| Reviewer (Nostr npub or GitHub handle) | Channel    | Date sent | Status        | Attestation event ID |
| -------------------------------------- | ---------- | --------- | ------------- | -------------------- |
| (queued) — semisol (reputation NIPs)   | Nostr DM   | PENDING   | queued        | —                    |
| (queued) — Vitor Pamplona (Amethyst)   | Nostr DM   | PENDING   | queued        | —                    |

## Outreach Template

```text
Hi <reviewer>,

I maintain Hunch (https://github.com/Silexperience210/hunch), a permissionless
Bitcoin-native prediction market protocol. I'd appreciate your review of HIP-X
(docs/HIP-X.md) given your prior work on <relevant area>.

The HIP is ~<word count> words. I'm looking for:
- Technical correctness against the cited references
- Compatibility concerns with related specs (NIP-XX / NUT-XX / dlcspecs)
- Edge cases the spec underspecifies

A signed Nostr attestation (kind:30891 reputation event targeting the HIP author
pubkey with scope `protocol_review`) or a GitHub review comment both count. I
will record your attestation in docs/HIPs-reviews.md with attribution.

Compensation: I can offer a small honorarium in sat (Lightning) for substantive
reviews — let me know if you'd accept and your invoice channel. No identity
disclosure required.

Thanks,
Silex (Hunch protocol maintainer)
```

## Status

**Phase 1 SC #1 progress:** 0/6 HIPs have external reviewer attestation. Outreach queue created. Awaiting user manual send (no SMTP / Nostr DM credentials in autonomous environment).

**Recommended outreach cadence (per Plan 02 calendar notes):**

- Week 1 (now): send DMs to Calle, conduition, joemphilips, fiatjaf, benthecarman, Vitor — high-priority cashu + nostr core
- Week 2: send DMs to DLC team (Tadge Dryja, Crypto Garage), FROST team (ZF), Trail of Bits
- Week 4–6: aggregate responses, log attestations here, transition HIPs from Draft toward Final

**Phase 1 close gate:** at least one external reviewer attestation per HIP (6 attestations total). Until then, HIPs remain Status: Draft.

**HIP-3 special gate:** HIP-3 remains Status: Draft until BOTH (a) external reviewer attestation AND (b) Plan 03 SPIKE-02 demonstrates working signet prototype against PR #337.
