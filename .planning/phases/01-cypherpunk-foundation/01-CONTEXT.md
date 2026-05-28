# Phase 1: Cypherpunk Foundation — Context

**Gathered:** 2026-05-28
**Status:** Ready for planning
**Source:** Inline decisions captured after RESEARCH.md surfaced 3 must-lock items

<domain>
## Phase Boundary

Phase 1 delivers three independent but co-staged streams that must complete before any mainnet code is committed:

1. **Protocol identity** — HIP-0..5 specs, monorepo scaffolding, public licensing
2. **Technical de-risking** — NUT-DLC + FROST signet prototypes that prove the architecture is buildable
3. **Legal foundation** — counsel engagement, offshore entity, ToS/privacy/PR playbook

Phase 1 explicitly does NOT include: production mint, oracle, relay, or frontend code. Those belong to Phase 2 (Mainnet Spine).

</domain>

<decisions>
## Implementation Decisions

### NUT-DLC Strategy (locked 2026-05-28)

- **Path A — NUT-CTF (PR #337)** is the chosen direction. Pivot from the obsolete PR #128 (closed 2025-05-20) to joemphilips's "NUTs for Prediction Markets" (Conditional Token Framework, oracle-agnostic mint).
- SPIKE-01 must contact `joemphilips`, `conduition`, and Calle (cashubtc maintainer) to validate the chosen path and confirm Hunch can contribute upstream.
- HIP-3 (Cashu NUT-DLC integration spec) MUST be written against PR #337's architecture, NOT the bilateral-DLC pattern documented in `.planning/research/ARCHITECTURE.md`.
- Path B (resurrect PR #128) and Path C (custodial-promise) are explicitly de-scoped from v1.
- **Corrigendum required:** `.planning/research/STACK.md`, `ARCHITECTURE.md`, `FEATURES.md`, `PITFALLS.md`, `SUMMARY.md` all reference PR #128 as live. They must be updated in Wave 0 of the protocol-specs plan before HIPs are drafted.

### Offshore Jurisdiction (deferred to LEGAL-01)

- Final jurisdiction selection (CH + BVI vs Panama vs BVI-alone vs other) is delegated to the counsel engaged in LEGAL-02.
- LEGAL-01 produces a **recommendation document** (not a decision) — the counsel's KYC banking observations drive the final pick.
- Working hypothesis for planning purposes only: Swiss Stiftung (protocol foundation) + BVI BC (mint operator) two-entity structure. Plans should not hardcode this; they should write LEGAL-01 as a comparison memo with counsel input.

### Maintainer Pseudonymity Scope (locked 2026-05-28)

- **Full pseudonymity** including frontend ops, not just protocol-core. Hardcore mode.
- Implications that ripple through plans:
  - All commits signed under pseudonym GPG key (already partially in place: `Silex_0xF777C5B8_SECRET.asc`)
  - No real-name attribution in repo (`CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`, `LICENSE` attribution use pseudonym only)
  - Frontend domains registered through privacy-respecting registrars (Njalla, Orangewebsite) with WHOIS proxies
  - Counsel engagement (LEGAL-02) MUST accept pseudonymous client (this becomes a counsel-selection filter)
  - Banking access (LEGAL-01 jurisdiction decision) MUST accept pseudonymous beneficial owner OR use nominee director arrangement
  - PR playbook (LEGAL-05) anticipates doxxing as a threat vector
- This decision is load-bearing post-Tornado Cash/Storm (§ 1960 conviction Aug 6 2025, sentencing Apr 9 2026) and post-Polymarket Coplan raid (Nov 13 2024).

### Stack (locked from research/STACK.md, with NUT-DLC pivot)

- Settlement: DLC via `rust-dlc` + DDK (alpha, pin in Cargo.lock)
- Liquidity: Cashu mint with **NUT-CTF (PR #337)** instead of NUT-DLC
- Lightning: LDK Node embedded
- Discovery/reputation/oracle: Nostr — kinds `30888` (market), `38888` (order), `30890` (dispute), `30891` (reputation), `88`/`89` (oracle, NIP-88 draft)
- Oracle: marketplace multi-oracle, FROST k-of-n with `frost-secp256k1-tr` v2.2+ (RFC 9591, BIP-445 advancing)
- Frontend: Next.js 15 static export + Tor hidden service + IPFS pin + Cloudflare
- **Lightning-DLC channels: NO-GO for v1** (atomic.finance acquired by Lygos Aug 2025; Crypto Garage rust-dlc Lightning work explicitly "not production-ready"). SPIKE-04 produces this go/no-go as a written assessment, not a working prototype.

### Repo Structure (locked from PROJECT.md / CLAUDE.md)

- Cargo workspace at root with crates: `hunch-protocol`, `hunch-mint`, `hunch-oracle`, `hunch-relay`, `hunch-matcher`, `hunch-cli`
- Apps subfolder: `apps/hunch-web` (Next.js, Bun toolchain)
- Specs at `docs/HIP-0.md` through `docs/HIP-5.md`
- MIT license at root
- GitHub primary, Radicle mirror
- Pseudonymous attribution throughout (no real-name in commits, README, CONTRIBUTING)

### Nostr Event Kinds (provisional, not yet reserved)

- `30888` market (parameterized replaceable)
- `38888` order
- `30890` dispute
- `30891` reputation
- `88` oracle announce (NIP-88 draft, PR #919 / PR #1681)
- `89` oracle attestation (NIP-88 draft)
- HIP-1 must include a kind-registration request against the NIP registry to prevent collision once specs go public.

### HIP Publication

- HIPs MUST be published in two channels:
  1. Repo: `docs/HIP-0.md` ... `docs/HIP-5.md` (canonical, MIT-licensed, version-controlled)
  2. Nostr long-form (NIP-23 kind `30023` replaceable events) under the project pseudonym key
- At least one external Bitcoin/Cashu/DLC contributor must publicly review each HIP (target list to be assembled in the protocol-specs plan).

### Claude's Discretion

- Test framework choice for Rust spikes (cargo test, insta, proptest) — planner picks based on stack norms
- Specific signet faucet selection
- Specific Tor hidden service hosting provider for Phase 1 (Phase 2 deals with production frontend)
- HIP-0 manifesto tone (cypherpunk register; planner can draft a strawman)
- FROST 3-of-5 ceremony coordination channel (NIP-44 gift-wrapped DMs are the research recommendation; planner can confirm)

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Project canon
- `C:\Users\Silex\Hunch\.planning\PROJECT.md` — project identity, mainnet hardcore principles
- `C:\Users\Silex\Hunch\.planning\REQUIREMENTS.md` — 84 reqs; Phase 1 covers PROTO-01..08, SPIKE-01..04, LEGAL-01..06
- `C:\Users\Silex\Hunch\.planning\ROADMAP.md` — Phase 1 goal, 3 plans, 4 success criteria
- `C:\Users\Silex\Hunch\CLAUDE.md` — project-specific guidelines (cypherpunk-first, no token, no KYC, no US)

### Research artifacts
- `C:\Users\Silex\Hunch\.planning\phases\01-cypherpunk-foundation\01-RESEARCH.md` — Phase 1 research (1188 lines, authoritative for HIP authoring, NUT-CTF pivot rationale, FROST DKG, legal jurisdiction comparison)
- `C:\Users\Silex\Hunch\.planning\research\STACK.md` — STALE re: NUT-DLC (Wave 0 corrigendum required)
- `C:\Users\Silex\Hunch\.planning\research\ARCHITECTURE.md` — STALE re: NUT-DLC (Wave 0 corrigendum required)
- `C:\Users\Silex\Hunch\.planning\research\FEATURES.md` — STALE re: NUT-DLC (Wave 0 corrigendum required)
- `C:\Users\Silex\Hunch\.planning\research\PITFALLS.md` — 5 critical risks; mostly current
- `C:\Users\Silex\Hunch\.planning\research\SUMMARY.md` — STALE re: NUT-DLC (Wave 0 corrigendum required)

### External specs
- `https://github.com/cashubtc/nuts/pull/337` — NUT-CTF "NUTs for Prediction Markets" (the NEW chosen path)
- `https://github.com/cashubtc/nuts/pull/128` — CLOSED PR #128 (historical reference only, do NOT implement against this)
- `https://github.com/nostr-protocol/nips` — NIP registry (verify kind reservations)
- `https://nips.nostr.com/23` — NIP-23 long-form content (for HIP publication channel)
- `https://github.com/nostr-protocol/nips/pull/919` — NIP-88 oracle event kinds (draft)
- `https://github.com/nostr-protocol/nips/pull/1681` — NIP-88 scope-reduced alternative
- `https://github.com/discreetlogcontracts/dlcspecs` — DLC spec canon (HIP-2 extends this)
- `https://datatracker.ietf.org/doc/rfc9591/` — FROST RFC (HIP-4 references)
- `https://github.com/ZcashFoundation/frost` — `frost-secp256k1-tr` reference impl

</canonical_refs>

<specifics>
## Specific Ideas

- Solo dev bootstrap; estimated 6-8 weeks. Plans must be solo-runnable, no team coordination assumed.
- HIP-0 manifesto draws from the cypherpunk register (Hughes, May, Hettinga). Keep it short (~500-1000 words), no fluff.
- Repo scaffold is partially started — `C:\Users\Silex\Hunch\` has `.planning/`, `Silex_0xF777C5B8_SECRET.asc`. Plans should account for existing artifacts.
- Counsel engagement (LEGAL-02) is the long-pole of Phase 1 (3-6 weeks intake). Plan must run this in parallel with HIP drafting and spikes.
- Spikes happen on signet (NUT-CTF SPIKE-02, FROST SPIKE-03). Faucet: signetfaucet.com or mutinynet.com.
- External reviewer recruitment for HIPs is itself a small workstream — plan should include outreach (Nostr DMs, GitHub issue cross-posts, conference network).

</specifics>

<deferred>
## Deferred Ideas

- **Path B (PR #128 fork)** and **Path C (custodial-promise fallback)** for NUT-DLC — explicitly de-scoped from v1. Reopen only if Path A SPIKE-01 hits a wall.
- **Lightning-DLC channels** — deferred to v2 (Phase 4 federation work or beyond) pending an upstream maintainer revival.
- **FROST production multi-oracle UX** — Phase 1 documents the playbook on signet only. Phase 2 implements the operator-facing tooling. Phase 4 (v2) is true federation.
- **Hunch.io domain registration + Tor hidden service setup** — happens in Phase 2 (Mainnet Spine).
- **CFTC counsel-recommended geo-block tech (IP+Tor exit list)** — Phase 2 frontend work.

</deferred>

---

*Phase: 01-cypherpunk-foundation*
*Context captured: 2026-05-28 after research surfaced PR #128 closure + 3 must-lock decisions*
