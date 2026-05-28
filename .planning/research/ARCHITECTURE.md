# Architecture Research

**Domain:** Bitcoin-native cypherpunk prediction market protocol (DLC + Cashu + Lightning + Nostr)
**Researched:** 2026-05-27
**Confidence:** HIGH for the high-level decomposition; MEDIUM for the precise DLC-mint integration (depends on NUT-DLC spec maturity).

## 2026-05-28 Corrigendum — NUT-DLC pivot to NUT-CTF (PR #337)

**Status update:** Cashu NUTs PR #128 (bilateral NUT-DLC by conduition) was **CLOSED 2025-05-20** by thesimplekid with comment "Closing as there is no active work. Please reopen if work continues." This document was written 2026-05-27 assuming PR #128 was the live critical path. That assumption is **WRONG**.

**New chosen path (locked in CONTEXT.md decision D-01):** Path A — **NUT-CTF (Conditional Token Framework, PR #337 by joemphilips, opened 2026-02-07)**. NUT-CTF is architecturally distinct: oracle-agnostic mint with conditional token issuance + split-merge for secondary market, instead of mint-as-bilateral-counterparty.

**References below to "NUT-DLC", "PR #128", "bilateral DLC mint" should be read as NUT-CTF / PR #337 / oracle-agnostic conditional token framework** unless explicitly marked as historical context. In particular, Pattern 3 in this document ("mint-as-bilateral-DLC-counterparty") is now superseded; HIP-3 will define the NUT-CTF integration shape instead.

See `.planning/phases/01-cypherpunk-foundation/01-RESEARCH.md` §3 for the deep-dive on this pivot.


## Standard Architecture

### System Overview

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              FRONTEND LAYER                                 │
│  (Multiple instances — hunch.io, hunch.onion, IPFS pin, self-hosted PWAs)   │
│                                                                             │
│  ┌────────────────┐   ┌──────────────────┐   ┌──────────────────────┐       │
│  │ Web (Next.js) │ │ PWA              │ │ Tor hidden service     │         │
│  └────────┬───────┘ └─────────┬────────┘ └───────────┬────────────┘         │
│           │                   │                      │                      │
└───────────┼───────────────────┼──────────────────────┼──────────────────────┘
            │                   │                      │
            └─────── NDK (Nostr) + WebLN + cashu-ts ────┘
                                │
┌───────────────────────────────┼─────────────────────────────────────────────┐
│                          PROTOCOL LAYER                                     │
│                                                                             │
│   ┌─────────────────────────────────────────────────────────────────┐       │
│   │                    NOSTR EVENT BUS                              │       │
│   │  Multiple relays. Anyone can run one. Reference: relay.hunch.   │       │
│   │                                                                 │       │
│   │  Kinds:                                                         │       │
│   │   • 30888 Market Announce (replaceable)                         │       │
│   │   • 38888 Order (param replaceable per market)                  │       │
│   │   • 38889 Order Match                                           │       │
│   │   • 30890 Dispute / Challenge                                   │       │
│   │   •  88   Oracle Event Announcement (NIP-88)                    │       │
│   │   •  89   Oracle Attestation (NIP-88)                           │       │
│   │   • 30891 Reputation Event                                      │       │
│   │   • 30892 Mint Announce                                         │       │
│   └─────────────────────────────────────────────────────────────────┘       │
│                                                                             │
│         ┌──────────────────────┬──────────────────────────┐                 │
│         │                      │                          │                 │
│         ▼                      ▼                          ▼                 │
│  ┌──────────────┐      ┌──────────────────┐    ┌──────────────────┐         │
│  │ ORACLES      │      │ CASHU MINTS      │    │ MATCHERS (P2P)   │         │
│  │ (k-of-n FROST│      │ (NUT-DLC)        │    │ Tier 2 only,     │         │
│  │  Schnorr)    │      │ Multiple mints,  │    │ Nostr-indexed,   │         │
│  │ Multi-op,    │      │ competing,       │    │ no custody       │         │
│  │ public attst │      │ DLC-backed       │    │                  │         │
│  └──────┬───────┘      └────────┬─────────┘    └────────┬─────────┘         │
│         │                       │                       │                   │
│         └──────────────┬────────┴───────────────────────┘                   │
│                        │                                                    │
└────────────────────────┼────────────────────────────────────────────────────┘
                         │
┌────────────────────────┼────────────────────────────────────────────────────┐
│                  BITCOIN SETTLEMENT LAYER                                   │
│                        │                                                    │
│   ┌────────────────────▼────────────────┐    ┌──────────────────────┐       │
│   │ DLC ESCROW (rust-dlc / DDK)         │    │ LIGHTNING (LDK Node) │       │
│   │  • Adaptor signatures               │◄──►│  • Deposits          │       │
│   │  • Multi-oracle (FROST adaptor)     │    │  • Withdrawals       │       │
│   │  • Refund timeout                   │    │  • Channel state     │       │
│   │  • CET per outcome                  │    │  • LSP for clients   │       │
│   └─────────────────┬───────────────────┘    └────────────┬─────────┘       │
│                     │                                     │                 │
│                     └─────────────► Bitcoin Mainnet ◄─────┘                 │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Component Responsibilities

| Component | Responsibility | Typical Implementation |
|-----------|----------------|------------------------|
| **Frontend (Web/PWA)** | UI, user identity (Nostr key), wallet (cashu-ts + WebLN), event publishing, indexing of Nostr events | Next.js 15 + NDK + cashu-ts + WebLN, static export, deployable anywhere |
| **Nostr Relay** | Store + serve Nostr events (markets, orders, attestations, reputation). Source of truth for off-chain state. | nostr-rs-relay or Strfry — anyone can run one; reference relay at `relay.hunch.markets` |
| **Cashu Mint (Hunch-flavored)** | Issue YES/NO tokens; back them with DLC collateral; provide LN deposit/withdraw; orderbook via atomic swaps; pay out on settlement | CDK fork with NUT-DLC extension (`hunch-mint` Rust binary) |
| **Oracle Service** | Publish oracle announcements (NIP-88); coordinate FROST DKG with co-oracles; sign attestations on event resolution; expose reputation history | `hunch-oracle` Rust binary with `frost-secp256k1-tr` + custom Nostr publisher |
| **Matcher / Indexer (optional, Tier 2)** | Cache + index Nostr orders for fast frontend queries; not custodial | `hunch-matcher` Rust service backed by SQLite |
| **Shared Protocol Library** | Common types, event schemas, DLC contract building blocks, FROST coordination helpers | `hunch-protocol` Rust crate |
| **Reference Frontend (hunch.io)** | Production Hunch operator's UI; geo-block US; document forks | Static Next.js deployed on Cloudflare Pages + IPFS pin + Tor onion |

## Recommended Project Structure

Cargo workspace + Bun monorepo (frontend in subfolder):

```
hunch/
├── Cargo.toml                    # Workspace
├── crates/
│   ├── hunch-protocol/           # Shared types + event schemas + DLC builders
│   │   ├── src/
│   │   │   ├── events.rs         # Nostr event kinds + serde
│   │   │   ├── market.rs         # Market data structure
│   │   │   ├── dlc.rs            # DLC contract construction
│   │   │   ├── oracle.rs         # Oracle attestation format
│   │   │   ├── reputation.rs     # Reputation event format
│   │   │   └── lib.rs
│   │   └── Cargo.toml
│   ├── hunch-mint/               # Cashu mint binary
│   │   ├── src/
│   │   │   ├── main.rs
│   │   │   ├── nut_dlc.rs        # NUT-DLC extension over CDK
│   │   │   ├── lightning.rs      # LDK Node integration
│   │   │   ├── dlc_backing.rs    # Open + manage DLCs as collateral
│   │   │   ├── api/              # HTTP routes (axum)
│   │   │   └── storage/          # SQLite via sqlx
│   │   └── Cargo.toml
│   ├── hunch-oracle/             # Oracle service binary
│   │   ├── src/
│   │   │   ├── main.rs
│   │   │   ├── frost.rs          # FROST DKG + signing coordination
│   │   │   ├── nostr_publisher.rs# Publish NIP-88 events
│   │   │   ├── event_watcher.rs  # Watch for external events to attest
│   │   │   └── storage/
│   │   └── Cargo.toml
│   ├── hunch-relay/              # Optional bundled Nostr relay binary
│   │   └── Cargo.toml             # Mostly a thin wrapper around nostr-rs-relay
│   ├── hunch-matcher/            # Tier 2 P2P matcher / indexer
│   │   └── Cargo.toml
│   └── hunch-cli/                # Operator CLI
│       └── Cargo.toml
├── apps/
│   └── hunch-web/                # Next.js frontend
│       ├── src/
│       │   ├── app/              # App Router
│       │   │   ├── markets/      # Browse + filter
│       │   │   ├── market/[id]/  # Market detail + bet
│       │   │   ├── create/       # Market creation
│       │   │   ├── account/      # Portfolio
│       │   │   └── settle/[tx]/  # Settlement verification UI
│       │   ├── lib/
│       │   │   ├── nostr/        # NDK wrappers, event types
│       │   │   ├── cashu/        # Mint client, atomic swap
│       │   │   ├── webln/        # Lightning bridge
│       │   │   ├── reputation/   # Reputation queries on Nostr
│       │   │   └── social/       # Web of trust filtering
│       │   └── components/       # UI components (shadcn-based)
│       ├── package.json
│       └── next.config.mjs
├── docs/
│   ├── HIP-0.md                  # Protocol overview
│   ├── HIP-1.md                  # Nostr event kinds spec
│   ├── HIP-2.md                  # DLC contract structure
│   ├── HIP-3.md                  # Cashu NUT-DLC extension
│   ├── HIP-4.md                  # Multi-oracle FROST protocol
│   ├── HIP-5.md                  # Reputation event format
│   ├── MANIFESTO.md              # The cypherpunk manifesto
│   └── ARCHITECTURE.md           # This document, lite version
├── deploy/
│   ├── docker/                   # Mint + oracle + relay images
│   ├── nginx/                    # Reverse proxy config (Tor-friendly)
│   └── tor/                      # hidden service descriptor
├── scripts/
│   ├── dev-regtest.sh            # Spin up local Bitcoin regtest + LN cluster
│   └── seed-markets.sh           # Test data for staging
├── .github/
│   └── workflows/                # CI: cargo test, frontend build, audit
└── README.md
```

### Structure Rationale

- **`crates/hunch-protocol`** is the heart — shared types must compile in mint, oracle, matcher, CLI. Bun frontend imports its TypeScript equivalent generated via `ts-rs` or similar.
- **Each service is a separate binary** in its own crate — runnable independently, dockerizable, contributors can pick one to work on.
- **`docs/HIP-N.md`** mirrors the NIP convention. These are protocol specs, not implementation docs. Specs live in `docs/` because protocol > code.
- **`apps/hunch-web`** as the reference frontend — explicitly named "apps" because we expect multiple frontends to be built.
- **No central database directory** — protocol state is on Nostr + Bitcoin; service state is per-binary in SQLite.

## Architectural Patterns

### Pattern 1: Nostr-as-Source-of-Truth

**What:** All market state (markets, orders, oracle announcements, attestations, disputes, reputation) is published as Nostr events. SQL databases in services are **caches**, never authority.

**When to use:** Any state that needs to be censorship-resistant, multi-frontend visible, and operator-independent.

**Trade-offs:**
- ✅ Anyone can index. Any frontend can serve any market.
- ✅ No vendor lock-in.
- ❌ Eventual consistency. Multiple relays may disagree.
- ❌ Spam burden falls on relay operators.

**Example (Rust):**
```rust
// Publishing a market — this is the canonical creation, not the DB insert
let market_event = EventBuilder::new(
    Kind::Custom(30888),
    serde_json::to_string(&market)?,
    vec![
        Tag::Identifier(market.id.clone()),
        Tag::Generic("category", vec!["politics"]),
        Tag::Generic("oracle", vec![oracle_pubkey.to_string()]),
        Tag::Generic("expiry", vec![market.expiry_ts.to_string()]),
    ],
)
.to_event(&creator_keys)?;

nostr_client.send_event(market_event).await?;
// Then cache it locally for performance
db.insert_market_cache(&market).await?;
```

### Pattern 2: DLC-as-Settlement-Primitive

**What:** Each market opens one DLC contract. The mint is the bilateral counterparty against the oracle's attestation. CETs encode payout per outcome. Settlement = oracle Schnorr sig + CET broadcast.

**When to use:** Always. This is the protocol's settlement layer.

**Trade-offs:**
- ✅ Trustless settlement at expiry — mint can't refuse to pay because the DLC executes via on-chain CET.
- ✅ Refund timeout fallback if oracle disappears.
- ❌ Bilateral DLC architecture is awkward for multi-bettor markets — we solve it via the mint pattern (mint is the DLC counterparty, mint issues fungible YES/NO tokens internally).
- ❌ On-chain footprint per market (initial funding tx + CET).

**Example (high-level Rust):**
```rust
let dlc_contract = DlcContractBuilder::new()
    .oracle_announcement(oracle_event_pubkey)
    .outcomes(vec![("YES", yes_pubkey), ("NO", no_pubkey)])
    .collateral(market.max_volume_sats)
    .funding(mint_funding_input)
    .refund_timeout(market.expiry_ts + REFUND_GRACE)
    .build()?;

let funding_tx = dlc_contract.create_funding_tx()?;
broadcast(funding_tx).await?;

// Later at resolution:
let oracle_sig = oracle.fetch_attestation(event_id).await?;
let cet = dlc_contract.execute_cet_with_attestation(&oracle_sig)?;
broadcast(cet).await?;
```

### Pattern 3: Cashu Mint as Internal Orderbook + DLC Wrapper

**What:** The mint issues YES/NO tokens fungibly to bettors. Internally, it tracks how much YES vs NO is outstanding. The mint's exposure is bounded by the DLC collateral.

**When to use:** Tier 1 liquidity for all markets.

**Trade-offs:**
- ✅ Lightning-fast UX (Cashu speed).
- ✅ Atomic-swap secondary market between YES and NO holders.
- ✅ Privacy (blind sigs).
- ❌ Mint operator can rug DURING the market lifetime (token issuance honor) — but cannot rug AT settlement (DLC controls funds).
- ❌ Requires NUT-DLC spec to be stable. If not stable, we fall back to "trusted mint with off-chain promise" for v1.

**Mitigations for during-lifetime rug:**
- Public mint state proofs (NUT-22-style)
- Mint reputation on Nostr
- Multiple competing mints — users can choose
- Future: federated mint with FROST multisig (reduces single-op rug)

### Pattern 4: Oracle Marketplace via Reputation

**What:** Anyone can run an oracle and publish NIP-88 announcements. Market creators choose their oracle(s) at creation. Reputation accrues via public attestation history + community signals.

**When to use:** From day 1, even with only Hunch's reference oracle.

**Trade-offs:**
- ✅ No "oracle DAO" governance token = no securities risk.
- ✅ Competitive market → best oracles win.
- ❌ Cold-start: market creators initially have only Hunch's oracle.
- ❌ Reputation is gameable; needs explicit defenses.

**Anti-gaming:**
- Cross-market reputation (oracle X has correctly resolved N markets over Y time)
- Public attestation history immutable on Nostr
- Community dispute events (kind:30890) when oracle attests wrong outcome

### Pattern 5: Permissionless Markets + Frontend Curation

**What:** Anyone publishes a market event. The protocol stores it. Frontends decide which markets to surface (UI-level filter) without removing them from the protocol.

**When to use:** Always. Foundational principle.

**Trade-offs:**
- ✅ True censorship resistance at protocol layer.
- ✅ Frontends can comply with local laws independently.
- ❌ Spam burden. Mitigated via social graph filter, reputation, manual mutes.
- ❌ Augur-class abuse markets WILL appear. Frontend curation handles taste; protocol stays neutral.

## Data Flow

### Market Creation Flow

```
[User on hunch.io]
    ↓
[Compose market form] → validate locally
    ↓
[Sign Nostr event (kind 30888) with NIP-07/46]
    ↓
[Publish to relays]
    ↓                              ┌─────────────────────────┐
[Indexers cache it]                │ Market is now LIVE       │
[Mint sees it via Nostr sub]      │ — no DLC funded yet,      │
[Mint waits for first taker]       │   no on-chain footprint    │
                                   └─────────────────────────┘
```

The market is just a Nostr event until someone bets. **Critical insight:** no on-chain cost to creating markets. Spam markets are cheap to create, cheap to ignore.

### Bet Placement Flow (Tier 1, via mint)

```
[User clicks "Buy YES 1000 sats"]
    ↓
[Frontend asks mint: quote for 1000 sats YES in market X?]
    ↓
[Mint replies: 1.0 YES tokens for 1000 sats (current price)]
    ↓
[Frontend → WebLN: pay this BOLT-11 invoice for 1000 sats]
    ↓
[User wallet pays via Lightning]
    ↓
[Mint receives payment]
    ↓
[Mint checks: is DLC funded? If not, fund it now from accumulated bets]
    ↓
[Mint issues YES tokens to user (Cashu blind signature)]
    ↓
[User stores tokens locally (in cashu-ts wallet)]
```

**Important:** the first bet on a market triggers DLC creation. The mint accumulates bets, periodically rebalancing the DLC collateral (or opening a new DLC layer if needed).

### Bet Sale Flow (atomic swap, Tier 1)

```
[User A holds 1.0 YES tokens, wants to sell at 70 sats each]
    ↓
[User A publishes Cashu swap offer on Nostr (kind 38888 — ORDER)]
    ↓
[User B sees offer, agrees]
    ↓
[Atomic swap: B's 70 sats Cashu tokens ↔ A's 1.0 YES tokens]
    ↓
[Swap settled via mint NUT-04 swap endpoint]
```

### Resolution Flow

```
[Market expires]
    ↓
[Oracle event triggers — oracle service watches external event]
    ↓
[Oracle service signs attestation (Schnorr) and publishes (NIP-88 kind 89)]
    ↓
[Optional: dispute period 24h, anyone can publish challenge event]
    ↓                                       ↓
[If undisputed]                    [If disputed: alternate path]
    ↓                                       ↓
[Mint reads attestation]           [Pause; community/oracle marketplace
    ↓                                signals; market resolves to "invalid"
[Mint executes DLC CET on-chain]    via 50/50 refund CET if challenge wins]
    ↓
[CET broadcast to Bitcoin mainnet]
    ↓
[Mint credits internal accounts:
   YES holders → sat balance (proportional)
   NO holders  → 0]
    ↓
[Users withdraw via LN]
```

### Reputation Update Flow

```
[Market resolves]
    ↓
[Bettors publish reputation events (kind 30891):
   "Oracle X correctly attested market Y" or "Oracle X attested wrong"]
    ↓
[Other users see these events when choosing oracles for new markets]
    ↓
[Frontend ranks oracles by reputation aggregate]
```

## State Management

### Authority Hierarchy

```
LEVEL 1 (Authoritative): Bitcoin mainnet — settlement, DLC, Lightning
   ↓
LEVEL 2 (Authoritative): Oracle Schnorr signatures — attestations
   ↓
LEVEL 3 (Authoritative): Nostr events — market state, orders, reputation (signed by author)
   ↓
LEVEL 4 (Cache): Service databases — index for performance
   ↓
LEVEL 5 (Local): Frontend caches — UI state
```

**Conflict resolution:** Always defer to higher level. If two relays disagree, fetch from many; if many disagree, trust the bettor's signed events + oracle's signed attestation.

## Scaling Considerations

| Scale | Architecture Adjustments |
|-------|--------------------------|
| 0-1k users | Single mint, single oracle, single relay. Sled-backed sqlite. ~10 markets/day. |
| 1k-10k users | Multiple mints (encouraged), 2-3 oracles, 3-5 relays. Postgres for mint state. CDN for static frontend. |
| 10k-100k users | Federated mint (FROST), oracle marketplace mature, multiple frontends, distributed relays. Multi-region deployments. |
| 100k+ users | At this point Hunch is its own ecosystem. Layered LN/cashu interop, Lightning DLC channels for instant settlement, multi-chain bridges via atomic swaps for stablecoin entry (no Hunch involvement). |

### Scaling Priorities

1. **First bottleneck (1k-10k users):** Mint single-process throughput for Cashu signing. **Fix:** horizontal scale by partitioning markets across multiple mint nodes, or by switching to a more performant signing backend.
2. **Second bottleneck:** DLC on-chain UTXO management. Many concurrent markets = many UTXOs. **Fix:** UTXO aggregation, smaller DLCs that close more often, eventually Lightning DLC channels.
3. **Third bottleneck:** Nostr relay query latency for market discovery. **Fix:** Frontend-side index built from Nostr stream + bloom-filtered queries.

## Anti-Patterns

### Anti-Pattern 1: Treating SQL as Source of Truth

**What people do:** Build a Postgres-backed orderbook and treat Nostr as a notification mechanism.

**Why it's wrong:** Defeats censorship resistance. If Hunch.io's DB is seized, markets disappear. Forks can't rebuild state.

**Do this instead:** Nostr is source of truth. DB is cache. Rebuildable from relays at any time.

### Anti-Pattern 2: Trusted Oracle Default

**What people do:** Hard-code Hunch's oracle as the default; users implicitly trust it.

**Why it's wrong:** Recreates the centralization Polymarket/UMA has. We become the trust anchor.

**Do this instead:** Make oracle selection explicit at market creation. Show oracle reputation prominently. Educate users to choose.

### Anti-Pattern 3: Hiding Settlement Mechanics

**What people do:** Just show "Resolved YES" without exposing the DLC tx + oracle sig.

**Why it's wrong:** Users can't verify. Becomes a black box. Lose cypherpunk credibility.

**Do this instead:** Settlement page shows: Bitcoin tx, oracle pubkey, oracle Schnorr sig, verification link. "Verify this resolution yourself" link.

### Anti-Pattern 4: Building Mobile App First

**What people do:** Native iOS/Android day-1.

**Why it's wrong:** App Store deplatforms gambling/prediction apps regularly. Damus tipping was removed. F-Droid only reaches ~5% of Bitcoin users.

**Do this instead:** PWA with install-to-home-screen. Mobile-grade UX from day 1. Native later when traction justifies the deplatform risk.

### Anti-Pattern 5: Multi-Outcome Markets in V1

**What people do:** Try to ship N-outcome markets from launch (sports scores, election with many candidates, etc.).

**Why it's wrong:** DLC CET explosion. Multi-outcome DLCs have CETs proportional to outcomes. Operational complexity 10×.

**Do this instead:** Binary YES/NO only in v1. Decompose multi-outcome into multiple binary markets. (E.g., election: "Will candidate A win?", "Will candidate B win?", "Will candidate C win?")

### Anti-Pattern 6: Central Matcher Engine

**What people do:** Build a Polymarket-style central CLOB matcher in Hunch's backend.

**Why it's wrong:** Hunch becomes single point of failure for matching. Latency bottleneck. Censorship vector.

**Do this instead:** Mint = orderbook (Tier 1, fast UX, mint operator's matching algorithm public + auditable). Nostr P2P = Tier 2 for power users (slower, fully decentralized).

## Integration Points

### External Services

| Service | Integration Pattern | Notes |
|---------|---------------------|-------|
| Bitcoin Core / electrs / Esplora | REST API via BDK Esplora client | Run our own electrs for self-sovereignty; mempool.space as fallback |
| LSP (Lightning Service Provider) | LSPS spec | Voltage, Olympus for production; self-host an Olympus instance later |
| Nostr Relays | NIP-01 over WebSocket | Run our own relay + recommend community relays (relay.damus.io, nos.lol, etc.) |
| External oracles (Suredbits, DLC.Link) | Optional integration via NIP-88 announce events | Allow them to publish on Nostr; users can choose them as alternatives |
| Block explorers | mempool.space, blockstream.info | Link to public explorers from settlement UI |

### Internal Boundaries

| Boundary | Communication | Notes |
|----------|---------------|-------|
| Frontend ↔ Mint | HTTP REST (Cashu protocol NUT spec) | Standard Cashu API |
| Frontend ↔ Oracle | Read-only via Nostr events | No direct API; oracle state is all on Nostr |
| Frontend ↔ Relay | WebSocket (NIP-01) | Standard Nostr protocol |
| Mint ↔ Oracle | Read-only via Nostr events (attestations) | Mint subscribes to NIP-88 for markets it backs |
| Mint ↔ Bitcoin | BDK + LDK Node | Run own electrs backend ideally |
| Oracle ↔ Co-oracles (FROST) | Custom protocol on top of Nostr DMs (NIP-04/44) for DKG ceremony + sign coordination | Off-band coordination acceptable for setup; signing rounds via Nostr |

## Critical Architectural Decisions

| Decision | Choice | Rationale | Alternative if Wrong |
|----------|--------|-----------|----------------------|
| Source of truth | Nostr events | Censorship resistance, multi-frontend, fork-friendly | If Nostr scales poorly: hybrid with content-addressed IPFS pin |
| Liquidity model | Cashu mint as orderbook, Nostr P2P as Tier 2 | Best UX + decentralization escape valve | Pure P2P (Robosats-style) — slower but more pure |
| Oracle model | Multi-oracle marketplace, FROST k-of-n for federation | Avoids governance token, encourages competition | Single trusted oracle (Polymarket-like) — falls back position only |
| Settlement primitive | DLC | Bitcoin-native, trustless, refundable | Custodial promise — debt of trust we never pay |
| Frontend architecture | Next.js 15 static export | Best DX + deployable anywhere (IPFS, Tor, CDN) | Astro for smaller bundle; SvelteKit for alternative |
| Build language | Rust for services, TS for frontend | Best Bitcoin ecosystem support + frontend pragmatism | Go for services if mint/oracle perf is critical |
| Multi-bettor DLC pattern | Mint is bilateral DLC counterparty; mint issues fungible tokens | Solves multi-bettor problem cleanly | Pure-DLC bilateral matching — only for power users |

## Open Architectural Questions (Need Spikes Before Phase 2)

1. **NUT-DLC integration depth**: Read PR #128 + #127 + dependencies. Determine: contribute upstream, fork, or write custom spec inheriting Cashu primitives.
2. **DLC refund timeout collision with mint operator**: If oracle disappears at expiry+grace, refund CET goes to whom? Mint? Bettors via mint? Design carefully.
3. **FROST DKG ceremony UX**: Who initiates? How do co-oracles discover each other? Multi-step UI vs CLI for ops?
4. **Atomic swap secondary market on Cashu DLC tokens**: NUT spec for swap of DLC-backed tokens needs verification — may need own NUT proposal.
5. **Relay strategy**: Run own + push to public? Push only to public? Bloom filter on our relay to spam-protect?
6. **Lightning DLC channels vs on-chain DLCs**: Atomic.finance + cara progress. May or may not be production by v1.
7. **Mint federation upgrade path**: How do we migrate a single-op mint to a FROST-federated mint without disrupting open markets?

## Build Order Recommendation

Phase 1 builds the absolute spine (Path A). Phase 2 adds decentralization. Phase 3 adds polish + multi-outcome.

```
Phase 1 — Spine
  ├── HIP-0..N protocol specs           (no code, parallel)
  ├── hunch-protocol crate              (shared types)
  ├── hunch-mint binary                 (Cashu mint + DLC + LDK Node)
  ├── hunch-oracle binary (single sig)  (NIP-88 publisher)
  ├── hunch-relay (deploy nostr-rs-relay)
  ├── hunch-web frontend                (browse, create, bet, sell, resolve)
  ├── Tor hidden service
  ├── Geo-block US infrastructure
  └── Anti-spam (social graph filter UI)

Phase 2 — Decentralization
  ├── Multi-oracle FROST k-of-n
  ├── Federated mint (FROST multisig operators)
  ├── Reputation event sophistication
  └── P2P Tier 2 matching

Phase 3 — Scale + multi-outcome
  ├── Multi-outcome DLC (Schnorr adaptor multi-sig)
  ├── Conditional / parlay markets
  ├── Lightning DLC channels
  ├── PWA polish + mobile install flows
  └── API + SDK for forks

Phase 4 — Sovereignty
  ├── Cross-mint atomic-swap routing
  ├── Programmatic oracles
  ├── Foundation (offshore entity)
  └── Multi-language UI
```

## Reference Architecture Comparison

| System | Source of truth | Liquidity | Oracle | Custody | Lessons |
|--------|-----------------|-----------|--------|---------|---------|
| Polymarket | CTF on Polygon | CLOB + AMM | UMA Optimistic | USDC in CTF | Single-token economy is simple but custodial; UMA is good but governance-token-dependent |
| Augur | Ethereum smart contracts | Orderbook | REP-staked reporters | ETH in contracts | True decentralization, but bad UX and REP-based oracle was capturable |
| Manifold | Centralized DB | Internal "house" | Manifold + creator | Internal | Easy UX, totally trusted, not our model |
| Robosats / Mostro | Nostr events (Mostro) | P2P maker/taker | None (Bitcoin price oracle) | None (Tor + escrow) | Tor-only UX works for committed users, but limits reach |
| Bisq | DAO + P2P | P2P maker/taker | None (Bitcoin price oracle) | None (BSQ token, multisig) | Long-running decentralized exchange model, BSQ governance has issues we want to avoid |
| **Hunch** | **Nostr events + Bitcoin tx** | **Cashu mint (Tier 1) + Nostr P2P (Tier 2)** | **Oracle marketplace + FROST k-of-n** | **None (DLC + Cashu, no operator custody of settled funds)** | **Combine the best of each** |

## Sources

- [DLC primer](https://benschroth.com/blog/dlcdevkit/)
- [Polymarket Resolution UMA docs](https://docs.polymarket.com/developers/resolution/UMA)
- [Mostro design](https://mostro.network)
- [Robosats design](https://learn.robosats.com)
- [Augur v2 architecture](https://augur.net/)
- [Cashu NUT specifications](https://cashubtc.github.io/nuts/)
- [LDK Node design](https://github.com/lightningdevkit/ldk-node)
- [Bisq architecture](https://bisq.network)

---
*Architecture research for: Bitcoin-native cypherpunk prediction market protocol*
*Researched: 2026-05-27*
