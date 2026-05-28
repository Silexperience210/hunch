# Feature Research

**Domain:** Permissionless Bitcoin-native prediction market (Polymarket-class product, cypherpunk philosophy)
**Researched:** 2026-05-27
**Confidence:** HIGH for table stakes and primary competitors (Polymarket V2 verified post-April-2026 launch, Augur fork state verified, Manifold deeply analyzed, Kalshi CFTC stance verified, Robosats / Mostro architecture verified); MEDIUM for cypherpunk-specific UX patterns (uncharted territory: Predyx is the closest analog and is custodial/centralized).

## Feature Landscape

### Table Stakes (Users Expect These)

Features users assume exist. Missing these = product feels incomplete.

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| **Browse markets by category + search** | Polymarket's 10 categories (Sports, Politics, Crypto, Pop Culture, Business, Economics, Tech, Weather, Oil & Commodities, Housing) are the dominant pattern as of 2026. Polymarket has 3,600+ active football markets, 5,400+ crypto markets, 360+ culture markets. | LOW | Nostr-indexed events with tags. SQLite cache for fast frontend queries. v1: 5 categories (Politics, Sports, Crypto, Culture, Meta-Nostr) — expand on demand. |
| **Trending / Most-Active markets** | Discovery of activity drives engagement; Polymarket front page is dominated by trending | LOW | Sort by Nostr `kind:38888` order event count + total volume |
| **Market detail page (price chart, resolution rules, oracle)** | Polymarket-grade detail page is the comparison benchmark. Must show oracle identity, resolution criteria, dispute window, on-chain settlement preview. | MEDIUM | Price history reconstructed from order events. Oracle identity + reputation visible. |
| **Buy YES / Buy NO with sat amount** | Single buy box like Polymarket | MEDIUM | WebLN payment → Cashu token mint → backed by DLC |
| **Sell position before resolution** | Closing positions is essential; otherwise feels like roach motel. Polymarket and Manifold both prioritize this. | MEDIUM | Atomic swap of YES/NO tokens via Cashu mint (Tier 1) or Nostr taker (Tier 2) |
| **Current price + odds display** | Probability % view (not just sat price) — Polymarket, Manifold, Kalshi all show % | LOW | Computed from latest order |
| **Account balance + portfolio** | "What I hold across markets" — universal expectation | MEDIUM | Locally-indexed Cashu wallet state |
| **Lightning deposit / withdraw** | Funds in/out without on-ramps. Predyx already does this for prediction markets. | MEDIUM | LDK Node or Mint's LN backend, BOLT-11 / BOLT-12 invoices |
| **Resolution & payout flow** | Automated at expiry — must "just work" | HIGH | Oracle Schnorr sig → DLC execution → mint redistributes |
| **Login via Nostr (NIP-07 / NIP-46)** | Standard for Nostr-native apps | LOW | NDK signer abstraction handles both |
| **Order history + transaction log** | Tax / audit / "what did I do" | MEDIUM | Local-first (privacy) with optional Nostr backup |
| **Market rules + dispute info clearly visible** | Trust requires understanding "who decides". Polymarket exposes UMA bond + DVM flow on every market. | LOW | Display oracle pubkey, oracle reputation, dispute period |
| **Mobile-responsive web (PWA)** | 60%+ traffic mobile in 2026 | MEDIUM | Next.js PWA with manifest, install prompt |
| **Tor-compatible (no JS-only flows that break in Tor Browser)** | Cypherpunk core audience uses Tor; Robosats demonstrates the standard | MEDIUM | Server-side rendering for first paint, no WebSocket-only flows |

### Differentiators (Competitive Advantage)

Features that set Hunch apart. Align with Core Value.

| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| **Permissionless market creation (anyone can ask anything)** | Polymarket end-users CANNOT create markets — they can only submit suggestions on Discord/Twitter for the Polymarket team to consider. Resolution proposers must hit the UMA whitelist (≥5 proposals + ≥95% accuracy in trailing 6-month window, snapshot on 2nd of each month). Manifold is permissionless but play-money. We let anyone create real-sats markets via a Nostr event. This IS the product. | HIGH | Nostr event with market schema. Spam filter via social graph, not curation. |
| **Multi-oracle marketplace (you choose your oracle)** | Polymarket has UMA (monolithic), Kalshi has internal staff team, Augur uses REPv2-staked reporters + Escalation Game forking, Manifold uses creator + moderators. We have a market of competing oracles with public Nostr reputation. | HIGH | Oracle profile on Nostr, attestation history, dispute records |
| **No KYC, no email, no phone — Nostr key only** | Polymarket V2 (post-April 2026) is actively pursuing CFTC approval for US users via their $112M QCEX acquisition, which will trigger KYC for that flow. Kalshi has full KYC. Manifold has Stripe-billed Mana purchases. We have none. | LOW | NIP-07/46 login is the only auth |
| **No custody — funds in DLCs or self-custodial wallet** | Polymarket V2 holds pUSD (ERC-20 1:1 USDC-backed) in their CTF. Kalshi holds USD in regulated custodian. Manifold holds Mana as a DB entry. We hold nothing during market lifetime; DLC + Cashu architecture means even mint compromise has bounded blast radius. | HIGH | DLC + Cashu mint architecture |
| **Public, machine-readable settlement (verify on-chain)** | Anyone can verify settlement via Bitcoin tx + Schnorr sig. Polymarket settles on Polygon (verifiable but $0 transaction context); we settle natively on Bitcoin L1 with Schnorr-signed oracle attestations. | MEDIUM | Settlement tx links displayed, oracle Schnorr sig verifiable |
| **Multiple competing frontends + mints** | Hunch.io can be down — protocol works. Polymarket is one entity; if their domain is seized, users are blocked. | MEDIUM | Protocol-first design; reference impls only |
| **Tor / IPFS / Radicle distribution** | Censorship-resistant access. Polymarket has no .onion despite their geo-block. | MEDIUM | Frontend pinned to IPFS, Tor hidden service, mirror on Cloudflare |
| **Atomic swap secondary market** | Trade YES/NO tokens via privacy-preserving Cashu swaps | HIGH | NUT-DLC + atomic swap NUT extensions |
| **Social discovery via Nostr web of trust** | Find good markets through who you follow on Nostr | MEDIUM | NIP-02 (follow lists) + Nostr reactions/zaps on markets |
| **Lightning native ⚡ everything (no on-ramps)** | Polymarket needs pUSD (Polygon ERC-20). Kalshi needs USD ACH. Manifold takes Stripe. We need sats. Zero KYC vector. Predyx is the only existing Lightning-native prediction market, but is centralized and custodial. | MEDIUM | All flows Lightning + Cashu |
| **Zap-to-bet from any Nostr client** | Power user UX: bet via Nostr reply with zap | MEDIUM | Custom Nostr event handler in Cashu mint |
| **Open API for forks + integrations** | Maximize forks, maximize survival | LOW | Document Nostr event kinds, mint API |
| **Multi-language UI (FR, EN, ES, PT, RU)** | Bitcoin community is global; Polymarket is English-mostly | MEDIUM | next-intl. Start FR+EN, add as community demands. |
| **Pro UX: hotkeys, dense view, advanced filters** | Polymarket loyalists are power users | MEDIUM | Linear/Vercel-grade keyboard nav |
| **Onion service from day 1** | Polymarket has no .onion. We do. | LOW | Tor hidden service for hunch.onion |

### Anti-Features (Commonly Requested, Often Problematic)

Features that seem good but create problems.

| Feature | Why Requested | Why Problematic | Alternative |
|---------|---------------|-----------------|-------------|
| **KYC for "high stake" markets** | Legal cover, regulatory comfort | Defeats the entire point. Once any KYC exists, it's a vector for regulatory pressure. Polymarket's path back into the US is via the QCEX-licensed exchange — i.e. KYC'd by design. We refuse that path. | Geo-block US, ToS-based deniability |
| **Centralized order matching engine** | Faster than P2P matching | Single point of censorship; reverts to Polymarket model | Cashu mint as orderbook (decentralizable via federation) + P2P Nostr Tier 2 |
| **Native HUNCH token / DAO governance** | Aligns incentives, raises funding | Securities law nightmare (Howey test, SEC). Augur's REP/REPv2 has had to migrate twice (v1→v2→v2-fork in 2026); the token IS the dispute mechanism, which means token-holder politics IS market resolution. No revenue alignment helps usage. | Bitcoin-only economy. Operator fees on Cashu mint. |
| **Mobile app (iOS/Android)** | More users prefer apps | App Store deplatforming risk (Apple removed Damus tipping). Approvals for "gambling apps" are a nightmare. | PWA with install-to-home-screen. Future: F-Droid + open mobile platforms. |
| **Email notifications / alerts** | "Don't miss my market resolution" | KYC vector; centralized SMTP dep; surveillance | Nostr DMs (NIP-17 gift-wrapped) for opt-in notifications |
| **Built-in oracle service we run** | Easier onboarding, "Hunch decides" | Hunch becomes the trust anchor. Antithesis of project. | Reference oracle code as one of many oracles. Educate market creators. |
| **Account recovery via "Forgot key?"** | Users lose Nostr keys, customer support burden | Custody by another name. | Education + Nostr NIP-26 delegation + hardware signers (Coldcard NIP-26) |
| **Real-money leveraged products** | Higher engagement, more revenue | Far higher legal risk. Margin calls require liquidation engine = custody. | Spot YES/NO only |
| **In-app chat / social feed** | Engagement, community | Moderation burden + spam | Link to Nostr clients; users discuss markets in their existing Nostr feed |
| **Internal "Hunch official" verified markets** | Increase trust in core markets | Two-tier system inevitably becomes curation. Polymarket's whitelist is exactly this and exactly the friction we reject. | All markets equal at protocol level; "trending" surfaces good ones |
| **Casino games (slots, dice) on the side** | Easy revenue | Different legal regime, dilutes the prediction market focus | None. Stay focused. |
| **"Take-down request" form** | Manage abuse complaints | Centralizes responsibility on Hunch the operator | Frontend operators can hide markets at their UI level (without removing from protocol); document the legal model |
| **Aggressive notifications / push** | Re-engagement, retention | Same surveillance issues | Calm tech. User pulls; we don't push. |
| **Margin / borrowing against positions** | DeFi composability | Requires liquidation engine = custody = trust point | Out of scope. Withdraw to LN + use other Lightning tools. |
| **Sweepstakes / play-money / dual-currency mode** | Avoid gambling regulation (Manifold's previous strategy) | Manifold ended its sweepstakes program in Feb 2025 — even play-money + sweepstakes attracted regulatory pressure. Half-measures don't help. | Pure sats. Lean fully into "this is a Bitcoin protocol, not a website." |

## Feature Dependencies

```
[Nostr login (NIP-07/46)]
    └──required by──> [Market browsing, account, ALL features]

[Cashu mint with NUT-DLC]
    └──required by──> [Buy YES/NO, Sell position, Resolution payout]

[Lightning deposit]
    └──required by──> [Buy YES/NO (sat funding)]

[Oracle Schnorr attestation infrastructure]
    └──required by──> [Resolution & payout, Dispute period]

[Nostr relay infrastructure]
    └──required by──> [Market discovery, Order matching Tier 2, Reputation]

[Multi-oracle FROST k-of-n]
    └──enhances──> [Trust minimization]
    └──depends on──> [Oracle attestation infrastructure]

[Permissionless market creation]
    └──depends on──> [Nostr market event schema (HIP-1)]
    └──depends on──> [Oracle marketplace UX]
    └──depends on──> [Anti-spam (social graph filtering + NIP-13 PoW)]

[Atomic swap secondary market]
    └──depends on──> [Cashu mint with NUT-DLC]
    └──enhances──> [Sell position]

[P2P matching Tier 2]
    └──depends on──> [Nostr order events (HIP-2)]
    └──conflicts with──> [Mint-only Tier 1 (different flow, can coexist as toggle)]

[Mobile PWA]
    └──enhances──> [All user flows]

[Tor hidden service]
    └──depends on──> [Frontend SSR / no-WebSocket fallback]

[Onchain settlement verifiability]
    └──depends on──> [DLC + Oracle attestation tx flow]
```

### Dependency Notes

- **All features require Nostr login**: This is the single auth primitive across the entire stack.
- **Cashu mint with NUT-DLC is THE critical path**: Without it, no liquidity, no UX. If NUT-DLC spec is unstable, we either ship without (pure custodial mint with off-chain promise) or contribute upstream.
- **Permissionless creation requires anti-spam BEFORE launch**: Augur's assassination markets showed up within 2 weeks of launch (July 2018). Manifold has been wrestling with spam markets publicly (see `manifold.markets/MachiNi/will-manifold-fix-the-rising-spam-m`). If we don't have social-graph filtering + NIP-13 PoW + NIP-51 mute lists ready, we'll get both spam AND tabloid headlines.
- **Multi-oracle FROST k-of-n is critical for credibility** but can be delivered as v1.5: ship with single-oracle markets first, prove the protocol, then add threshold.

## MVP Definition

### Launch With (v1, mainnet-no-cap target)

Minimum to be a credible mainnet prediction market.

- [ ] **Nostr login** — Auth foundation
- [ ] **Lightning deposit + withdraw** — Funds in/out
- [ ] **Browse markets (category + search + trending)** — Discovery
- [ ] **Permissionless market creation (Nostr event publish)** — Core differentiator
- [ ] **Binary YES/NO markets** — Resolution math is tractable; multi-outcome v2
- [ ] **Oracle selection at creation time** — Market creator picks one or more oracles
- [ ] **Buy YES / Buy NO via Cashu mint** — Core flow
- [ ] **Sell position via Cashu atomic swap** — Closing trades is table-stakes
- [ ] **Resolution & payout via oracle Schnorr sig** — DLC executes, mint pays out
- [ ] **Reference oracle service (run by Hunch initially)** — Bootstrap one credible oracle
- [ ] **Reference Cashu mint with NUT-DLC** — One mint operator (Hunch) at launch
- [ ] **Reference Nostr relay** — `relay.hunch.markets`
- [ ] **Onchain settlement verification UI** — Show settle tx + sig
- [ ] **Tor hidden service** — From day 1
- [ ] **Geo-block US (IP + Tor exit list)** — Legal mitigation
- [ ] **Account / portfolio view** — User's positions across markets
- [ ] **Anti-spam: social graph filter (mute users not in extended follow graph) + NIP-13 PoW on market events + NIP-51 mute list integration** — Augur-lesson prevention, layered defense
- [ ] **"Mark market as invalid" community signal** — Augur-style invalidity reporting (community curation)
- [ ] **HIP-0..N specs published as Nostr long-form notes + GitHub + Radicle** — Protocol-first

### Add After Validation (v1.x, 3-6 months post-launch)

- [ ] **Multi-oracle FROST k-of-n attestation** — When demand for higher-stake markets exceeds single-oracle trust
- [ ] **P2P Tier 2 matching via Nostr** — When power users ask for non-custodial path during market lifetime
- [ ] **Atomic-swap secondary market on Cashu** — Liquidity boost
- [ ] **Federated mint (FROST multisig)** — Reduce mint operator trust
- [ ] **Nostr DM resolution alerts (NIP-17 gift-wrap)** — Opt-in notifications
- [ ] **Multi-language (FR added as v1+, then ES/PT/RU)** — Geographic expansion
- [ ] **Pro UX (hotkeys, dense view)** — Trader UX
- [ ] **Zap-to-bet from Nostr client** — Cross-app UX

### Future Consideration (v2+)

- [ ] **Multi-outcome markets (>YES/NO)** — Schnorr adaptor sig multi-outcome (Manifold proved demand: their multiple-choice + numeric markets are highly popular)
- [ ] **Just-in-time arbitrage between Yes/No legs of multi-outcome** — Manifold's mechanism for linking liquidity across options
- [ ] **Conditional / parlay markets** — Composable conditions across markets
- [ ] **Lightning DLC channels (instant settle)** — When atomic.finance/cara primitives are production
- [ ] **Mobile native (PWA hits 70%+ engagement = signal to invest)** — Tauri or React Native
- [ ] **Reputation algorithm v2 (machine learning over Nostr graph)** — When v1 fails
- [ ] **Cross-mint atomic-swap routing** — Multi-mint interop
- [ ] **Programmatic oracles (verifiable computation)** — For deterministic markets (sports score = block height comparison)
- [ ] **API / SDK for external traders + bots** — Power users
- [ ] **Multi-frontend explicit support (theming, branding kits)** — Encourage forks
- [ ] **DAO governance via Nostr signed votes** — Only after multi-operator federation exists

## Feature Prioritization Matrix

| Feature | User Value | Implementation Cost | Priority |
|---------|------------|---------------------|----------|
| Nostr login | HIGH | LOW | P1 |
| Lightning deposit/withdraw | HIGH | MEDIUM | P1 |
| Browse + search markets | HIGH | LOW | P1 |
| Buy YES/NO via Cashu | HIGH | HIGH | P1 |
| Sell position via Cashu | HIGH | HIGH | P1 |
| Resolution & payout | HIGH | HIGH | P1 |
| Permissionless market creation | HIGH | HIGH | P1 |
| Single oracle (reference) | HIGH | MEDIUM | P1 |
| Settlement verification UI | MEDIUM | LOW | P1 |
| Tor + Geo-block + IPFS pin | MEDIUM | LOW | P1 |
| Anti-spam (social graph + NIP-13 PoW + NIP-51) | HIGH | MEDIUM | P1 |
| Multi-oracle FROST k-of-n | HIGH | HIGH | P2 |
| P2P Tier 2 (Nostr matching) | MEDIUM | HIGH | P2 |
| Federated mint | HIGH | HIGH | P2 |
| Multi-outcome markets | MEDIUM | HIGH | P3 |
| Mobile native | LOW (PWA enough) | HIGH | P3 |
| DAO governance | LOW (premature) | HIGH | P3 |
| Programmatic oracles | MEDIUM | HIGH | P3 |

**Priority key:**
- P1: Must have for mainnet launch
- P2: Add when v1 proves traction (post-launch 3-6 mo)
- P3: Future, defer until clear product-market fit

## Competitor Feature Analysis

The canonical comparison anchor. Updated for 2026 state of all named competitors.

| Feature | Polymarket V2 (Apr 2026+) | Augur (2026 fork) | Manifold | Kalshi | Mostro/Robosats | Predyx | Hunch (us) |
|---------|---------------------------|-------------------|----------|--------|-----------------|--------|------------|
| Market creation | End-users cannot create. Suggest via Twitter/Discord; PM team curates. | Permissionless via REPv2 staking | Permissionless, anyone, <30s with textbox + close date | CFTC-listed contracts only; Kalshi staff approve | N/A (not predictions) | Users can create markets (custodial/centralized) | **Permissionless via Nostr event** |
| Resolution proposer | UMA whitelist: ≥5 proposals + ≥95% accuracy in trailing 6mo (snapshot monthly 2nd) | Any REPv2 staker | Creator resolves; mods can override for abuse/error | Kalshi internal staff | N/A | Centralized (Predyx team) | **Any oracle; market creator picks; multi-oracle FROST optional** |
| Oracle | UMA Optimistic Oracle V2 ($750 pUSD bond, 2h challenge, escalation to DVM 48–96h) | REPv2-staked reporters + Escalation Game forking (8-week dispute → universe split if 2.5% REP threshold hit) | Creator + Manifold mods | Internal Kalshi staff | N/A | Predyx team | **Multi-oracle marketplace; Schnorr Schnorr-attested; INVALID is a first-class outcome via DLC** |
| Resolution time | Undisputed ≈2h after proposal; disputed = 4-6 days via DVM | Up to 8-week fork in extreme case; routine resolutions ~24-48h | Instant once creator marks; mods may delay | Hours after event | N/A | Hours (centralized) | **2-24h depending on oracle (configurable per market)** |
| Custody | pUSD ERC-20 on Polygon, held in Polymarket CTF contract (Conditional Token Framework). 1:1 USDC-backed by smart contract; not directly redeemable freely. | ETH/DAI in Augur contracts | Mana = DB entry, no real custody risk (it's not money) | USD held by Kalshi (CFTC-regulated DCM) | None (P2P escrow via LN hold invoices) | Custodial Lightning balance | **DLC collateral on Bitcoin L1 + Cashu YES/NO tokens; bounded mint blast radius via DLC** |
| KYC | Off-shore exchange (current): some KYC for fiat ramps. Pending QCEX US relaunch will require full KYC for US-side. | None | None for play-money use; Stripe handles fiat for Mana purchases | Full KYC (US-regulated DCM) | None | None | **None — Nostr key only** |
| US access | Geo-blocked since 2022 CFTC settlement; pending re-entry via $112M QCEX acquisition (full KYC) | Not blocked (decentralized) | US-legal as play-money; sweepstakes ended Feb 2025 | US-licensed (CFTC) | None / Tor | None apparent | **Geo-blocked (official frontend); protocol stays neutral** |
| Fiat | pUSD (USDC-backed ERC-20); ACH/debit on US side post-QCEX | DAI | Mana (USD-pegged play money; $1 ≈ M100) | USD | None | None (Lightning only) | **None — Lightning only** |
| Token | None at platform; pUSD = USDC stable | REPv2 (governance + oracle staking; was REP v1, migrated; another fork-migration June-Aug 2026) | Mana (DB entry, not crypto, no redemption) | None | None | None | **None** |
| Mobile | Web + iOS app | Web (Augur app legacy) | Web + iOS app | Web + iOS + Android | Robosats: Tor browser + Android; Mostro: Flutter mobile + web client | Web | **PWA only (v1)** |
| Dispute mechanism | $750 pUSD bond, 2h window, second-dispute escalates to UMA DVM (token-holder vote 48–96h) | Escalation Game → REPv2 staking → fork into competing universes if 2.5% REPv2 dispute threshold reached | Report to mods; "re-resolution" possible; N/A resolution for cancellation | Internal Kalshi review; CFTC complaint path exists | Mostro: human escrow review; Robosats: staff arbitration | Centralized resolution | **Schnorr attestation + Nostr challenge + reputation slashing (social, not staked) + INVALID outcome built into DLC** |
| Censorship resistance | LOW (US-blocked, KYC inbound, pUSD freezable on contract level) | HIGH (decentralized, but TVL only $1.7M vs Polymarket $428M = dormant) | LOW (Manifold can delete/edit any market) | LOW (CFTC-supervised) | HIGH (Tor + P2P + Nostr) | LOW (single operator) | **HIGH by design** |
| Liquidity model | CLOB V2 (orderbook) + AMM hybrid; $1M maker-rewards program at V2 launch | Orderbook + Constant-Product with Invalid Insurance (Augur Turbo experiment) | AMM + limit orders combined (just-in-time arbitrage between Yes/No legs of multi-choice) | CLOB | P2P maker/taker matching | Internal house/AMM | **Cashu mint orderbook (Tier 1) + Nostr P2P (Tier 2)** |
| Settlement | On-chain Polygon (pUSD ERC-20 transfers) | On-chain Ethereum | Internal DB | Internal DB (CFTC-supervised) | On-chain Bitcoin (LN hold invoice settle) | Custodial Lightning | **On-chain Bitcoin via DLC** |
| Volume / size (2026) | $428M TVL (DefiLlama) | $1.7M TVL — effectively dormant despite the fork | ~$0 in real-money (Mana only) | $52B cumulative event contracts as of Mar 2026; ~87% sports | Robosats: thousands of monthly users | Small (active community, no large public metric) | **Pre-launch** |
| Recent regulatory | DOJ/CFTC dropped investigations July 2025; pursuing US re-entry via QCEX | Self-inflicted Apr-Aug 2026 fork over Artemis II launch dispute (intentional test by Micah Zoltu) | Sweepstakes program ended Feb 2025 under regulatory pressure | CFTC March 2026 staff advisory: real-time monitoring obligation; Kalshi banned candidate self-betting + sports-insider trading | Robosats: zero KYC, Tor-default, unchanged | None public | **Pre-launch; offshore entity, geo-block US** |

## Key UX Patterns to Steal / Adapt

Now with concrete attribution to specific competitor flows.

1. **Polymarket's single-buy-box on market page** — Still the best UX in the space across V1 and V2. Adapt directly: outcome chip toggle (YES / NO), sats input, slippage preview, "Buy" CTA.
2. **Polymarket V2's `Resolved` badge with on-chain verification link** — Build trust by exposing settlement. We do better: link to Bitcoin tx + oracle Schnorr sig verifier in-page.
3. **Polymarket's $1M maker-rewards program at V2 launch (Apr 28, 2026)** — Liquidity incentives matter at launch. Consider mirroring with operator-funded sat rewards for early makers (decision pending; opens token-vs-not-token line we're firm on, so likely just sat grants).
4. **Augur's three-outcome resolution (YES / NO / INVALID)** — Crucial for permissionless markets. We adapt to INVALID as a third Cashu token type that pays out 50/50 if oracle attests invalid. Augur's "Invalid Insurance" pattern (Augur Turbo) is also worth studying for AMM design.
5. **Manifold's market creation in <30 seconds (textbox + close date + resolution criteria)** — Lower friction than anyone else. Aim for parity: title + close date + oracle pick + initial liquidity in one form. Their permissionless model is exactly our model.
6. **Manifold's multiple-choice with linked-liquidity AMM** — JIT arbitrage between Yes/No legs of multi-option markets. ~3ms arbitrage on 4 options in browser. Defer to v2 (Schnorr adaptor sig needed for multi-outcome DLC) but the UX target is clear.
7. **Manifold's market creator resolves their own market (with mod override)** — In our model, the *oracle* the creator selected resolves (not the creator). This separates "who proposes the market" from "who settles", which Manifold conflates. Manifold's clarity-of-control UX is still adoptable.
8. **Robosats' robot avatar + nym + private-token recovery flow** — Privacy-first onboarding pattern: random avatar, no email, single recovery token. Map to Nostr key generation flow for new users.
9. **Robosats' Tor-only mode toggle** — Simple cypherpunk-friendly UX. Show clearnet warning when not on Tor.
10. **Mostro's mobile order-book card UI** — Flutter app uses card-based order list with real-time updates and step-by-step trade guidance. Worth studying for our Tier 2 P2P matcher UI.
11. **Mostro's NIP-17 gift-wrapped messaging for trade chat** — Use the same primitive for oracle ⟷ disputant communication and creator ⟷ challenger communication.
12. **Polymarket's category chips with live volume counts** — Politics, Sports, Crypto, Pop Culture as quick filters with active-market counts.
13. **Polymarket's price chart with bet placement overlay** — Tradingview-lite charts with click-to-bet (we ship simpler v1: SVG sparkline + bet box).
14. **Kalshi's contract-spec page** — Every market shows clear underlying contract + resolution source + dispute path. Steal the discipline of always linking to the *source of truth* (block height for Bitcoin events, API endpoint for sports scores, etc.).
15. **Augur's "Volume / Open Interest / 24h Change" market header triad** — Adopt the trader-grade metric strip; even Polymarket's UX is light on these.

## Anti-spam Strategy for Permissionless Markets

The Augur assassination markets in 2018 showed the failure mode for permissionless creation. Manifold continues to wrestle with spam in 2026 (`manifold.markets/MachiNi/will-manifold-fix-the-rising-spam-m`). The core insight from both: **content moderation cannot be centralized in our model, so the spam strategy must be defensive in depth and shift cost to the spammer**.

Our layered defense:

1. **NIP-13 Proof-of-Work on market creation events** — Every new market Nostr event must include leading-zero bits per NIP-13. Configurable difficulty per relay; reference relay starts at 20 bits (≈1M hashes ≈ seconds on commodity CPU). Cost scales with spam volume; legitimate creators don't notice.
2. **NIP-65 outbox model relay discovery** — Markets propagate via author's preferred-relays NIP-65 list, not a single firehose. Spammers can't blast a global relay because there isn't one.
3. **Social graph filter by default** — UI only shows markets created by people the user follows or is N hops from on Nostr (configurable: 1-hop, 2-hop, all). Default = 2-hop, with an "all markets" toggle for explorers.
4. **NIP-51 (kind 10000) mute list integration** — Frontend honors the user's existing Nostr mute list automatically. Mute a creator on Damus = they vanish from Hunch too. Mute keyword "assassinate" = filter applied across markets.
5. **Community "INVALID" flag** — Any oracle can attest "invalid" → market resolves 50/50 (refund) automatically via DLC. Per Augur's model: payout halfway between min and max for scalar; equal payout for binary. This makes spam markets economically pointless (no profitable resolution).
6. **Reputation-aware sorting** — Markets sorted by creator's historical resolution accuracy, oracle's accuracy, dispute history. New creators with no history are sorted last unless they have follow-graph proximity.
7. **Per-frontend moderation** — `hunch.io` can hide markets from its UI without removing them from the protocol; this delegates "what's tasteful" to the frontend operator. Other frontends can apply different policies. Manifold's model — `Manifold believes users should have autonomy over their own markets as much as possible, and tries to keep moderation to a minimum and empower users with tools to control the content that they want to see` — is the right philosophical baseline.
8. **Explicit "Hide" buttons** — User can mute creator, mute topic, or mute markets matching keywords. Mute actions write to user's NIP-51 list so they propagate across Nostr clients.
9. **No "trending" until N participants** — Markets need minimum participation (≥5 distinct Nostr pubkeys placing orders) to be discoverable beyond the creator's followers.
10. **Slow-roll discovery** — New markets gain visibility gradually based on reactions, follows, zaps received, time-since-creation, and follow-graph weight.
11. **Operator ToS for hosted frontend** — `hunch.io` explicitly prohibits certain content (markets directly tied to identifiable individual death; markets that incentivize crime; markets resolving on classified information per Murphy/Levin proposed bill discussion). Protocol stays neutral. Forks free to host differently.
12. **No central index for market discovery** — Discovery is entirely Nostr-driven. There's no global database for a spammer to dump 10K markets into and own a category.

### Real-world data backing the strategy

- **Augur 2018**: Assassination markets surfaced within ~2 weeks of launch with no defensive infrastructure. The Newsweek + Futurism coverage shaped public perception of decentralized prediction markets for years. Augur subsequently introduced the INVALID outcome but only as a reporting mechanism, not a default filter.
- **Manifold 2025-2026**: Active community markets (e.g. `Will Manifold fix the rising spam markets issue by end of week`) demonstrate that even with mods + soft policy, permissionless creation produces spam pressure continuously. Their philosophy of `user autonomy + minimal moderation + tooling to filter` is the closest validated model to ours.
- **Manifold death-market controversy (Mar 2026)**: Manifold's nuclear-detonation markets prompted the Murphy/Levin bill proposing to restrict markets resolving on military actions / deaths. Lesson: even play-money markets attract legislative attention. Our defense is not a content policy at the protocol level (we can't enforce one) but rigorous operator-level frontend curation + clearly documented protocol-frontend separation.

## Permissionless Market Creation: Lessons from the Field

The social cost of "anyone can create a market":

- **Augur (2018)**: Assassination markets. Negative press defined the project for years; never recovered narrative.
- **Manifold (2023-2026)**: Continuous spam pressure; intentionally-controversial markets (`IsaacKing/will-i-stop-creating-intentionally`); nuclear-detonation markets prompting US legislative action. Survives because play-money lowers individual market harm.
- **Polymarket**: Chose the opposite — curated creation, permissioned proposers — explicitly because they cannot afford the Augur narrative as a regulated entity.
- **Predyx**: Centralized operator can refuse/remove; users *can* create markets but operator gates publication. This is "permissionless lite".

Our model uniquely combines: permissionless protocol creation + frontend operator curation + real-sats stakes. We accept the social risk because:
1. We're a protocol, not a single frontend (forks absorb the moderation policy decision).
2. INVALID outcome neutralizes economic incentive for bad-faith markets (no settlement = no payout = no profit motive).
3. Social graph filtering means most users never see spam markets.
4. We pre-document the legal model with counsel before launch; we won't be surprised by Murphy/Levin-style legislation.

**Open risk**: a high-profile abusive market still attracts press regardless of social-graph filtering, because journalists explicitly look for the worst content. PR playbook required.

## Sources

### Polymarket V2 (April 2026 launch)
- [Polymarket Exchange Upgrade: April 28, 2026 (Help Center)](https://help.polymarket.com/en/articles/14762452-polymarket-exchange-upgrade-april-28-2026)
- [Migrating to CLOB V2 (Polymarket Documentation)](https://docs.polymarket.com/v2-migration)
- [Polymarket V2 Launch: A New Phase for Prediction Markets (KuCoin)](https://www.kucoin.com/blog/polymarket-v2-launch-pusd-prediction-market-infrastructure-2026)
- [Polymarket rolls out pUSD migration with CLOB v2 (AMBCrypto)](https://ambcrypto.com/polymarket-rolls-out-pusd-migration-with-clob-v2-exchange-upgrade/)
- [Polymarket Moves to Regain U.S. Access With CFTC Approval Push (CryptoTimes Apr 28, 2026)](https://www.cryptotimes.io/2026/04/28/polymarket-moves-to-regain-u-s-access-with-cftc-approval-push/)
- [Polymarket's CLOB v2 Goes Live With $1M Rewards, New pUSD Token (CryptoTimes)](https://www.cryptotimes.io/2026/04/28/polymarkets-clob-v2-goes-live-with-1m-rewards-new-pusd-token/)
- [Polymarket Seeks CFTC Approval to Open Main Exchange to US Traders (Cryptopolitan)](https://www.cryptopolitan.com/polymarket-seeks-cftc-approval-us-traders/)

### Polymarket resolution / proposer model
- [Polymarket Resolution Docs (UMA Oracle)](https://docs.polymarket.com/developers/resolution/UMA)
- [Polymarket Help: How Markets Are Disputed](https://help.polymarket.com/en/articles/13364551-how-are-markets-disputed)
- [How to Propose Polymarket Resolutions 2026 — Whitelist, Bonds, Rewards](https://startpolymarket.com/learn/how-to-propose-resolutions/)
- [Default Proposer Whitelist (UMA Documentation)](https://docs.uma.xyz/developers/managedoptimisticoraclev2/default-proposer-whitelist)
- [Managed Proposers Update (UMA Blog)](https://blog.uma.xyz/articles/managed-proposers-update)
- [How Are Markets Created? (Polymarket Help Center)](https://help.polymarket.com/en/articles/13364541-how-are-markets-created)
- [All Polymarket Market Categories 2026 (Polycopy)](https://polycopy.app/polymarket-market-categories)
- [Inside UMA Optimistic Oracle (Rocknblock)](https://rocknblock.io/blog/how-prediction-markets-resolution-works-uma-optimistic-oracle-polymarket)

### Augur 2026 state and fork
- [Augur is Rebooting (augur.net)](https://www.augur.net/)
- [The Augur Fork is Here (Augur Blog)](https://www.augur.net/blog/the-augur-fork-is-here/)
- [Augur Fork Begins to Test Prediction Market Dispute System (Bitcoin Foundation)](https://bitcoinfoundation.org/news/prediction-markets/augur-fork-begins-dispute/)
- [Augur v2: REP Migration (GitHub)](https://github.com/AugurProject/V2Migration/blob/master/V2Deployment.md)
- [Augur Help: Market Creation Explained](https://augur.gitbook.io/help-center/market-creation/market-creation-explained)
- [Augur Help: Trading Page Explained (INVALID outcome)](https://augur.gitbook.io/help-center/trading/trading-page-explained)
- [Augur Constant Product with Invalid Insurance (Micah Zoltu, Medium)](https://micah-zoltu.medium.com/augur-constant-product-with-invalid-insurance-385fca7efbc7)
- [Augur Assassination Markets retrospective (Newsweek 2018)](https://www.newsweek.com/welcome-augur-cryptocurrency-death-market-where-you-can-bet-donald-trump-1043571)
- [Augur Dark Side (Futurism 2018)](https://futurism.com/augur-assassination-marketplace-decentralized-blockchain)

### Manifold Markets (in depth)
- [Manifold FAQ (docs.manifold.markets/faq)](https://docs.manifold.markets/faq)
- [Manifold Markets Review 2026 (Cryptoslate)](https://cryptoslate.com/prediction-markets/manifold-predictions-review/)
- [Manifold Markets Review 2026 (Cryptonews)](https://cryptonews.com/cryptocurrency/manifold-markets-review/)
- [Manifold Multiple Choice Markets announcement](https://news.manifold.markets/p/multiple-choice-markets)
- [Will Manifold fix the rising spam markets issue (community market)](https://manifold.markets/MachiNi/will-manifold-fix-the-rising-spam-m)
- [Will I stop creating intentionally controversial markets (Isaac King)](https://manifold.markets/IsaacKing/will-i-stop-creating-intentionally)
- [Will we have a clear policy on Manifold market deletion/edits (Isaac King)](https://manifold.markets/IsaacKing/will-we-have-a-clear-policy-by-the)
- [Manifold vs the World (Above The Fold blog)](https://news.manifold.markets/p/manifold-vs-the-world)
- [Death Markets analysis (The Diff)](https://www.thediff.co/archive/death-markets/)
- [CNBC: Prediction markets face questions on Iran war bets, regime change, nuclear detonation (Mar 2026)](https://www.cnbc.com/2026/03/09/prediction-markets-wagers-bets-iran-war-kalshi-polymarket.html)
- [Manifold (Longterm Wiki)](https://www.longtermwiki.com/wiki/E546)

### Kalshi
- [Kalshi (Wikipedia)](https://en.wikipedia.org/wiki/Kalshi)
- [Kalshi (Britannica Money)](https://www.britannica.com/money/Kalshi-Inc)
- [How are prediction markets regulated? (Kalshi Market Integrity Hub)](https://kalshi.com/market-integrity/regulation)
- [CFTC Issues New Guidance for Prediction Markets (Regulatory Oversight, Mar 2026)](https://www.regulatoryoversight.com/2026/03/cftc-issues-new-guidance-for-prediction-markets/)
- [Prediction Markets: Policy Issues for Congress (CRS)](https://www.congress.gov/crs-product/IF13187)
- [Kalshi Expansion Prompts CFTC to Reexamine Trader Position Reporting (Bloomberg)](https://www.bloomberg.com/news/articles/2026-05-01/cftc-reviews-trader-data-report-as-kalshi-expands-in-commodities)

### Robosats
- [RoboSats GitHub README](https://github.com/RoboSats/robosats)
- [RoboSats: Exchanging Bitcoin Easily, Privately And KYC-Free (Bitcoin Magazine)](https://bitcoinmagazine.com/business/robosats-private-bitcoin-exchange)
- [Learn RoboSats](https://learn.robosats.org/)
- [RoboSats Quick Start: Tor docs](https://github.com/RoboSats/robosats/blob/main/docs/_pages/docs/00-quick-start/02-tor.md)
- [RoboSats Review 2026 (Blockdyor)](https://blockdyor.com/robosats/)

### Mostro
- [Mostro homepage](https://mostro.network/)
- [Mostro Community](https://mostro.community/)
- [Mostro mobile (GitHub, Flutter)](https://github.com/MostroP2P/mobile)
- [Mostro daemon (GitHub)](https://github.com/MostroP2P/mostro)
- [Mostro web client (GitHub)](https://github.com/MostroP2P/mostro-web)
- [MostroP2P architecture (DeepWiki)](https://deepwiki.com/MostroP2P/mostro)

### Predyx (Bitcoin Lightning prediction market)
- [Predyx Beta site](https://beta.predyx.com/)
- [Predyx Markets browser](https://beta.predyx.com/market)
- [Predyx (X / Twitter)](https://x.com/predyx_markets/status/1948273728783286758)
- [Predyx: A No-KYC Bitcoin-Native Prediction Market (YouTube)](https://www.youtube.com/watch?v=U7U0auM4KWQ)
- [Predyx Lightning Network node profile](https://lightningnetwork.plus/node/03b465d6fcd7305c9fbfac523ec6c4a179756ad03e0400ed220ee303161bedbbf4/node_channels)

### Adjacent / lesser-known platforms
- [PredictionStrike (sports stock-market style)](https://predictionstrike.com/)
- [Insight Prediction](https://insightprediction.com/)
- [PredictIt Review 2026 (tech-insider.org)](https://tech-insider.org/prediction-markets/platforms/predictit-review/)
- [Civ Kit P2P marketplace on Nostr + Lightning (Bitcoin Magazine)](https://bitcoinmagazine.com/technical/bitcoin-researchers-introduce-civ-kit-p2p-marketplace)
- [Bitcoin Hivemind / Truthcoin (Drivechain-based prediction market proposal)](https://bitcoinhivemind.com/)
- [Best Prediction Markets in 2026 (CryptoSlate)](https://cryptoslate.com/prediction-markets/)
- [Best Prediction Markets 2026 (Bitcoin Foundation)](https://bitcoinfoundation.org/news/opinion/best-prediction-markets-2026/)
- [Top Polymarket Alternatives 2026 (Laika Labs)](https://laikalabs.ai/prediction-markets/top-polymarket-alternatives)

### Nostr spam mitigation primitives
- [NIP-13 Proof of Work](https://nips.nostr.com/13)
- [NIP-51 Lists (mute lists, kind 10000)](https://nips.nostr.com/51)
- [NIP-65 Relay List Metadata (outbox model)](https://github.com/nostr-protocol/nips/blob/master/65.md)
- [Nostr NIPs Reference](https://nips.nostr.com/)

---
*Feature research for: Bitcoin-native cypherpunk prediction market protocol*
*Enriched and verified: 2026-05-27*
