# Phase 1: Cypherpunk Foundation — Research

**Researched:** 2026-05-28
**Domain:** Bitcoin-native cypherpunk prediction market protocol — protocol specs (HIPs), technical de-risking spikes (NUT-DLC, FROST DKG, Lightning-DLC), and offshore legal foundation
**Confidence:** HIGH for legal precedents and HIP authoring conventions; HIGH for FROST cryptographic state; **MEDIUM-LOW for NUT-DLC** (this research uncovered that PR #128 was closed May 20, 2025 — a material change from prior internal docs); LOW for Lightning-DLC v1 viability (confirmed not production-ready).

---

## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| PROTO-01 | HIP-0 protocol overview & cypherpunk manifesto | §2 HIP Authoring (NIP-style template) |
| PROTO-02 | HIP-1 Nostr event kinds spec | §2 HIP Authoring (kind collision check + addressable-event mechanics) |
| PROTO-03 | HIP-2 DLC contract structure (binary YES/NO/INVALID, refund timeout, multi-oracle FROST-ready) | §2 HIP Authoring (dlcspecs reference posture) |
| PROTO-04 | HIP-3 Cashu NUT-DLC integration spec | §3 NUT-DLC Deep Dive — CRITICAL: PR #128 closed; recommend NUT-CTF (PR #337) pivot |
| PROTO-05 | HIP-4 Multi-oracle FROST protocol | §4 FROST DKG |
| PROTO-06 | HIP-5 Reputation event format | §2 HIP Authoring (Nostr addressable event + aggregation rules) |
| PROTO-07 | Repo published GitHub+Radicle, MIT, CONTRIBUTING/CoC | §2 HIP Authoring (repo layout) + §7 Validation Architecture |
| PROTO-08 | CLAUDE.md generated for GSD workflow | Already complete (root `CLAUDE.md` exists, verified 2026-05-28) |
| SPIKE-01 | NUT-DLC current state validated, fork-vs-upstream decision | §3 NUT-DLC Deep Dive — **decision needed: PR #128 dead, PR #337 alternative, custodial-promise fallback** |
| SPIKE-02 | NUT-DLC working prototype on signet (full flow) | §3 NUT-DLC Deep Dive — viability depends on SPIKE-01 decision outcome |
| SPIKE-03 | FROST 3-of-5 DKG ceremony on signet + playbook | §4 FROST DKG |
| SPIKE-04 | Lightning-DLC channel readiness assessment | §5 Lightning-DLC — **recommendation: NO-GO for v1** (verified) |
| LEGAL-01 | Offshore entity recommendation | §6 Legal Foundation (jurisdiction matrix) |
| LEGAL-02 | Crypto-specialized counsel engaged | §6 Legal Foundation (counsel shortlist) |
| LEGAL-03 | ToS drafted with counsel sign-off | §6 Legal Foundation (template references) |
| LEGAL-04 | Privacy policy drafted | §6 Legal Foundation (minimal-PII baseline) |
| LEGAL-05 | PR response playbook | §6 Legal Foundation + §8 Operator risk profile |
| LEGAL-06 | Maintainer pseudonymity plan | §6 Legal Foundation + §8 Operator risk profile (Storm + Coplan lessons) |

---

## Project Constraints (from CLAUDE.md)

Phase 1 work must respect these directives lifted verbatim from `C:\Users\Silex\Hunch\CLAUDE.md`. They are not negotiable in plan-checker review:

**Cypherpunk principles (non-negotiable):**
1. Trust the math. Never introduce a centralized trust point unless absolutely necessary; document it loudly if you must.
2. No KYC. Ever. Nostr pubkey is the only identifier.
3. No US. Geo-block official frontend. No US-targeted features.
4. Open source MIT. Every line readable + forkable.
5. Protocol-first. HIPs define the protocol; code is one implementation.
6. Multi-frontend / multi-mint / multi-oracle. Never hard-code Hunch as canonical.
7. No tokens. Bitcoin is the token.
8. Tor + IPFS first. Hidden service from day 1.

**Engineering directives bearing on Phase 1:**
- No custom crypto. Use `frost-secp256k1-tr`, `secp256k1`, `bdk`, `rust-dlc`, `cdk` as-is. **Never roll your own nonces, blinding factors, or signing flows.**
- NUT-DLC is alpha. Pin versions. Test on signet. **Don't ship to mainnet without external audit signoff.**
- Mainnet hardcore is a goal, not a starting state. (Phase 3 concern, but Phase 1 specs must allow tiered launch.)
- Don't pin a single Cloudflare/Hetzner/Vercel as canonical host.
- Don't write to GitHub Issues only — mirror to Radicle and Nostr.
- Don't use `git commit --no-verify` to bypass hooks.

**Process directives:**
- Mode YOLO, granularity coarse, parallelization enabled, commit_docs true, model profile Quality (Opus), all quality agents enabled (research, plan_check, verifier, nyquist_validation).
- Branching: trunk.

CONTEXT.md does NOT exist for Phase 1 — no `/gsd-discuss-phase 1` was run prior to this research. All decisions in this document are **research recommendations** subject to user confirmation in the planning phase. Where this research takes a position, it is marked **RECOMMENDATION** and an Open Question is logged in §10.

---

## 1. Executive Summary

**The single most important finding of this re-research:** Cashu NUT-DLC PR #128 — referenced throughout `.planning/research/STACK.md`, `ARCHITECTURE.md`, `PITFALLS.md`, and `SUMMARY.md` as the critical-path liquidity primitive — **was closed by maintainer `thesimplekid` on May 20, 2025 with the comment "Closing as there is no active work. Please reopen if work continues."** A successor proposal exists (PR #337 "NUTs for prediction markets" by joemphilips, opened Feb 7 2026, **architecturally different** — Conditional Token Framework / oracle-agnostic mint instead of bilateral-DLC mint). Phase 1 SPIKE-01 must validate this against the actual GitHub state and decide between three forks of the future:
- **Path A:** Adopt NUT-CTF (PR #337), rewrite HIP-3 from CTF angle, contribute upstream.
- **Path B:** Resurrect PR #128 (offer to take ownership from conduition), commit to maintaining as a fork.
- **Path C:** Ship v1 with custodial-promise mint (NO NUT-DLC), document trust assumption loudly, plan migration once protocol-level spec stabilizes.

**The single most important strategic confirmation:** the legal-foundation thesis from `PITFALLS.md` (CFTC Blockratize $1.4M Jan 3 2022, FBI raid on Coplan Nov 13 2024, Storm § 1960 conviction Aug 6 2025 with sentencing pending April 9 2026, El Salvador Bitcoin Law rollback Jan 29 2025) holds firm. The recommendation is **Switzerland Stiftung (foundation) for protocol IP + BVI/Panama operating company for mint operator**, with maintainer pseudonymity preserved for protocol-core roles and counsel engaged from one of MME / Bär & Karrer / Walkers / Anderson Legal **before any mainnet code ships**.

**Other Phase 1 conclusions (this research):**
- **NIP-88 (the foundation of HIP-1's oracle event kinds, kinds 88 + 89) is itself an unmerged Nostr PR (#919 by benthecarman, last activity Jan 2025; conduition's scope-reduced version is PR #1681).** HIP-1 must explicitly cite the NIP-88 draft state and provide a stability commitment ("Hunch tracks NIP-88 PR #1681 and will move to the merged form when ratified").
- **Hunch-chosen event kinds 30888 / 38888 / 38889 / 30890 / 30891 / 30892 are NOT in the official NIP list and are NOT reserved.** Documented in `.planning/research/ARCHITECTURE.md` but never verified against `nostrbook.dev/kinds/`. HIP-1 should reserve them via a public Nostr announce + GitHub PR to the `nips` repo registering Hunch's kind ranges, even though there is no central authority to grant reservation.
- **DLC spec convention:** the canonical reference is `github.com/discreetlogcontracts/dlcspecs` (Introduction.md + Oracle.md + Messaging.md + MultiOracle.md + NumericOutcome.md + PayoutCurve.md + v0Milestone.md). HIP-2 should declare itself a **superset for binary YES/NO/INVALID markets with multi-oracle FROST adapter-readiness**, citing dlcspecs as the canonical primitive layer. (`CITED: github.com/discreetlogcontracts/dlcspecs`)
- **FROST cryptographic stack is healthy.** `frost-secp256k1-tr` v2.2+ incorporates the Trail of Bits Feb 2024 Pedersen-DKG coefficient-vector-length fix. RFC 9591 ratified. BIP-445 (FROST for BIP-340) advancing. ChillDKG (Blockstream BIP-DKG) is the emerging async DKG primitive. 3-of-5 ceremony playbook is achievable with documented coordination via Nostr DMs (NIP-44 gift-wrapped).
- **Lightning-DLC channels are NOT v1-ready.** Atomic.finance was acquired by Lygos Finance in Aug 2025 and pivoted to Bitcoin-collateralized lending (no longer a Lightning-DLC channel product). The Crypto Garage / rust-dlc Lightning-DLC channel work from Nov 2022 was explicitly flagged "not production-ready" and there has been no production update since. SPIKE-04 deliverable is a **documented NO-GO for v1**, with research notes for v2 reconsideration. (Defers complexity per `.planning/research/PITFALLS.md` Pitfall 5 launch staging.)
- **El Salvador remains de-recommended for the operating entity** following the Jan 29 2025 Bitcoin Law rollback under IMF pressure (CITED: Reason / IMF Country Report 25/58 / `.planning/research/PITFALLS.md` Pitfall 17). El Salvador acceptable for individual contributor residency only.
- **Maintainer pseudonymity must be load-bearing for protocol-core roles** post-Storm (Aug 6 2025 conviction on § 1960(b)(1)(C); sentencing hearing scheduled Apr 9 2026; DOJ pursuing retrial on hung counts). Pre-Coplan-raid (Nov 13 2024) practices like maintaining a single public identity for both code + frontend operations are now contraindicated by primary-source precedent.
- **Nyquist validation gate for Phase 1 requires per-requirement falsification conditions** (see §7). The challenging cases are LEGAL requirements (verification = counsel sign-off documents) and HIP peer review (verification = named external reviewer attestation on Nostr). Test infrastructure for Rust spike code (SPIKE-02 / SPIKE-03) uses `cargo test` + signet faucet, runnable in <30s for quick iteration.

**Primary recommendation:** Run Phase 1 in three parallel plans (per ROADMAP.md): **(1) Protocol Specs & Repo** (HIP-0..5 + repo setup, ~3 weeks), **(2) Technical Spikes** (SPIKE-01 dependency-chain blocker for SPIKE-02; SPIKE-03 FROST in parallel; SPIKE-04 short paper deliverable, ~4 weeks total), **(3) Legal Foundation** (counsel engagement is the long-pole, ~6-8 weeks elapsed). Total Phase 1: 6-8 weeks solo as estimated in ROADMAP.md, contingent on counsel responsiveness in the legal track.

---

## Architectural Responsibility Map

Phase 1 is specs + spikes + legal, not running code. The "tiers" here are document/process tiers, not deployment tiers.

| Capability | Primary Tier | Secondary Tier | Rationale |
|------------|-------------|----------------|-----------|
| Protocol specifications (HIPs) | `docs/HIP-*.md` (Markdown source-of-truth) | Nostr long-form (NIP-23, kind 30023) for public announce | Spec lives in repo for diff-ability + on Nostr for censorship-resistance |
| Manifesto | `docs/MANIFESTO.md` | Nostr long-form + PGP-signed release | Personal voice; signed for provenance |
| Reference implementation hints in HIPs | `crates/hunch-protocol` (type sketches only in Phase 1) | (Phase 2 builds the real impl) | Phase 1 ships types compilable to validate spec; behavior is Phase 2 |
| NUT-DLC / NUT-CTF spike (SPIKE-01/02) | `crates/hunch-mint-spike` (throwaway crate) | signet (Mutinynet preferred for fast blocks) | Spike code is throwaway; real implementation in Phase 2 |
| FROST DKG spike (SPIKE-03) | `crates/hunch-oracle-spike` (throwaway) | signet + 5 separate processes simulating 5 oracles | DKG ceremony tested in isolation; production setup is Phase 2 |
| Lightning-DLC assessment (SPIKE-04) | `docs/SPIKE-04-lightning-dlc-assessment.md` | (no code — research deliverable) | No-go recommendation expected; no code needed |
| Legal entity formation | Counsel-led (external) | Foundation operator records | Counsel drives; Hunch retains decision authority |
| Terms of Service draft | `docs/legal/TERMS.md` (Markdown source-of-truth) | Counsel review + sign-off | Drafted by maintainer + counsel; final form lives in repo + frontend |
| Privacy policy | `docs/legal/PRIVACY.md` | Counsel review + sign-off | Same pattern |
| PR response playbook | `docs/legal/PR_PLAYBOOK.md` (internal — encrypted at rest pre-launch) | Encrypted GitHub repo OR offline storage | Sensitive; not committed publicly until launch |
| Maintainer pseudonymity plan | `docs/legal/PSEUDONYMITY.md` (internal) | OPSEC review (counsel + maintainer) | Sensitive; not public |
| GitHub + Radicle repo setup | GitHub (clear) + Radicle (peer-to-peer) | Codeberg (mirror, EU non-MS) | Tri-mirror from day 1 per CLAUDE.md "Don't write to GitHub Issues only" |

---

## 2. HIP Authoring

**Confidence: HIGH** (NIP conventions are well-documented and conduition's NIP-88 draft is a known precedent for DLC-on-Nostr authoring style).

### 2.1 NIP-style spec conventions (model for HIP authoring)

Hunch Improvement Proposals (HIPs) should follow the structural conventions of Nostr NIPs (which themselves modeled on Bitcoin BIPs). The reference for HIP authors is the NIPs repo at `github.com/nostr-protocol/nips` — specifically the structure of NIP-01 (basic protocol), NIP-23 (long-form content), and NIP-88 PR (DLC oracle events). (CITED: `nips.nostr.com/23`, `github.com/nostr-protocol/nips/pull/919`)

**Recommended HIP template structure:**
```markdown
# HIP-N: <Title>

**Status:** Draft | Proposed | Final | Deprecated
**Author(s):** <Nostr npub + optional pseudonym>
**Created:** <YYYY-MM-DD>
**Updated:** <YYYY-MM-DD>
**Replaces:** (optional, HIP-M reference)
**Depends on:** (optional, HIP-M reference + external specs)

## Abstract
(2-3 sentence summary of the proposal)

## Motivation
(Why this HIP exists, what problem it solves)

## Specification
(The actual normative text — what implementers MUST do)

### <Subsection>
(Concrete rules, event schemas, byte layouts as applicable)

## Rationale
(Why the design choices were made, alternatives rejected)

## Reference Implementation
(Link to `crates/hunch-protocol` types + any prototype code)

## Backwards Compatibility
(How this HIP relates to prior versions; deprecation strategy)

## Security Considerations
(Threats addressed, threats out of scope)

## Test Vectors
(For event kinds: example JSON; for cryptographic protocols: KAT-style vectors)

## References
(Links to NIPs, BIPs, RFCs, dlcspecs, Cashu NUTs, papers)
```

**Conventions to enforce:**
- All HIPs MIT-licensed in the same repo as the code.
- `Status: Draft` until at least one external reviewer attests on Nostr (kind:1 reply or kind:30023 review).
- Major changes (post-`Status: Final`) require a new HIP that supersedes the old one (mirroring NIP practice; NIP-23 calls this out for long-form content).
- HIP numbering: HIP-0 (overview + manifesto reference), HIP-1, HIP-2, ... — no gaps; reserve HIP-N+1 etc. before assigning.

### 2.2 Nostr event-kind collision check

Per the requested research: **verify the Hunch-chosen kind numbers against the current NIP registry.** ([VERIFIED: nips.nostr.com listing 2026-05-28, github.com/nostr-protocol/nips master branch as of search 2026-05-28]):

| Hunch Kind | Purpose (per ARCHITECTURE.md) | NIP-registry collision? | Confidence |
|------------|------------------------------|-------------------------|------------|
| 30888 | Market announce (replaceable) | None found in NIP master | [VERIFIED: WebSearch+nips.nostr.com 2026-05-28] |
| 38888 | Order (param replaceable per market) | None found | [VERIFIED: same] |
| 38889 | Order match | None found | [VERIFIED: same] |
| 30890 | Dispute / Challenge | None found | [VERIFIED: same] |
| 30891 | Reputation event | None found | [VERIFIED: same] |
| 30892 | Mint announce | None found | [VERIFIED: same] |
| 88 | Oracle Event Announcement | **Reserved by NIP-88 (DRAFT, unmerged PR #919)** | [CITED: github.com/nostr-protocol/nips/pull/919] |
| 89 | Oracle Attestation | **Reserved by NIP-88 (DRAFT, unmerged PR #919)** | [CITED: same] |

**Status of NIP-88 (kinds 88 + 89):** PR #919 by `benthecarman` opened circa 2024; conduition reduced scope in companion PR #1681 (Jan 2025) to "oracle announcement and attestation event kinds only" (DLC offer/encrypted-message kinds deferred). **As of 2026-05-28, neither PR is merged.** ConduIT-ion stated "I have since shifted my time towards other areas" and offered maintainership to joemphilips. (CITED: github.com/nostr-protocol/nips/pull/1681 fetched 2026-05-28)

**Recommendation:** HIP-1 should:
1. Cite NIP-88 PR #1681 as the active draft for kinds 88/89.
2. Commit Hunch to track that PR; if it merges with material changes, Hunch publishes a HIP-1a corrigendum.
3. For the 308xx / 388xx kind ranges: HIP-1 explicitly reserves them on Hunch's authority + publishes a kind reservation as a Nostr long-form event AND opens a PR to the NIPs repo registering the Hunch kind ranges (even though there is no formal NIP registry authority, the public reservation creates collision avoidance for any future protocol that searches the same way we did).
4. **Phase 1 deliverable:** open the NIPs-repo PR with the Hunch kind reservation before HIP-1 reaches `Status: Final`.

**Risk:** kinds 30888-30892 and 38888-38889 are in addressable + parameterized-replaceable ranges (30000-39999 = addressable per NIP-01) which is correct for Hunch's market/order/dispute/reputation/mint events (all author-replaceable). Kinds 88 and 89 are in the regular (non-replaceable) range which is correct for one-shot oracle announce/attestation. [VERIFIED: NIP-01 specification at github.com/nostr-protocol/nips/blob/master/01.md]

### 2.3 NIP-23 long-form publication mechanics (how HIPs publish on Nostr)

**Kind 30023** is the addressable (parameterized replaceable) Markdown long-form content event. Required tags:
- `d` (identifier — used for replaceability; recommend `hip-N` as the d-tag value)
- `title` (recommended)
- `summary` (recommended)
- `published_at` (Unix seconds — set to first publish; do NOT update on edits — use `created_at` for that)
- `t` (hashtag — recommend `hunch`, `hip`, `bitcoin`, `dlc`, `nostr` as applicable)

**Content rules per NIP-23:** Markdown only. **No hard line-breaks in paragraphs.** No HTML. Replies use NIP-22 kind 1111 comments. (CITED: `nips.nostr.com/23` fetched 2026-05-28)

**Recommendation for HIP publishing flow:**
1. Author writes `docs/HIP-N.md` (canonical source).
2. On `Status: Final` transition, run `scripts/publish-hip.ts` (Phase 1 deliverable) which:
   - Reads `docs/HIP-N.md`
   - Sets event tags as above with `d=hip-N`
   - Signs with maintainer Nostr key (pseudonymous for protocol-core HIPs; see §6 / §8)
   - Publishes to `wss://relay.hunch.markets` + 3+ community relays (`relay.damus.io`, `nos.lol`, `relay.nostr.band`)
3. Add a footer to `docs/HIP-N.md`: `Published on Nostr as <naddr1...>` (NIP-19 addressable identifier).

Drafts can also be published as kind:30024 (NIP-23 draft kind) for community comment before `Status: Final`.

### 2.4 DLC spec convention reference posture

The canonical DLC specification is at `github.com/discreetlogcontracts/dlcspecs`. Key documents Hunch HIP-2 should reference + extend:
- `Introduction.md` — what DLCs are and the bilateral assumption (CITED: dlcspecs/Introduction.md)
- `Oracle.md` — oracle announcement + attestation format (single-nonce for enumerated outcomes — what Hunch uses for binary YES/NO/INVALID) (CITED: dlcspecs/Oracle.md)
- `Messaging.md` — offer/accept/sign protocol (Hunch may wrap these in Nostr DM via NIP-44)
- `MultiOracle.md` — multi-oracle composition (where Hunch's FROST k-of-n diverges to use adapter-signatures over a threshold-Schnorr-signed attestation)
- `NumericOutcome.md` — digit-decomposition for range markets (NOT v1 scope; cite as future work in HIP-2 §Backwards Compatibility)
- `PayoutCurve.md` — payout function spec (Hunch binary uses trivial 1.0 / 0.0 / 0.5 (INVALID))
- `v0Milestone.md` — what dlcspecs considers spec-complete

**Recommended HIP-2 stance:** "Hunch DLC contracts conform to dlcspecs Oracle and Messaging primitives for the bilateral mint↔counterparty contract. The binary outcome set {YES, NO, INVALID} is an enumerated-outcome contract per dlcspecs (single nonce per oracle). Multi-oracle composition uses FROST k-of-n threshold Schnorr (RFC 9591), with the aggregated attestation signature consumable by a single adapter signature per CET — i.e., the mint sees one oracle pubkey + one signature, while the underlying federation requires k-of-n cooperation."

### 2.5 Cashu NUT spec conventions

Cashu specs follow a NUT-NN.md convention in `github.com/cashubtc/nuts`. Existing NUTs in scope for HIP-3:
- NUT-00 through NUT-15: ratified base protocol (mint, melt, swap, restore, DLEQ proofs, P2PK, etc.). HIP-3 references these as prerequisites.
- NUT-DLC (PR #128): **closed inactive 2025-05-20.** [VERIFIED: github.com/cashubtc/nuts/pull/128 fetched 2026-05-28]
- NUT-CTF (PR #337, joemphilips, opened 2026-02-07): three sub-NUTs (NUT-CTF base, NUT-CTF-split-merge, NUT-CTF-numeric). **Open, awaiting review.** Architecturally distinct from PR #128 (CTF = conditional-token-framework, oracle-agnostic mint; bilateral DLC = mint-as-counterparty). [VERIFIED: github.com/cashubtc/nuts/pull/337 fetched 2026-05-28]

**Recommended HIP-3 stance:** SPIKE-01 must decide between Path A (NUT-CTF integration), Path B (resurrect NUT-DLC), Path C (custodial-promise fallback). HIP-3 should be drafted **after** SPIKE-01 completes, not before, so the HIP can lock the chosen path. Provisionally HIP-3 = "Hunch Cashu integration profile" with the chosen path baked in.

---

## 3. NUT-DLC Deep Dive

**Confidence: MEDIUM-LOW** — primary risk surfaced in this research; the existing `.planning/research/*.md` corpus did not capture the PR #128 closure.

### 3.1 Current state (verified 2026-05-28)

**PR #128 — NUT for DLC execution (conduition):**
- Status: **CLOSED 2025-05-20** by thesimplekid: *"Closing as there is no active work. Please reopen if work continues."*
- Last review comments: November 2024 (reviewer: gudnuf).
- Depends on PR #127 (spending condition trees / NUT-10 multiplex) which was **merged** prior.
- Architecturally: mint acts as blind intermediary for **bilateral** DLCs between two parties. Aligns with `.planning/research/ARCHITECTURE.md` Pattern 3 (mint = bilateral DLC counterparty).
- 21 commits, extensive technical discussion on blinding secrets / denomination support / signature mechanisms.
- conduition's blog disclosures of design (July + November 2025) post-date the PR closure — conduition continues to think publicly about the design but has not reopened the PR.

[VERIFIED: github.com/cashubtc/nuts/pull/128 fetched via WebFetch 2026-05-28]

**PR #337 — NUTs for Prediction Markets (joemphilips):**
- Status: **OPEN, awaiting review**. Opened 2026-02-07. Author: joemphilips.
- Three sub-NUTs proposed:
  - NUT-CTF — Conditional token redemption through oracle-verified outcome keysets
  - NUT-CTF-split-merge — Operations for trading positions (i.e., atomic-swap secondary market is first-class)
  - NUT-CTF-numeric — Numeric outcome conditions with digit decomposition (defers to dlcspecs/NumericOutcome.md)
- Architecturally: mint is **oracle-agnostic**; each market outcome is a **distinct tradeable token type**. This is the Polymarket Conditional Token Framework pattern translated to Cashu primitives.
- Author quote: "PR #128 (DLC) focuses on bilateral conditional contracts between two parties; PR #337 (CTF) targets open prediction markets requiring composable positions and free entry/exit. Both can share oracle infrastructure from the DLC specification."

[VERIFIED: github.com/cashubtc/nuts/pull/337 fetched via WebFetch 2026-05-28]

### 3.2 Fork-vs-upstream-vs-pivot decision matrix

Phase 1 SPIKE-01 must produce a binary decision among three paths. Comparison:

| Criterion | Path A: NUT-CTF (PR #337) | Path B: Resurrect NUT-DLC (PR #128) | Path C: Custodial-promise fallback |
|-----------|---------------------------|---------------------------------------|--------------------------------------|
| Spec stability | OPEN, single-author, < 4 months old | CLOSED, would need re-opening + ownership claim | N/A (Hunch designs its own internal spec) |
| Architectural fit | **Strong** — purpose-built for open multi-bettor prediction markets | Conceptually fits ARCHITECTURE.md Pattern 3 but bilateral-DLC abstraction is awkward for multi-bettor | Weak — abandons cypherpunk non-custody principle during market lifetime |
| Implementation effort | Build alongside upstream; contribute back | High — adopt 21 commits' worth of work + maintain a fork | Lowest — Hunch's mint just promises to honor token redemptions, no DLC backing the mint's promise |
| Cypherpunk principles | Preserves no-custody-at-settlement (DLC fires) | Preserves no-custody-at-settlement (DLC fires) | **VIOLATES** "trust the math" principle during market lifetime; must be documented loudly per CLAUDE.md directive |
| Upstream relationship | Friendly: joemphilips is active, open to collaboration | Hostile: thesimplekid closed it; resurrection requires positive justification | N/A |
| Time to working prototype | 2-4 weeks (built on open PR) | 4-6 weeks (rebase + rebuild) | 1 week (no spec work, just mint code) |
| Risk if spec evolves | LOW if Hunch contributes upstream actively | HIGH if upstream picks a different path (PR #337 likely the upstream path) | HIGH — Hunch builds something nobody else uses |
| Audit confidence at Phase 3 | MEDIUM — depends on PR #337 stabilizing | LOW — building on closed-PR code is a red flag for auditors | LOW — custodial promise is auditable but criticized |
| External narrative | "Hunch implements Cashu prediction market NUTs" — clean story | "Hunch maintains a fork of an unmaintained Cashu NUT-DLC proposal" — fragile story | "Hunch is custodial during market lifetime — for now" — fragile story |

**RECOMMENDATION (research level — needs user confirmation):** **Path A (NUT-CTF, PR #337).** Reasons:
1. Architecturally cleaner for Hunch's permissionless-many-bettor model — `.planning/research/FEATURES.md` describes the user flow as "buy YES with sat amount, sell position via atomic swap" which is **exactly the NUT-CTF pattern** (split-merge of conditional tokens). Bilateral DLC (PR #128) requires the mint to mentally model itself as the counterparty against all bettors — which is what `.planning/research/ARCHITECTURE.md` Pattern 3 already does, but with extra abstraction overhead.
2. PR #337 is the live upstream conversation. Contributing to PR #337 puts Hunch inside the Cashu protocol design loop, which is exactly the protocol-first stance CLAUDE.md prescribes.
3. PR #128 closure (May 2025) is signal that the bilateral model didn't capture maintainer momentum.

**However, this recommendation is conditional on SPIKE-01 doing the following:**
- Read PR #337 in full, including all sub-NUT proposals.
- Contact joemphilips on Nostr / GitHub to assess: (a) is collaboration welcomed? (b) how stable is the design? (c) what's the realistic merge timeline?
- Contact conduition on Nostr to assess: (a) would they support Hunch picking up PR #128 if PR #337 stalls? (b) do they think PR #337 supersedes their work?
- Prototype an end-to-end signet flow (create market → bet → resolve → withdraw) using PR #337's primitives. If it works in 2-4 weeks, lock Path A. If it doesn't, decide Path B vs Path C.

**SPIKE-02 (working prototype) depends on SPIKE-01's path decision.** If Path A: prototype against PR #337's NUT-CTF + NUT-CTF-split-merge. If Path B: rebase PR #128 against current CDK + prototype. If Path C: write minimal Hunch-flavored "promise mint" extension to CDK + prototype.

### 3.3 Compatible Cashu mint implementations

For SPIKE-02 prototype, the Rust CDK (`cashubtc/cdk` — referenced in `.planning/research/STACK.md` as v0.14+) is the right base. The Python Nutshell (`cashubtc/nutshell`) and the cdk-axum HTTP server crate are the alternatives — but CDK Rust is the project's mandated stack per CLAUDE.md ("Rust services are critical infrastructure"). [CITED: github.com/cashubtc/cdk, .planning/research/STACK.md]

### 3.4 Signet faucet availability

For SPIKE-02 end-to-end on signet, Hunch needs signet sats. **Mutinynet (signet variant with 30-second blocks)** is the recommended Bitcoin test network per `.planning/research/STACK.md`. Faucet at `https://faucet.mutinynet.com`. [CITED: .planning/research/STACK.md] Standard signet faucets at `https://signetfaucet.com` and `https://signet.bc-2.jp` exist but the slow block time (10 min) is impractical for iterating on DLC flows.

### 3.5 Custodial-promise fallback (Path C) — what it looks like

If both Path A and Path B fail, Hunch ships v1 with a custodial-promise mint:
- Mint accepts Lightning deposits, issues YES and NO tokens **without a DLC backing them on-chain during market lifetime.**
- Mint promises (in HIP-3 + ToS) that on oracle attestation, the mint will redeem winning tokens for sats from its operator-held reserves.
- Reserves proofs published weekly per `.planning/research/REQUIREMENTS.md` MINT-10 — operator transparency is the only protection.
- Trust assumption: **the mint operator is honest during market lifetime.** Settlement still uses an oracle Schnorr attestation, but the mint pays out from its own pocket rather than from an on-chain DLC CET.
- Mainnet T0-T2 caps (per `.planning/research/PITFALLS.md` Pitfall 5 staging) are mandatory under this path because the trust assumption is much heavier.
- Migration path: when NUT-CTF or NUT-DLC stabilizes upstream, Hunch migrates with a documented hand-off; open markets at migration time are special-cased.

**Trade-off:** Path C ships fastest but violates the "trust the math" principle (CLAUDE.md). If chosen, CLAUDE.md mandates loud documentation of the trust assumption — banner on every market page; FAQ entry; HIP-3 explicit non-trust statement.

---

## 4. FROST DKG (SPIKE-03)

**Confidence: HIGH** for cryptographic state; **MEDIUM** for operational UX patterns.

### 4.1 Crate status

**`frost-secp256k1-tr`** (ZcashFoundation, Taproot-compatible FROST):
- Pinned per `.planning/research/STACK.md`: v2.2+
- **Incorporates the Trail of Bits Feb 2024 Pedersen-DKG coefficient-vector-length fix.** [CITED: blog.trailofbits.com/2024/02/20, zfnd.org Pedersen DKG remediation]
- RFC 9591 (FROST) compliant. [CITED: datatracker.ietf.org/doc/rfc9591/]
- BIP-445 (FROST for BIP-340) — advancing as of 2026-05; not yet merged into Bitcoin's BIP repo as Final. [CITED: github.com/siv2r/bip-frost-signing per `.planning/research/STACK.md`]

`frost-core` v2.x is the underlying core; ZF maintains the family of curve-specific crates.

**Recommendation:** SPIKE-03 uses `frost-secp256k1-tr` directly. No custom wrappers (per CLAUDE.md "No custom crypto"). Pin the exact version in the workspace `Cargo.toml`; verify `cargo audit` clean before SPIKE-03 begins.

### 4.2 DKG ceremony protocol

Pedersen DKG (the FROST default) is **two-round + verification**:

1. **Round 1 (per participant):**
   - Each participant generates a random polynomial of degree `t-1` where `t` = threshold (for Hunch SPIKE-03: t=3, so degree 2).
   - Commits to coefficients of polynomial: publishes a vector of length `t` (which is `t+1 - 1` since constant term + t-1 higher-order terms... actually `t` commitments for threshold `t`; **this is the load-bearing length check from Trail of Bits Feb 2024 — the vector MUST be exactly length `t`, no more, no less**).
   - Publishes signature over the commitment vector (proof of knowledge of constant term).
2. **Round 2 (per participant):**
   - Each participant evaluates their polynomial at every other participant's identifier and sends the result privately (encrypted) to that participant.
3. **Finalization (per participant):**
   - Each participant verifies all received shares against the public commitment vectors.
   - **Length validation:** for each received commitment vector, verify it has exactly `t` entries. **This is the Trail of Bits fix.** If any commitment vector deviates from `t` length, abort the DKG.
   - Combine to derive own secret share.
   - Group public key = sum of all constant-term commitments.

**Failure modes:**
- Any participant offline at round 1 or 2 → DKG aborts; restart from scratch (no partial progress preserved in classical Pedersen DKG).
- Malicious participant with wrong-length commitment vector → would silently raise threshold without fix; with Trail of Bits fix, detected at finalization and DKG aborts.
- Network partition → DKG aborts; participants need to coordinate offline.
- **ChillDKG (Blockstream's BIP-DKG)** is an emerging async-friendly Pedersen DKG variant that supports paused/resumed ceremonies. Hunch should track ChillDKG for v2; v1 SPIKE-03 uses the synchronous DKG ceremony as per `frost-secp256k1-tr` API.

[CITED: eprint.iacr.org/2020/852 (FROST paper), datatracker.ietf.org/doc/rfc9591/, github.com/BlockstreamResearch/bip-frost-dkg]

### 4.3 3-of-5 reference participant playbook

**SPIKE-03 deliverable: a written playbook (`docs/playbooks/FROST-DKG-3of5.md`) tested by running the ceremony on signet at least 3 times.** Outline:

1. **Pre-ceremony preparation** (per oracle operator):
   - Generate a long-term Nostr identity (NIP-44 capable, since coordination uses gift-wrapped DMs).
   - Provision hardware: signing happens on a dedicated machine (recommend: cold air-gapped laptop running Linux + frost-secp256k1-tr binary), share storage encrypted at rest with passphrase + hardware-backed (TPM / YubiKey).
   - Verify `frost-secp256k1-tr` version pinned ≥ 2.2 with TOB fix.
2. **Coordination phase** (one operator acts as coordinator; coordinator's role is purely transport, not authority):
   - Coordinator publishes a Nostr event (kind 30888 or new kind reserved in HIP-4) announcing DKG ceremony intent: t=3, n=5, list of participants' npubs, planned start time.
   - Each participant confirms via NIP-44 gift-wrapped DM to coordinator.
3. **Round 1 (synchronous within a 1-hour window):**
   - Each participant generates their commitments + proof, sends to coordinator via NIP-44.
   - Coordinator broadcasts each participant's commitments to all other participants (relay messages, do not modify).
   - Each participant validates each received commitment vector length = t.
4. **Round 2 (synchronous, 1-hour window):**
   - Each participant computes shares for every other participant and sends point-to-point via NIP-44 (NOT through coordinator — coordinator should not see shares).
5. **Finalization:**
   - Each participant validates shares, combines to derive secret share, computes group public key.
   - All 5 participants publish a confirmation event (kind 89 or HIP-4-defined kind) over the group public key — agreement on the group pubkey confirms the ceremony succeeded.
6. **Persistence:**
   - Each participant stores their secret share encrypted on hardware + writes the group public key + their participant identifier to a published HIP-4 reference event.
7. **Rotation / refresh:**
   - When membership changes (add/remove operator, key refresh), re-run DKG from scratch (no incremental membership change in classical Pedersen DKG — ROAST / FROST3 research areas).
   - HIP-4 should document that the group public key changes on rotation, and explain how open markets are migrated (option: open markets keep old group pubkey; new markets use new; old federation stays alive until old markets resolved).

**Signing ceremony (post-DKG, for attesting a real outcome):**
- FROST signing is **two-round** (commit phase + reveal/sign phase).
- Coordinator (any of the operators) collects commitments from k participants, broadcasts shared commitments back, collects signature shares, aggregates.
- Coordinator is **not trusted** — coordinator can withhold or reorder but cannot forge.
- **Per CLAUDE.md "Reserves proofs published weekly":** every signing ceremony's aggregate signature is published as a kind:89 attestation; the attestation is the only public artifact of the federation's existence beyond the kind:88 announcement.

### 4.4 Communication channel for DKG coordination

Per `.planning/research/ARCHITECTURE.md`: "Custom protocol on top of Nostr DMs (NIP-04/44) for DKG ceremony + sign coordination."

**Refined recommendation (this research):**
- **NIP-44 (gift-wrapped DMs)**, NOT NIP-04 (deprecated due to metadata leak). [CITED: nips.nostr.com/44]
- Hunch maintains 1-2 dedicated relays for federation coordination (`coord.hunch.markets` private relay with authenticated writes) so DKG traffic doesn't pollute public relays.
- Out-of-band confirmation (Signal / Briar / physical meeting for initial federation formation) is acceptable for the very first ceremony to bootstrap pubkey trust. SPIKE-03 documents this.

### 4.5 Failure / abort / restart procedures

- **Abort during round 1:** harmless, just re-coordinate.
- **Abort during round 2:** harmless if no participant has finalized; finalized participants need to discard shares.
- **Detection of malicious participant (wrong-length vector, invalid share, etc.):** all participants abort; identify the malicious participant from event signatures; future ceremonies exclude that npub.
- **Lost share (one operator's hardware fails post-DKG):** k-of-n means up to (n-k) operators can be lost without losing signing capability, BUT once you drop below k operators, the federation is dead. Operators must back up shares (encrypted, hardware-protected) AND document recovery procedure.

---

## 5. Lightning-DLC (SPIKE-04)

**Confidence: HIGH** for the NO-GO recommendation; **LOW** for any specific v2-timeline projection.

### 5.1 atomic.finance state

**atomic.finance was acquired by Lygos Finance in August 2025.** Lygos's product focus is **Bitcoin-collateralized lending via DLCs, NOT Lightning-DLC channels.** The original atomic.finance Lightning-DLC channel research from 2021-2023 has not been productized.

[CITED: blockspace.media — Lygos Finance acquires Atomic Finance 2025; atomic.finance/blog/an-atomic-pivot]

### 5.2 cara / Crypto Garage / rust-dlc Lightning-DLC

"cara" appears to be a misremembering of either the Crypto Garage Lightning-DLC work or another internal project name. The actual work is:

- **Crypto Garage / Tibo Le Guilly's rust-dlc fork** with LDK fork supporting Lightning channel splitting. November 21, 2022: first mainnet DLC embedded in Lightning channel executed.
- Self-disclosed status (from the Crypto Garage Medium post): **"this is not production-ready... very unstable and using it with mainnet coins is very likely to lead to loss of funds."**
- No subsequent production milestone update has been published (search 2026-05-28).

[CITED: medium.com/crypto-garage/dlc-on-lightning, github.com/cryptogarageinc/rust-dlc, tftc.io/issue-1286]

### 5.3 Go/no-go decision matrix

| Criterion | Status | Verdict |
|-----------|--------|---------|
| Production-ready software | **NO** (atomic.finance pivoted away; Crypto Garage explicitly flagged unstable) | NO-GO |
| Active maintainer commitment | NO (atomic.finance team focused on Lygos lending; Crypto Garage's last public update was ~2022) | NO-GO |
| LDK upstream support | NO (Crypto Garage maintains a fork; channel-splitting isn't merged into mainline LDK) | NO-GO |
| Audit/security record | NO (no public audit of Lightning-DLC channel construction) | NO-GO |
| Hunch architectural fit | YES if it existed (would enable instant-settle markets per `.planning/research/REQUIREMENTS.md` v2 item "Lightning DLC channels for instant settle") | But irrelevant — no implementation to fit against |

**SPIKE-04 deliverable: `docs/SPIKE-04-lightning-dlc-assessment.md`** — short paper (3-5 pages) documenting:
1. State of atomic.finance (acquired, pivoted).
2. State of Crypto Garage rust-dlc + LDK fork (2022 milestone, unmaintained since).
3. The architectural shape Lightning-DLC would take if shipped (channel splitting via glue + split transactions per Crypto Garage's design).
4. **NO-GO recommendation for v1.** On-chain DLCs (per `.planning/research/ARCHITECTURE.md` Pattern 2) are the v1 settlement primitive.
5. v2 reconsideration triggers: (a) atomic.finance / Lygos publishes a Lightning-DLC product, or (b) LDK mainline adds channel-splitting support, or (c) BOLT spec adds DLC channels, or (d) an independent implementation reaches production with audit signoff.
6. References: every URL cited above.

---

## 6. Legal Foundation (LEGAL-01..06)

**Confidence: HIGH** — anchored on the primary-source-verified content in `.planning/research/PITFALLS.md` (CFTC Docket 22-09, 18 USC § 1960, IMF CR 25/58, Polymarket Amended Order Nov 25 2025). This research re-confirmed those sources are still authoritative.

### 6.1 Jurisdiction comparison

| Jurisdiction | Structure | Setup cost (USD) | Annual cost (USD) | Counsel availability | Banking | Reputation | US treaty exposure | Recommendation |
|--------------|-----------|------------------|-------------------|----------------------|---------|------------|--------------------|----------------|
| **Switzerland (Stiftung)** | Foundation | 20-40K | 10-25K | Excellent (MME, Bär & Karrer, Kellerhals Carrard) | Strong (Sygnum, SEBA, Bitcoin-friendly cantonal banks Zug/Zurich) | High — Crypto Valley credibility | Moderate (DTAA exists; FATCA applies; FINMA technology-neutral) | **RECOMMENDED for foundation/IP holding** |
| **BVI (Foundation or BVI BC)** | Foundation or Business Company | 5-15K | 5-12K | Strong (Walkers, Maples, Ogier) | Limited (most major banks pulled out; small private banks remain; Lightning-friendly options near zero) | Moderate (offshore reputation; FATF gray list 2023, off in 2024) | LOW — no MLAT with US for civil matters, weaker info-exchange | **STRONG alternative for operating mint entity** |
| **Panama (Foundation of Private Interest)** | Foundation | 3-8K | 3-6K | Adequate (Mossack Fonseca legacy taint, prefer Morgan & Morgan, ICAZA) | Moderate (US correspondent-bank-heavy; KYC required at bank not foundation level) | Mixed — Panama Papers stigma still active in 2026 | LOW | Acceptable alternative if BVI banking issues block |
| **Liechtenstein (Stiftung)** | Foundation | 25-50K | 15-30K | Good (Marxer, Walch & Schurti) | Strong (LGT, VP Bank, Bank Frick — explicitly crypto-friendly per TVTG) | High — TVTG (Blockchain Act 2020) explicitly recognizes token issuance | Moderate (close to Switzerland in posture) | **Excellent secondary option** if Swiss costs too high |
| **El Salvador** | LLC / Bitcoin Service Provider | 5-15K | 5-10K | Limited (Lemon Legal, but ecosystem thin) | Difficult since Jan 2025 IMF rollback | **DOWNGRADED** since Bitcoin Law rollback Jan 29 2025 | LOW | **NOT RECOMMENDED for operating entity** per `.planning/research/PITFALLS.md` Pitfall 17 (CITED) |
| **Cayman (Foundation Company)** | Foundation Company | 15-30K | 10-20K | Strong (Walkers, Maples) | Moderate | High in crypto (many large DeFi projects domiciled here) | Moderate | Backup; less Bitcoin-specific than BVI for this use case |

[VERIFIED for all rows: `.planning/research/PITFALLS.md` Pitfalls 1-4 + 17 primary sources; some cost numbers are 2025 market rates from counsel pricing sheets [ASSUMED]]

**RECOMMENDATION (research level — needs user confirmation in /gsd-discuss-phase or counsel input):**

**Two-entity structure:**
- **Hunch Foundation** = Swiss Stiftung. Holds protocol IP, signs HIPs, publishes manifesto, holds defense-fund treasury (foundation never operates a service that could trigger MSB exposure).
- **Hunch Mint Operator Co.** = BVI BC (or Liechtenstein if BVI banking proves too thin for Lightning operations). Operates the reference Cashu mint + oracle + relay. Thin balance sheet. Different legal name + branding to make protocol-vs-operator distinction obvious.

This mirrors the model `.planning/research/PITFALLS.md` Pitfall 4 prescribes ("Mint operator has separate legal entity from foundation").

### 6.2 Counsel shortlist

**Foundation jurisdiction counsel (Switzerland):**
- **MME (Zurich)** — Luka Müller-Studer is the named senior crypto partner; MME has structured several major DeFi / DAO projects.
- **Bär & Karrer (Zurich)** — top tier, broader corporate; Stéphanie Hodara-El Bez heads digital assets.
- **Kellerhals Carrard (Zug)** — Crypto Valley local; explicitly Bitcoin-friendly historical track record.

**Operating-entity counsel (BVI):**
- **Walkers (BVI office)** — Lucy Frew leads structured / fintech.
- **Maples and Calder (BVI)** — alternate strong choice.
- **Ogier (BVI)** — third-best fallback.

**Operating-entity counsel (Liechtenstein, if BVI fails):**
- **Marxer & Partner** — drafted parts of TVTG.
- **Walch & Schurti** — strong crypto bench.

**US-facing criminal defense bench (reserve relationship, do NOT engage proactively but have on speed-dial):**
- **DeFi Education Fund** legal team (filed amicus in U.S. v. Storm) — Amanda Tuminelli.
- **Coin Center** — Peter Van Valkenburgh.
- **EFF** — Andrew Crocker (Bernstein-style code-as-speech defense).
- **Anderson Kill (NYC)** — Preston Byrne or Stephen Palley for civil regulatory matters (Anderson Kill has done extensive crypto work; both are non-employees as of 2025 but still operate in the space; verify current firm affiliations during counsel-engagement Phase 1).
- **Empire Litigation (Manhattan)** — adversarial proceedings (sub-spec).

[Counsel name attribution VERIFIED for MME (PITFALLS.md), DeFi Education Fund (US v. Storm timeline), Coin Center, EFF (US v. Storm amicus filings); ASSUMED for specific partner names in the firms — re-verify during outreach as personnel may have moved].

**Engagement budget:** $30-80K for structuring opinion + entity formation (per `.planning/research/PITFALLS.md` Pitfall 1). Reserve additional $50-150K for ongoing counsel through Phase 2-3.

### 6.3 Terms of Service template references

Template models for crypto-native non-custodial protocols:

1. **Tornado Cash pre-OFAC ToS (archive.org)** — bare-minimum protocol-not-product framing. Lesson: even with this, Storm was convicted under § 1960(b)(1)(C). ToS is necessary but not sufficient.
2. **Uniswap Labs frontend ToS** — explicit US-targeted geo-block + sanctioned-jurisdiction list + "you assume all responsibility for your transactions" + "we do not custody funds" language. Most copy-able structure.
3. **dYdX Foundation (Zug) operator ToS** — foundation operator vs protocol distinction. Tightens jurisdictional language.
4. **Robosats ToS** — purest cypherpunk model. Short. Anonymous. Tor-aware.
5. **Mostro ToS** — Nostr-native distribution; references protocol neutrality.

**Drafting principles for Hunch ToS (research-level recommendation):**
1. **Prohibit US users explicitly.** Geo-block at the frontend layer (IP-based + Tor exit list); ToS gate at first visit with "are you in the US? are you a US person?" attestation.
2. **Prohibit EU strict-regime users** (France, Germany, Italy, Spain, Netherlands, Belgium, Portugal) per `.planning/research/PITFALLS.md` Pitfall 3 (MiCA + national gambling regulators).
3. **Prohibit illegal-use markets.** Establishes constructive notice that operator does NOT consent to illegal/sanctioned use (per `.planning/research/PITFALLS.md` Pitfall 2 mitigation #3).
4. **No investment advice disclaimer.** Hunch is a protocol; bets are not investments.
5. **Tax disclaimer.** Users responsible for their own tax compliance; no reporting.
6. **No warranty / limitation of liability.** Standard.
7. **Forum selection.** Specify counsel's recommended jurisdiction (BVI court for operator co.; Swiss court for foundation matters).
8. **Acknowledgment that the protocol is open-source and Hunch operates only one frontend.** Critical for the "no operator to sue" defense per `.planning/research/PITFALLS.md` Pitfall 19.

ToS deliverable for Phase 1: `docs/legal/TERMS.md` draft, sent to counsel for sign-off. **Counsel sign-off is a falsification condition for LEGAL-03** (see §7).

### 6.4 Privacy policy minimums

Even though Hunch collects no PII beyond Nostr pubkey:
- GDPR Art. 5 still applies for any EU user reachable (even if geo-blocked, residual access).
- Cookie policy: **no cookies on the protocol layer** (per CLAUDE.md "no telemetry"); session storage only.
- Data minimization: Nostr pubkey is the only stored identifier; no IP logs beyond what infrastructure forces (Cloudflare logs are a separate operator-level concern, not user-PII).
- No analytics. No fingerprinting. (Per CLAUDE.md.)

Draft `docs/legal/PRIVACY.md` for LEGAL-04. Counsel sign-off.

### 6.5 PR response playbook (LEGAL-05)

Per `.planning/research/PITFALLS.md` Pitfall 10 mitigation #6 + Pitfall 1 mitigation #5: drafted **before launch**. Contents:

1. **Triggers** — when to activate the playbook:
   - Mainstream press inquiry on abuse market (Augur replay)
   - Subpoena to any Hunch infrastructure provider (Cloudflare, Hetzner, GitHub, registrar)
   - CFTC inquiry letter to foundation
   - FBI visit to any maintainer (Coplan precedent — happened without prior charge)
   - Sister-protocol disaster (Cashu mint rug, oracle attack at competitor)
2. **Communication channels:**
   - Primary: Nostr (kind:1 from foundation pubkey + kind:30023 long-form for detailed statements)
   - Mirror: GitHub Discussions, Radicle issues, Codeberg
   - **NOT** Twitter as primary (deplatform risk); use only as mirror
3. **Spokesperson assignment:**
   - For protocol matters: pseudonymous protocol maintainer
   - For operator matters: foundation-named officer (counsel-suggested non-US resident)
   - For legal matters: counsel only
4. **Statement templates** (5-7 pre-drafted):
   - "Abuse market hidden on official frontend" — emphasizes protocol-not-curator
   - "Regulatory inquiry received" — neutral acknowledgment, counsel engagement, no admission
   - "Mint reserves anomaly investigation" — community signal, defensive forced-close, public post-mortem timeline
   - "Oracle attestation disputed" — INVALID outcome triggered, dispute window engaged
   - "Maintainer indicted / detained" — defense-fund activation, EFF / Coin Center outreach, "the protocol survives any single operator" framing
   - "App Store deplatform" — PWA fallback already operational, expected outcome per Damus precedent
   - "Frontend deplatform" — Tor + IPFS + Radicle mirrors active, recovery-from-zero drill verified
5. **Internal coordination:**
   - 30-minute initial response on Nostr (any kind: acknowledgment, more to come)
   - 24-hour detailed statement after counsel review
   - 7-day post-mortem on Nostr long-form
6. **Defense-fund activation triggers** — written-in-advance criteria for releasing reserve sats to defense / re-hosting / community continuity.

Deliverable for Phase 1: `docs/legal/PR_PLAYBOOK.md` (committed to a counsel-controlled private repo OR encrypted in main repo; not public pre-launch — exposing the playbook reveals strategy).

### 6.6 Maintainer pseudonymity plan (LEGAL-06)

Lessons from U.S. v. Storm (Aug 6 2025 conviction; pre-Coplan FBI raid Nov 13 2024):

| Storm's mistakes | Coplan's mistakes | Hunch's response |
|------------------|-------------------|------------------|
| Public identity, US-resident | Public identity, US-resident, NYC apartment | Pseudonymous protocol-core maintainers, non-US residency |
| Operated relayer (took fees) | Operated centralized matcher (Polymarket Inc.) | Hunch maintainer takes no per-trade fees; protocol revenue goes to foundation as IP licensing, not transaction fees |
| Dictated frontend (single product) | Single canonical product | Protocol-first; multi-frontend explicitly encouraged; reference frontend is one of N |
| Half-screened (optional compliance) | Geo-blocked but otherwise unrestricted | No screening (per CLAUDE.md cypherpunk principles); all-or-nothing |
| GitHub repos in his own name | Polymarket Inc as visible operator | Hunch repos under foundation account; protocol-core maintainers pseudonymous |

**Specific recommendations for HIP-LEGAL-06 (`docs/legal/PSEUDONYMITY.md`):**
1. **Protocol-core roles are pseudonymous.** Lead protocol maintainer (writes HIPs, controls main repo) operates under a Nym (call it "satoshi-style"). Public Nostr pubkey, no doxxing of legal identity.
2. **Frontend operator role can be public** because frontend is just one of many; deplatforming the frontend operator doesn't kill the protocol.
3. **Foundation officer (Stiftung president) can be public** — must be public for entity formation. Should be a counsel-recommended non-US resident with no other crypto-prosecution exposure.
4. **Communications hygiene:**
   - Nostr key never used from a device that also reads US-IP-source email or accesses US banking
   - No travel to US, UK (extradition partner), or jurisdictions with active MLATs with the US for crypto matters
   - PGP-signed git commits under pseudonym
   - Voice / video appearances avoided (voiceprint biometrics)
   - Code review of all protocol-core PRs by pseudonym
5. **Bus-factor without doxxing:**
   - Pseudonymous maintainer's keys backed up with multi-sig held by counsel + foundation officers
   - Pre-written hand-off statement triggered if pseudonym goes silent for >30 days
   - Successor pseudonym can be announced via counter-signed event from foundation pubkey
6. **What pseudonymity does NOT protect against:**
   - Cooperating service providers (Cloudflare logs, ISP, hardware vendor)
   - § 1960(b)(1)(C) knowledge element if the pseudonym is shown to have transmitted criminal proceeds (Storm precedent — knowledge attaches to the pseudonym's actions, not the underlying legal person)
   - Travel by the legal person under their real name (US can request extradition based on circumstantial evidence linking pseudonym to person)
   - Operational mistakes (re-use of an SSH key, IP address leak, etc.)

`docs/legal/PSEUDONYMITY.md` should NOT be committed publicly with operational specifics (specifics aid deanonymization). Public version = principles; private version (counsel + maintainer only) = operational details.

---

## 7. Validation Architecture (Nyquist gate)

Per `.planning/config.json` workflow.nyquist_validation = true, Phase 1 must declare per-requirement falsification conditions. Phase 1 is unusual because it spans three radically different deliverable types:
- **HIPs** = Markdown specs → verified by external peer review
- **Repo / CLAUDE.md** = files in git → verified by file existence + content checks
- **Spikes** = code prototypes → verified by automated test on signet
- **Legal** = counsel sign-off → verified by counsel-signed document

### 7.1 Test framework

| Property | Value |
|----------|-------|
| Framework (spike code) | `cargo test` for Rust spikes (SPIKE-02, SPIKE-03) |
| Config file | `crates/hunch-mint-spike/Cargo.toml` + `crates/hunch-oracle-spike/Cargo.toml` (created Wave 0) |
| Quick run command (per spike crate) | `cargo test -p hunch-mint-spike --tests` |
| Full suite command | `cargo test --workspace` at repo root |
| Spec verification | Manual peer review (kind:1 reply on Nostr by named external reviewer); falsified if no review received within 4 weeks of `Status: Final` |
| Legal verification | Counsel-signed PDF stored in `docs/legal/signoff/` (private branch or encrypted at rest); falsified if no signed PDF exists |
| Repo verification | `scripts/verify-repo.sh` (Wave 0 deliverable) — checks LICENSE file MIT, CONTRIBUTING.md exists, CODE_OF_CONDUCT.md exists, Radicle mirror reachable, GitHub repo public |

### 7.2 Phase requirements → test map

| Req ID | Behavior | Test Type | Automated Command / Falsification Condition | File Exists? |
|--------|----------|-----------|--------------------------------------------|--------------|
| PROTO-01 | HIP-0 published in repo + on Nostr long-form | manual-only (peer review) | `docs/HIP-0.md` exists, Markdown lints clean (`mdl docs/HIP-0.md`), naddr1 link in footer, ≥1 external review on Nostr within 4 weeks | ❌ Wave 0 (script to lint + check naddr) |
| PROTO-02 | HIP-1 (Nostr event kinds spec) | manual + automated kind-collision check | `docs/HIP-1.md` exists; `scripts/verify-kind-collisions.ts` checks Hunch kinds against `nostrbook.dev/kinds/` registry | ❌ Wave 0 |
| PROTO-03 | HIP-2 (DLC contract structure spec) | manual peer review (Bitcoin / DLC expert) | `docs/HIP-2.md` exists; ≥1 reviewer with dlcspecs / rust-dlc history reviews on Nostr | ❌ Wave 0 |
| PROTO-04 | HIP-3 (Cashu NUT-DLC / NUT-CTF integration spec) | manual peer review (Cashu expert: Calle, conduition, joemphilips, Gandlaf) | `docs/HIP-3.md` exists; ≥1 reviewer from the Cashu maintainer set reviews on Nostr | ❌ Wave 0 |
| PROTO-05 | HIP-4 (Multi-oracle FROST protocol) | manual peer review (cryptographer: Tim Ruffing, Jonas Nick, TOB crypto) | `docs/HIP-4.md` exists; ≥1 cryptographer reviews | ❌ Wave 0 |
| PROTO-06 | HIP-5 (Reputation event format) | manual peer review (Nostr / social-graph expert) | `docs/HIP-5.md` exists; ≥1 reviewer with Nostr-protocol-design history reviews | ❌ Wave 0 |
| PROTO-07 | Repo published GitHub + Radicle, MIT, CONTRIBUTING.md, CoC | automated | `bash scripts/verify-repo.sh` (Wave 0): checks LICENSE exists + is MIT + has correct copyright + CONTRIBUTING.md exists + CODE_OF_CONDUCT.md exists + Radicle remote URL reachable | ❌ Wave 0 |
| PROTO-08 | CLAUDE.md generated for GSD workflow | automated | `test -f CLAUDE.md && grep -q "GSD" CLAUDE.md` | ✅ (already exists, verified 2026-05-28) |
| SPIKE-01 | NUT-DLC current state validated, fork-vs-upstream decision | manual document review | `docs/SPIKE-01-nut-dlc-decision.md` exists with decision Path A/B/C + rationale + maintainer-contact transcripts | ❌ Wave 0 |
| SPIKE-02 | NUT-DLC working prototype on signet (full flow) | automated end-to-end on signet | `cargo test -p hunch-mint-spike --test e2e_signet -- --nocapture` runs the create→bet→resolve→withdraw flow against Mutinynet and asserts success. Falsified if test fails or skipped. | ❌ Wave 0 (signet test harness) |
| SPIKE-03 | FROST 3-of-5 DKG ceremony on signet + playbook | automated DKG + manual playbook completion | `cargo test -p hunch-oracle-spike --test frost_dkg_3of5` runs the DKG ceremony with 5 simulated participants + asserts group pubkey agreement + signs a test attestation. Plus `docs/playbooks/FROST-DKG-3of5.md` exists and was executed 3+ times on signet (verification: log of ceremony attempts in the playbook). | ❌ Wave 0 |
| SPIKE-04 | Lightning DLC channel readiness assessment | manual document review | `docs/SPIKE-04-lightning-dlc-assessment.md` exists with NO-GO recommendation + v2 reconsideration triggers + cited primary sources | ❌ Wave 0 |
| LEGAL-01 | Offshore entity recommendation written | manual document review | `docs/legal/JURISDICTION-DECISION.md` exists + counsel-attached PDF in `docs/legal/signoff/` | ❌ Wave 0 (private or encrypted branch) |
| LEGAL-02 | Crypto-specialized counsel engaged | manual document | engagement letter signed (PDF in `docs/legal/signoff/`) | ❌ Wave 0 |
| LEGAL-03 | ToS drafted with counsel sign-off | manual document | `docs/legal/TERMS.md` exists + counsel sign-off PDF in `docs/legal/signoff/` | ❌ Wave 0 |
| LEGAL-04 | Privacy policy drafted | manual document | `docs/legal/PRIVACY.md` exists + counsel sign-off PDF | ❌ Wave 0 |
| LEGAL-05 | PR response playbook ready | manual document | `docs/legal/PR_PLAYBOOK.md` exists (private or encrypted) | ❌ Wave 0 |
| LEGAL-06 | Maintainer pseudonymity plan | manual document | `docs/legal/PSEUDONYMITY.md` (private) exists; public version in `docs/legal/PSEUDONYMITY-public.md` summarizes principles | ❌ Wave 0 |

### 7.3 Sampling rate

- **Per task commit:** lint check on the touched HIP `mdl docs/HIP-N.md`; for spike code, `cargo test -p <crate> --tests` (target <30s).
- **Per wave merge:** full repo verification `bash scripts/verify-repo.sh && cargo test --workspace && bash scripts/verify-hips.sh` (target <2 min).
- **Phase gate (`/gsd-verify-work`):** all 18 requirements pass falsification conditions; Nyquist verifier reports green.

### 7.4 Wave 0 gaps (must exist before Phase 1 plans execute)

- [ ] `scripts/verify-repo.sh` — checks LICENSE / CONTRIBUTING / CODE_OF_CONDUCT / Radicle remote
- [ ] `scripts/verify-kind-collisions.ts` — checks Hunch kinds vs nostrbook.dev/kinds + nips.nostr.com
- [ ] `scripts/verify-hips.sh` — for each `docs/HIP-N.md`: lint + Markdown structure + frontmatter check
- [ ] `scripts/publish-hip.ts` — publishes HIP-N as kind 30023 Nostr long-form
- [ ] `crates/hunch-mint-spike/Cargo.toml` + skeleton + `tests/e2e_signet.rs` test harness against Mutinynet
- [ ] `crates/hunch-oracle-spike/Cargo.toml` + skeleton + `tests/frost_dkg_3of5.rs` test harness
- [ ] `docs/legal/signoff/.gitignore` (commit empty directory; PDFs in it are NOT committed publicly; or commit to a private branch)
- [ ] CI workflow (`.github/workflows/verify.yml`) that runs `scripts/verify-repo.sh` + `cargo test --workspace` on every push

---

## 8. External Operator Risk Profile (informs LEGAL-05 + LEGAL-06)

**Confidence: HIGH** — content drawn from `.planning/research/PITFALLS.md` primary sources, re-confirmed in this research session.

### 8.1 Precedents (recent, defensible postures)

| Event | Date | Outcome | Defensible posture for Hunch |
|-------|------|---------|------------------------------|
| Polymarket FBI raid (Coplan) | Nov 13, 2024 | No prior charge; phone + electronics seized; eventually QCX acquisition + Amended Order Nov 25 2025 = compliance path | Don't be a public US-resident operator; geo-block US; no US marketing |
| Roman Storm conviction (Tornado Cash) | Aug 6, 2025 | Conviction § 1960(b)(1)(C); sentencing pending Apr 9 2026; DOJ retrial on hung counts | Maintainer non-US residency; pseudonymity; no half-KYC (paradox: half-screening was used against Storm); aggressive anti-abuse ToS |
| Samourai Wallet charges | Apr 24, 2024 | Founder Keonne Rodriguez + Hill arrested in PT (extradition pending) | Don't operate from a country with active US MLAT for crypto matters; CH / BVI / Panama / Liechtenstein have weaker MLATs |
| El Salvador Bitcoin Law rollback | Jan 29, 2025 | Bitcoin no longer mandatory; IMF-pressured | El Salvador NOT a safe jurisdiction; per `.planning/research/PITFALLS.md` Pitfall 17 |
| Polymarket UMA oracle attack | Mar 24-25, 2025 | $7M settled to attacker; refused refunds | Oracle reputation must be social, not staked; multi-oracle FROST for high-value; INVALID outcome built-in |
| PredictIt no-action rescission + 5th Cir. injunction | Aug 2022 → Jul 2023 → Jul 2025 | Eventually amended no-action letter issued | Don't rely on US regulatory grace; build for "no operator to sue" defense |
| Bitcoin Magazine Print discontinued | 2024 | Not relevant to Hunch directly; signals cypherpunk media consolidation | N/A; verify if listed in user objective — probably not Phase-1-relevant |

[VERIFIED: all rows except "Bitcoin Magazine Print" which is mentioned in user objective but not load-bearing for Phase 1 deliverables.]

### 8.2 Geo-block enforcement effectiveness — is it CFTC-defensible?

The CFTC Blockratize order (Jan 3, 2022) and Coplan FBI raid (Nov 13, 2024) provide the test: Polymarket geo-blocked US starting after their 2022 settlement, but **continued to allow US users via VPN / non-US-IP access** (or at least, prosecutors alleged so). The Coplan raid was investigating whether Polymarket violated the 2022 settlement.

**Implication:** geo-block alone is NOT sufficient if the operator (a) markets to US users implicitly via English-language site, presence at US conferences, US-targeted press, or (b) tolerates trivial circumvention (VPNs known to be used by US users).

**Layered defense for Hunch (CFTC-defensible posture):**
1. **IP-based geo-block** — Cloudflare GeoIP at frontend layer (block US IPs).
2. **Tor exit-list block** — block known US-located Tor exits; allow non-US Tor exits.
3. **ToS attestation at first visit** — user must affirm not-in-US, not-US-person; pseudonymous but contemporaneous.
4. **No US-targeted marketing** — no English-language US ad spend, no US conference talks framing Hunch as a tradable product (research talks framing it as a *protocol* are defensible).
5. **VPN-friendly accessibility on principle but not VPN-incentivized** — frontend works over Tor or non-US VPN if the user does the work themselves; Hunch does not advertise "use a VPN to access from the US."
6. **Document the defensible posture rigorously** — counsel signs off on the geo-block configuration as best-effort + reasonable.

CFTC posture vs DOJ posture is different: CFTC = civil enforcement (Blockratize $1.4M was civil); DOJ = criminal (Storm § 1960 was criminal). Geo-block helps with civil; criminal § 1960(b)(1)(C) requires the knowledge element to attach, which is harder to establish if Hunch never had US presence in the first place.

**Conclusion:** geo-block + ToS + no-US-marketing + non-US-operator-residency is a defensible posture against CFTC enforcement and reduces (but does not eliminate) § 1960 exposure. The "no operator to sue" structural posture (foundation in CH, no US persons in operator roles) is what closes the residual gap.

---

## 9. Don't Hand-Roll

Phase 1 has limited code, but the discipline matters for what little there is:

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| FROST DKG | Custom DKG protocol | `frost-secp256k1-tr` v2.2+ | Per CLAUDE.md "No custom crypto" + Trail of Bits Feb 2024 disclosure history |
| Nostr event signing | Custom signer | `nostr-sdk` (rust-nostr) v0.34+ for Rust spikes; NDK for any TS scripts | Battle-tested NIP-01 implementation |
| Cashu blind signatures | Custom blind-sig math | `cdk` v0.14+ | Per CLAUDE.md mandate |
| DLC contract construction | Custom CET enumeration | `rust-dlc` + `ddk` for spike (alpha — pin version) | dlcspecs reference impl |
| Markdown HIP linting | Custom Markdown parser | `mdl` or `markdownlint-cli` | Standard tooling |
| HIP publishing to Nostr | Custom relay client | `nostr-sdk` (Rust) or `@nostr-dev-kit/ndk` (TS) for `scripts/publish-hip.ts` | Standard relay management |
| Counsel-signed PDF storage | Custom encryption | `age` (modern alternative to GPG) for PDFs in `docs/legal/signoff/` if committing publicly; OR private branch | Standard age-encryption tooling |
| Kind-collision checking | Custom NIP registry scraper | Fetch `https://github.com/nostr-protocol/nips/blob/master/README.md` + `https://nostrbook.dev/kinds/` for the check | No central authority but multiple sources to cross-check |

---

## 10. Open Questions (planner should flag for user decision)

These need user input or counsel input before Phase 1 plans lock. Items marked **DECISION REQUIRED** block specific plans from progressing past Wave 0.

1. **DECISION REQUIRED: NUT-DLC strategy (Path A / B / C).**
   - What we know: PR #128 closed May 2025. PR #337 (NUT-CTF) is open Feb 2026, architecturally distinct.
   - What's unclear: would joemphilips welcome Hunch as a major contributor to PR #337? would conduition support a Hunch revival of PR #128? what's the realistic merge timeline for either?
   - Recommendation: SPIKE-01 contacts joemphilips + conduition + Calle, prototypes against PR #337 (Path A), reports back in 2 weeks. If Path A is non-viable, fall back to Path B or C with documented rationale.
   - Risk: this is the largest unknown in Phase 1; could expand SPIKE-01 from 1 week to 4+ weeks if upstream conversation goes badly.

2. **DECISION REQUIRED: Offshore jurisdiction final selection.**
   - What we know: Switzerland (Stiftung) for foundation/IP + BVI/Liechtenstein for operating mint co. is the research-level recommendation.
   - What's unclear: counsel-specific advice; banking accessibility for BVI Lightning operations in 2026; tax-treaty implications for individual contributors' residency.
   - Recommendation: engage MME (CH) + Walkers (BVI) in parallel during Phase 1; first deliverable is jurisdiction-decision opinion at week 4.
   - Risk: counsel-pace is the long-pole; 6-8 week Phase 1 may extend if counsel takes longer.

3. **DECISION REQUIRED: Maintainer pseudonymity scope.**
   - What we know: pseudonymity for protocol-core roles is structurally required post-Storm.
   - What's unclear: who is the "protocol-core maintainer" — current user (Silex) or a new pseudonymous identity? Does the current user accept the operational hygiene cost (no US travel, voiceprint avoidance, OPSEC) or do they want a separation-of-identities approach?
   - Recommendation: this is a personal decision and must be made by the user, counsel-informed. Cannot be decided by research.
   - Risk: pseudonymity decisions affect every public-facing artifact in Phase 1 (HIPs, repo, manifesto).

4. **Question: HIP-3 path decision dependency on SPIKE-01.**
   - HIP-3 (Cashu NUT-DLC integration spec) cannot be written until SPIKE-01 decides Path A / B / C. Should HIP-3 be deferred to Phase 1 Week 5+? Or written in parallel with two paths and pruned?
   - Recommendation: defer HIP-3 to after SPIKE-01 decision. Plan accordingly.

5. **Question: NIP-88 reservation strategy for kinds 88/89.**
   - HIP-1 references kinds 88/89 from NIP-88 PR #1681 (unmerged). What if PR #1681 stalls?
   - Recommendation: HIP-1 §Backwards Compatibility commits Hunch to track + adopt the merged form; if PR #1681 stalls for 12 months, Hunch should consider taking maintainership (with conduition's blessing — they offered it to joemphilips already, suggesting they're open).

6. **Question: HIP authoring voice / style.**
   - Should HIPs be in the "neutral protocol-spec" voice (NIP-style) or have a more cypherpunk-manifesto voice in HIP-0?
   - Recommendation: HIP-0 (overview + manifesto) = manifesto voice. HIP-1..5 = neutral spec voice.

7. **Question: Repo monorepo bootstrap.**
   - Current `C:\Users\Silex\Hunch\` directory contains the `.planning/` tree but no `Cargo.toml`, no `apps/`, no `docs/HIP-*.md`. Phase 1 PROTO-07 deliverable requires bootstrapping the actual repo structure described in `.planning/research/ARCHITECTURE.md`. Should this be Wave 0 or Plan 1's first task?
   - Recommendation: Wave 0. The repo skeleton (Cargo.toml workspace, apps/ scaffold, docs/ scaffold, LICENSE, CONTRIBUTING.md, CODE_OF_CONDUCT.md, .github/, .gitignore) is prerequisite to everything else.

8. **Question: Radicle setup timing.**
   - Radicle requires running a node + initial seed configuration. Is this a Phase 1 setup task or a Phase 2 deferred task?
   - Recommendation: Phase 1 (per ROADMAP.md PROTO-07 says "Repo published GitHub + Radicle mirror"). But Radicle setup is operationally non-trivial; allocate 1-2 days inside Plan 1.

9. **Question: Mutinynet vs vanilla signet.**
   - For SPIKE-02 / SPIKE-03 signet testing, Mutinynet (30s blocks) is much faster but is a signet variant. Are spike results on Mutinynet acceptable as evidence for Phase 1 success criterion #2 ("on Bitcoin signet, end-to-end")?
   - Recommendation: YES — Mutinynet is signet for the purposes of the spec. Document the choice in spike reports.

10. **Question: Audit firm shortlisting timing.**
    - `.planning/research/PITFALLS.md` Pitfall 5 names a Phase 3 audit firm shortlist: Trail of Bits Bitcoin team, Block Digital Contracting, Cure53, Quarkslab, NCC Group, Galaxy Audit. Should Phase 1 already begin outreach for Phase 3 audit slots (which have 8-14 week lead times)?
    - Recommendation: YES, low-priority parallel task — send initial inquiry emails to top 2-3 firms in Phase 1 Week 4-5 to lock Q4 2026 audit slot. Not in scope for the 18 declared requirements but a Phase-1-week-7 prudent activity.

---

## 11. Standard Stack (Phase 1 subset)

Pulled from `.planning/research/STACK.md`, narrowed to what Phase 1 actually uses. Version check performed 2026-05-28 where feasible.

### Core (Rust, for spikes)

| Library | Version (per STACK.md) | Purpose in Phase 1 | Verification |
|---------|------------------------|--------------------|--------------|
| `cdk` | 0.14+ | SPIKE-02 NUT-DLC prototype mint base | [CITED: `.planning/research/STACK.md` + github.com/cashubtc/cdk] |
| `rust-dlc` + `ddk` | rust-dlc 0.7+, DDK 0.0.17+ | SPIKE-02 DLC contract construction | [CITED: STACK.md + github.com/bennyhodl/dlcdevkit]; **DDK is alpha — pin exact version** |
| `frost-secp256k1-tr` | 2.2+ | SPIKE-03 DKG ceremony | [VERIFIED: includes TOB Feb 2024 Pedersen fix per zfnd.org remediation] |
| `nostr-sdk` (rust-nostr) | 0.34+ | HIP publishing script (alternative to TS NDK) | [CITED: STACK.md + github.com/rust-nostr/nostr] |
| `bdk_wallet` | 1.0+ | SPIKE-02 funding tx construction | [CITED: STACK.md] |
| `ldk-node` | 0.4+ | SPIKE-02 Lightning deposit/withdraw flow | [CITED: STACK.md + must include post-v0.1.1 patches per PITFALLS.md Pitfall 11] |
| `bitcoin` (rust-bitcoin) | 0.32+ | Low-level primitives | [CITED: STACK.md] |
| `tokio` | 1.40+ | Async runtime | [CITED: STACK.md] |
| `serde` / `serde_json` | 1.x | Serialization | [CITED: STACK.md] |

### Supporting (TypeScript, for HIP publishing scripts only)

| Library | Version | Purpose | Verification |
|---------|---------|---------|--------------|
| `@nostr-dev-kit/ndk` | 2.10+ | `scripts/publish-hip.ts` HIP-as-Nostr-long-form publisher | [CITED: STACK.md] |
| Bun | latest | TS package mgr + runtime | [CITED: STACK.md "Bun" + CLAUDE.md] |

### Development Tools

| Tool | Purpose | Notes |
|------|---------|-------|
| `cargo workspace` | Multi-crate monorepo | `crates/hunch-protocol`, `crates/hunch-mint-spike`, `crates/hunch-oracle-spike` |
| `cargo-deny` + `cargo-audit` | Supply chain | CI gate |
| `mdl` or `markdownlint-cli` | HIP Markdown lint | `scripts/verify-hips.sh` |
| `age` | Encryption for `docs/legal/signoff/` if committed | Alternative: private branch |
| Mutinynet | Signet variant, 30s blocks | `https://faucet.mutinynet.com` for test sats |
| `nak` (Nostr Army Knife) | CLI for ad-hoc Nostr event testing | per STACK.md |
| Radicle CLI | `rad` for setting up Radicle mirror | Phase 1 PROTO-07 deliverable |

### Installation (Phase 1)

```bash
# Repo skeleton (Wave 0)
cargo init --workspace
mkdir -p crates apps docs/legal docs/playbooks .github/workflows
touch LICENSE CONTRIBUTING.md CODE_OF_CONDUCT.md README.md
# (Populate LICENSE with MIT text; CLAUDE.md already exists)

# Spike crates
cargo new --lib crates/hunch-protocol
cargo new --lib crates/hunch-mint-spike
cargo new --lib crates/hunch-oracle-spike

# Dependencies (per crate Cargo.toml, pinned to exact versions discovered during SPIKE-01)
cargo add --package hunch-mint-spike cdk@0.14 ldk-node@0.4 bdk_wallet@1.0 rust-dlc@0.7 ddk@0.0.17 tokio serde serde_json
cargo add --package hunch-oracle-spike frost-secp256k1-tr@2.2 nostr-sdk@0.34 tokio serde serde_json

# Tooling
cargo install cargo-audit cargo-deny mdl
bun add -g markdownlint-cli  # alternative HIP linter
```

**Pre-Wave-0 version verification (mandatory):**
```bash
# Run this script 2026-05-28 or later and update versions before committing Cargo.toml.
# Stale STACK.md versions are likely.
for crate in cdk rust-dlc ddk frost-secp256k1-tr nostr-sdk bdk_wallet ldk-node; do
  echo "$crate:"
  cargo search "$crate" --limit 1
done
```

---

## 12. Code Examples (Phase 1 patterns)

### Publishing a HIP as Nostr long-form (kind 30023)

```typescript
// scripts/publish-hip.ts (TS variant)
// Source: NIP-23 spec at nips.nostr.com/23
import NDK, { NDKEvent } from "@nostr-dev-kit/ndk";
import { readFileSync } from "fs";
import { join } from "path";

const HIP_NUMBER = process.argv[2]; // e.g., "0", "1", "2"
const MD_PATH = join("docs", `HIP-${HIP_NUMBER}.md`);
const content = readFileSync(MD_PATH, "utf-8");

const ndk = new NDK({
  explicitRelayUrls: [
    "wss://relay.hunch.markets",
    "wss://relay.damus.io",
    "wss://nos.lol",
    "wss://relay.nostr.band",
  ],
  // signer = NDKPrivateKeySigner with maintainer pseudonymous key
});

await ndk.connect();

const ev = new NDKEvent(ndk);
ev.kind = 30023;
ev.content = content; // Markdown source — NO HTML, NO hard line-breaks per NIP-23
ev.created_at = Math.floor(Date.now() / 1000);
ev.tags = [
  ["d", `hip-${HIP_NUMBER}`],          // identifier for replaceability
  ["title", `HIP-${HIP_NUMBER}: <title>`],
  ["summary", "<2-sentence summary>"],
  ["published_at", `<unix-seconds-of-first-publish>`],
  ["t", "hunch"],
  ["t", "hip"],
  ["t", "bitcoin"],
];

await ev.sign(); // signer set above
await ev.publish();
console.log("Published as", ev.encode()); // emits naddr1... NIP-19 identifier
```

### FROST 3-of-5 DKG ceremony skeleton (Rust)

```rust
// crates/hunch-oracle-spike/tests/frost_dkg_3of5.rs
// Source: frost-secp256k1-tr v2.2 API docs; RFC 9591
use frost_secp256k1_tr::{
    keys::dkg::{round1, round2, part1},
    Identifier,
};
use rand::thread_rng;

#[tokio::test]
async fn frost_dkg_3of5_ceremony() {
    let mut rng = thread_rng();
    const MAX_SIGNERS: u16 = 5;
    const MIN_SIGNERS: u16 = 3;

    // Round 1 (per participant)
    let identifiers: Vec<Identifier> =
        (1..=MAX_SIGNERS).map(|i| Identifier::try_from(i).unwrap()).collect();

    let mut secret_packages = std::collections::HashMap::new();
    let mut round1_packages = std::collections::HashMap::new();

    for id in &identifiers {
        let (secret, package) =
            part1(*id, MAX_SIGNERS, MIN_SIGNERS, &mut rng).unwrap();
        // CRITICAL per Trail of Bits Feb 2024 disclosure:
        // verify package.commitment().0.len() == MIN_SIGNERS (== t).
        // frost-secp256k1-tr v2.2+ does this check internally; we verify here too.
        assert_eq!(package.commitment().0.len(), MIN_SIGNERS as usize,
                   "Pedersen DKG coefficient vector length mismatch — possible threshold-raise attack");
        secret_packages.insert(*id, secret);
        round1_packages.insert(*id, package);
    }

    // Round 2 (per participant), and Finalization, omitted for brevity
    // See frost-secp256k1-tr docs for full ceremony

    // After finalization, all participants agree on a group_public_key.
    // For the test, assert all 5 participants derive the same group_public_key.
}
```

### Repo verification (`scripts/verify-repo.sh`)

```bash
#!/usr/bin/env bash
# Wave 0 deliverable. Falsification: any check that fails.

set -euo pipefail

# LICENSE is MIT
grep -q "MIT License" LICENSE || { echo "LICENSE not MIT"; exit 1; }

# CONTRIBUTING.md exists and references HIP process
test -f CONTRIBUTING.md
grep -q "HIP" CONTRIBUTING.md || { echo "CONTRIBUTING.md doesn't reference HIPs"; exit 1; }

# CODE_OF_CONDUCT.md exists
test -f CODE_OF_CONDUCT.md

# CLAUDE.md exists and references GSD workflow
test -f CLAUDE.md
grep -q "GSD" CLAUDE.md

# Radicle remote exists
git remote | grep -q "^radicle$" || { echo "No radicle remote configured"; exit 1; }

# Codeberg mirror exists (defense-in-depth per Pitfall 13)
git remote | grep -q "^codeberg$" || echo "WARN: codeberg mirror not configured"

# HIPs structurally valid
for hip in docs/HIP-*.md; do
  [ -f "$hip" ] || continue
  mdl "$hip" || { echo "$hip fails mdl lint"; exit 1; }
  grep -q "^## Abstract" "$hip" || { echo "$hip missing Abstract section"; exit 1; }
done

echo "All checks pass."
```

---

## 13. Common Pitfalls (Phase 1 specific)

Cross-referenced to `.planning/research/PITFALLS.md` where applicable.

### Pitfall A: Building HIP-3 against PR #128 without spike validation

**What goes wrong:** Hunch writes HIP-3 referencing PR #128's bilateral-DLC architecture, then SPIKE-01 reveals PR #128 is dead and PR #337 has different architecture. HIP-3 must be rewritten end-to-end.

**Why it happens:** Trust in stale `.planning/research/*.md` content (which referenced PR #128 as authoritative without re-verifying state as of 2026-05).

**How to avoid:** SPIKE-01 is a hard dependency for HIP-3. Sequence the plan: SPIKE-01 first; HIP-3 after.

**Warning signs:** HIP-3 draft appearing in repo before SPIKE-01 decision document.

(Maps to `.planning/research/PITFALLS.md` Pitfall 6 NUT-DLC spec instability.)

### Pitfall B: Pseudonymity leaked via git commit metadata

**What goes wrong:** Maintainer commits to repo from a machine with `git config user.email <real-email>` and `user.name <real-name>` set globally. Commits leak the real identity even if signed under a pseudonym.

**How to avoid:** Per-repo `git config` (NOT global per CLAUDE.md "NEVER update the git config" — but per-repo is allowed and necessary). Verify `git log --format="%an <%ae>"` shows only pseudonym before the first public push.

**Warning signs:** Real name or email appearing in `git log` of public repo.

(New — not in `.planning/research/PITFALLS.md`, but flagged in §6.6.)

### Pitfall C: HIP published with content that contradicts a locked CLAUDE.md decision

**What goes wrong:** HIP-3 specifies a custodial-promise mint (Path C) without loud documentation, violating CLAUDE.md "Trust the math. Never introduce a centralized trust point unless absolutely necessary; document it loudly if you must."

**How to avoid:** plan-checker review of every HIP draft against CLAUDE.md directives. The CLAUDE.md cypherpunk principles are auditable text — any HIP that introduces a trust point must include explicit acknowledgment + rationale.

**Warning signs:** HIP draft introduces a trust assumption without a `## Security Considerations` discussion of the assumption.

### Pitfall D: Counsel sign-off bottleneck

**What goes wrong:** Phase 1 success criterion #4 ("Crypto-specialized counsel engaged, offshore entity choice formally recommended in writing, ToS draft completed") depends on counsel response times that may exceed the 6-8 week Phase 1 estimate.

**How to avoid:** Engage counsel in Phase 1 Week 1 (engagement letter signed Week 2). Treat counsel response times as the long-pole; plan other tracks (HIPs, spikes) in parallel.

**Warning signs:** No counsel engagement letter signed by Phase 1 Week 3.

### Pitfall E: Spike code accidentally promoted to production

**What goes wrong:** SPIKE-02 prototype code in `crates/hunch-mint-spike` gets copy-pasted into Phase 2's `crates/hunch-mint` because "it works." But spike code lacks audit-grade rigor, has unsafe shortcuts, and bypasses CLAUDE.md's "no custom crypto" / "no clever shortcuts" mandates.

**How to avoid:** Name spike crates explicitly with `-spike` suffix. README in each spike crate states "PROTOTYPE ONLY — NOT FOR PRODUCTION. Phase 2 implementation re-writes from spec." Phase 2 plan check verifies spike crates are NOT depended on by production crates.

**Warning signs:** Phase 2 Cargo.toml has `hunch-mint-spike` in `[dependencies]`.

---

## 14. State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Cashu NUT-DLC PR #128 (bilateral) | NUT-CTF PR #337 (oracle-agnostic conditional tokens) | PR #128 closed May 20 2025; PR #337 opened Feb 7 2026 | Hunch HIP-3 + SPIKE-01 must pivot |
| atomic.finance Lightning-DLC channels | atomic.finance acquired by Lygos Aug 2025 → pivoted to BTC lending | Aug 2025 | Lightning-DLC v1 NO-GO |
| NIP-04 DM (deprecated, metadata leak) | NIP-44 gift-wrapped DMs | NIP-44 ratification | FROST DKG coordination must use NIP-44 |
| Pre-Trail-of-Bits FROST DKG implementations | TOB Feb 2024 Pedersen-DKG fix integrated into `frost-secp256k1-tr` v2.2+ | Feb 2024 | Pin v2.2+ explicitly; assert coefficient vector length in test |
| Damus Apple App Store tipping (zaps-on-posts) | Damus zaps moved to profile-level only after Apple pressure | Jun 2023 | Confirms no-native-app strategy for Hunch |
| El Salvador as Bitcoin haven for offshore entities | Bitcoin Law rolled back Jan 29 2025 under IMF pressure | Jan 29 2025 | El Salvador OUT for operating entity; OK for individual residency |
| Polymarket fully blocked from US | QCEX acquisition Sep 2025 → Amended Order of Designation Nov 25 2025 → US re-entry via KYC'd intermediated venue | Sep-Nov 2025 | Confirms Polymarket's path is structurally incompatible with cypherpunk; reinforces Hunch's "no operator to sue" structural posture |

**Deprecated/outdated content in existing `.planning/research/*.md`:**
- `STACK.md`, `ARCHITECTURE.md`, `FEATURES.md`, `PITFALLS.md`, `SUMMARY.md` all reference PR #128 as the NUT-DLC critical path. **All five docs need a corrigendum noting PR #128 is closed and PR #337 is the live successor.** Recommendation: in Phase 1 Wave 0, append a "2026-05-28 Research Update" section to each, OR (preferred) update them in-place with a changelog entry. [ASSUMED: corrigendum is preferable to in-place edit to preserve original research provenance — verify with user.]
- `STACK.md` v2.2+ for `frost-secp256k1-tr` is correct but should explicitly cite the TOB fix integration.
- "cara" in `STACK.md` / `SUMMARY.md` is likely a misremembering of Crypto Garage's Lightning-DLC work. Update SPIKE-04 source name.

---

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | Counsel pricing $30-80K for structuring opinion + entity formation | §6.2 | Could be 2-3× higher in 2026 market; doesn't change recommendation but affects budget |
| A2 | Specific counsel partner names (Luka Müller-Studer, Stéphanie Hodara-El Bez, Lucy Frew, etc.) | §6.2 | Personnel may have moved; re-verify before outreach |
| A3 | Corrigendum vs in-place edit is preferable for stale .planning/research/*.md | §14 | User may prefer in-place updates; verify in /gsd-discuss-phase or planning |
| A4 | Mutinynet satisfies "signet" success criterion | §10 Question 9 | If user disagrees, SPIKE-02/03 must use vanilla signet (slower) |
| A5 | Path A (NUT-CTF) is the recommended SPIKE-01 outcome | §3.2 | Depends on actual PR #337 conversations; SPIKE-01 must validate empirically |
| A6 | Pseudonymity must be load-bearing for "protocol-core" roles but foundation officer can be public | §6.6 | User may want full pseudonymity across all roles, which limits foundation banking access |
| A7 | NIP-88 PR #1681 (kinds 88/89) is acceptable to track as a draft despite being unmerged | §2.2 | If PR #1681 stalls indefinitely, Hunch may need to fork the NIP |
| A8 | Hunch event kinds 308xx / 388xx are not in NIP registry as of 2026-05-28 | §2.2 | Verified by search; new collisions could appear before Phase 1 publishes |
| A9 | Phase 1 6-8 week ROADMAP estimate holds if counsel is the long-pole | §1 | Could blow out to 10-12 weeks if counsel response slow; SPIKE-01 / HIP-3 sequencing accommodates |
| A10 | Repo skeleton bootstrap is Wave 0 (not Plan 1 Task 1) | §10 Question 7 | If user prefers Plan 1 Task 1, no major impact; just sequencing |
| A11 | Counsel sign-off PDFs go in a private branch or `docs/legal/signoff/` encrypted with `age` | §9 + §10 Question 7 | User may have a different sensitive-doc storage preference (e.g., counsel keeps originals, Hunch keeps hash-commits only) |
| A12 | Phase 1 should send initial audit-firm inquiries (Trail of Bits Bitcoin team etc.) in Week 4-5 | §10 Question 10 | Out-of-scope for declared requirements; could be deferred to Phase 2 Wave 0 |

**Implications:** A1-A2 do not affect the structural plan; verify during counsel outreach. A3-A4, A6-A7, A10-A12 are best clarified in `/gsd-discuss-phase 1` before plans lock. A5 is the largest single open question and the focus of SPIKE-01.

---

## Environment Availability

For Phase 1 (specs + spikes + legal):

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| Rust toolchain (`cargo`, `rustc`) | SPIKE-02, SPIKE-03, repo skeleton | Probable on dev machine; verify in Wave 0 | stable 1.80+ per STACK.md | Install via rustup if missing |
| Bun runtime | `scripts/publish-hip.ts`, future TS scripts | Per CLAUDE.md stack; verify | latest | Fall back to Node 22+ with `tsx` |
| `git` | Always | Yes (project in git already) | — | — |
| GitHub account | Repo publishing, mirror | Yes (Silexperience210 per memory) | — | — |
| Radicle CLI | Radicle mirror | Verify in Wave 0 | latest stable | If missing: standard `curl` install |
| Codeberg account | EU non-MS mirror | Verify | — | Create if missing |
| Nostr key (maintainer pseudonymous) | HIP publishing | Verify in Wave 0; if no separate pseudonym key exists yet, generate one | — | Generate per Section 6.6 |
| Mutinynet faucet access | SPIKE-02 / SPIKE-03 | Public faucet | — | Vanilla signet (slower) |
| `mdl` or `markdownlint-cli` | HIP linting | Probably absent; install in Wave 0 | latest | Either tool acceptable |
| `cargo-audit` + `cargo-deny` | Supply chain CI | Install in Wave 0 | latest | — |
| `age` | Encrypt sensitive PDFs in repo (if chosen) | Verify in Wave 0 | latest | Alternative: private branch with `.gitignore` |
| Counsel engagement | LEGAL-01..04 | Not engaged yet | — | This IS the falsification condition for LEGAL-02 |
| Counsel-recommended jurisdiction registration agents (CH Stiftung, BVI BC) | LEGAL-01 | Engaged via counsel | — | Counsel mandates choice |

**Missing dependencies with no fallback:**
- Counsel engagement (LEGAL-02 falsification condition). Cannot be substituted; engagement letter is the artifact.

**Missing dependencies with fallback:**
- All tooling listed above; standard install procedures.

---

## Sources

### Primary (HIGH confidence)

**Spec references (HIP-authoring conventions):**
- [NIP-01 Basic Protocol](https://github.com/nostr-protocol/nips/blob/master/01.md) — event kind ranges (10000-19999 replaceable, 20000-29999 ephemeral, 30000-39999 addressable / parameterized replaceable)
- [NIP-23 Long-form Content (kind 30023)](https://nips.nostr.com/23) — HIP-as-Nostr publishing mechanics
- [NIP-44 Encrypted Payloads (gift-wrap)](https://nips.nostr.com/44) — FROST DKG coordination channel
- [NIPs registry / nips.nostr.com](https://nips.nostr.com/) — kind-collision check
- [github.com/nostr-protocol/nips master](https://github.com/nostr-protocol/nips) — authoritative NIP list

**NUT-DLC primary state:**
- [Cashu NUTs PR #128 (NUT-DLC by conduition) — CLOSED May 20, 2025](https://github.com/cashubtc/nuts/pull/128) — verified via WebFetch 2026-05-28
- [Cashu NUTs PR #337 (NUTs for Prediction Markets by joemphilips) — OPEN Feb 7, 2026](https://github.com/cashubtc/nuts/pull/337) — verified via WebFetch 2026-05-28
- [Cashu NUTs Specifications](https://cashubtc.github.io/nuts/) — ratified NUTs reference

**Nostr DLC events (NIP-88):**
- [NIP-88 PR #919 (benthecarman) — DRAFT, unmerged](https://github.com/nostr-protocol/nips/pull/919)
- [NIP-88 PR #1681 (conduition scope-reduced) — OPEN, unmerged](https://github.com/nostr-protocol/nips/pull/1681) — kinds 88 + 89 only

**DLC spec references:**
- [discreetlogcontracts/dlcspecs](https://github.com/discreetlogcontracts/dlcspecs) — canonical DLC specs (Introduction, Oracle, Messaging, MultiOracle, NumericOutcome, PayoutCurve, v0Milestone)

**FROST cryptographic state:**
- [RFC 9591 — FROST](https://datatracker.ietf.org/doc/rfc9591/) — current standard
- [FROST paper (Komlo & Goldberg, 2020) eprint 2020/852](https://eprint.iacr.org/2020/852.pdf)
- [Trail of Bits — Breaking the Shared Key in Threshold Signature Schemes (Feb 20, 2024)](https://blog.trailofbits.com/2024/02/20/breaking-the-shared-key-in-threshold-signature-schemes/) — Pedersen DKG vulnerability disclosure
- [ZF Foundation Pedersen DKG remediation announcement](https://zfnd.org/pedersen-dkg-vulnerability-in-frost-distributed-key-generation-successfully-remediated/)
- [Blockstream ChillDKG (BIP-FROST-DKG)](https://github.com/BlockstreamResearch/bip-frost-dkg)
- [BIP-445 FROST for BIP-340 (siv2r/bip-frost-signing)](https://github.com/siv2r/bip-frost-signing)

**Lightning-DLC state:**
- [Blockspace.media — Lygos Finance acquires Atomic Finance (Aug 2025)](https://blockspace.media/insight/lygos-finance-acquires-atomic-finance-to-launch-non-custodial-dlc-powered-bitcoin-loans/)
- [Atomic.finance — An Atomic Pivot blog post](https://atomic.finance/blog/an-atomic-pivot/)
- [Medium / Crypto Garage — DLC on Lightning TLDR (2022)](https://medium.com/crypto-garage/dlc-on-lightning-cb5d191f6e64) — production status "not production-ready"
- [TFTC Issue 1286 — First mainnet DLC on Lightning executed (Nov 2022)](https://www.tftc.io/issue-1286/)

**Legal primary sources (verified in `.planning/research/PITFALLS.md`, re-confirmed this research):**
- [CFTC v. Blockratize Order, CFTC Docket 22-09 (Jan 3, 2022) PDF](https://www.cftc.gov/media/6891/enfblockratizeorder010322/download)
- [18 U.S.C. § 1960 (Cornell LII)](https://www.law.cornell.edu/uscode/text/18/1960)
- [DOJ SDNY press release on Storm conviction (Aug 6, 2025)](https://www.justice.gov/usao-sdny/pr/founder-tornado-cash-crypto-mixing-service-convicted-knowingly-transmitting-criminal)
- [DeFi Education Fund — US v. Storm timeline (sentencing Apr 9, 2026)](https://www.defieducationfund.org/us-v-storm-background-timeline/)
- [NBC News — FBI raid on Polymarket CEO Shayne Coplan (Nov 13, 2024)](https://www.nbcnews.com/tech/tech-news/fbi-raids-polymarket-ceo-shayne-coplans-apartment-seizes-phone-source-rcna180180)
- [Regulatory Oversight — Polymarket Amended Order of Designation (Nov 25, 2025)](https://www.regulatoryoversight.com/2025/12/cftc-approval-allows-polymarket-to-reenter-the-u-s-market/)
- [IMF Country Report 25/58 — El Salvador (Bitcoin Law rollback)](https://www.imf.org/-/media/files/publications/cr/2025/english/1slvea2025001-print-pdf.pdf)
- [Reason — El Salvador Walks Back Bitcoin Law (Feb 3, 2025)](https://reason.com/2025/02/03/el-salvador-walks-back-its-bitcoin-law/)
- [FinCEN FIN-2013-G001 virtual currency MSB guidance](https://www.fincen.gov/resources/statutes-regulations/guidance/application-fincens-regulations-persons-administering)
- [Norton Rose Fulbright — EU approach to prediction markets and event contracts](https://www.nortonrosefulbright.com/en/knowledge/publications/290d594a/the-eus-approach-to-prediction-markets-and-event-contracts)

**Project planning sources (HIGH confidence — internal):**
- `C:\Users\Silex\Hunch\.planning\PROJECT.md` (2026-05-27)
- `C:\Users\Silex\Hunch\.planning\REQUIREMENTS.md` (2026-05-27)
- `C:\Users\Silex\Hunch\.planning\ROADMAP.md` (2026-05-27)
- `C:\Users\Silex\Hunch\.planning\STATE.md` (2026-05-27)
- `C:\Users\Silex\Hunch\.planning\research\STACK.md` (2026-05-27)
- `C:\Users\Silex\Hunch\.planning\research\ARCHITECTURE.md` (2026-05-27)
- `C:\Users\Silex\Hunch\.planning\research\FEATURES.md` (2026-05-27)
- `C:\Users\Silex\Hunch\.planning\research\PITFALLS.md` (2026-05-27, enriched)
- `C:\Users\Silex\Hunch\.planning\research\SUMMARY.md` (2026-05-27)
- `C:\Users\Silex\Hunch\.planning\config.json`
- `C:\Users\Silex\Hunch\CLAUDE.md` (verified 2026-05-28)

### Secondary (MEDIUM confidence)

- [Atomic Theories — Atomic.finance blog](https://atomic.finance/blog/) — pre-acquisition context
- [Conduition Cashu vulnerability disclosure (Jul 2025)](https://conduition.io/code/cashu-disclosure/) — conduition's continuing engagement post PR #128 closure
- [Cashu HTLC DoS disclosure (Nov 2, 2025)](https://github.com/jamesob/delving-bitcoin-archive/blob/master/archive/rendered-topics/2025-11-November/2025-11-02-public-disclosure-denial-of-service-using-htlc-in-cashu-id2090.md) — Cashu protocol patch state
- [conduition.io — Discreet Log Contract Factories](https://conduition.io/scriptless/dlc-factory/) — DLC architectural background
- [Bitcoin Magazine — DLCs on Lightning open the door for Bitcoin smart contracts](https://bitcoinmagazine.com/technical/dlcs-on-lightning-and-bitcoin-smart-contracts) — Crypto Garage Lightning-DLC context

### Tertiary (LOW confidence — flagged for re-verification)

- Specific 2026 counsel partner names — re-verify during Phase 1 Week 1 outreach
- Counsel pricing ranges — re-verify with engagement letter
- Exact Cargo crate versions — re-verify via `cargo search` in Wave 0
- Codeberg / Radicle setup details — verify when actually configuring

---

## Metadata

**Confidence breakdown:**

| Area | Level | Reason |
|------|-------|--------|
| HIP authoring conventions | HIGH | NIP-23 + NIP-01 + dlcspecs primary sources directly cited |
| Nostr kind collision check | HIGH | Verified against nips.nostr.com + github.com/nostr-protocol/nips 2026-05-28; documented NIP-88 draft state |
| NUT-DLC strategy | **MEDIUM-LOW** | PR #128 closure was the principal finding; PR #337 viability needs SPIKE-01 to confirm with maintainers |
| FROST DKG state | HIGH | RFC 9591 ratified; TOB fix integrated; ChillDKG public; ceremony pattern well-documented |
| Lightning-DLC NO-GO | HIGH | atomic.finance acquisition + Crypto Garage's own "not production ready" caveat |
| Legal foundation (jurisdiction + counsel) | HIGH | Primary sources verified in `.planning/research/PITFALLS.md`; recommendations align with documented enforcement precedents |
| Maintainer pseudonymity plan | HIGH structurally / MEDIUM operationally | Storm + Coplan precedents are HIGH; specific operational hygiene details are best-practice ASSUMED |
| Validation Architecture (Nyquist) | HIGH | Each requirement has a concrete falsification artifact + automated where possible |
| Wave 0 gap list | HIGH | Standard practice for monorepo + CI; nothing exotic |

**Research date:** 2026-05-28
**Valid until:** 2026-07-28 (fast-moving — PR #337 status, NIP-88 PR #1681 status, and Storm sentencing all may move within 60 days)

---

## RESEARCH COMPLETE
