# Hunch — Project State

**Generated:** 2026-05-27
**Last updated:** 2026-05-28 after Wave 2 execution (Plans 02 + 03 + 04 docs done)

## Project Reference

See: `.planning/PROJECT.md` (updated 2026-05-27)

**Core value:** Anyone, anywhere (except where geo-blocked), can bet on any verifiable question without KYC, without custody, and without trusting Hunch as an institution — only the oracle's Schnorr signature and the math of the DLC.

**Current focus:** Phase 1 — Cypherpunk Foundation (specs + spikes + legal)

## Current Status

- **Active milestone:** v1 — Mainnet Cypherpunk Spine
- **Active phase:** Phase 1 — Cypherpunk Foundation (Waves 1 + 2 docs done; user-action items pending)
- **Plans completed:** 4 / 4 (Plans 01–04 docs delivered; checkpoint:human-action items queued for user)
- **Phases completed:** 0 / 4 (Phase 1 doc-complete; awaits user actions to close)
- **Requirements (v1):** 2 / 84 fully closed (PROTO-07 LIVE, PROTO-08 verified); 12 partial / awaiting user action

## Milestone Progress

| Phase | Name | Status | Completed | Total |
|-------|------|--------|-----------|-------|
| 1 | Cypherpunk Foundation | Docs done (4/4 plans); awaits user actions (keygen, Nostr publish, outreach send, counsel engagement) | 2 | 18 |
| 2 | Mainnet Spine | Not started | 0 | 62 |
| 3 | Mainnet Launch & Hardening | Not started | 0 | 19 |
| 4 | Decentralization & Federation (v2 deferred) | Future | — | — |

## Workflow Configuration

See `.planning/config.json`:
- Mode: **YOLO**
- Granularity: **Coarse**
- Parallelization: **Enabled**
- Commit docs to git: **Yes**
- Model profile: **Quality (Opus)**
- Workflow agents enabled: **Research, Plan Check, Verifier, Nyquist Validation**

## Recent Activity

| Date | Event | Artifact |
|------|-------|----------|
| 2026-05-27 | Project initialized | `.planning/PROJECT.md` |
| 2026-05-27 | Workflow config committed | `.planning/config.json` |
| 2026-05-27 | Research completed (4 dimensions + summary) | `.planning/research/` |
| 2026-05-27 | v1 requirements defined (84 reqs) | `.planning/REQUIREMENTS.md` |
| 2026-05-27 | Roadmap created (4 phases, 3 active v1) | `.planning/ROADMAP.md` |
| 2026-05-28 | Phase 1 research completed (1188 lines; surfaced PR #128 closure + NUT-CTF pivot) | `.planning/phases/01-cypherpunk-foundation/01-RESEARCH.md` |
| 2026-05-28 | Phase 1 context locked (NUT-CTF Path A, full pseudonymity, counsel-driven jurisdiction) | `.planning/phases/01-cypherpunk-foundation/01-CONTEXT.md` |
| 2026-05-28 | Phase 1 planned — 4 plans, all 18 requirements covered (PASSED iteration 2) | `.planning/phases/01-cypherpunk-foundation/01-0[1-4]-PLAN.md` |
| 2026-05-28 | Plan 01 executed (Wave 1) — Cargo workspace + corrigendum + scripts + outreach queue. User approved checkpoint. PROTO-07 partial (repo URL PENDING), PROTO-08 verified | `.planning/phases/01-cypherpunk-foundation/01-01-SUMMARY.md` |
| 2026-05-28 | GitHub repo LIVE — `https://github.com/Silexperience210/hunch` published, PROTO-07 GitHub channel closed | commit `f90cee1` |
| 2026-05-28 | Plan 02 executed (Wave 2) — 6 HIPs + MANIFESTO + reviewer tracker + NIPs PR draft + publish-hip.ts. PROTO-01..06 drafted; Nostr publication + reviewer DM send queued for user (Plan 02 Tasks 2-5) | `.planning/phases/01-cypherpunk-foundation/01-02-SUMMARY.md` |
| 2026-05-28 | Plan 03 executed (Wave 2) — SPIKE-01 outreach drafts + SPIKE-03 FROST DKG 3-of-5 playbook + SPIKE-04 Lightning-DLC NO-GO assessment. SPIKE-02 signet prototype deferred (user opsec); SPIKE-04 closed | `.planning/phases/01-cypherpunk-foundation/01-03-SUMMARY.md` |
| 2026-05-28 | Plan 04 executed (Wave 2) — 6 LEGAL strawmen (jurisdiction memo + counsel shortlist + outreach log + engagement-letter-status + TERMS + PRIVACY + PR_PLAYBOOK + PSEUDONYMITY + public + FOLLOWUP + signoff). All marked COUNSEL SIGN-OFF PENDING | `.planning/phases/01-cypherpunk-foundation/01-04-SUMMARY.md` |

## Key Decisions Made

1. **Cypherpunk-first philosophy** — Trust the math, not Hunch. Manifesto guides all tradeoffs.
2. **Bitcoin-only stack** — DLC + Cashu + Lightning + Nostr. No altchains, no stablecoins.
3. **Protocol-first design** — HIPs as specs, multiple implementations encouraged, fork-friendly.
4. **Permissionless market creation** — Anyone can ask any question. Anti-spam via social graph + reputation, not curation.
5. **Multi-oracle marketplace** — No "Hunch oracle." Market creators choose. Reputation is the trust signal.
6. **Cashu mint as multi-bettor DLC adapter** — Solves bilateral DLC limitation while preserving on-chain settlement.
7. **Mainnet hardcore without caps** — But after audit, after Mutinynet, with tiered launch (invite → caps → no-caps).
8. **Offshore entity** — Switzerland / Panama / BVI / El Salvador (counsel-recommended).
9. **MIT license** — Maximum freedom for forks, no contributor ownership claims.
10. **Geo-block US on official frontend** — Polymarket-lesson legal mitigation. Protocol stays neutral.

## Next Steps — User Action Queue (Phase 1 close gates)

**Phase 1 close gate items** (must complete to close Phase 1):

1. **Generate HUNCH_NSEC** (Plan 02 Task 2) — project Nostr pseudonym key, under user opsec. Then publish 6 HIPs via `bun scripts/publish-hip.ts docs/HIP-N.md` for N in 0..5.
2. **Send counsel outreach** — 3 firms minimum (MME / Walkers / Marxer & Partner) from `docs/legal/counsel-outreach-log.md`. Scoping calls 1-3 weeks later. Engagement letter signing target Phase 1 Week 6-8.
3. **Send reviewer outreach DMs** — 6+ targets from `docs/HIPs-reviews.md`. At least 1 attestation per HIP unlocks HIP transition Draft → Final (HIP-3 also gated on SPIKE-02 success).
4. **Send 3 SPIKE-01 maintainer outreach** — joemphilips (GitHub), conduition (Nostr), Calle (Nostr) per `spikes/nut-ctf/PATH-A-VALIDATION.md`.
5. **Send 6 audit firm outreach emails** — from `docs/audit-shortlist-outreach.md` (Phase 1 Week 4-5 timing).
6. **Optional: open NIPs registry PR** for kind reservation per `docs/nips-pr-kind-reservation.md`.

**Phase 1+ followup items** (track in `docs/legal/PHASE-1-FOLLOWUP.md` and `docs/HIPs-reviews.md`):

- Counsel-signed PDFs land 4-8 weeks after engagement letter signing
- Entity formation 6-12 weeks after engagement
- HIP Status: Final transitions as reviewer attestations land
- SPIKE-02 + SPIKE-03 implementations land in Phase 2 (Mainnet Spine)

## Phase 1 Plan Structure (4 plans, 2 waves)

| Plan | Name | Wave | Reqs | Files |
|------|------|------|------|-------|
| 01 | Repo Foundation + Corrigendum | 1 | PROTO-07, PROTO-08 | 26 (12 trivial stubs) |
| 02 | HIPs Drafting + Nostr Publication | 2 | PROTO-01..06 | 12 |
| 03 | Technical Spikes (NUT-CTF + FROST + Lightning-DLC NO-GO) | 2 | SPIKE-01..04 | 19 |
| 04 | Legal Foundation (counsel + jurisdiction + ToS + privacy + PR + pseudonymity) | 2 | LEGAL-01..06 | 13 |

## Locked Phase 1 Decisions (CONTEXT.md)

1. **NUT-DLC = Path A (NUT-CTF / PR #337)** — PR #128 closed 2025-05-20, pivot confirmed
2. **Lightning-DLC NO-GO for v1** — atomic.finance acquired by Lygos Aug 2025; SPIKE-04 = written assessment, not prototype
3. **Jurisdiction deferred to LEGAL-01** — counsel produces recommendation; CH Stiftung + BVI BC is working hypothesis only
4. **Full pseudonymity including frontend** — load-bearing post-Storm § 1960 conviction; counsel must accept pseudonymous client

## Outstanding Open Questions

1. Final offshore jurisdiction selection — DEFERRED to counsel in LEGAL-01/02
2. Audit firm shortlist for Phase 3 — outreach in Phase 1 Plan 01
3. First external oracle partner — engage during Phase 2
4. LSP partner for Lightning liquidity — engage during Phase 2
5. External HIP reviewers (Bitcoin / Cashu / DLC contributors) — outreach in Phase 1 Plan 02

---
*State synced with PROJECT.md, REQUIREMENTS.md, ROADMAP.md*
