# Project Research Summary

**Project:** Hunch
**Domain:** Permissionless Bitcoin-native cypherpunk prediction market protocol
**Researched:** 2026-05-27
**Confidence:** MEDIUM-HIGH (high on stack, features, architecture; medium on NUT-DLC spec maturity and Lightning-DLC channel readiness)

## Executive Summary

Hunch is a Bitcoin-only, no-KYC, permissionless prediction market protocol combining four primitives: **DLCs** (rust-dlc + DDK) for trustless settlement, **Cashu mints** (CDK with NUT-DLC extension) for liquid YES/NO token markets, **Lightning** (LDK Node) for deposits/withdrawals, and **Nostr** for market discovery, oracle attestations, and reputation. The protocol is neutral; Hunch operates a reference frontend (`hunch.io`) plus a reference mint and oracle, but the protocol survives any operator failure.

The recommended approach is a **Rust workspace** for protocol services + **Next.js 15** static-export frontend + **Tor hidden service** mirror, all MIT-licensed and mirrored to Radicle. The single biggest technical risk is **NUT-DLC spec instability** (Cashu PR #128 by conduition is unmerged); the single biggest non-technical risk is **CFTC enforcement** following the Polymarket Blockratize 2022 precedent and the Tornado Cash dev conviction (Roman Storm, Aug 2025). Both risks are mitigatable: spike NUT-DLC in Phase 0, structure operator as offshore entity with non-US contributors, geo-block US on official frontend, and document protocol neutrality rigorously.

The path to mainnet without artificial caps requires: external security audit ($50-150K), 2-3 months of Mutinynet testing, a bug bounty live before launch, and a tiered launch despite the "no caps" goal (week 1-4 with 100k sat market caps to limit blast radius from undiscovered bugs).

## Key Findings

### Recommended Stack

Rust everywhere for protocol-critical services (CDK 0.14+, LDK Node 0.4+, BDK 1.0+, rust-dlc + DDK 0.0.17+, frost-secp256k1-tr 2.2+, nostr-sdk 0.34+). TypeScript for frontend (Next.js 15 + NDK 2.10+ + cashu-ts 2.x + WebLN). See [STACK.md](./STACK.md) for full detail.

**Core technologies:**

- **CDK (Cashu Dev Kit, Rust)** — Cashu mint with NUT-DLC extension. The critical path; mint = orderbook + DLC counterparty.
- **rust-dlc + DDK** — DLC contract construction. Alpha software, pin versions carefully.
- **frost-secp256k1-tr** — Threshold Schnorr signing for multi-oracle FROST k-of-n. RFC 9591 compliant, BIP-445 advancing.
- **LDK Node** — Embedded Lightning. WASM-capable for future browser-LDK.
- **BDK** — Bitcoin wallet primitives, PSBT, Esplora backend.
- **nostr-sdk (rust-nostr) + NDK (TS)** — Nostr layer.
- **Next.js 15 + Tailwind + shadcn/ui + NDK + cashu-ts + WebLN** — Frontend stack.

### Expected Features

See [FEATURES.md](./FEATURES.md) for the full catalogue.

**Must have (table stakes):**
- Nostr login (NIP-07 / NIP-46)
- Lightning deposit + withdraw
- Browse markets (categories, search, trending)
- Buy YES / Buy NO with sat amount via Cashu
- Sell position before resolution (atomic swap)
- Resolution & payout via oracle Schnorr sig
- Market detail page (price, oracle, resolution rules)
- Account / portfolio
- Tor hidden service from day 1
- Mobile-responsive web (PWA)

**Should have (cypherpunk differentiators):**
- Permissionless market creation (Nostr event) — **this IS the product**
- Multi-oracle marketplace (choose your oracle)
- Zero custody (DLC + Cashu, never operator-held funds at settlement)
- Public on-chain settlement verifiability
- Multi-frontend protocol design (forks encouraged)
- Tor + IPFS + Radicle distribution
- Social discovery via Nostr web of trust
- Lightning-native ⚡ end-to-end (no on-ramps)
- Open API for forks

**Defer (v2+):**
- Multi-outcome markets (>YES/NO) — Schnorr adaptor multi-sig complexity
- Mobile native — App Store deplatform risk; PWA is enough
- DAO governance — premature
- Programmatic oracles (verifiable computation)
- Lightning DLC channels (instant settle) — primitive not yet production
- Federated mint (FROST multi-op) — v1.x once single-op mint validated

### Architecture Approach

See [ARCHITECTURE.md](./ARCHITECTURE.md) for the full design.

Three layers: **Frontend layer** (multiple Next.js instances, all stateless, connect directly to protocol); **Protocol layer** (Nostr event bus as source of truth, plus oracles + Cashu mints + optional matchers); **Bitcoin settlement layer** (DLC + Lightning). Nostr is source of truth for market state; Bitcoin is source of truth for settlement.

**Major components:**

1. **hunch-protocol crate** — Shared types, event schemas, DLC builders, FROST helpers (Rust)
2. **hunch-mint binary** — Cashu mint with NUT-DLC, LDK Node, DLC backing (Rust)
3. **hunch-oracle binary** — NIP-88 publisher, FROST coordinator (Rust)
4. **hunch-relay** — nostr-rs-relay deployment (Rust)
5. **hunch-matcher** — Optional Tier 2 P2P indexer (Rust)
6. **hunch-web app** — Next.js frontend (TypeScript)
7. **docs/HIP-N** — Protocol specs (Markdown, NIP-style)

The critical architectural insight: **the Cashu mint is the multi-bettor DLC adapter**. Pure DLCs are bilateral; the mint takes one side of the DLC against the oracle's outcome, then issues fungible YES/NO tokens to bettors. This solves the bilateral-DLC limitation while preserving on-chain settlement guarantees.

### Critical Pitfalls

See [PITFALLS.md](./PITFALLS.md) for the full catalog.

Top 5:

1. **CFTC enforcement (Polymarket Blockratize precedent, $1.4M, 2022)** — Avoid via offshore entity, geo-block US, no US-resident operators, protocol neutrality documented.
2. **Tornado-Cash-style developer prosecution (Roman Storm, Aug 2025, § 1960 conviction)** — Maintainer pseudonymity where possible, no US residence for key contributors, aggressive anti-abuse ToS.
3. **NUT-DLC spec instability (Cashu PR #128 unmerged)** — Spike first (Phase 0), contribute upstream, plan fallback to custodial-promise mint if spec stalls.
4. **Augur-style abuse markets (assassination market replay, July 2018)** — Frontend curation + social graph filter + INVALID outcome built into DLC + PR playbook ready.
5. **Mainnet hardcore launch without sufficient audit** — Mandatory external audit ($50-150K), 2-3 months Mutinynet, bug bounty live, tiered caps despite "no caps" policy goal.

Additional high-severity: mint operator rug during market lifetime, oracle lies, Schnorr nonce reuse, censorship via providers, cold-start liquidity, solo dev burnout.

## Implications for Roadmap

Based on research, the suggested phase structure (coarse granularity per config):

### Phase 1: Cypherpunk Foundation (Specs + Spikes + Legal)

**Rationale:** Protocol-first identity — write the HIPs before code so the project can survive any single implementation. Spike the riskiest unknowns (NUT-DLC, FROST UX) before locking architecture. Establish legal structure before mainnet exposure.

**Delivers:**
- HIP-0..N protocol specifications published
- NUT-DLC spike (working signet prototype, integration plan)
- FROST DKG ceremony spike
- Legal structure (offshore entity recommendation, counsel engagement)
- Repo structure live on GitHub + Radicle, MIT licensed
- Public manifesto + initial community presence

**Addresses:** Permissionless market creation principle, multi-oracle vision, protocol neutrality
**Avoids:** NUT-DLC spec drift surprise, FROST operational unknowns, legal exposure

### Phase 2: Mainnet Spine

**Rationale:** Build the minimum viable Hunch — single oracle, single mint, web frontend — sufficient for real mainnet use with operator-seeded liquidity.

**Delivers:**
- hunch-protocol Rust crate (types + Nostr event schemas)
- hunch-mint binary (CDK fork with NUT-DLC, LDK Node integration, DLC backing)
- hunch-oracle binary (single-key NIP-88 publisher initially, FROST-ready code paths)
- hunch-relay deployment (nostr-rs-relay)
- hunch-web frontend (browse, create, bet, sell, resolve, account)
- Tor hidden service + IPFS pin
- Geo-block US infrastructure (IP + Tor exit list)
- Anti-spam (social graph filter UI from launch)
- Operator-seeded markets ready

**Uses:** Full STACK.md
**Implements:** Frontend layer + Protocol layer + Bitcoin settlement layer
**Avoids:** Centralized auth, custody, KYC, single-frontend dependency, cold-start liquidity void

### Phase 3: Mainnet Launch + Hardening

**Rationale:** Security and operational readiness for actual mainnet. Audit, bug bounty, monitoring, incident response, then progressive launch.

**Delivers:**
- External security audit (DLC + mint + frontend) — audit report public
- Bug bounty live (HackerOne / Hexens / Cantina)
- Mutinynet stress test (2-3 months)
- Tiered launch: invite-only mainnet → public mainnet with caps → caps removed
- Monitoring (Prometheus + Grafana) on mint + oracle
- Incident response runbook + public status page
- ToS published with counsel signoff
- 50+ operator-seeded markets at launch
- Liquidity bootstrap (operator LP for first weeks)

**Implements:** Operational excellence layer
**Avoids:** Day-one catastrophic exploit, deplatforming surprise, regulatory ambush

### Phase 4: Decentralization & Federation

**Rationale:** Reduce trust in Hunch operator. Multi-oracle FROST, federated mint, P2P Tier 2 matching, advanced reputation. Triggered when single-op spine is validated (3-6 months in production without major incident).

**Delivers:**
- FROST k-of-n multi-oracle on production markets
- Federated mint (FROST multisig operators)
- P2P Tier 2 matching via Nostr (Robosats-style for power users)
- Advanced reputation algorithm (cross-market, time-decayed)
- INVALID outcome enforcement via DLC built-in
- Documented onboarding for new oracle operators
- Documented onboarding for new frontend operators

**Avoids:** Single-operator trust anchor, oracle monoculture, "Hunch decides" perception

### Phase Ordering Rationale

- **Phase 1 must come first** because spec drift (NUT-DLC) and legal structure are existential. Building before either could be wasted work.
- **Phase 2 builds the spine in one bundle** because all components are co-dependent: mint needs DLC needs oracle needs Nostr needs frontend. Splitting them creates artificial boundaries.
- **Phase 3 is a "go/no-go" gate** rather than a feature phase — audit findings could send us back to Phase 2 for rework.
- **Phase 4 deferred until production validation** because federation is operationally heavy and shouldn't be paid in complexity before product-market fit.

### Research Flags

Phases likely needing deeper research during planning:

- **Phase 1 (specs + spikes):** NUT-DLC current PR state, FROST DKG UX patterns from existing federated systems (Fedimint), Lightning DLC channels current production readiness
- **Phase 2 (build):** CDK extension patterns vs fork tradeoffs, Cashu blind sig + DLC token formats, multi-bettor DLC operational patterns
- **Phase 3 (launch):** Audit firm selection, bug bounty platform comparison for Bitcoin/Cashu projects, KYC-free LSP partners
- **Phase 4 (federation):** ROAST signing + FROST member changes, Fedimint federation governance patterns

Phases with standard patterns (lighter research needed):

- Frontend implementation (Next.js + shadcn/ui is well-trodden)
- Tor / IPFS deployment (standard ops)
- Nostr relay deployment (nostr-rs-relay is well-documented)

## Confidence Assessment

| Area | Confidence | Notes |
|------|------------|-------|
| Stack | HIGH | Library versions verified against crates.io / GitHub; alternatives reasoned through |
| Features | HIGH | Competitor analysis solid (Polymarket, Augur, Manifold, Robosats), permissionless creation needs additional UX validation |
| Architecture | HIGH | Conventional layering, novel only in DLC-mint integration which is the right call given Cashu's primitives |
| Pitfalls | HIGH | Anchored on documented incidents (Polymarket CFTC settlement, Augur 2018, Tornado Cash conviction, Cashu/Nostr known issues) |
| NUT-DLC viability | MEDIUM | Active PR, active development, but unmerged. Spike required. |
| Lightning DLC channels | LOW | Atomic.finance / cara work in progress; assume on-chain DLCs for v1 |
| FROST operational UX | MEDIUM | Crypto libraries are production; real-world DKG ceremony patterns are research |

**Overall confidence:** MEDIUM-HIGH

### Gaps to Address

- **NUT-DLC PR #128 latest state** — Verify before Phase 1 locks. Plan: read latest discussion, prototype on signet, contact conduition / Calle.
- **Audit firm shortlist** — Identify 2-3 firms with Bitcoin DLC + Cashu expertise (Trail of Bits Bitcoin team, Block Digital Contracting, etc.). Plan: outreach during Phase 1.
- **Offshore jurisdiction final selection** — Switzerland vs Panama vs BVI vs El Salvador. Plan: counsel consultation in Phase 0.
- **Multi-bettor DLC operational patterns** — How exactly does mint manage DLC collateral as bets accumulate? Plan: spike during Phase 1.
- **Real-world Lightning liquidity for prediction market workload** — Inbound vs outbound balance under prediction market load. Plan: LSP partnership discussion in Phase 2.
- **Frontend operator legal exposure compared to protocol operator** — Different roles have different exposure. Plan: counsel sign-off.

## Sources

### Primary (HIGH confidence)

- [CFTC v. Blockratize order (Jan 2022)](https://www.cftc.gov/PressRoom/PressReleases/8478-22)
- [DOJ Roman Storm conviction (Aug 2025)](https://www.justice.gov/usao-sdny/pr/founder-tornado-cash-crypto-mixing-service-convicted-knowingly-transmitting-criminal)
- [Cashu NUT specifications](https://cashubtc.github.io/nuts/)
- [Cashu NUT-DLC PR #128](https://github.com/cashubtc/nuts/pull/128)
- [Polymarket Resolution UMA docs](https://docs.polymarket.com/developers/resolution/UMA)
- [rust-dlc / DDK](https://github.com/bennyhodl/dlcdevkit)
- [frost-secp256k1-tr on crates.io](https://crates.io/crates/frost-secp256k1-tr)
- [RFC 9591 — FROST](https://datatracker.ietf.org/doc/rfc9591/)
- [LDK Node](https://github.com/lightningdevkit/ldk-node)

### Secondary (MEDIUM confidence)

- [Augur assassination markets retrospective (Newsweek)](https://www.newsweek.com/welcome-augur-cryptocurrency-death-market-where-you-can-bet-donald-trump-1043571)
- [UMA Optimistic Oracle deep dive (Rocknblock)](https://rocknblock.io/blog/how-prediction-markets-resolution-works-uma-optimistic-oracle-polymarket)
- [Tornado Cash trial implications (Mayer Brown)](https://www.mayerbrown.com/en/insights/publications/2025/08/the-tornado-cash-trials-mixed-verdict-implications-for-developer-liability)
- [DDK alpha state](https://benschroth.com/blog/dlcdevkit/)
- [Robosats / Mostro architectural patterns](https://learn.robosats.com)

### Tertiary (needs validation during implementation)

- Lightning DLC channels production status (Atomic.finance, cara) — verify Phase 1
- Cashu federation patterns — verify before Phase 4
- BIP-445 (FROST) standardization timeline — verify Phase 4

---
*Research completed: 2026-05-27*
*Ready for roadmap: yes*
