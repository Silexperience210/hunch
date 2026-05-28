# Phase 1+ Followup: Counsel Sign-Off PDFs

**Purpose:** Track the LEGAL deliverables that land AFTER Phase 1 closes — specifically, counsel-signed PDFs of the LEGAL strawmen. These are intrinsically asynchronous (4-8 weeks after engagement letter signing) and are NOT in the Phase 1 close gate.

**Created:** 2026-05-28
**Closure model:** see `engagement-letter-status.md` §Async Closure Model

## Phase 1 Close Gate (recap)

Phase 1 closes when:
1. ✓ All 6 LEGAL-0X strawmen drafted (this directory)
2. PENDING — Counsel engagement letter signed
3. PENDING — Jurisdiction memo updated with counsel recommendation

Items 2-3 land 4-8 weeks after outreach send. The remaining items below land AFTER Phase 1 closes.

## Phase 1+ Followup Items

| # | Deliverable | Source strawman | Expected close | Status |
|---|-------------|-----------------|----------------|--------|
| F1 | TERMS.md counsel-signed PDF | `TERMS.md` | Phase 1+ Week 4-8 | PENDING |
| F2 | PRIVACY.md counsel-signed PDF | `PRIVACY.md` | Phase 1+ Week 4-8 | PENDING |
| F3 | PR_PLAYBOOK.md counsel-signed PDF | `PR_PLAYBOOK.md` | Phase 1+ Week 4-6 | PENDING |
| F4 | PSEUDONYMITY.md counsel-acknowledged PDF (optional) | `PSEUDONYMITY.md` | Phase 1+ Week 4-6 | OPTIONAL |
| F5 | Foundation entity formation papers | `jurisdiction-memo.md` | Phase 1+ Week 6-12 | PENDING |
| F6 | Operating entity formation papers | `jurisdiction-memo.md` | Phase 1+ Week 6-12 | PENDING |
| F7 | Banking introductions completed | `jurisdiction-memo.md` | Phase 1+ Week 8-16 | PENDING |
| F8 | Foundation board roster + statutes | post-formation | Phase 1+ Week 12-16 | PENDING |

## Sign-Off PDF Storage

Per CONTEXT.md decision D-05 (full pseudonymity scope), counsel-signed PDFs MUST NOT be committed to the public repo. They contain:
- Counsel firm contact information
- Engagement letter terms (privileged)
- Beneficial-owner identification (KYC-confidential)
- Possibly real-name signatures of nominee directors

PDFs are stored in `docs/legal/signoff/` which is `.gitignore`d. See `docs/legal/signoff/README.md` for the storage convention.

## Update Cadence

This file is updated as items close:
- F1-F4 close when counsel returns the signed PDF; user moves the PDF to `signoff/` + updates this table.
- F5-F8 close as the entity formation + banking processes complete; user updates this table with the formation date + entity number.

## Trigger to Migrate Items Out of Followup

When all 8 items above close, this followup file is archived and the LEGAL closure is fully complete. The recommended workflow:

1. Update each table row from PENDING → COMPLETE with date + PDF filename
2. Update `engagement-letter-status.md` Gate 2 closure section with the final dates
3. Commit a closure note as `docs(legal): Phase 1+ LEGAL followup closed — all sign-off PDFs received`
4. Optionally archive this file (rename to `PHASE-1-FOLLOWUP-CLOSED.md`)

## References

- `engagement-letter-status.md` — Async closure model
- `jurisdiction-memo.md` — Working hypothesis for entity formation
- CONTEXT.md decisions D-04 + D-05 — Jurisdiction deferral + pseudonymity scope
