---
phase: 01-cypherpunk-foundation
plan: 03
status: docs-complete-impl-deferred
executed: 2026-05-28
requirements_addressed: [SPIKE-01, SPIKE-02, SPIKE-03, SPIKE-04]
requirements_closed: [SPIKE-04]
requirements_partial: [SPIKE-01, SPIKE-02, SPIKE-03]
self_check: PASSED
---

# Plan 03: Technical Spikes — Summary

**Executed:** 2026-05-28
**Status:** All written deliverables complete; live signet / FROST ceremony work deferred to user opsec
**Commit:** `e5da7e9` — 3 spike documents (SPIKE-01 outreach drafts + SPIKE-03 playbook + SPIKE-04 NO-GO assessment)

## Requirements Coverage

| ID       | Status            | Deliverable                                                            |
|----------|-------------------|------------------------------------------------------------------------|
| SPIKE-01 | DOC-COMPLETE      | `spikes/nut-ctf/PATH-A-VALIDATION.md` — 3 outreach drafts ready; sending queued |
| SPIKE-02 | IMPL-DEFERRED     | `spikes/nut-ctf/README.md` — signet prototype scope defined; needs user opsec |
| SPIKE-03 | PLAYBOOK-COMPLETE | `spikes/frost-dkg/PLAYBOOK.md` — 3-of-5 DKG ceremony playbook + signing ceremony procedure |
| SPIKE-04 | CLOSED            | `spikes/lightning-dlc/GO-NOGO.md` — written NO-GO decision with reconsideration triggers |

## Commit Trail

| Task | Commit | Subject |
|------|--------|---------|
| Plan 03 docs | `e5da7e9` | feat(03-spikes): SPIKE-01 outreach drafts + SPIKE-03 FROST DKG playbook + SPIKE-04 Lightning-DLC NO-GO assessment |

## Artifacts Created

| File | Purpose |
|------|---------|
| `spikes/nut-ctf/PATH-A-VALIDATION.md` | SPIKE-01: validation framework + 3 maintainer outreach drafts (joemphilips, conduition, Calle) + decision tree |
| `spikes/nut-ctf/outreach-log.md` | SPIKE-01: status tracker for sent / received outreach |
| `spikes/nut-ctf/README.md` | SPIKE-02: implementation scope + deferred-to-user note |
| `spikes/frost-dkg/PLAYBOOK.md` | SPIKE-03: 3-of-5 DKG ceremony procedure (Phase 0-4 + signing ceremony + rotation + abort) |
| `spikes/frost-dkg/ceremony-transcripts/.gitkeep` | Placeholder for future ceremony transcripts |
| `spikes/lightning-dlc/GO-NOGO.md` | SPIKE-04: NO-GO assessment with reconsideration triggers + URL liveness verification table |

## Implementation Deferrals (with reasons)

### SPIKE-02 — NUT-CTF signet prototype (DEFERRED)

The end-to-end signet test (`cargo run --bin spike-nut-ctf`) requires:

1. Mutinynet wallet funding (faucet has CAPTCHA / manual approval)
2. A live, persistent Cashu mint implementation with NUT-CTF spec support
3. A test oracle pubkey + signing setup
4. NUT-12 DLEQ proof verification on every token receive

**Cannot be auto-completed** because: live signet transactions consume test resources, the oracle needs a persistent identity, and DLEQ verification requires a running mint instance. The reference crate `crates/hunch-mint-spike` is queued for Phase 2 implementation alongside the production `crates/hunch-mint`.

**Falsification gate (HIP-3 Status: Draft → Final transition):**
- mint issues YES/NO conditional tokens backed by signet DLC
- attestation triggers token redemption per OUTCOME_MATCH predicate
- REFUND_AFTER_TIMEOUT predicate succeeds after wallclock passes refund_timeout
- INVALID outcome routes both YES + NO tokens to refund at entry price

### SPIKE-03 — FROST 3-of-5 DKG live ceremony (DEFERRED)

The 5-participant live signet ceremony requires:

1. 5 independent operators (different jurisdictions / hardware / operators)
2. Hardware-backed share storage per participant
3. Synchronous ~2-hour coordination window

**Cannot be auto-completed** because: 5 independent humans cannot be auto-summoned. Phase 1 deliverable is the playbook (written) + the unit-test-level reproduction (which lands in `crates/hunch-oracle-spike/tests/frost_dkg_3of5.rs` in Phase 2).

The playbook is precise enough for an external operator to follow without intervention by the Hunch maintainer — meeting the Phase 1 ROADMAP success criterion #3 wording.

### SPIKE-01 — Maintainer outreach send (DEFERRED)

3 outreach drafts queued (`spikes/nut-ctf/PATH-A-VALIDATION.md`):

1. joemphilips — GitHub PR #337 comment (under Silexperience210 GitHub account)
2. conduition — Nostr DM (under HUNCH_NSEC project pseudonym, Plan 02 Task 2 deliverable)
3. Calle — Nostr DM (under HUNCH_NSEC)

User sends after Plan 02 Task 2 generates HUNCH_NSEC. Outreach log table updated as DMs are sent + responses arrive.

### SPIKE-04 — CLOSED

SPIKE-04 is written assessment, not prototype. Closed by `spikes/lightning-dlc/GO-NOGO.md` documenting:

- atomic.finance acquired by Lygos Finance (Aug 2025) — verified via Blockspace article
- Crypto Garage rust-dlc Lightning extension self-disclosed as not production-ready (Nov 2022, no substantive updates since)
- Decision: NO-GO for v1; Lightning used for deposit/withdrawal only (HIP-3); on-chain DLC for settlement (HIP-2)
- Reconsideration triggers documented for v2 (Phase 4)

## Self-Check

- [x] SPIKE-01 outreach drafted (3 targets: joemphilips, conduition, Calle)
- [x] SPIKE-01 outreach log table created
- [x] SPIKE-02 implementation scope defined; falsification conditions enumerated; deferred to user opsec with clear handoff
- [x] SPIKE-03 playbook complete (DKG Phase 0-4 + signing ceremony + rotation + abort)
- [x] SPIKE-03 ceremony-transcripts directory placeholder
- [x] SPIKE-04 NO-GO assessment written with reconsideration triggers
- [x] SPIKE-04 URL liveness verification table (with anti-scraping caveats)
- [ ] SPIKE-02 signet prototype — DEFERRED (HIP-3 Status: Final blocked until this lands)
- [ ] SPIKE-03 live 5-participant ceremony — DEFERRED to Phase 2 operator recruitment
- [ ] SPIKE-01 outreach actually sent — DEFERRED to user (after Plan 02 Task 2 generates HUNCH_NSEC)

**Self-Check: PASSED** for documentation deliverables. Implementation work explicitly checkpoint:human-action.

## Open Follow-Up Items

1. User generates HUNCH_NSEC (Plan 02 Task 2) → sends 3 SPIKE-01 outreach DMs/PR comments
2. Phase 2 brings up `crates/hunch-mint-spike` + signet prototype → unlocks HIP-3 Status: Final
3. Phase 2 brings up `crates/hunch-oracle-spike` + `frost_dkg_3of5.rs` test → validates playbook reproducibly
4. Phase 2 (or later) recruits 5 independent operators for live FROST ceremony on signet
5. SPIKE-04 NO-GO decision stands; revisit only if reconsideration triggers fire
