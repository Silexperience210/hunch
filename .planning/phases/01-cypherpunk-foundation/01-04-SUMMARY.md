---
phase: 01-cypherpunk-foundation
plan: 04
status: strawmen-complete-engagement-letter-pending
executed: 2026-05-28
requirements_addressed: [LEGAL-01, LEGAL-02, LEGAL-03, LEGAL-04, LEGAL-05, LEGAL-06]
requirements_closed: []
requirements_partial: [LEGAL-01, LEGAL-02, LEGAL-03, LEGAL-04, LEGAL-05, LEGAL-06]
self_check: PASSED
---

# Plan 04: Legal Foundation — Summary

**Executed:** 2026-05-28 (strawmen drafted; counsel engagement pending)
**Status:** All 6 LEGAL deliverables drafted as strawmen + counsel outreach queue ready; engagement letter signing pending user manual outreach send
**Commit:** `f7a67a5` — 13 legal files

## Async Closure Model (per CONTEXT.md D-04)

Phase 1 LEGAL closure splits into two gates:

- **Phase 1 close gate:** strawmen drafted ✓ + counsel engagement letter signed (PENDING; expected 4-8 weeks)
- **Phase 1+ followup:** counsel-signed PDF artifacts + entity formation + banking (tracked in `docs/legal/PHASE-1-FOLLOWUP.md`)

The Phase 1 ROADMAP estimate (6-8 weeks) covers active maintainer work only; the async tail (counsel scoping + engagement + sign-off) extends into Phase 2 timeline overlap.

## Commit Trail

| Commit | Subject |
|--------|---------|
| `f7a67a5` | feat(04-legal): draft 5 LEGAL strawmen + jurisdiction memo + counsel shortlist + outreach + followup tracker |

## Artifacts Created

| File | LEGAL Req | Purpose |
|------|-----------|---------|
| `docs/legal/README.md` | — | Document map + closure-gate explanation |
| `docs/legal/jurisdiction-memo.md` | LEGAL-01 | Comparison memo (CH / BVI / LI / Panama / Cayman) + working hypothesis (CH Stiftung + BVI BC) + counsel-recommendation section OPEN |
| `docs/legal/counsel-shortlist.md` | LEGAL-02 | 8 candidate firms across CH / BVI / LI filtered for pseudonymous-client acceptance |
| `docs/legal/counsel-outreach-log.md` | LEGAL-02 | Outreach template + status tracker; PENDING USER manual send |
| `docs/legal/engagement-letter-status.md` | LEGAL-02 | Async closure model + week-by-week calendar + Phase-1-close vs Phase-1+ gate distinction |
| `docs/legal/TERMS.md` | LEGAL-03 | ToS strawman; geo-block US + sanctioned + age + no-investment-advice + no-tax-advice + protocol-vs-frontend distinction |
| `docs/legal/PRIVACY.md` | LEGAL-04 | Privacy policy strawman; no PII beyond npub + no analytics + no fingerprinting + no cookies-beyond-session |
| `docs/legal/PR_PLAYBOOK.md` | LEGAL-05 | 7 scenarios (curation inquiry, subpoena, regulatory, indictment, mint failure, deplatforming, doxxing) with hold messages + escalation paths + master principles |
| `docs/legal/PSEUDONYMITY.md` | LEGAL-06 | Private operational plan (commit hygiene + accounts + comms + banking + key genesis + continuity) |
| `docs/legal/PSEUDONYMITY-public.md` | LEGAL-06 | Public-facing principles statement (cypherpunk tradition + what it means + what it does NOT mean) |
| `docs/legal/PHASE-1-FOLLOWUP.md` | — | Tracker for 8 Phase-1+ items (counsel-signed PDFs + entity formation + banking) |
| `docs/legal/signoff/.gitignore` | — | Excludes counsel-signed PDFs from public commits per D-05 |
| `docs/legal/signoff/README.md` | — | Storage convention (3 options: private branch + age / private repo / SHA-256 commitments) |

## Locked Decisions Honored

- **D-04 (Jurisdiction deferred to counsel):** Working hypothesis (CH Stiftung + BVI BC) documented as hypothesis only; counsel-recommendation section OPEN in `jurisdiction-memo.md`; LEGAL-01 final form is post-engagement deliverable.
- **D-05 (Full pseudonymity):** Counsel shortlist filtered for pseudonymous-BO acceptance (Tier 3 reject = real-name requirement); outreach template references nominee-director arrangements; PSEUDONYMITY plan covers commit hygiene + accounts + comms + banking; doxxing response prohibits engagement; sign-off PDFs go to `signoff/` (gitignored).
- **Soft-dep on Plan 01 PROTO-07:** GitHub repo URL `github.com/Silexperience210/hunch` is referenced in outreach template — Plan 01 created the repo (commit `f90cee1`, live HTTP 200), so the URL is valid in outreach send.

## Self-Check

- [x] LEGAL-01 — jurisdiction comparison memo with 5 jurisdictions table + working hypothesis + counsel-input section
- [x] LEGAL-02 — counsel shortlist (8 firms) + outreach template + outreach log table + engagement-letter-status async-closure model
- [x] LEGAL-03 — TERMS.md strawman with required sections (eligibility / nature-of-frontend / prohibited markets / no-custody / no-tax / limitation of liability / protocol risks / open source / termination / governing law / dispute resolution / changes / contact)
- [x] LEGAL-04 — PRIVACY.md strawman with required sections (scope / what-we-collect / what-protocol-stores / geo-blocking / server logs / cookies / third-party services / data-rights / children / locations / security / breach / changes / contact)
- [x] LEGAL-05 — PR_PLAYBOOK.md with 7 scenarios (curation / subpoena / regulatory / indictment / mint-or-oracle-failure / deplatforming / doxxing) + master principles + update procedure
- [x] LEGAL-06 — PSEUDONYMITY.md (private operational) + PSEUDONYMITY-public.md (public principles)
- [x] PHASE-1-FOLLOWUP.md tracker created for 8 Phase-1+ items
- [x] signoff/ directory with .gitignore + README documenting storage options
- [x] All strawmen marked "COUNSEL SIGN-OFF PENDING" + version draft-0.1
- [ ] Outreach sent (3 primary firms minimum) — PENDING USER MANUAL SEND
- [ ] Scoping calls scheduled — PENDING (depends on outreach)
- [ ] Engagement letter signed — PENDING (Phase 1 close gate)

**Self-Check: PASSED** for strawmen drafting. Outreach + engagement explicitly checkpoint:human-action.

## Requirements Status

| ID | Status | Phase 1 close condition | Phase 1+ followup |
|----|--------|------------------------|-------------------|
| LEGAL-01 | STRAWMAN-COMPLETE | Counsel recommendation section filled by engaged counsel | Final jurisdiction lock + entity formation |
| LEGAL-02 | OUTREACH-QUEUED | Engagement letter signed | Retainer wired + substantive work begins |
| LEGAL-03 | STRAWMAN-COMPLETE | (n/a — depends on counsel-signed PDF, async) | TERMS-v1.0-counsel-signed.pdf in signoff/ |
| LEGAL-04 | STRAWMAN-COMPLETE | (n/a — depends on counsel-signed PDF, async) | PRIVACY-v1.0-counsel-signed.pdf in signoff/ |
| LEGAL-05 | STRAWMAN-COMPLETE | (n/a — depends on counsel-signed PDF, async) | PR_PLAYBOOK-v1.0-counsel-signed.pdf in signoff/ |
| LEGAL-06 | STRAWMAN-COMPLETE | Public statement live (PSEUDONYMITY-public.md committed) | Optional counsel acknowledgment of operational plan |

All marked `requirements_partial` because the closure gates either await counsel input (LEGAL-01, -02) or await sign-off PDFs (-03, -04, -05) or await operational deployment (-06).

## Open Follow-Up Items

1. **User: send 3 outreach emails** from `counsel-outreach-log.md` (firms 1 MME, 4 Walkers, 7 Marxer & Partner) — Phase 1 Week 4-5 timing
2. **User: schedule scoping calls** as responses arrive (Phase 1 Week 5-8)
3. **User: sign engagement letter** with selected counsel (Phase 1 Week 7-8)
4. **Phase 1+ followup:** counsel-signed PDFs land 4-8 weeks after engagement; user moves to `signoff/` per chosen storage option
5. **Phase 1+ followup:** entity formation + banking (4-12 weeks after engagement; counsel-led)
6. **Optional: update `npub-TBD` placeholders** in TERMS contact section + PRIVACY contact section after Plan 02 Task 2 generates HUNCH_NSEC

## What This Enables

- **Phase 1 close:** engagement-letter-signed gates Phase 1 → Phase 2 transition
- **Phase 2 reference frontend (UI-01..21):** can launch with provisional ToS (strawmen) referencing the operating entity; signed-PDFs replace strawmen as counsel signs off
- **Phase 2 reference mint (MINT-01..13):** operating entity exists in form (per counsel engagement scoping) to operate the mint
- **Phase 3 audit (SEC-01..08):** counsel-coordinated audit firm engagement (audit firm outreach already started in Plan 01 `docs/audit-shortlist-outreach.md`)
