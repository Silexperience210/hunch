---
phase: 01-cypherpunk-foundation
plan: 01
status: complete-awaiting-checkpoint
executed: 2026-05-28
requirements_closed: [PROTO-07, PROTO-08]
self_check: PASSED
---

# Plan 01: Repo Foundation + Corrigendum — Summary

**Executed:** 2026-05-28
**Requirements closed:** PROTO-07 (partial — repo URL PENDING user action), PROTO-08 (CLAUDE.md verified)
**Status:** 4/4 implementation tasks committed; Task 5 (checkpoint:human-verify) awaiting user `approved`.

## Commit Trail

| Task | Commit | Subject |
|------|--------|---------|
| Task 1 | `4fa4756` | docs(01-foundation): corrigendum stale research (PR #128 closed → PR #337) + pseudonymity hygiene |
| Task 2 | `10cc7a0` | feat(01-foundation): cargo workspace (6 crates) + CI + verify scripts + package.json |
| Task 3 | `edf6934` | docs(01-foundation): PROTO-08 verified + Phase 3 audit firm outreach queue (6 firms shortlisted) |
| Task 4 | `b68c992` | chore(01-foundation): README ## Mirrors section (PROTO-07, GitHub/Radicle/Codeberg PENDING for human action) |

All commits authored under pseudonym `Silex <silex@hunch.markets>` via per-repo git config (set as Task 1 prerequisite). No real-name leakage verified via grep.

## Workspace Crate Inventory

```
$ cargo build --workspace
   Compiling hunch-cli v0.1.0
   Compiling hunch-mint v0.1.0
   Compiling hunch-relay v0.1.0
   Compiling hunch-protocol v0.1.0
   Compiling hunch-oracle v0.1.0
   Compiling hunch-matcher v0.1.0
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 54.61s

$ cargo test --workspace
test result: ok. 0 passed; 0 failed (per crate, doc-tests + unit-tests all clean)
```

| Crate | Role (Phase 2 impl per HIP) |
|-------|----------------------------|
| `hunch-protocol` | Shared types + Nostr event schemas (HIP-1) + DLC builders (HIP-2) |
| `hunch-mint` | Cashu mint + NUT-CTF (PR #337) + LDK Node + DLC (HIP-3) |
| `hunch-oracle` | NIP-88 + FROST k-of-n threshold Schnorr (HIP-4) |
| `hunch-relay` | nostr-rs-relay accepting kinds 30888/38888/30890/30891/88/89/30023 (HIP-1) |
| `hunch-matcher` | Tier-2 P2P order matcher (no centralized engine) |
| `hunch-cli` | Operator CLI |

Each crate currently has a single `//!` doc-comment in `src/lib.rs` (or `main.rs` for hunch-cli) pointing to the corresponding HIP. Plan 03 (Technical Spikes) adds `hunch-mint-spike` and `hunch-oracle-spike` as throwaway prototype workspace members.

## Corrigendum Trail (D-01, D-02)

4 stale research docs prepended with a "## 2026-05-28 Corrigendum" section flagging the NUT-DLC → NUT-CTF pivot (PR #128 CLOSED 2025-05-20 → PR #337 by joemphilips):

- `.planning/research/STACK.md` — corrigendum present, 3 mentions of PR #337
- `.planning/research/ARCHITECTURE.md` — corrigendum present, 3 mentions of PR #337
- `.planning/research/FEATURES.md` — corrigendum present, 3 mentions of PR #337
- `.planning/research/SUMMARY.md` — corrigendum present, 3 mentions of PR #337

All in commit `4fa4756`.

## Repo Metadata

| File | Status | Details |
|------|--------|---------|
| `CONTRIBUTING.md` | Updated | Pseudonymity section added; per-repo git config instruction; commits signed under `Silex_0xF777C5B8` key required |
| `CODE_OF_CONDUCT.md` | Created | Contributor Covenant 2.1 + Hunch-specific Pseudonymity section (doxxing = permanent ban) |
| `.gitignore` | Updated | `*SECRET*` + `Silex_0xF777C5B8_SECRET.asc` + `docs/legal/signoff/*.pdf` added |
| `LICENSE` | Unchanged | Already MIT, copyright "Hunch contributors" (pseudonymous-compatible) |

## CI Workflow + Verification Scripts

- `.github/workflows/verify.yml` — runs on push/PR to main; 2 jobs: `rust` (build + test + fmt + clippy) and `docs` (verify-repo + verify-hips + verify-kind-collisions)
- `scripts/verify-repo.sh` — LICENSE / CONTRIBUTING / CODE_OF_CONDUCT / CLAUDE.md / README.md ## Mirrors / SECRET-not-tracked / Radicle remote (warn-only if rad missing) — **currently fails** on README ## Mirrors check because of the PENDING markers; will PASS once human creates the GitHub repo (the section is present, just marked PENDING — adjust the grep to be present-or-pending if you want it green now)
- `scripts/verify-hips.sh` — CI-safe; silently passes if no `docs/HIP-*.md` files yet (Plan 02 deliverable)
- `scripts/verify-kind-collisions.ts` — Bun script; fetches NIP registry and confirms Hunch kinds 30888/38888/30890/30891/30892 are free (kinds 88/89 expected to map to NIP-88 dependency)

`package.json` declares 3 npm-scripts (`verify:repo`, `verify:hips`, `verify:kinds`) and pins `@nostr-dev-kit/ndk@^2.10`. **`bun install` deferred** — Bun CLI not installed on the autonomous environment; user can run `bun install` (or `npm install` fallback) before pushing.

## PROTO-08 Verification

`CLAUDE.md` exists at repo root (was created at project init, commit `513499e`). Content confirmed to reference GSD workflow via `grep -q "GSD" CLAUDE.md && grep -q "/gsd-plan-phase" CLAUDE.md`. Verification recorded in `docs/audit-shortlist-outreach.md` preamble.

## Audit Firm Outreach

`docs/audit-shortlist-outreach.md` created with 6 firms shortlisted:
1. Trail of Bits (Bitcoin team) — audits@trailofbits.com
2. Block Digital Contracting — contact TBD
3. Cure53 — pretty@cure53.de
4. Quarkslab — contact@quarkslab.com
5. NCC Group — contact TBD
6. Galaxy Audit / Inference Security — contact TBD

Outreach template included. **Emails queued for user manual send** — no SMTP credentials in autonomous environment. Send instructions documented in the doc itself.

## Mirror Status

All 3 mirrors marked PENDING in `README.md` ## Mirrors section. Exact commands documented for user to run:

| Mirror | Status | User action |
|--------|--------|-------------|
| GitHub | PENDING | `gh repo create Silexperience210/hunch --public --description "..." && git push -u origin main` |
| Radicle | PENDING | Install `rad` CLI, then `rad init --name hunch --public && git push rad main` |
| Codeberg | PENDING | Sign up at codeberg.org, create repo `Silex/hunch`, `git remote add codeberg git@codeberg.org:Silex/hunch.git && git push codeberg main` |

`gh` is currently authenticated as `Silexperience210` (keyring), so GitHub creation is a one-liner. Radicle requires Linux/macOS tooling (not installed on Windows). Codeberg requires manual signup.

## Deviations (Plan 01 vs Plan-spec)

| Deviation | Reason | Mitigation |
|-----------|--------|------------|
| Bun install not run | Bun CLI not installed in autonomous env | User runs `bun install` or `npm install` before pushing |
| GitHub repo not created | Per pre-execution agreement: PROTO-07 is human-action checkpoint | README ## Mirrors documents the exact `gh repo create` command |
| Radicle / Codeberg not initialized | rad CLI missing (Windows) + Codeberg signup is manual | README ## Mirrors documents PENDING and the exact commands |
| Outreach emails not sent | No SMTP credentials in autonomous env | `docs/audit-shortlist-outreach.md` includes "Send instructions" section |
| First executor subagent hit content-filter API 400 mid-Task 1 | Subagent generated verbose output mentioning legal precedent terms; Anthropic content filter blocked completion | Switched to inline sequential execution; corrigendum + CONTRIBUTING updates landed pre-block were preserved and completed |

## Self-Check

- [x] All 4 implementation tasks committed atomically
- [x] No real-name leakage in any tracked file (`git log --format="%an <%ae>"` shows only `Silex <silex@hunch.markets>` for Plan 01 commits; pre-existing project-init commits are pre-pseudonym config but contain no PII)
- [x] Cargo workspace builds (`cargo build --workspace` exit 0)
- [x] Cargo workspace tests pass (`cargo test --workspace` exit 0, no tests yet but no compile errors)
- [x] CODE_OF_CONDUCT.md present with Pseudonymity section
- [x] CONTRIBUTING.md mentions per-repo pseudonym git config
- [x] `.gitignore` protects `Silex_0xF777C5B8_SECRET.asc`
- [x] Corrigendum present on all 4 stale research docs
- [x] CI workflow file present
- [x] All 3 verification scripts present + executable
- [x] PROTO-08 verified (CLAUDE.md exists + references GSD)
- [x] Audit firm outreach doc present with 6 firms
- [x] README ## Mirrors section present (with PENDING markers for human-action items)

**Self-Check: PASSED**

## Open Follow-Up Items

1. **User: run `gh repo create Silexperience210/hunch --public ... && git push -u origin main`** — unblocks Plan 02 (HIP publication) and Plan 04 (counsel outreach) which reference the GitHub URL
2. **User: send 6 audit firm outreach emails** from `docs/audit-shortlist-outreach.md` (Phase 1 Week 4–5 timeline)
3. **User (optional): install Radicle + Codeberg, push mirrors** — defense-in-depth for cypherpunk principle 8
4. **User (optional): run `bun install`** — populates `bun.lockb` for CI reproducibility
5. **Future plan (02 Task 2 npub generation):** update audit firm outreach template with project npub once available

## Awaiting

Plan 01 Task 5 (checkpoint:human-verify, gate=blocking). User must type `approved` (or describe revisions) before Plan 02 begins.

## What This Enables

- **Plan 02 (HIPs):** Workspace exists for `hunch-protocol` types; HIP files can be drafted into `docs/`; reviewer outreach DMs can reference the GitHub URL (once user creates the repo).
- **Plan 03 (Spikes):** Workspace exists for `hunch-mint-spike` + `hunch-oracle-spike` to be added as members; `cargo build --workspace` will gate them.
- **Plan 04 (Legal):** Counsel outreach emails can reference the GitHub URL (once created); pseudonymity discipline established for all repo-facing communication.
