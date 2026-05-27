# Hunch

> *"Trust the math. Trust the oracle's signature. Don't trust us — we shouldn't be a trust point."*

## What This Is

Hunch is a **permissionless, cypherpunk prediction market protocol on Bitcoin**. Anyone can create a market about any question (politics, sport, culture, crypto, news, meta-Nostr) and bet on the outcome using Lightning. Settlement uses Discreet Log Contracts (DLCs); liquidity flows through competing Cashu mints (NUT-DLC); market discovery and oracles publish on Nostr. The protocol is neutral — Hunch the organization runs the reference frontend and a reference Cashu mint, but the protocol survives any single operator.

## Core Value

**Anyone, anywhere (except where geo-blocked), can bet on any verifiable question without KYC, without custody, and without trusting Hunch as an institution — only the oracle's Schnorr signature and the math of the DLC.**

## Requirements

### Validated

<!-- Shipped and confirmed valuable. -->

(None yet — ship to validate)

### Active

<!-- Current scope. Building toward these. v1 hypotheses until shipped. -->

**Protocol layer**
- [ ] Define HIP-0..N specifications (Nostr event kinds, DLC contract structure, Cashu NUT-DLC extension, oracle attestation format)
- [ ] Open-source reference implementations under MIT license

**Settlement layer**
- [ ] DLC-based escrow for each market (rust-dlc / DDK)
- [ ] Multi-oracle threshold Schnorr (FROST k-of-n) signing for resolution
- [ ] Refund timeout fallback if oracle disappears
- [ ] Lightning deposits and withdrawals (LDK Node)

**Liquidity layer**
- [ ] Reference Cashu mint with NUT-DLC support (cdk fork or extension)
- [ ] YES/NO token issuance backed by DLC collateral
- [ ] Atomic-swap secondary market for tokens
- [ ] Protocol-level support for multiple competing mints (no single mint required)

**Market layer**
- [ ] Permissionless market creation (anyone can publish a market on Nostr)
- [ ] Binary YES/NO markets in v1
- [ ] Oracle marketplace: anyone can announce themselves as an oracle; market creators choose
- [ ] Reputation events on Nostr for oracles, mints, and market creators
- [ ] Anti-spam / discovery via Nostr social graph

**Frontend layer**
- [ ] Web frontend (Next.js, static export) — deployable on Cloudflare, IPFS, Tor hidden service
- [ ] WebLN integration for Lightning payments
- [ ] Nostr login (NIP-07 / NIP-46)
- [ ] Geo-block IP-based for US on the official frontend (ToS + technical block)

**Cypherpunk infrastructure**
- [ ] Repo mirrored on GitHub + Radicle
- [ ] Tor hidden service from day 1
- [ ] IPFS pin for frontend
- [ ] No telemetry, no analytics, no cookies on the protocol layer

### Out of Scope

<!-- Explicit boundaries. -->

- **Token / governance token** — Legal risk (securities). Hunch has no native token. Bitcoin is the token. *Decision: revisit if/when DAO governance becomes critical.*
- **Fiat on-ramps** — Lightning-only entry. Users bring their own sats. *Decision: avoids KYC vector entirely.*
- **US user access on the official frontend** — Geo-block + ToS. Forks are free to host elsewhere. *Decision: legal liability mitigation, lesson from Polymarket DOJ case.*
- **Custodial backup / account recovery** — Self-sovereign means user keys are user responsibility. *Decision: Nostr key + Lightning wallet under user control.*
- **KYC of any kind** — Period. *Decision: foundational principle.*
- **Mobile native (iOS/Android) in v1** — Web PWA is enough. Native in v3+ if traction. *Decision: scope discipline.*
- **AMM / liquidity pools** — Orderbook + atomic swap only. AMMs need smart contracts Bitcoin doesn't have. *Decision: technical constraint.*
- **Multi-outcome markets (>YES/NO) in v1** — Binary first. Multi-outcome in v2 via Schnorr adaptor signatures. *Decision: scope discipline.*
- **Closed-source dependencies** — Every layer must be auditable open source. *Decision: cypherpunk foundational principle.*

## Context

### Domain landscape

**Polymarket** (Polygon, Ethereum L2) is the dominant prediction market today. CTF (Gnosis Conditional Token Framework) for shares, UMA optimistic oracle for resolution, USDC, orderbook + AMM hybrid. Geo-blocks US after $1.4M DOJ settlement in 2022. Strong product, but custodial relative to user funds via USDC, and on Ethereum.

**Augur** (Ethereum) was the cypherpunk OG — pure on-chain, REP governance token, decentralized oracle market. Failed on UX and liquidity. Lessons: don't make oracles a governance token problem; orderbook needs to be fast; markets need curation.

**Bisq / Robosats / Mostro** prove that Bitcoin/Lightning P2P trading via Nostr can work for niche but committed audiences. Robosats has thousands of monthly users with zero custody and zero KYC.

**DLC ecosystem** is maturing. rust-dlc (LavaXr / Itchysats), DLC.link, Suredbits oracle. Limited but production-grade. Lightning channels with DLCs (atomic.finance) are emerging.

**Cashu** ecosystem is on fire 2025-2026. Multiple mints (Calle, Gandlaf, others). NUT-12 (DLEQ proofs), NUT-11 (P2PK locks), NUT-DLC proposal in active development.

**Nostr** as a discovery + identity + reputation layer is the missing piece older Bitcoin protocols didn't have. Mostro and Robosats already use it for P2P matching.

### Technical building blocks

| Layer | Tool | Maturity |
|---|---|---|
| Bitcoin tx | BDK | Mature |
| Lightning | LDK / CLN | Mature |
| DLC | rust-dlc / DDK (dlcdevkit) | Production for binary outcomes |
| Cashu mint | cdk (Rust) | Mature for v1 NUTs |
| Cashu NUT-DLC | proposal in dev (Calle/Gandlaf) | Early, may need to upstream contributions |
| Nostr | NDK / nostr-sdk | Mature |
| Oracle | rust-dlc-oracle + custom NIP-88 | Mature for binary; need custom FROST k-of-n |
| Frontend | Next.js + cashu-ts + WebLN + NDK | Mature |

### Prior work / inspirations

- Polymarket UX (the gold standard for what users expect)
- Augur (lessons in what NOT to do)
- Robosats (model for Tor / P2P / Bitcoin-only / no-KYC)
- Mostro (Nostr-native order matching)
- Bisq (decentralized exchange that has survived since 2014)
- Cashu mints (custodial-but-private model)

## Constraints

- **Tech stack**: Rust for protocol-critical services (mint, oracle, DLC); TypeScript for frontend. No JVM, no Python. All open source MIT.
- **Bitcoin-only settlement**: No altcoins, no stablecoins, no L2s except Lightning and Cashu (which is a Bitcoin-only Chaumian mint protocol).
- **Solo developer initially**: Roadmap must be achievable by one experienced developer (then opened to contributors). 6-12 month horizon to v1 mainnet.
- **Mainnet hardcore launch**: No artificial caps. But this means **mandatory** external security audit before mainnet, plus extensive testnet/signet validation.
- **Geographic exclusion**: Official frontend geo-blocks US. Protocol stays neutral. ToS shifts legal exposure.
- **No US presence**: No US-based legal entity, no US-domiciled key contributors involved in operations, no US marketing.
- **License**: MIT, no contributor agreements that imply ownership.
- **Discovery on Nostr**: No central database of markets. Markets are Nostr events, indexable by any relay.

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Name = "Hunch" | Short, evokes intuition + truth-seeking | — Pending |
| Protocol-first architecture (specs before code) | Survival beyond Hunch Inc, attracts contributors | — Pending |
| Cashu mint as Tier 1 liquidity, P2P DLC as Tier 2 | Best UX + non-custodial purist path coexist | — Pending |
| FROST k-of-n multi-oracle for non-trivial markets | No single oracle can rug; threshold cryptography native to Schnorr/Bitcoin | — Pending |
| Permissionless market creation (Nostr events) | "Anyone can ask any question" was a hard requirement from project owner | — Pending |
| Reputation, not slashing, for oracles/mints | Bitcoin has no smart contract to slash; reputation works for social systems (cf. Robosats) | — Pending |
| Mainnet without caps, but only after audit | Project owner choice. Higher credibility + higher risk; audit is the safety net | — Pending |
| MIT license, GitHub + Radicle mirror | Maximum freedom for forks, censorship resistance | — Pending |
| No token, ever (so far) | Securities law nightmare; misaligned incentives historically (cf. Augur REP) | — Pending |
| Solo dev → contributors open | Realistic for one person to bootstrap; community attracted by working code | — Pending |
| Geo-block US on official frontend | Mitigates DOJ exposure (Polymarket lesson); protocol neutral so others can host | — Pending |
| Operator revenue via mint fees only | Aligned incentives (mint serves users); no token = no securities risk | — Pending |

## Evolution

This document evolves at phase transitions and milestone boundaries.

**After each phase transition** (via `/gsd-transition`):
1. Requirements invalidated? → Move to Out of Scope with reason
2. Requirements validated? → Move to Validated with phase reference
3. New requirements emerged? → Add to Active
4. Decisions to log? → Add to Key Decisions
5. "What This Is" still accurate? → Update if drifted

**After each milestone** (via `/gsd-complete-milestone`):
1. Full review of all sections
2. Core Value check — still the right priority?
3. Audit Out of Scope — reasons still valid?
4. Update Context with current state

---
*Last updated: 2026-05-27 after initialization*
