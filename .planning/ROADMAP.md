# Hunch — Roadmap

**Milestone:** v1 — Mainnet Cypherpunk Spine
**Generated:** 2026-05-27
**Granularity:** Coarse (3 phases for v1, 1 deferred for v2)
**Status:** Active

## Overview

| # | Phase | Goal | Requirements | Success Criteria |
|---|-------|------|--------------|------------------|
| 1 | **Cypherpunk Foundation** | Lock the protocol identity (specs), de-risk biggest unknowns (NUT-DLC + FROST), establish legal structure before code commits | PROTO-01..08, SPIKE-01..04, LEGAL-01..06 | 4 |
| 2 | **Mainnet Spine** | Build the operating reference implementation — mint, oracle, relay, frontend — sufficient for real mainnet operation | MINT-01..13, ORACLE-01..09, RELAY-01..06, UI-01..21, CURATE-01..07, OPS-01..06 | 5 |
| 3 | **Mainnet Launch & Hardening** | Pass audit, run tiered mainnet launch, validate the protocol with real volume | SEC-01..08, LAUNCH-01..06, OPS-07..11 | 5 |
| 4 | **Decentralization & Federation** *(v2 — deferred)* | Reduce trust in Hunch operator: multi-oracle FROST, federated mint, P2P Tier 2 matching | (v2 items, not yet REQ-tracked) | (TBD when v2 starts) |

**Phase ordering rationale:**
- Phase 1 is foundational: spec drift and legal exposure are existential risks that can't be unwound. The biggest unknowns (NUT-DLC, FROST UX) must be spiked before architecture locks.
- Phase 2 builds the spine as one coherent bundle because mint+oracle+frontend+relay are co-dependent. Splitting them artificially creates integration gaps.
- Phase 3 is a go/no-go gate, not feature work: audit findings can force return to Phase 2 rework. Launch tiering despite "no caps" goal is honest engineering.
- Phase 4 is deferred deliberately until single-operator spine is validated in production (3-6 months). Premature federation = operational complexity without product-market fit.

## Phase Details

### Phase 1 — Cypherpunk Foundation

**Goal:** Establish protocol identity (HIPs published), validate the riskiest technical unknowns (NUT-DLC + FROST), and lock in legal structure before any mainnet exposure.

**Requirements:** PROTO-01..08, SPIKE-01..04, LEGAL-01..06 (18 requirements)

**Plans (3):**
1. **Protocol Specifications & Repo Setup** — Write HIP-0..5, publish manifesto, set up monorepo (Cargo workspace + Bun apps subfolder), MIT license, GitHub + Radicle mirror, contributor guide.
2. **Technical Spikes** — NUT-DLC working prototype on signet, FROST DKG ceremony tested with 3-of-5, Lightning DLC channel readiness assessment.
3. **Legal Foundation** — Counsel engagement, offshore entity recommendation, ToS draft, privacy policy, PR playbook.

**Success Criteria:**
1. HIP-0..5 published in repo and accessible on Nostr long-form (NIP-23) — content fully reviewed by at least one external Bitcoin protocol contributor
2. NUT-DLC working prototype demonstrates full flow (create market → bet via Lightning → resolve via oracle Schnorr → withdraw winnings) on Bitcoin signet, end-to-end
3. FROST k-of-n DKG ceremony executed successfully on signet with documented playbook (init → DKG → multi-round signing → reset/rotate procedures)
4. Crypto-specialized counsel engaged, offshore entity choice formally recommended in writing, ToS draft completed

**Estimated effort:** 6-8 weeks solo

---

### Phase 2 — Mainnet Spine

**Goal:** Build the minimum viable Hunch — single oracle, single mint, web frontend, Tor presence — sufficient for real mainnet operation with operator-seeded liquidity.

**Requirements:** MINT-01..13, ORACLE-01..09, RELAY-01..06, UI-01..21, CURATE-01..07, OPS-01..06 (62 requirements)

**Plans (3):**
1. **Protocol Services** — hunch-protocol crate + hunch-mint (Cashu + NUT-DLC + LDK Node + DLC) + hunch-oracle (NIP-88 + FROST-ready) + hunch-relay deployment.
2. **Reference Frontend** — Next.js 15 web app with Nostr login, Lightning deposit/withdraw, market browse/create/bet/sell/resolve, account portfolio, oracle marketplace, settings.
3. **Anti-Spam & Distribution** — Social graph filter, INVALID outcome in DLC, frontend curation policy, hunch.io + hunch.onion + IPFS pin + GitHub/Radicle mirror, monitoring stack.

**Success Criteria:**
1. End-to-end flow on Bitcoin signet: user creates market → user funds Lightning wallet → places bet → mint issues YES token → oracle attests outcome → DLC executes → user withdraws winnings (verified manually + via integration tests)
2. Reference frontend deployed on hunch.io (Cloudflare Pages) + hunch.onion (Tor hidden service) + IPFS pin (web3.storage) — all three serve identical builds
3. `hunch-mint`, `hunch-oracle`, `hunch-relay` documented for external operators — anyone can deploy in under 1 hour following the operator guide
4. Anti-spam working: social graph filter active by default, INVALID outcome CET supported in DLC, frontend can hide markets without removing from protocol
5. Geo-block US functional (IP-based + Tor exit list) + ToS gate on first visit; counsel-signed-off

**Estimated effort:** 12-16 weeks solo (the heaviest phase)

---

### Phase 3 — Mainnet Launch & Hardening

**Goal:** Pass external security audit, run tiered mainnet launch (invite → public-with-caps → no-caps), and reach 50+ active markets with operator-seeded liquidity.

**Requirements:** SEC-01..08, LAUNCH-01..06, OPS-07..11 (19 requirements)

**Plans (3):**
1. **Security Audit & Hardening** — Engage audit firm with Bitcoin DLC + Cashu expertise, complete 2-3 months Mutinynet stress test, resolve all High/Critical findings, supply chain audit, cryptographic-specific review.
2. **Operations Readiness** — Bug bounty program live, monitoring/alerting (Prometheus + Grafana), incident response playbook documented, public status page, operator key custody hardened (HSM), operator continuity plan written.
3. **Launch Sequence** — Invite-only mainnet (week 1-4, 100k sat cap) → public mainnet (month 2-3, 1M sat cap) → caps removed (after 90 days no incident); operator-seeded 50+ markets at public launch with LP both sides.

**Success Criteria:**
1. Audit report received from reputable firm with all High and Critical findings remediated; audit report public
2. Bug bounty live on HackerOne/Cantina/equivalent before any mainnet deployment, with at least one Critical-severity scope clearly defined
3. Invite-only mainnet phase completed without major incident (no funds loss, no oracle manipulation, no mint solvency issue) — 100k sat cap held for minimum 4 weeks
4. Public mainnet launch with 50+ markets actively trading across politics/sport/crypto/culture categories, operator-provided liquidity on first 20 markets
5. At least one external (non-Hunch) oracle running in production within 90 days of public launch — validates the marketplace model is real

**Estimated effort:** 8-12 weeks solo + 4-8 weeks audit firm time (overlap possible)

---

### Phase 4 — Decentralization & Federation *(v2, deferred)*

**Goal:** Reduce trust in Hunch operator. Multi-oracle FROST on production, federated mint with multi-operator multisig, P2P Tier 2 matching for power users, advanced reputation.

**Status:** Deferred. Trigger: v1 mainnet validated for 3-6 months without major incident, plus signal of demand for higher-trust-minimization (community asks for federation, power users want non-custodial Tier 2).

**Requirements:** Not yet REQ-tracked (v2 items in REQUIREMENTS.md). Will be re-tracked when v2 milestone opens.

**Anticipated plans:**
1. Multi-oracle FROST k-of-n production (using already-built code paths)
2. Federated mint multi-operator launch
3. P2P Tier 2 matching via Nostr

## Coverage Validation

All 84 v1 REQ-IDs are mapped to a phase.

- **Phase 1:** 18 requirements (PROTO 8 + SPIKE 4 + LEGAL 6)
- **Phase 2:** 62 requirements (MINT 13 + ORACLE 9 + RELAY 6 + UI 21 + CURATE 7 + OPS 6)
- **Phase 3:** 19 requirements (SEC 8 + LAUNCH 6 + OPS 5)
- **Phase 4:** Future (v2 items not in v1 count)

**Total:** 99 mapping entries; some requirements appear in overlap (OPS spans Phase 2 + Phase 3 by design — basic ops in Phase 2, full readiness in Phase 3).

## Phase Dependencies

```
Phase 1 (Foundation)
    ↓ specs + spikes + legal locked
Phase 2 (Mainnet Spine)
    ↓ working reference implementation
Phase 3 (Launch & Hardening)
    ↓ audited + launched + validated
Phase 4 (Decentralization)  [deferred until v2]
```

Each phase strictly depends on prior phase completion. No parallelization across phases. Within a phase, plans may run in parallel where dependencies allow.

## Risk Mitigations by Phase

| Phase | Top Risk | Mitigation in Phase |
|-------|----------|---------------------|
| 1 | NUT-DLC spec instability | SPIKE-02 working prototype before any production commit |
| 1 | Legal exposure (CFTC, § 1960) | LEGAL-01..06 done before mainnet code |
| 2 | Mint operator rug | MINT-10 weekly reserves proofs from day 1 |
| 2 | Augur-style abuse markets | CURATE-01..07 social graph filter + INVALID outcome default |
| 2 | Censorship of frontend | OPS-01..04 multi-host (Cloudflare + Tor + IPFS) from launch |
| 3 | Day-1 exploit | SEC-01..06 audit + bug bounty + Mutinynet |
| 3 | Cold-start liquidity | OPS-10..11 operator-seeded markets + LP |
| 3 | Crypto bugs | SEC-05 cryptographer review on FROST specifically |
| 4 | Federation operational complexity | Wait for v1 validation before starting |

## UI Indicators

- Phase 1: **UI hint:** no (specs + spikes + legal)
- Phase 2: **UI hint:** yes (reference frontend is core deliverable)
- Phase 3: **UI hint:** partial (status page, launch UI polish)
- Phase 4: **UI hint:** yes (federation UX, Tier 2 P2P UI)

---
*Roadmap generated: 2026-05-27*
*Next: `/gsd-discuss-phase 1` to gather plan context, or `/gsd-plan-phase 1` to plan directly*
