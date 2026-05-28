# Hunch — Project State

**Generated:** 2026-05-27
**Last updated:** 2026-05-28 after Phase 1 planning

## Project Reference

See: `.planning/PROJECT.md` (updated 2026-05-27)

**Core value:** Anyone, anywhere (except where geo-blocked), can bet on any verifiable question without KYC, without custody, and without trusting Hunch as an institution — only the oracle's Schnorr signature and the math of the DLC.

**Current focus:** Phase 1 — Cypherpunk Foundation (specs + spikes + legal)

## Current Status

- **Active milestone:** v1 — Mainnet Cypherpunk Spine
- **Active phase:** Phase 1 — Cypherpunk Foundation (planned 2026-05-28, ready to execute via `/gsd-execute-phase 1`)
- **Phases completed:** 0 / 4
- **Requirements (v1):** 0 completed / 84 total

## Milestone Progress

| Phase | Name | Status | Completed | Total |
|-------|------|--------|-----------|-------|
| 1 | Cypherpunk Foundation | Ready to execute (4 plans) | 0 | 18 |
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

## Next Steps

Run `/gsd-execute-phase 1` to start executing the 4 planned plans (Foundation → HIPs+Spikes+Legal in wave 2).

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
