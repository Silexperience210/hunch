# Phase 2: Mainnet Spine — Context

**Gathered:** 2026-05-28
**Status:** Ready for planning
**Source:** Inline context — carries Phase 1 locked decisions + adds Phase 2 specifics

<domain>
## Phase Boundary

Phase 2 delivers the **operational reference implementation** for Hunch v1:

1. **Protocol services** (Rust): `hunch-mint` (Cashu + NUT-CTF + LDK Node + DLC), `hunch-oracle` (NIP-88 + FROST-ready single-key v1), `hunch-relay` (nostr-rs-relay configured for Hunch kinds)
2. **Reference frontend** (TypeScript / Next.js 15): full bet flow on hunch.io + hunch.onion + IPFS pin
3. **Anti-spam + distribution infrastructure**: social-graph filter, INVALID outcome CET, curation policy, Tor hidden service, monitoring stack

Phase 2 explicitly does NOT include: external security audit (Phase 3), live mainnet launch (Phase 3), multi-oracle federation (Phase 4 deferred), Lightning-DLC channels (NO-GO per Phase 1 SPIKE-04).

**Target operational state at Phase 2 close:**

- Working end-to-end signet flow (create → bet → resolve → withdraw)
- Three frontend mirrors live (hunch.io / hunch.onion / IPFS pin) serving identical builds
- External operators can deploy mint+oracle+relay in <1 hour following the operator guide
- Geo-block US functional (IP + Tor exit list + ToS gate)
- Single Hunch-operated oracle live + at least one external oracle registered via NIP-88

</domain>

<decisions>
## Implementation Decisions

### Stack (carried from Phase 1 CONTEXT.md D-08, locked)

**Rust services:**
- Workspace at root: `hunch-protocol`, `hunch-mint`, `hunch-oracle`, `hunch-relay`, `hunch-matcher`, `hunch-cli` (stubs created Plan 01 commit `10cc7a0`)
- Cashu: `cdk` (cashubtc/cdk, Rust reference) — Hunch tracks cdk's NUT-CTF support as PR #337 stabilizes
- DLC: `rust-dlc` (p2pderivatives) + `dlcdevkit` / DDK (bennyhodl) — alpha; pin in Cargo.lock
- Lightning: `ldk-node` v0.1.1+ — embedded, NOT atomic.finance-fork-based (Phase 1 SPIKE-04 NO-GO)
- Threshold sigs: `frost-secp256k1-tr` v2.2+ — Phase 2 implements FROST-ready code paths (single-key v1 + multi-key code structure for v2)
- Nostr: `nostr-sdk` (rust-nostr) — production-stable
- Bitcoin: `bdk_wallet` 1.0+ — wallet primitives for DLC funding inputs

**Frontend stack:**
- Next.js 15 (App Router) with static export (output: 'export')
- TypeScript strict mode
- Tailwind CSS + shadcn/ui (cyberpunk dark theme — UI-02)
- NDK (`@nostr-dev-kit/ndk`) for Nostr — already in package.json from Plan 01 + 02
- cashu-ts for mint operations
- WebLN for Lightning (Alby / Mutiny / Phoenix compatible)
- Bun toolchain (package.json + scripts/* established in Plan 01)
- Build target: static HTML/JS/CSS bundle deployable to Cloudflare Pages + IPFS + Tor hidden service

### Multi-Bettor DLC Pattern (NUT-CTF, locked per Phase 1 D-01)

Per HIP-3 (drafted Plan 02): the Hunch mint is the **bilateral DLC counterparty** at the Bitcoin layer; bettors transact with the mint via **NUT-CTF conditional tokens** (PR #337).

For Phase 2 mint impl:
- `MINT-04` "NUT-DLC extension" → re-read as "NUT-CTF integration per HIP-3 + PR #337"
- `MINT-06` "DLC construction" → backs the mint as bilateral counterparty per HIP-2
- `MINT-07` "DLC settlement" → triggered by oracle kind:89 attestation per HIP-1
- `MINT-08` "DLC refund timeout" → matches HIP-2 refund branch (CSV-locked)

### Lightning-DLC NO-GO (locked per Phase 1 D-03)

Lightning is for **deposit + withdrawal only** (Cashu mint → LDK Node → user). DLC settlement happens on-chain (Bitcoin transactions). No DLC inside Lightning channels in v1. Reconsideration triggers documented in `spikes/lightning-dlc/GO-NOGO.md`; revisit only in Phase 4.

### Oracle: Single-Key for v1, FROST-Ready Code Paths

Per `ORACLE-02`: v1 ships with a single Hunch-operated oracle + Schnorr signing. The code is structured so swapping the single-key with a FROST aggregate signer (HIP-4) does not require API changes downstream — oracle key handling is abstracted behind a `OracleSigner` trait that has both `SingleKeySigner` and (Phase 4) `FrostSigner` implementations.

External oracles register via NIP-88 (kind:88 announce) and can publish attestations alongside the Hunch oracle. Market creators select the oracle they trust; the protocol does not endorse one.

### Reference Frontend Wallet Integration

Per `UI-03`: support **NIP-07 (browser extension signer)** + **NIP-46 (remote signer / Nostr Connect)**. NIP-07 is the default (Alby, nos2x, getalby.com extensions). NIP-46 enables mobile users + hardware-backed Nostr signers (Amber, nsec.app).

No bunker mode auto-generated keys in v1 (security risk — user opsec must be deliberate). The onboarding flow (`UI-06`) explicitly guides users to install Alby or use NIP-46.

### Anti-Spam Pattern (locked from PITFALLS Pitfall 4)

Per `CURATE-01..07`:
- **Default ON 1-2 hop social graph filter** on browse page (kind:30000 contact lists; markets from creators not in the user's follow graph at distance ≤ 2 are filtered)
- **User mute lists** (NIP-51 lists) for creators + topics
- **INVALID outcome CET** in every DLC (per HIP-2 §INVALID Outcome Semantics + MINT-04 + CURATE-04)
- **Frontend curation policy** (TERMS §5 strawman from Phase 1; hunch.io ToS hides violence / harm / sanctioned-entity markets at frontend layer — protocol stays neutral)
- **New-creator gradual visibility** (`CURATE-07`): markets from creators with no prior reputation surface slowly; promotion accelerates as reputation accrues per HIP-5

### Distribution Channels (per CLAUDE.md principle 8 — Tor + IPFS first)

`OPS-01..06`:
- `hunch.io` — Cloudflare Pages static deployment (clearnet convenience)
- `hunch.onion` — Tor hidden service serving identical static bundle
- IPFS pin via Pinata + web3.storage (immutable backup)
- GitHub + Radicle + Codeberg mirrors (defense in depth)
- Public status page at `status.hunch.markets` (uptime monitor)
- Prometheus + Grafana on mint + oracle + relay

### Pseudonymity (carried from Phase 1 D-05, locked)

All commits in Phase 2 signed under `Silex <silex@hunch.markets>` per-repo git config. No real-name attribution in code, comments, commit messages, or operator-facing documentation. Domains (hunch.io, hunch.markets) registered through privacy-respecting registrar (Njalla recommended) with WHOIS proxy.

### Audit Hold (locked per CLAUDE.md "Don't")

Phase 2 ships to **signet only**. Mainnet exposure is gated on Phase 3 audit signoff + bug bounty + 2-3 months Mutinynet stress test. Phase 2 verification criteria explicitly forbid mainnet deployment as a "done" gate.

### Claude's Discretion (planner picks)

- Specific shadcn/ui components (button, card, dialog, etc.) — planner picks based on UX requirements
- Specific Lightning routing strategy within LDK Node — planner picks
- Specific Prometheus metric set per service — planner picks based on operability heuristics
- Specific Tor hidden service hosting (riseup.net, OnionShare, self-host) — planner picks
- Specific monitoring dashboard layout — planner picks
- Test framework (`cargo test` + `insta` + `proptest` for Rust; `vitest` + `playwright` for frontend) — planner picks

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Project canon
- `C:\Users\Silex\Hunch\.planning\PROJECT.md` — project identity
- `C:\Users\Silex\Hunch\.planning\REQUIREMENTS.md` — 84 reqs; Phase 2 covers MINT-01..13 + ORACLE-01..09 + RELAY-01..06 + UI-01..21 + CURATE-01..07 + OPS-01..06 = 62 reqs
- `C:\Users\Silex\Hunch\.planning\ROADMAP.md` — Phase 2 section: 3 plans + 5 success criteria + 12-16 weeks solo estimate
- `C:\Users\Silex\Hunch\CLAUDE.md` — cypherpunk principles + Engineering Principles + Don'ts + Dos

### Phase 1 deliverables (used as Phase 2 inputs)
- `C:\Users\Silex\Hunch\docs\HIP-0.md` — Protocol overview
- `C:\Users\Silex\Hunch\docs\HIP-1.md` — Nostr event kinds (Phase 2 mint/oracle/relay/frontend all consume)
- `C:\Users\Silex\Hunch\docs\HIP-2.md` — DLC contract structure (Phase 2 MINT impl follows)
- `C:\Users\Silex\Hunch\docs\HIP-3.md` — Cashu NUT-CTF integration (Phase 2 MINT-04 follows; Status: Draft until SPIKE-02 success)
- `C:\Users\Silex\Hunch\docs\HIP-4.md` — Multi-oracle FROST (Phase 2 ORACLE-02 FROST-ready code paths)
- `C:\Users\Silex\Hunch\docs\HIP-5.md` — Reputation event format (Phase 2 UI-16 publishes; backend ignores)
- `C:\Users\Silex\Hunch\.planning\phases\01-cypherpunk-foundation\01-CONTEXT.md` — Phase 1 locked decisions (NUT-CTF pivot, Lightning-DLC NO-GO, pseudonymity)
- `C:\Users\Silex\Hunch\.planning\phases\01-cypherpunk-foundation\01-RESEARCH.md` — 1188-line research; Phase 2 inherits

### Cargo workspace state
- `C:\Users\Silex\Hunch\Cargo.toml` — 6 stub crates registered; Phase 2 implements the bodies
- `C:\Users\Silex\Hunch\crates\hunch-*` — 6 stub crates with `//!` doc-comments pointing to HIPs

### External specs (read before implementing)
- `https://github.com/cashubtc/nuts/pull/337` — NUT-CTF (the chosen path; MINT-04 implements against this)
- `https://github.com/cashubtc/cdk` — CDK Rust mint reference
- `https://github.com/p2pderivatives/rust-dlc` — DLC primitives
- `https://github.com/bennyhodl/dlcdevkit` — DDK higher-level DLC toolkit
- `https://github.com/lightningdevkit/ldk-node` — Embedded Lightning
- `https://github.com/rust-nostr/nostr` — Rust Nostr SDK
- `https://github.com/nostr-protocol/nips/pull/1681` — NIP-88 (oracle kinds; draft, Phase 2 tracks)
- `https://github.com/nostr-dev-kit/ndk` — TS Nostr SDK (frontend)
- `https://github.com/cashubtc/cashu-ts` — TS Cashu SDK (frontend)
- `https://nextjs.org/docs/app` — Next.js 15 App Router
- `https://ui.shadcn.com` — shadcn/ui component library

</canonical_refs>

<specifics>
## Specific Ideas

- **Mutinynet signet faucet** for end-to-end testing: https://faucet.mutinynet.com
- **Cypherpunk visual register**: UI-02 cyberpunk dark theme — high-contrast monospace + accent terminal-green or hot-pink; planner picks one accent
- **Reserves proof publication**: weekly Nostr event (kind:30892 mint announce + reserves URL); planner specifies URL scheme + signing
- **Self-hosting guide** (UI-21): docker-compose + bash script + 1-pager; targets a competent operator 1-hour deploy
- **Cargo workspace additions**: `crates/hunch-mint-spike` (Plan 03 SPIKE-02) + `crates/hunch-oracle-spike` (Plan 03 SPIKE-03) become workspace members for the test impls; Phase 2 production crates (`hunch-mint`, `hunch-oracle`) ship alongside
- **Reference Lightning wallet**: Phase 2 ships against Mutiny / Phoenix / Alby (all WebLN); no proprietary wallet lock-in
- **Frontend i18n**: EN + FR for v1 (UI-17). Phase 2+ adds more languages.
- **PWA mode**: install prompt (UI-19) — manifest.json + service worker

</specifics>

<deferred>
## Deferred Ideas

- **Multi-oracle FROST live ceremony** — code paths ready in v1 per HIP-4; live ceremony in Phase 4 with 5 independent operators
- **HSM-backed operator key custody** (MINT-12, ORACLE-09) — Phase 2 ships with hardware-signer support (Ledger / Trezor / ColdCard); HSM-grade custody is Phase 3 hardening
- **Lightning-DLC channels** — NO-GO confirmed Phase 1 SPIKE-04; reconsideration triggers in `spikes/lightning-dlc/GO-NOGO.md`
- **Federated mint** — single Hunch mint in v1; multi-mint federation is Phase 4
- **Tier 2 P2P matcher production rollout** — `hunch-matcher` crate exists as stub; Phase 2 implements basic ephemeral kind:38888 order publishing in the frontend, full matcher engine in Phase 4
- **Mobile native app** — Phase 2 ships PWA only; iOS / Android native apps Phase 4+
- **Mainnet launch with caps** — Phase 3 deliverable (audit + bug bounty + tiered launch)
- **Counsel sign-off PDFs** — Phase 1+ followup (`docs/legal/PHASE-1-FOLLOWUP.md`); Phase 2 ships against Phase 1 strawmen with explicit "draft, counsel-pending" disclosure in frontend onboarding

</deferred>

---

*Phase: 02-mainnet-spine*
*Context captured: 2026-05-28 inline (skipped /gsd-discuss-phase per user preference; all Phase 1 locked decisions carried over)*
