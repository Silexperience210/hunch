# Hunch — v1 Requirements

**Project:** Hunch
**Milestone:** v1 — Mainnet Cypherpunk Spine
**Status:** Active — pending phase implementation
**Generated:** 2026-05-27

## v1 Requirements

### Protocol Foundation (PROTO)

- [ ] **PROTO-01**: HIP-0 — Protocol overview & cypherpunk manifesto published (Markdown, MIT-licensed, in repo `docs/HIP-0.md`)
- [ ] **PROTO-02**: HIP-1 — Nostr event kinds specification (market, order, oracle announce, oracle attestation, dispute, reputation, mint announce)
- [ ] **PROTO-03**: HIP-2 — DLC contract structure spec (binary YES/NO/INVALID outcome, refund timeout, multi-oracle FROST-ready)
- [ ] **PROTO-04**: HIP-3 — Cashu NUT-DLC integration spec for Hunch (extension or fork of upstream PR #128)
- [ ] **PROTO-05**: HIP-4 — Multi-oracle FROST protocol (for v2, designed in v1 to keep code paths compatible)
- [ ] **PROTO-06**: HIP-5 — Reputation event format and aggregation rules
- [ ] **PROTO-07**: Repo published GitHub (mirror Radicle), MIT license, contributor guide (`CONTRIBUTING.md`), code of conduct (`CODE_OF_CONDUCT.md`)
- [ ] **PROTO-08**: Project instruction file (`CLAUDE.md`) generated for GSD workflow consistency

### Spike Validations (SPIKE)

- [ ] **SPIKE-01**: NUT-DLC current state validated (read PR #128 + #127, contact maintainers, decide fork-vs-upstream)
- [ ] **SPIKE-02**: NUT-DLC working prototype on signet — single market, single mint, single oracle, full flow (create → bet → resolve → withdraw)
- [ ] **SPIKE-03**: FROST k-of-n DKG ceremony tested on signet with 3-of-5 oracle setup; document playbook
- [ ] **SPIKE-04**: Lightning DLC channel current readiness assessment (atomic.finance, cara) — go/no-go decision for v1

### Legal & Governance (LEGAL)

- [ ] **LEGAL-01**: Offshore entity recommendation (Switzerland / Panama / BVI / El Salvador) with counsel input
- [ ] **LEGAL-02**: Crypto-specialized counsel engaged in chosen jurisdiction
- [ ] **LEGAL-03**: Terms of Service drafted with counsel sign-off (prohibit US users, prohibit illegal-use markets, no investment advice disclaimer, tax disclaimer)
- [ ] **LEGAL-04**: Privacy policy drafted (no PII collection beyond Nostr pubkey, no analytics)
- [ ] **LEGAL-05**: PR response playbook ready (abuse markets, CFTC inquiry, deplatforming response)
- [ ] **LEGAL-06**: Maintainer pseudonymity plan documented (if applicable)

### Protocol Services — Mint (MINT)

- [ ] **MINT-01**: `hunch-mint` Rust binary builds and runs from CDK base
- [ ] **MINT-02**: Cashu NUT-04..15 support (mint quote, melt quote, swap, restore, mint info)
- [ ] **MINT-03**: NUT-12 DLEQ proofs implemented (mandatory)
- [ ] **MINT-04**: NUT-DLC extension implemented (issue YES/NO tokens backed by DLC)
- [ ] **MINT-05**: LDK Node integration for Lightning deposits and withdrawals
- [ ] **MINT-06**: DLC contract construction + funding via rust-dlc / DDK
- [ ] **MINT-07**: DLC settlement (CET broadcast on oracle attestation)
- [ ] **MINT-08**: DLC refund timeout handling (if oracle disappears)
- [ ] **MINT-09**: Atomic swap support for secondary market (NUT extension if needed)
- [ ] **MINT-10**: Mint state proofs / reserves audit publishing (weekly Nostr event)
- [ ] **MINT-11**: HTTP API documented (OpenAPI / Cashu NUTs reference)
- [ ] **MINT-12**: Operator key custody via hardware signer or HSM
- [ ] **MINT-13**: Anomaly monitoring + Prometheus metrics

### Protocol Services — Oracle (ORACLE)

- [ ] **ORACLE-01**: `hunch-oracle` Rust binary builds
- [ ] **ORACLE-02**: Single-key Schnorr oracle for v1 (FROST-ready code paths)
- [ ] **ORACLE-03**: NIP-88 oracle announcement publishing
- [ ] **ORACLE-04**: NIP-88 attestation publishing on event resolution
- [ ] **ORACLE-05**: External event watcher (configurable per event — news API, Bitcoin block height, etc.)
- [ ] **ORACLE-06**: Manual override CLI for operator-resolved events
- [ ] **ORACLE-07**: Dispute period support (24-48h between attestation and final settlement)
- [ ] **ORACLE-08**: Public attestation history queryable via Nostr (kind:89 events)
- [ ] **ORACLE-09**: Oracle key custody via hardware signer

### Protocol Services — Relay (RELAY)

- [ ] **RELAY-01**: `relay.hunch.markets` deployed (nostr-rs-relay)
- [ ] **RELAY-02**: Accepts Hunch event kinds (30888, 38888, 38889, 30890, 30891, 30892, 88, 89)
- [ ] **RELAY-03**: NIP-13 PoW filter to deter spam
- [ ] **RELAY-04**: NIP-65 outbox model documented for users
- [ ] **RELAY-05**: Tor-accessible
- [ ] **RELAY-06**: Backup community relays recommended in docs

### Frontend (UI)

- [ ] **UI-01**: Next.js 15 + TypeScript app builds static export
- [ ] **UI-02**: Tailwind + shadcn/ui base theme (cyberpunk dark mode)
- [ ] **UI-03**: NDK integration for Nostr (NIP-07 + NIP-46 signer support)
- [ ] **UI-04**: cashu-ts integration for mint operations
- [ ] **UI-05**: WebLN integration for Lightning payments
- [ ] **UI-06**: Onboarding flow (explains: not Hunch's product, no KYC, you self-custody, oracle marketplace)
- [ ] **UI-07**: Browse markets page (categories, search, trending, social-graph filter default)
- [ ] **UI-08**: Market detail page (oracle info + reputation, price chart, resolution rules, buy/sell box)
- [ ] **UI-09**: Buy YES / Buy NO flow (sat amount → WebLN pay → Cashu token receive)
- [ ] **UI-10**: Sell position flow (publish atomic swap order on Nostr or via mint)
- [ ] **UI-11**: Resolution flow (oracle attestation visible, settlement tx link, "verify yourself" UI)
- [ ] **UI-12**: Permissionless market creation form (question + outcome rule + oracle pick + expiry)
- [ ] **UI-13**: Account / portfolio page (positions across markets, P&L per market, tx history)
- [ ] **UI-14**: Oracle marketplace page (browse oracles, see their reputation, attestation history)
- [ ] **UI-15**: Settings page (mute users, mute topics, choose relays, choose mints)
- [ ] **UI-16**: Reputation event publishing UI (vouch / report oracles, markets, creators)
- [ ] **UI-17**: Multi-language (EN + FR for v1)
- [ ] **UI-18**: Tor Browser compatibility (no flows broken under Tor)
- [ ] **UI-19**: Mobile responsive (works on phones / PWA install prompt)
- [ ] **UI-20**: Geo-block US infrastructure (IP-based + Tor exit list + ToS gate)
- [ ] **UI-21**: Documented self-hosting guide (anyone can deploy a Hunch frontend in <1h)

### Anti-Spam & Curation (CURATE)

- [ ] **CURATE-01**: Default social graph filter (1-2 hop follow filter on browse page)
- [ ] **CURATE-02**: User mute lists (mute creators, mute topic keywords)
- [ ] **CURATE-03**: Trending threshold (minimum N unique participants before market reaches trending)
- [ ] **CURATE-04**: INVALID outcome in DLC (oracle attests invalid → 50/50 refund CET)
- [ ] **CURATE-05**: Frontend curation policy (hunch.io ToS hides violence / harm / sanctioned-entity markets)
- [ ] **CURATE-06**: Community report event support (kind:30891 with "invalid" flag, summed for UI signal)
- [ ] **CURATE-07**: New-creator gradual visibility (markets from unknown creators surface slowly)

### Distribution & Operations (OPS)

- [ ] **OPS-01**: hunch.io deployed (Cloudflare Pages)
- [ ] **OPS-02**: hunch.onion Tor hidden service live
- [ ] **OPS-03**: IPFS pin of frontend bundle (Pinata + web3.storage)
- [ ] **OPS-04**: GitHub + Radicle mirror sync
- [ ] **OPS-05**: Public status page (status.hunch.markets)
- [ ] **OPS-06**: Monitoring dashboard (Prometheus + Grafana) on mint + oracle + relay
- [ ] **OPS-07**: Incident response playbook documented (mint rug suspicion, oracle lie, audit findings, regulatory inquiry)
- [ ] **OPS-08**: Operator key backup tested (recovery from cold storage in <24h)
- [ ] **OPS-09**: Public Nostr presence + Discord/Matrix room for community
- [ ] **OPS-10**: Operator-seeded markets ready at launch (50+ markets across politics, sport, crypto, culture)
- [ ] **OPS-11**: Operator-provided liquidity (LP both sides of first markets for first weeks)

### Security & Audit (SEC)

- [ ] **SEC-01**: External security audit firm selected (Bitcoin DLC + Cashu expertise)
- [ ] **SEC-02**: Mutinynet stress test completed (2-3 months minimum before mainnet)
- [ ] **SEC-03**: Audit report received, all High / Critical findings resolved
- [ ] **SEC-04**: Bug bounty program live BEFORE mainnet (HackerOne / Cantina / equivalent)
- [ ] **SEC-05**: Crypto-specific review by cryptographer (FROST integration, blind sig usage)
- [ ] **SEC-06**: Supply chain audit (cargo-audit + cargo-deny + cargo-vet, npm audit clean)
- [ ] **SEC-07**: Operator key custody hardened (HSM / hardware signer with documented procedures)
- [ ] **SEC-08**: No-custody-by-design documented and verifiable (mint never holds settled funds; settlement via DLC)

### Mainnet Launch Gate (LAUNCH)

- [ ] **LAUNCH-01**: Invite-only mainnet phase complete (week 1-4, max 100k sats per market)
- [ ] **LAUNCH-02**: Public mainnet with moderate caps (max 1M sats per market) — month 2-3
- [ ] **LAUNCH-03**: Caps removed (full mainnet hardcore) after 90 days without major incident
- [ ] **LAUNCH-04**: 50+ markets actively trading at public launch
- [ ] **LAUNCH-05**: Multi-relay coverage (Hunch + 3+ community relays publishing market events)
- [ ] **LAUNCH-06**: At least one external oracle running (independent of Hunch) to validate marketplace model

## v2 Requirements (Deferred After v1 Validation)

- [ ] FROST k-of-n multi-oracle on production markets
- [ ] Federated mint (multi-operator FROST multisig)
- [ ] P2P Tier 2 matching via Nostr (Robosats-style for non-custodial path during market lifetime)
- [ ] Multi-outcome markets (>YES/NO) via Schnorr adaptor signatures
- [ ] Conditional / parlay markets
- [ ] Lightning DLC channels for instant settle
- [ ] Advanced reputation algorithm (time-decayed, cross-market correlation)
- [ ] Nostr DM resolution notifications (NIP-44)
- [ ] Multi-language expansion (ES, PT, RU, DE)
- [ ] Pro UX (hotkeys, dense view, advanced filters)
- [ ] Zap-to-bet from any Nostr client
- [ ] Operator continuity / handoff playbook (open-source governance)
- [ ] OpenSats / HRF / Spiral grant applications

## v3+ (Future)

- [ ] Cross-mint atomic-swap routing
- [ ] Programmatic oracles (verifiable computation, block-height-triggered resolution)
- [ ] API / SDK for external traders + bots
- [ ] Mobile native (Tauri or React Native, only if PWA hits engagement signal)
- [ ] DAO governance via Nostr signed votes (only after federation proven)
- [ ] Foundation establishment (formalize offshore entity)
- [ ] BIP-445 FROST native Bitcoin support integration

## Out of Scope (Explicit Exclusions)

- **Token / governance token** — Securities law risk. Hunch has no native token.
- **Fiat on-ramps** — Lightning-only entry. Users bring their own sats.
- **US user access** (official frontend) — Geo-block + ToS. Forks free to host elsewhere.
- **Custodial backup / account recovery** — Self-sovereign. User keys = user responsibility.
- **KYC of any kind** — Foundational principle.
- **AMM / liquidity pools** — Orderbook + atomic swap only (smart contracts not available on Bitcoin).
- **Closed-source dependencies** — Every layer must be auditable open source.
- **Centralized order matching engine** — Mint orderbook (Tier 1) + Nostr P2P (Tier 2).
- **Email / SMS / phone notifications** — KYC vector. Nostr DMs only.
- **In-app chat / social feed** — Moderation burden. Link to Nostr clients.
- **Real-money leveraged products** — Different legal regime.
- **Casino-style side games** — Dilutes prediction market focus.
- **Margin / borrowing against positions** — Requires custody/liquidation engine.

## Traceability

| REQ-ID | Phase | Status |
|--------|-------|--------|
| PROTO-01 | Phase 1 — Cypherpunk Foundation | Pending |
| PROTO-02 | Phase 1 — Cypherpunk Foundation | Pending |
| PROTO-03 | Phase 1 — Cypherpunk Foundation | Pending |
| PROTO-04 | Phase 1 — Cypherpunk Foundation | Pending |
| PROTO-05 | Phase 1 — Cypherpunk Foundation | Pending |
| PROTO-06 | Phase 1 — Cypherpunk Foundation | Pending |
| PROTO-07 | Phase 1 — Cypherpunk Foundation | Pending |
| PROTO-08 | Phase 1 — Cypherpunk Foundation | Pending |
| SPIKE-01 | Phase 1 — Cypherpunk Foundation | Pending |
| SPIKE-02 | Phase 1 — Cypherpunk Foundation | Pending |
| SPIKE-03 | Phase 1 — Cypherpunk Foundation | Pending |
| SPIKE-04 | Phase 1 — Cypherpunk Foundation | Pending |
| LEGAL-01 | Phase 1 — Cypherpunk Foundation | Pending |
| LEGAL-02 | Phase 1 — Cypherpunk Foundation | Pending |
| LEGAL-03 | Phase 1 — Cypherpunk Foundation | Pending |
| LEGAL-04 | Phase 1 — Cypherpunk Foundation | Pending |
| LEGAL-05 | Phase 1 — Cypherpunk Foundation | Pending |
| LEGAL-06 | Phase 1 — Cypherpunk Foundation | Pending |
| MINT-01..13 | Phase 2 — Mainnet Spine | Pending |
| ORACLE-01..09 | Phase 2 — Mainnet Spine | Pending |
| RELAY-01..06 | Phase 2 — Mainnet Spine | Pending |
| UI-01..21 | Phase 2 — Mainnet Spine | Pending |
| CURATE-01..07 | Phase 2 — Mainnet Spine | Pending |
| OPS-01..06 | Phase 2 — Mainnet Spine | Pending |
| OPS-07..11 | Phase 3 — Mainnet Launch & Hardening | Pending |
| SEC-01..08 | Phase 3 — Mainnet Launch & Hardening | Pending |
| LAUNCH-01..06 | Phase 3 — Mainnet Launch & Hardening | Pending |

**Coverage:** 84 v1 requirements, 100% mapped to a phase. v2 items (Phase 4 / future milestones) tracked separately when v2 milestone opens.

---
*Requirements generated: 2026-05-27*
*Mode: YOLO • Granularity: Coarse • Configured for solo dev*
