# Project Research Summary

**Project:** Hunch
**Domain:** Permissionless Bitcoin-native cypherpunk prediction market protocol
**Researched:** 2026-05-27, enriched 2026-05-28
**Confidence:** HIGH for legal precedents + feature landscape; MEDIUM-HIGH for stack + architecture (STACK / ARCHITECTURE remain at first-draft depth; FEATURES / PITFALLS enriched with primary sources).

## 2026-05-28 Corrigendum — NUT-DLC pivot to NUT-CTF (PR #337)

**Status update:** Cashu NUTs PR #128 (bilateral NUT-DLC by conduition) was **CLOSED 2025-05-20** by thesimplekid with comment "Closing as there is no active work. Please reopen if work continues." This document was written 2026-05-27 assuming PR #128 was the live critical path. That assumption is **WRONG**.

**New chosen path (locked in CONTEXT.md decision D-01):** Path A — **NUT-CTF (Conditional Token Framework, PR #337 by joemphilips, opened 2026-02-07)**. NUT-CTF is architecturally distinct: oracle-agnostic mint with conditional token issuance + split-merge for secondary market, instead of mint-as-bilateral-counterparty.

**References below to "NUT-DLC", "PR #128", "bilateral DLC mint" should be read as NUT-CTF / PR #337 / oracle-agnostic conditional token framework** unless explicitly marked as historical context. The executive narrative about Cashu mints with NUT-DLC providing liquidity now refers to mints implementing PR #337's NUT-CTF + NUT-CTF-split-merge sub-NUTs.

See `.planning/phases/01-cypherpunk-foundation/01-RESEARCH.md` §3 for the deep-dive on this pivot.


## Executive Summary

Hunch is a Bitcoin-only, no-KYC, permissionless prediction market protocol combining four primitives: **DLCs** (rust-dlc + DDK) for trustless settlement, **Cashu mints** (CDK with NUT-DLC extension, currently in Cashu PR #128 by `conduition`) for liquid YES/NO token markets, **Lightning** (LDK Node) for deposits/withdrawals, and **Nostr** for market discovery, oracle attestations, and reputation. The protocol is neutral; Hunch operates a reference frontend (`hunch.io`) plus a reference mint and oracle, but the protocol survives any operator failure.

The recommended approach is a **Rust workspace** for protocol services + **Next.js 15** static-export frontend + **Tor hidden service** mirror, all MIT-licensed and mirrored to Radicle. The single biggest technical risk is **NUT-DLC spec instability** (PR #128 is unmerged); the single biggest non-technical risk is **CFTC enforcement** following Blockratize ($1.4M, Jan 3, 2022) — and critically, **the FBI raided Polymarket CEO Coplan's apartment November 13, 2024 without prior charge**, then Polymarket bought a CFTC-licensed exchange (QCEX, ~$112M, Sept 2025) and obtained an Amended Order of Designation Nov 25, 2025 to re-enter the US. Polymarket's "fix" is **structurally incompatible with cypherpunk principles** (KYC, intermediated venue). Hunch's only viable defense is to be so structurally non-US, non-custodial, and protocol-neutral that no operator entity exists for the CFTC to charge.

The path to mainnet without artificial caps requires: external security audit ($50-150K), 2-3 months of Mutinynet testing, a bug bounty live before launch, and a tiered launch despite the "no caps" goal. Roman Storm's § 1960 conviction (Aug 6, 2025; sentencing not yet pronounced; hearing scheduled April 9, 2026; DOJ pursuing retrial on hung counts) means maintainer non-US residency is now load-bearing for project survival.

## Key Findings

### Recommended Stack

Rust everywhere for protocol-critical services (CDK 0.14+, LDK Node 0.4+, BDK 1.0+, rust-dlc + DDK 0.0.17+, frost-secp256k1-tr 2.2+, nostr-sdk 0.34+). TypeScript for frontend (Next.js 15 + NDK 2.10+ + cashu-ts 2.x + WebLN). See [STACK.md](./STACK.md) for full detail.

**Core technologies:**

- **CDK (Cashu Dev Kit, Rust)** — Cashu mint with NUT-DLC extension. The critical path; mint = orderbook + DLC counterparty.
- **rust-dlc + DDK** — DLC contract construction. Alpha software (DDK 0.0.17), pin versions carefully.
- **frost-secp256k1-tr** — Threshold Schnorr signing for multi-oracle FROST k-of-n. RFC 9591 compliant, BIP-445 advancing. ⚠️ **Trail of Bits disclosed a FROST DKG vulnerability in Feb 2024 — verify implementation incorporates the fix (see PITFALLS Pitfall 7).**
- **LDK Node** — Embedded Lightning. WASM-capable for future browser-LDK.
- **BDK** — Bitcoin wallet primitives, PSBT, Esplora backend.
- **nostr-sdk (rust-nostr) + NDK (TS)** — Nostr layer.
- **Next.js 15 + Tailwind + shadcn/ui + NDK + cashu-ts + WebLN** — Frontend stack.

### Expected Features

See [FEATURES.md](./FEATURES.md) for the full enriched catalogue (Polymarket V2 / pUSD / CLOB v2 verified post-April 2026; Augur fork state; Manifold deep analysis including Murphy/Levin bill).

**Must have (table stakes):**
- Nostr login (NIP-07 / NIP-46)
- Lightning deposit + withdraw
- Browse markets (categories, search, trending) — Polymarket 10 categories as reference benchmark
- Buy YES / Buy NO with sat amount via Cashu
- Sell position before resolution (atomic swap)
- Resolution & payout via oracle Schnorr sig
- Market detail page (oracle identity + reputation visible, resolution rules, dispute window)
- Account / portfolio
- Tor hidden service from day 1
- Mobile-responsive web (PWA)

**Should have (cypherpunk differentiators):**
- **Permissionless market creation** — Polymarket end-users CANNOT create markets (they submit suggestions only); UMA proposers gated by whitelist (≥5 proposals + ≥95% accuracy over 6-month trailing window). Manifold is permissionless but play-money. We are uniquely permissionless + real-sats + non-custodial.
- **Multi-oracle marketplace** — Polymarket uses UMA monolithic; Kalshi uses internal team; Augur uses REPv2 staked reporters. We use competing oracles selected per market.
- **Zero KYC** — Polymarket V2 has pUSD + KYC path via QCEX FCMs for US users; Kalshi full KYC; Manifold has Stripe-billed Mana. We have none.
- **Zero custody during market lifetime + zero custody at settlement** — Polymarket holds pUSD (1:1 USDC-backed ERC-20 on Polygon) in their CTF. We hold nothing at settlement (DLC + Cashu pattern).
- **Public, machine-readable Bitcoin settlement** — Polymarket settles on Polygon; we settle natively on Bitcoin L1 with Schnorr-signed oracle attestations.
- **Multi-frontend / multi-mint protocol design** — fork-friendly, censorship-resistant.
- **Tor + IPFS + Radicle distribution** from day 1.

**Defer (v2+):**
- Multi-outcome markets (>YES/NO) — Schnorr adaptor multi-sig complexity
- Mobile native — App Store deplatform risk (Damus tipping removed)
- DAO governance — premature
- Programmatic oracles (verifiable computation)
- Lightning DLC channels (instant settle) — primitive not yet production
- Federated mint (FROST multi-op) — v1.x once single-op mint validated

### Architecture Approach

See [ARCHITECTURE.md](./ARCHITECTURE.md) for the full design.

Three layers: **Frontend layer** (multiple Next.js instances, stateless, connect directly); **Protocol layer** (Nostr event bus as source of truth, plus oracles + Cashu mints + optional matchers); **Bitcoin settlement layer** (DLC + Lightning). Nostr is source of truth for market state; Bitcoin is source of truth for settlement.

**Major components:**

1. **hunch-protocol crate** — Shared types, event schemas, DLC builders, FROST helpers (Rust)
2. **hunch-mint binary** — Cashu mint with NUT-DLC, LDK Node, DLC backing (Rust)
3. **hunch-oracle binary** — NIP-88 publisher, FROST coordinator (Rust)
4. **hunch-relay** — nostr-rs-relay deployment (Rust)
5. **hunch-matcher** — Optional Tier 2 P2P indexer (Rust)
6. **hunch-web app** — Next.js frontend (TypeScript)
7. **docs/HIP-N** — Protocol specs (Markdown, NIP-style)

The critical architectural insight: **the Cashu mint is the multi-bettor DLC adapter**. Pure DLCs are bilateral; the mint takes one side of the DLC against the oracle's outcome, then issues fungible YES/NO tokens to bettors. This solves the bilateral-DLC limitation while preserving on-chain settlement guarantees.

### Critical Pitfalls (Top 7 — see [PITFALLS.md](./PITFALLS.md) for all 23)

1. **CFTC enforcement (Polymarket Blockratize $1.4M, Jan 3, 2022)** — Sections 4c(b), 5h(a)(1) CEA + Regs 32.2 + 37.3(a)(1). FBI raid on Coplan Nov 13, 2024 *without prior charge*. Polymarket "fixed" via $112M QCEX acquisition + Nov 25, 2025 Amended Order — incompatible with cypherpunk. Our defense: offshore foundation (CH Stiftung / BVI / Panama / Liechtenstein — **NOT El Salvador** due to IMF-forced Bitcoin Law rollback), zero US residents in operator roles, geo-block US, document protocol neutrality + non-custody.
2. **Tornado-Cash § 1960 conviction (Storm, Aug 6, 2025)** — Knowledge element under § 1960(b)(1)(C). Storm did not custody funds. **Sentencing pending (April 9, 2026 hearing); DOJ pursuing retrial on hung counts.** Mitigation: maintainer non-US residency + pseudonymity where workable + aggressive anti-abuse ToS + no half-KYC (paradoxically half-screening was used against Storm).
3. **MiCA + EU member-state gambling law (NEW)** — MiCA transitional sunset July 1, 2026. National gambling regimes may bite even if MiCA itself doesn't directly cover prediction markets. Mitigation: counsel addresses EU exposure per member-state for any frontend serving EU users.
4. **Cashu mint operator as money transmitter (NEW)** — Even non-custodial mints route sats. Could trigger state money transmitter laws + § 1960. Mitigation: structural separation (foundation owns nothing, mint operates as protocol service, document non-custody legal opinion).
5. **NUT-DLC spec instability** — PR #128 by `conduition`, unmerged, depends on PR #127. **conduition published disclosures of the Cashu DLC design in July and November 2025.** Mitigation: Phase 1 spike, contribute upstream, have a fallback custodial-promise mint design ready if spec stalls.
6. **FROST cryptographic attacks (DEEPENED)** — Trail of Bits disclosed a FROST DKG vulnerability in Feb 2024 affecting rogue-key + BBSS-style attacks. frost-secp256k1-tr incorporated the fix; verify integration. Use frost-secp256k1-tr v2.2+ exclusively. Document the DKG ceremony rigorously.
7. **Permissionless market abuse (Augur 2018 + Manifold 2026)** — Augur assassination markets defined the project narrative for years. Manifold's nuclear-detonation markets prompted the Murphy/Levin bill in 2026 to restrict markets resolving on military actions / deaths. Mitigation: frontend curation (operator-level), INVALID outcome built into DLC (50/50 refund neutralizes profit motive), social graph filter default, PR playbook ready BEFORE launch.

Additional high-severity pitfalls (see PITFALLS.md): mint operator rug, oracle lies, Schnorr nonce reuse, censorship via providers + App Store deplatform, frontend cascade recovery, cold-start liquidity, solo dev burnout, Lightning channel vulnerabilities (Wormhole + Riptide + flood-and-loot disclosed 2024-2025), PredictIt no-action rescission precedent.

## Implications for Roadmap

Based on research, the suggested phase structure (coarse granularity per config):

### Phase 1: Cypherpunk Foundation (Specs + Spikes + Legal)

**Rationale:** Protocol-first identity — write the HIPs before code so the project can survive any single implementation. Spike the riskiest unknowns (NUT-DLC, FROST DKG with TrailOfBits fix) before locking architecture. Establish legal structure before mainnet exposure (CFTC + § 1960 are existential).

**Delivers:**
- HIP-0..5 protocol specifications published
- NUT-DLC spike (working signet prototype, integration plan)
- FROST DKG ceremony spike (with Feb 2024 vulnerability mitigation verified)
- Legal structure (offshore entity — Switzerland / BVI / Panama / Liechtenstein — NOT El Salvador)
- Repo structure live on GitHub + Radicle, MIT licensed
- Public manifesto + initial community presence

**Addresses:** Permissionless market creation principle, multi-oracle vision, protocol neutrality
**Avoids:** NUT-DLC spec drift surprise, FROST operational unknowns, legal exposure (CFTC + § 1960)

### Phase 2: Mainnet Spine

**Rationale:** Build the minimum viable Hunch — single oracle, single mint, web frontend — sufficient for real mainnet use with operator-seeded liquidity.

**Delivers:**
- hunch-protocol Rust crate (types + Nostr event schemas)
- hunch-mint binary (CDK fork with NUT-DLC, LDK Node integration, DLC backing)
- hunch-oracle binary (single-key NIP-88 publisher initially, FROST-ready code paths)
- hunch-relay deployment (nostr-rs-relay)
- hunch-web frontend (browse, create, bet, sell, resolve, account, oracle marketplace)
- Tor hidden service + IPFS pin + Cloudflare frontend
- Geo-block US infrastructure (IP + Tor exit list)
- Anti-spam (social graph filter UI from launch)
- INVALID outcome built into DLC contracts
- Operator-seeded markets ready

**Avoids:** Centralized auth, custody, KYC, single-frontend dependency, cold-start liquidity void, Augur/Manifold spam-market PR disaster

### Phase 3: Mainnet Launch + Hardening

**Rationale:** Security and operational readiness for actual mainnet. Audit, bug bounty, monitoring, incident response, then progressive launch.

**Delivers:**
- External security audit (DLC + mint + frontend) — audit report public
- Bug bounty live (HackerOne / Cantina / Hexens)
- Mutinynet stress test (2-3 months)
- Tiered launch: invite-only mainnet → public mainnet with caps → caps removed (after 90 days no incident)
- Monitoring (Prometheus + Grafana) on mint + oracle + relay
- Incident response runbook + public status page
- ToS published with counsel signoff
- 50+ operator-seeded markets at launch
- Liquidity bootstrap (operator LP for first weeks)

**Avoids:** Day-one catastrophic exploit, deplatforming surprise, regulatory ambush

### Phase 4: Decentralization & Federation *(v2, deferred)*

**Rationale:** Reduce trust in Hunch operator. Multi-oracle FROST on production, federated mint with multi-operator multisig, P2P Tier 2 matching for power users, advanced reputation.

**Status:** Deferred until v1 mainnet validated for 3-6 months without major incident.

### Phase Ordering Rationale

- **Phase 1 must come first** because spec drift (NUT-DLC) and legal structure (CFTC + § 1960) are existential. Building before either could be wasted work.
- **Phase 2 builds the spine in one bundle** because all components are co-dependent.
- **Phase 3 is a go/no-go gate** rather than a feature phase.
- **Phase 4 deferred until production validation.**

### Research Flags

Phases likely needing deeper research during planning:

- **Phase 1:** NUT-DLC PR #128 current state, FROST DKG UX patterns (Fedimint), Lightning DLC channels production readiness, jurisdiction selection (CH vs BVI vs Panama vs Liechtenstein)
- **Phase 2:** CDK extension vs fork tradeoffs, Cashu blind sig + DLC token formats, multi-bettor DLC operational patterns, LSP partnership (KYC-free)
- **Phase 3:** Audit firm selection (Trail of Bits Bitcoin, Galaxy Audit, Block Digital Contracting, Cashu auditors via Calle network)
- **Phase 4:** ROAST signing + FROST member changes, Fedimint federation governance patterns

Phases with lighter research needs:

- Frontend implementation (Next.js + shadcn/ui well-trodden)
- Tor / IPFS deployment (standard)
- Nostr relay deployment (nostr-rs-relay well-documented)

## Confidence Assessment

| Area | Confidence | Notes |
|------|------------|-------|
| Stack | HIGH | Library versions verified against crates.io / GitHub; alternatives reasoned through. ⚠️ STACK.md remains at first-draft depth (agent enrichment hit session limit). Re-spike during Phase 1. |
| Features | HIGH | Polymarket V2 / pUSD / CLOB v2 verified April 28 2026; Manifold spam + Murphy/Levin bill verified; Augur fork state verified; Predyx custodial nature noted. |
| Architecture | HIGH | Conventional layering, novel only in DLC-mint integration. ARCHITECTURE.md remains at first-draft depth (agent enrichment hit session limit). Validate during Phase 1 spike. |
| Pitfalls | HIGH | Anchored on primary sources: CFTC Docket 22-09 PDF; FBI raid on Coplan Nov 13 2024; Polymarket QCX acquisition + Amended Order Nov 25 2025; § 1960 Cornell LII text; Trail of Bits FROST DKG Feb 2024; Storm sentencing pending April 9 2026 hearing. |
| NUT-DLC viability | MEDIUM | Active PR by conduition with disclosed designs July+Nov 2025. Spike required Phase 1. |
| Lightning DLC channels | LOW | Atomic.finance / cara work in progress; assume on-chain DLCs for v1 |
| FROST operational UX | MEDIUM | Crypto libraries are production with Feb 2024 fix integrated; real-world DKG ceremony patterns are research |

**Overall confidence:** HIGH (with explicit re-spike of STACK + ARCHITECTURE in Phase 1)

### Gaps to Address

- **STACK.md and ARCHITECTURE.md depth** — Agent enrichment hit session limit; re-spike in Phase 1 against current crates.io versions + DDK / CDK GitHub state + NUT-DLC PR #128 latest commit.
- **NUT-DLC PR #128 latest state** — Verify before Phase 1 locks. Plan: read latest discussion, prototype on signet, contact conduition / Calle.
- **Audit firm shortlist** — Trail of Bits Bitcoin team, Block Digital Contracting, Galaxy Audit, plus a Cashu specialist via Calle's network. Outreach during Phase 1.
- **Offshore jurisdiction final selection** — Switzerland (Stiftung) vs Panama (Foundation) vs BVI (Foundation) vs Liechtenstein (Stiftung). Counsel firms: MME or Bär & Karrer (Zurich), Walkers (BVI), Anderson Legal (Cayman). Engage Phase 1.
- **Multi-bettor DLC operational patterns** — When does mint open DLC? Per-market? Per-batch? Lazy vs eager? Spike Phase 1.
- **Real-world Lightning liquidity for prediction market workload** — LSP partnership discussion in Phase 2.
- **Frontend operator vs protocol maintainer legal exposure separation** — Different roles have different exposure. Counsel sign-off in Phase 1.

## Sources

### Primary (HIGH confidence)

**Legal precedents:**
- [CFTC v. Blockratize Order, Docket 22-09 PDF (Jan 3, 2022)](https://www.cftc.gov/media/6891/enfblockratizeorder010322/download)
- [CFTC Press Release 8478-22](https://www.cftc.gov/PressRoom/PressReleases/8478-22)
- [FBI raid on Coplan (NBC News, Nov 14, 2024)](https://www.nbcnews.com/tech/tech-news/fbi-raids-polymarket-ceo-shayne-coplans-apartment-seizes-phone-source-rcna180180)
- [Polymarket Amended Order of Designation (Nov 25, 2025)](https://www.regulatoryoversight.com/2025/12/cftc-approval-allows-polymarket-to-reenter-the-u-s-market/)
- [DOJ SDNY Press Release: Storm Tornado Cash conviction (Aug 6, 2025)](https://www.justice.gov/usao-sdny/pr/founder-tornado-cash-crypto-mixing-service-convicted-knowingly-transmitting-criminal)
- [18 U.S.C. § 1960 (Cornell LII)](https://www.law.cornell.edu/uscode/text/18/1960)
- [DeFi Education Fund — US v. Storm timeline](https://www.defieducationfund.org/us-v-storm-background-timeline/)

**Technical primary:**
- [Cashu NUT-DLC PR #128 (conduition)](https://github.com/cashubtc/nuts/pull/128)
- [Cashu NUTs Specifications](https://cashubtc.github.io/nuts/)
- [Polymarket V2 Migration docs](https://docs.polymarket.com/v2-migration)
- [Polymarket V2 Upgrade (April 28, 2026)](https://help.polymarket.com/en/articles/14762452-polymarket-exchange-upgrade-april-28-2026)
- [rust-dlc / DDK GitHub](https://github.com/bennyhodl/dlcdevkit)
- [frost-secp256k1-tr (crates.io)](https://crates.io/crates/frost-secp256k1-tr)
- [LDK Node](https://github.com/lightningdevkit/ldk-node)
- [RFC 9591 — FROST](https://datatracker.ietf.org/doc/rfc9591/)
- [Trail of Bits FROST DKG disclosure (Feb 2024)](https://blog.trailofbits.com/2024/02/13/insecure-dkg-protocols-pose-risk-to-distributed-systems-using-frost/)
- [Augur INVALID outcome (Constant Product with Invalid Insurance)](https://micah-zoltu.medium.com/augur-constant-product-with-invalid-insurance-385fca7efbc7)

### Secondary (MEDIUM confidence)

- [Polymarket QCX acquisition (Blockhead, Sep 4, 2025)](https://www.blockhead.co/2025/09/04/cftc-clears-polymarkets-return-to-us-after-three-year-ban/)
- [Manifold Murphy/Levin nuclear-detonation markets controversy (2026)](https://manifold.markets/MachiNi/will-manifold-fix-the-rising-spam-m)
- [Augur Fork is Here](https://www.augur.net/blog/the-augur-fork-is-here/)
- [Robosats / Mostro architectural patterns](https://learn.robosats.com)
- [Conduition disclosures on Cashu DLC design (July + Nov 2025)](https://conduition.io)

### Tertiary (needs validation during implementation)

- Lightning DLC channels production status (Atomic.finance, cara) — verify Phase 1
- Cashu federation patterns — verify before Phase 4
- BIP-445 (FROST) standardization timeline — verify Phase 4

---
*Research completed: 2026-05-27, enriched 2026-05-28*
*Ready for roadmap: yes; STACK + ARCHITECTURE re-spike recommended in Phase 1*
