# Stack Research

**Domain:** Bitcoin-native cypherpunk prediction market protocol (DLC + Cashu + Lightning + Nostr)
**Researched:** 2026-05-27
**Confidence:** HIGH (most components verified against crates.io / GitHub releases) with one MEDIUM-CONFIDENCE area: Cashu NUT-DLC spec stability.

## Recommended Stack

### Core Technologies

| Technology | Version | Purpose | Why Recommended |
|------------|---------|---------|-----------------|
| **Rust** | 1.80+ (stable) | Protocol services (mint, oracle, matcher) | Every Bitcoin-native primitive we need (DLC, BDK, LDK, CDK, frost-secp256k1) is Rust-native. Single language across critical infra. |
| **BDK (Bitcoin Dev Kit)** | 1.0+ (`bdk_wallet`) | Bitcoin wallet primitives, PSBTs | De facto Bitcoin wallet library for Rust. Powers ItchySats, Mutiny, mempool tools. Modular (Esplora/Electrum backends). |
| **LDK Node** | 0.4+ | Embedded Lightning node | Lightning library that works as embedded library (vs LND/CLN daemon). Best fit for mint operator + future mobile. WASM-capable. |
| **rust-dlc + DDK (dlcdevkit)** | rust-dlc 0.7+ / DDK 0.0.17+ | DLC contracts, oracle integration | Production-ish DLC stack maintained by bennyhodl. ⚠️ DDK is alpha — APIs will move. Used in Atomic Finance, ItchySats. |
| **CDK (Cashu Dev Kit)** | 0.14+ | Cashu mint + wallet primitives | Reference Cashu in Rust. Supports NUT-04..28 (incl. P2PK, P2BK, batch minting, DLEQ). Active dev in 2026 (NUT-29 batch minting). |
| **frost-secp256k1-tr** | 2.2+ | FROST threshold Schnorr (Taproot-compatible) | ZF Foundation crate, Taproot-compatible signatures. RFC 9591 compliant. **BIP-445 (2026-01-30)** moving toward Bitcoin standardization. |
| **nostr-sdk (rust-nostr)** | 0.34+ | Nostr client/relay for Rust services | Reference Rust Nostr library by yukibtc. Used by Mostro, multiple production Nostr apps. |
| **Next.js** | 15.x (App Router, static export) | Frontend | Best static-export DX, deployable on Cloudflare Pages, IPFS, Tor hidden service. PWA capable. |
| **TypeScript** | 5.5+ | Frontend language | Standard for Nostr/Cashu/WebLN ecosystem. |
| **NDK (Nostr Dev Kit)** | 2.10+ | Nostr client for frontend | Used by primal, damus.io web, Highlighter. Best abstraction for relay management + signer (NIP-07/46). |
| **cashu-ts** | 2.x | Cashu wallet for frontend | Reference TypeScript Cashu library. Maintained by Calle. |
| **WebLN** | spec | Lightning bridge for browser | Standard Lightning browser API (Alby, Mutiny, Zeus). |

### Supporting Libraries

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `bitcoin` (rust-bitcoin) | 0.32+ | Low-level Bitcoin primitives | Direct script construction, PSBT manipulation if BDK abstraction is insufficient. |
| `lightning` (rust-lightning) | 0.0.124+ | Lightning protocol underneath LDK Node | If we need lower-level channel control beyond LDK Node. |
| `secp256k1` | 0.29+ | Crypto primitives | All Bitcoin-related crypto via rust-bitcoin re-export. |
| `nostr-relay-builder` / `nostr-rs-relay` | latest | Run our own relay | Reference Nostr relay for `relay.hunch.markets`. Rust-based, performant. |
| `axum` | 0.7+ | HTTP server for mint/oracle | Used by cdk-axum (mint HTTP layer). Standard. |
| `tokio` | 1.40+ | Async runtime | Standard. |
| `sqlx` | 0.8+ | SQL toolkit (sqlite/postgres) | Mint and oracle state persistence. |
| `tracing` | 0.1+ | Structured logging | Standard. |
| `serde` / `serde_json` | 1.x | Serialization | Standard. |
| `frost-core` | 2.x | FROST DKG / signing protocol core | Underlying core for frost-secp256k1-tr. |
| `zod` (TS) | 3.x | Runtime validation on frontend | Validate Nostr events before trusting. |
| `react-hook-form` (TS) | 7.x | Frontend forms | Market creation forms. |
| `tailwindcss` | 4.x | Frontend styling | Standard. |
| `shadcn/ui` | latest | UI components | Best DX for Next.js, easy to fork/theme cypherpunk. |
| `nuqs` (TS) | latest | URL-as-state | Filter/sort markets via URL — great for share-linking. |

### Development Tools

| Tool | Purpose | Notes |
|------|---------|-------|
| **cargo workspace** | Multi-crate monorepo | `hunch-protocol`, `hunch-mint`, `hunch-oracle`, `hunch-matcher`, `hunch-relay` all in one workspace. |
| **Just / cargo-make** | Task runner | Cross-platform task automation. |
| **Bitcoin Core regtest + Esplora** | Local development network | Stack of `bitcoind` + `electrs` + `esplora` for local DLC testing. |
| **Polar** | Lightning regtest cluster | Easiest way to spin up multi-node Lightning network for testing. |
| **Mutinynet** | Bitcoin signet with fast blocks | 30-second blocks. Better than testnet3 (deprecated) and testnet4. |
| **Cashu wallet (web)** | Manual testing | wallet.nutstash.app for manual mint debugging. |
| **nak** (Nostr Army Knife) | CLI for Nostr | Test Nostr events from CLI before integrating. |
| **trunk / Vite-WASM** | Build LDK Node WASM bindings | If we ship LDK Node in browser. |
| **Bun** | JS package manager + runtime | Faster than npm/yarn. Used by Hunch's MeshPay-Nostr already. |
| **Biome** | TS linter + formatter | Faster than ESLint+Prettier. |
| **Vitest** | TS unit tests | Standard for Next.js TS. |
| **Playwright** | E2E browser tests | For frontend flows. |
| **cargo-deny + cargo-audit** | Supply chain security | Mandatory for mainnet-targeted code. |

## Installation

```bash
# Backend (Rust workspace)
cargo new --lib hunch-protocol
cargo add bdk_wallet ldk-node cdk rust-dlc ddk \
          frost-secp256k1-tr nostr-sdk \
          axum tokio sqlx serde serde_json tracing

# Frontend (Bun + Next.js)
bun create next-app@latest hunch-app --typescript --app --static
cd hunch-app
bun add @nostr-dev-kit/ndk cashu-ts @getalby/sdk \
        zod react-hook-form @hookform/resolvers \
        nuqs lucide-react sonner
bun add -d tailwindcss@latest biome playwright
bunx shadcn@latest init
```

## Alternatives Considered

| Recommended | Alternative | When to Use Alternative |
|-------------|-------------|-------------------------|
| **LDK Node** | LND (daemon, Go) | If we need maximum Lightning compatibility and battle-tested production. Trade-off: extra daemon to operate vs embedded. |
| **LDK Node** | CLN / Core Lightning | If we want a stable third-party Lightning node we don't ship. Used by Greenlight. Trade-off: daemon dependency. |
| **rust-dlc / DDK** | suredbits/dlc-rs (legacy) | Don't — superseded. DDK is the current direction. |
| **CDK (Rust)** | Nutshell (Python) | Only if rapid prototyping in Python is critical. We need Rust for performance + mainnet credibility. |
| **CDK (Rust)** | cashu-rs-mint (legacy) | Don't — merged into CDK ecosystem. |
| **frost-secp256k1-tr** | secp256k1-zkp musig2 | MuSig2 is for n-of-n, not k-of-n. We need k-of-n threshold for oracle robustness — must be FROST. |
| **frost-secp256k1-tr** | bancaditalia/secp256k1-frost (C) | Only if we needed C bindings. Rust direct is simpler. |
| **Next.js** | SvelteKit + Adapter Static | If we wanted a smaller bundle for IPFS. Trade-off: smaller ecosystem for Nostr/Cashu/WebLN. |
| **Next.js** | Astro | If we wanted truly minimal JS. Trade-off: less DX for highly interactive Polymarket-like UI. |
| **NDK** | nostr-tools | NDK is higher-level (relay pool, signer abstraction). nostr-tools is lower-level but more flexible. NDK wins for product code. |
| **nostr-rs-relay** | Strfry (C++) | Strfry is faster but C++ is harder to audit/contribute to. We start with rust-relay, switch if perf matters. |
| **Bun** | pnpm / npm | If we need maximum compatibility with old tooling. Bun wins on speed. |
| **TypeScript frontend** | Rust + WASM (Leptos, Yew) | Cypherpunk would love a Rust-only stack. Trade-off: 6× harder hiring future contributors. v3 consideration. |

## What NOT to Use

| Avoid | Why | Use Instead |
|-------|-----|-------------|
| **Ethereum / Solana / any altchain** | Defeats the entire point (Bitcoin-only cypherpunk) | Bitcoin + Lightning |
| **Custodial Lightning service (Alby Hub, Strike API)** | Custodial dependency. Project philosophy demands self-sovereign. | LDK Node self-hosted |
| **Tornado-Cash-style mixing on top** | Different threat model + legal escalation | Cashu blind sigs are sufficient privacy |
| **Centralized DB for markets (PostgreSQL as truth source)** | Single point of censorship and recovery risk | Nostr events as source of truth, SQLite cache layer for indexing only |
| **Polygon / L2 EVM bridges to USDC** | Not Bitcoin. KYC pressure. Custody risk via stablecoin issuer (Circle/Tether). | Native BTC + Cashu tokens |
| **CLOB on Polymarket/Augur model directly** | Their CLOB requires smart contracts to be trustless | Hybrid: Cashu mint orderbook (Tier 1) + Nostr P2P matching (Tier 2) |
| **OAuth / email login** | KYC by another name — links to identity | Nostr pubkey login (NIP-07 browser ext, NIP-46 remote signer) |
| **Google Analytics / Cloudflare Analytics / any telemetry** | Tracking violates principle | None. Operator-side aggregate logs only. |
| **AWS / GCP / Azure for hosting backend services** | Centralized infrastructure dependency | Self-hosted Hetzner / OVH / Njalla (privacy-friendly) + Cloudflare for static frontend only |
| **GitHub Issues as exclusive issue tracker** | GitHub can deplatform | GitHub + Radicle mirror + Nostr issue tracking (GitNostr / Pierre) |
| **Mempool.space exclusive backend** | Single source of mempool data | Use Esplora REST as API spec, run our own electrs + esplora |
| **dlc-lib (legacy)** | Outdated | rust-dlc + DDK |
| **suredbits/dlc-oracle (legacy)** | Closed/proprietary now | rust-dlc-oracle (open) or custom |
| **bitcoinjs-lib in browser for DLC** | DLC support immature in JS, plus we want Rust auditability | All DLC logic in Rust services |

## Stack Patterns by Variant

**If running just the protocol reference (no Hunch frontend):**
- Use the Rust workspace + `cargo run --bin hunch-relay/mint/oracle`
- Skip Next.js entirely
- Reference for contributors and forks

**If running for high-trust operator (you, initial Hunch instance):**
- LDK Node embedded in mint binary (single process)
- Single-tenant deployment
- Operator key in HSM or air-gapped signer

**If federating the mint later (multi-operator FROST):**
- Add `frost-secp256k1-tr` for mint key DKG
- ROAST robust signing protocol on top of FROST (active research in DDK ecosystem)
- More complex deployment but eliminates single-op rug risk

**If browser-only LDK (zero-backend):**
- LDK Node WASM build
- IndexedDB-backed channel state
- Mobile-equivalent privacy and self-custody (still requires LSP)

## Version Compatibility

| Package A | Compatible With | Notes |
|-----------|-----------------|-------|
| `cdk` 0.14+ | `bitcoin` 0.32, `tokio` 1.40+ | Pin major versions in workspace. |
| `ldk-node` 0.4+ | `bitcoin` 0.32, `bdk_wallet` 1.0 | LDK Node depends on a specific LDK version; check release notes. |
| `rust-dlc` 0.7+ | `bitcoin` 0.32, `secp256k1` 0.29 | Coordinate version bumps across DDK. |
| `nostr-sdk` 0.34+ | `tokio` 1.40+ | Mature, stable API at 0.30+. |
| `frost-secp256k1-tr` 2.2+ | `frost-core` 2.x | Single-source upgrades via workspace patch. |
| `next` 15 | `react` 19, `typescript` 5.5+ | Next 15 = stable App Router + static export. |
| `@nostr-dev-kit/ndk` 2.10+ | NDK Signer interfaces | Verify NIP-07 + NIP-46 work paths. |
| `cashu-ts` 2.x | Cashu protocol NUT-04 through NUT-15+ | Verify mint supports the NUTs the wallet uses. |

## Critical Unknowns / Spike Targets

These need de-risking via spike before Phase 1 planning locks:

1. **NUT-DLC (PR #128) status**: The proposal is by `conduition`, depends on PR #127 (well-known secrets path). Need to: (a) read latest discussion, (b) prototype in CDK or fork, (c) decide whether to upstream-or-fork. **Severity: CRITICAL.**
2. **FROST DKG operational complexity**: ZF FROST crate is RFC 9591 compliant, but real-world DKG ceremony for k-of-n oracle setup needs UX design. **Severity: HIGH.**
3. **DLC + Lightning channel state**: Atomic.finance / cara are building Lightning DLCs. Production readiness unclear. May need to stick to on-chain DLCs for v1. **Severity: MEDIUM.**
4. **LDK Node WASM real-world memory**: WASM build of LDK Node is feasible but channel state in IndexedDB has memory limits per browser. **Severity: MEDIUM (only relevant for browser-LDK Tier).**
5. **Multi-bettor DLC pattern**: Pure DLCs are bilateral. The "mint-as-DLC-counterparty" pattern needs precise spec — how does mint commit to DLC outcome distribution? **Severity: CRITICAL.** (Architecture section dives deeper.)

## Sources

- [Cashu NUTs PR #128 (NUT-DLC)](https://github.com/cashubtc/nuts/pull/128)
- [Cashu NUTs Specifications](https://cashubtc.github.io/nuts/)
- [cdk on crates.io](https://crates.io/crates/cdk)
- [dlcdevkit (DDK) GitHub](https://github.com/bennyhodl/dlcdevkit)
- [DDK on lib.rs](https://lib.rs/crates/ddk)
- [frost-secp256k1-tr on crates.io](https://crates.io/crates/frost-secp256k1-tr)
- [BIP-445 FROST for BIP340](https://github.com/siv2r/bip-frost-signing)
- [RFC 9591 — FROST](https://datatracker.ietf.org/doc/rfc9591/)
- [rust-bitcoin organization](https://github.com/rust-bitcoin)
- [LDK Node releases](https://github.com/lightningdevkit/ldk-node)
- [rust-nostr (nostr-sdk)](https://github.com/rust-nostr/nostr)

---
*Stack research for: Bitcoin-native cypherpunk prediction market protocol*
*Researched: 2026-05-27*
