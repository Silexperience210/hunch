# Hunch — Project State

**Generated:** 2026-05-27
**Last updated:** 2026-05-27 after initialization

## Project Reference

See: `.planning/PROJECT.md` (updated 2026-05-27)

**Core value:** Anyone, anywhere (except where geo-blocked), can bet on any verifiable question without KYC, without custody, and without trusting Hunch as an institution — only the oracle's Schnorr signature and the math of the DLC.

**Current focus:** Phase 1 — Cypherpunk Foundation (specs + spikes + legal)

## Current Status

- **Active milestone:** v1 — Mainnet Cypherpunk Spine
- **Active phase:** None (Phase 1 ready to start via `/gsd-discuss-phase 1` or `/gsd-plan-phase 1`)
- **Phases completed:** 0 / 4
- **Requirements (v1):** 0 completed / 84 total

## Milestone Progress

| Phase | Name | Status | Completed | Total |
|-------|------|--------|-----------|-------|
| 1 | Cypherpunk Foundation | Ready to start | 0 | 18 |
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

Run `/gsd-discuss-phase 1` to gather context for Phase 1 planning (recommended), or `/gsd-plan-phase 1` to skip discussion and plan directly.

## Outstanding Open Questions

1. Final offshore jurisdiction selection (CH vs PA vs BVI vs SV) — needs counsel input in Phase 1
2. Audit firm shortlist for Phase 3 (engage outreach in Phase 1)
3. First external oracle partner (engage during Phase 2)
4. LSP partner for Lightning liquidity (engage during Phase 2)
5. Manifesto author / co-author (engage early Phase 1 for brand voice)

---
*State synced with PROJECT.md, REQUIREMENTS.md, ROADMAP.md*
