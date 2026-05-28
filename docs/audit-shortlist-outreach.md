# Audit Firm Shortlist & Outreach (Phase 1 → Phase 3)

**Purpose:** Phase 3 (Mainnet Launch & Hardening) requires external security audit by a firm with Bitcoin DLC + Cashu expertise. Audit firms typically have 8–14 week lead times. This document tracks Phase-1 (Week 4–5) initial inquiry outreach to secure a Q4 2026 audit slot.

**PROTO-08 verification (recorded here per Plan 01 Task 3):** CLAUDE.md exists at repo root with GSD workflow references. Verification commit: see `git log --oneline` for the commit containing this file.

**Created:** 2026-05-28 (Plan 01 Task 3)

## Shortlist (priority order per RESEARCH §6.2 + PITFALLS Pitfall 5)

| # | Firm | Specialty | Primary contact | Status |
|---|------|-----------|-----------------|--------|
| 1 | **Trail of Bits** (Bitcoin team) | Threshold sigs, FROST DKG (Feb 2024 disclosure) | audits@trailofbits.com | PENDING |
| 2 | **Block Digital Contracting** | Bitcoin-specific, DLC | TBD (research firm contact) | PENDING |
| 3 | **Cure53** | Frontend + crypto integration | pretty@cure53.de | PENDING |
| 4 | **Quarkslab** | Low-level crypto | contact@quarkslab.com | PENDING |
| 5 | **NCC Group** | Protocol audits, Lightning history | TBD (research firm contact) | PENDING |
| 6 | **Galaxy Audit / Inference Security** | Bitcoin DLC-specific | TBD (research firm contact) | PENDING |

## Outreach Template

```text
Subject: Audit inquiry — Hunch (Bitcoin prediction market protocol, Q4 2026 timeline)

Hi <firm>,

I'm the protocol maintainer for Hunch (https://github.com/Silexperience210/hunch),
a permissionless Bitcoin-native prediction market protocol combining DLC + Cashu NUT-CTF
(PR #337) + FROST k-of-n (frost-secp256k1-tr v2.2+, RFC 9591) + Lightning + Nostr.

We are scoping our Phase 3 security audit (per ROADMAP) and would like to discuss
your firm's availability for a comprehensive audit in Q4 2026.

Scope candidates:
- Cashu mint code (NUT-CTF integration, NUT-12 DLEQ, blind-sig flow)
- DLC contract construction (CET enumeration, refund timeout, adapter sigs)
- FROST DKG + signing (Pedersen DKG with TOB Feb 2024 fix verified)
- LDK Node integration (post-v0.1.1 fixes)
- Oracle Schnorr signing (BIP-340 + NIP-88)

Budget range: USD 75–180K single-firm, 120–250K dual-firm.

Maintainer operates under pseudonym. Legal entity contracting is via foundation
counsel (Phase 1 LEGAL-02 deliverable). Communications can be Nostr-native or
PGP-encrypted email — please indicate your preferred channel.

Would your team be open to a scoping call?

Thanks,
Silex — Hunch protocol maintainer
```

## Outreach Log

| Firm | Date sent | Channel | Response | Notes |
|------|-----------|---------|----------|-------|
| _(none yet — queued for user manual send)_ |  |  |  |  |

## Note on npub placeholder

The outreach template currently references `https://github.com/Silexperience210/hunch` for project verification. Once Plan 02 publishes the project Nostr npub, follow-up messages should include the npub for Nostr-native contact.

## Send instructions

This document queues outreach emails. **The user must send them manually** (no SMTP configured in the autonomous environment). Recommended cadence:

1. Send to firms 1–3 (Trail of Bits, Block, Cure53) in Phase 1 Week 4 — these are the highest-confidence candidates.
2. Send to firms 4–6 in Phase 1 Week 5 — fallback options if 1–3 are unavailable.
3. Update the Outreach Log table above with each send (date, channel, eventual response).
4. Scoping calls expected Week 5–8 of Phase 1.

## Selection criteria (per PITFALLS Pitfall 5 + RESEARCH §6.2)

- Bitcoin-native expertise (not generic crypto audit)
- Prior DLC or Cashu work history
- Acceptance of pseudonymous client (counsel handles contracting via foundation entity)
- Reasonable Q4 2026 availability
- Insurance: at least one firm with cryptographic-specific review depth (Trail of Bits or Quarkslab)
