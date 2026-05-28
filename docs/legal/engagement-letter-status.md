# LEGAL-02: Engagement Letter Status

**Status:** PENDING — no engagement letter signed yet
**Updated:** 2026-05-28

## Async Closure Model (per Plan 04 must_haves)

Phase 1 LEGAL closure is split into two gates to honor the realistic counsel-engagement timeline:

### Gate 1 — Phase 1 close gate (in scope)

Phase 1 closes when **at least one** crypto-specialized counsel:

1. Accepts the pseudonymous-BO scope (Tier 1 or Tier 2 per counsel-shortlist.md)
2. Has been scheduled for or completed a scoping call
3. Has provided an engagement letter (draft or final) covering at minimum:
   - Foundation incorporation (or recommendation against, with rationale)
   - Reference mint operating entity incorporation
   - ToS / privacy / PR-playbook sign-off scope
   - Hourly rates + retainer structure
   - Pseudonymous-BO confirmation in writing
4. Engagement letter is **signed** (electronically or via PGP signature)

Expected timeline: 4-8 weeks from outreach send.

### Gate 2 — Phase 1+ followup (out of Phase 1 scope)

After Gate 1 closes (engagement letter signed), the counsel begins substantive work:

5. Jurisdiction-memo.md finalized with counsel recommendation
6. TERMS.md counsel-reviewed + signed off as PDF
7. PRIVACY.md counsel-reviewed + signed off as PDF
8. PR_PLAYBOOK.md counsel-reviewed + signed off as PDF
9. PSEUDONYMITY.md counsel-reviewed (optional sign-off; private operational doc)
10. Foundation entity formed; share certificates / Stiftung statutes filed
11. Operating entity formed; VASP registration filed if applicable
12. Banking introductions made; bank accounts opened

Expected timeline: 4-8 weeks after Gate 1.

**Counsel-signed PDFs land in `signoff/` per CONTEXT.md D-05 (private storage, gitignored).**

## Why Async Closure

This split exists because:

1. **Counsel engagement is intrinsically multi-week.** Scoping → engagement letter → retainer wire → substantive work each takes 1-2 weeks; full cycle ≥ 4 weeks.
2. **Phase 1 ROADMAP estimate (6-8 weeks) is for active maintainer work,** not for waiting on counsel sign-off. The ROADMAP entry was scoped to maintainer-time; the async tail extends beyond.
3. **Substantive work has dependencies on engagement letter.** ToS and Privacy sign-off cannot happen before counsel is engaged. If we wait for sign-off, Phase 1 cannot close for months.

The Phase 1 deliverable is therefore reframed as: **strawmen drafted + counsel engaged + engagement letter signed**. Counsel sign-off PDFs are tracked as Phase 1+ followup (see `PHASE-1-FOLLOWUP.md`).

## Calendar (Working Estimate)

| Week | Activity | Owner |
|------|----------|-------|
| Phase 1 W1 | Strawmen drafted (this directory) | Hunch maintainer |
| Phase 1 W2 | Outreach send (3 primary firms) | User (after Plan 02 Task 2) |
| Phase 1 W3-4 | Scoping calls scheduled | User + counsel |
| Phase 1 W4-5 | Scoping calls completed | User + counsel |
| Phase 1 W5-6 | Counsel proposals received | Counsel |
| Phase 1 W6-7 | Engagement letter negotiated | User + counsel |
| Phase 1 W7-8 | **Engagement letter SIGNED — Phase 1 close gate** | User |
| ----- | ----- | ----- |
| Phase 1+ W1-2 | Retainer wired; counsel begins work | Counsel |
| Phase 1+ W2-4 | Jurisdiction memo finalized; entity choice locked | Counsel + user |
| Phase 1+ W4-8 | ToS / Privacy / PR-playbook signed PDFs delivered | Counsel |
| Phase 1+ W4-8 | Entity formed; banking introductions | Counsel + bank |

## Current State

- [x] Strawmen drafted (TERMS, PRIVACY, PR_PLAYBOOK, PSEUDONYMITY, jurisdiction-memo)
- [x] Counsel shortlist drafted (`counsel-shortlist.md`)
- [x] Outreach template drafted (`counsel-outreach-log.md`)
- [ ] Outreach sent (3 primary firms) — PENDING USER
- [ ] Scoping calls scheduled — PENDING (depends on outreach)
- [ ] **Engagement letter signed** — PENDING (Phase 1 close gate)
- [ ] Counsel-signed PDFs of TERMS / PRIVACY / PR_PLAYBOOK — PENDING (Phase 1+ followup, tracked in `PHASE-1-FOLLOWUP.md`)

## Definition of "Engaged"

For Phase 1 closure purposes, **counsel is "engaged" when:**

1. Engagement letter signed by both parties (electronically or PGP-signed)
2. Retainer wired (or escrow arrangement in place)
3. Counsel has confirmed scope acceptance in writing
4. Counsel has acknowledged pseudonymous-BO scope in writing

A scoping call alone does NOT constitute engagement.

## References

- `counsel-shortlist.md` — Tier 1 / Tier 2 acceptance criteria
- `counsel-outreach-log.md` — Outreach status tracker
- `PHASE-1-FOLLOWUP.md` — Tracker for Phase 1+ deliverables landing after engagement
- Hunch CONTEXT.md decision D-04 — Jurisdiction deferred to counsel
- Hunch CONTEXT.md decision D-05 — Full pseudonymity scope (counsel-acceptance filter)
