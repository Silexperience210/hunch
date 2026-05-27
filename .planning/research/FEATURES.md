# Feature Research

**Domain:** Permissionless Bitcoin-native prediction market (Polymarket-class product, cypherpunk philosophy)
**Researched:** 2026-05-27
**Confidence:** HIGH for table stakes (analyzed Polymarket, Augur, Manifold, Kalshi); MEDIUM for differentiators and cypherpunk-specific UX patterns (uncharted territory).

## Feature Landscape

### Table Stakes (Users Expect These)

Features users assume exist. Missing these = product feels incomplete.

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| **Browse markets by category + search** | Polymarket's "Politics / Sports / Crypto" tabs are the dominant pattern | LOW | Nostr-indexed events with tags. SQLite cache for fast frontend queries. |
| **Trending / Most-Active markets** | Discovery of activity drives engagement | LOW | Sort by Nostr `kind:38888` order event count + total volume |
| **Market detail page (price chart, resolution rules, oracle)** | Polymarket-grade detail page is the comparison benchmark | MEDIUM | Price history reconstructed from order events. Oracle identity + reputation visible. |
| **Buy YES / Buy NO with sat amount** | Single buy box like Polymarket | MEDIUM | WebLN payment → Cashu token mint → backed by DLC |
| **Sell position before resolution** | Closing positions is essential; otherwise feels like roach motel | MEDIUM | Atomic swap of YES/NO tokens via Cashu mint (Tier 1) or Nostr taker (Tier 2) |
| **Current price + odds display** | Probability % view (not just sat price) | LOW | Computed from latest order |
| **Account balance + portfolio** | "What I hold across markets" | MEDIUM | Locally-indexed Cashu wallet state |
| **Lightning deposit / withdraw** | Funds in/out without on-ramps | MEDIUM | LDK Node or Mint's LN backend, BOLT-11 / BOLT-12 invoices |
| **Resolution & payout flow** | Automated at expiry — must "just work" | HIGH | Oracle Schnorr sig → DLC execution → mint redistributes |
| **Login via Nostr (NIP-07 / NIP-46)** | Standard for Nostr-native apps | LOW | NDK signer abstraction handles both |
| **Order history + transaction log** | Tax / audit / "what did I do" | MEDIUM | Local-first (privacy) with optional Nostr backup |
| **Market rules + dispute info clearly visible** | Trust requires understanding "who decides" | LOW | Display oracle pubkey, oracle reputation, dispute period |
| **Mobile-responsive web (PWA)** | 60%+ traffic mobile in 2026 | MEDIUM | Next.js PWA with manifest, install prompt |
| **Tor-compatible (no JS-only flows that break in Tor Browser)** | Cypherpunk core audience uses Tor | MEDIUM | Server-side rendering for first paint, no WebSocket-only flows |

### Differentiators (Competitive Advantage)

Features that set Hunch apart. Align with Core Value.

| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| **Permissionless market creation (anyone can ask anything)** | Polymarket only lets curated proposers create markets (5+ accurate proposals, 6mo, 95% accuracy). We let anyone. This IS the product. | HIGH | Nostr event with market schema. Spam filter via social graph, not curation. |
| **Multi-oracle marketplace (you choose your oracle)** | Polymarket has UMA, Kalshi has internal team. We have a market of competing oracles with public reputation. | HIGH | Oracle profile on Nostr, attestation history, dispute records |
| **No KYC, no email, no phone — Nostr key only** | Polymarket has KYC for US-blocked users. We have none. | LOW | NIP-07/46 login is the only auth |
| **No custody — funds in DLCs or self-custodial wallet** | Polymarket holds USDC in their CTF. We hold nothing. | HIGH | DLC + Cashu mint architecture |
| **Public, machine-readable settlement (verify on-chain)** | Anyone can verify settlement via Bitcoin tx + Schnorr sig. No "trust the oracle" black box. | MEDIUM | Settlement tx links displayed, oracle Schnorr sig verifiable |
| **Multiple competing frontends + mints** | Hunch.io can be down — protocol works | MEDIUM | Protocol-first design; reference impls only |
| **Tor / IPFS / Radicle distribution** | Censorship-resistant access | MEDIUM | Frontend pinned to IPFS, Tor hidden service, mirror on Cloudflare |
| **Atomic swap secondary market** | Trade YES/NO tokens via privacy-preserving Cashu swaps | HIGH | NUT-DLC + atomic swap NUT extensions |
| **Social discovery via Nostr web of trust** | Find good markets through who you follow on Nostr | MEDIUM | NIP-02 (follow lists) + Nostr reactions/zaps on markets |
| **Lightning native ⚡ everything (no on-ramps)** | Polymarket needs USDC. We need sats. Zero KYC vector. | MEDIUM | All flows Lightning + Cashu |
| **Zap-to-bet from any Nostr client** | Power user UX: bet via Nostr reply with zap | MEDIUM | Custom Nostr event handler in Cashu mint |
| **Open API for forks + integrations** | Maximize forks, maximize survival | LOW | Document Nostr event kinds, mint API |
| **Multi-language UI (FR, EN, ES, PT, RU)** | Bitcoin community is global, Polymarket is English-mostly | MEDIUM | next-intl. Start FR+EN, add as community demands. |
| **Pro UX: hotkeys, dense view, advanced filters** | Polymarket loyalists are power users | MEDIUM | Linear/Vercel-grade keyboard nav |
| **Onion service from day 1** | Polymarket has no .onion. We do. | LOW | Tor hidden service for hunch.onion |

### Anti-Features (Commonly Requested, Often Problematic)

Features that seem good but create problems.

| Feature | Why Requested | Why Problematic | Alternative |
|---------|---------------|-----------------|-------------|
| **KYC for "high stake" markets** | Legal cover, regulatory comfort | Defeats the entire point. Once any KYC exists, it's a vector for regulatory pressure. | Geo-block US, ToS-based deniability |
| **Centralized order matching engine** | Faster than P2P matching | Single point of censorship; reverts to Polymarket model | Cashu mint as orderbook (decentralizable via federation) + P2P Nostr Tier 2 |
| **Native HUNCH token / DAO governance** | Aligns incentives, raises funding | Securities law nightmare (Howey test, SEC). Augur's REP was a constant liability. No revenue alignment helps usage. | Bitcoin-only economy. Operator fees on Cashu mint. |
| **Mobile app (iOS/Android)** | More users prefer apps | App Store deplatforming risk (Apple removed Damus tipping). Approvals for "gambling apps" are a nightmare. | PWA with install-to-home-screen. Future: F-Droid + open mobile platforms. |
| **Email notifications / alerts** | "Don't miss my market resolution" | KYC vector; centralized SMTP dep; surveillance | Nostr DMs (NIP-04 / NIP-44) for opt-in notifications |
| **Built-in oracle service we run** | Easier onboarding, "Hunch decides" | Hunch becomes the trust anchor. Antithesis of project. | Reference oracle code as one of many oracles. Educate market creators. |
| **Account recovery via "Forgot key?"** | Users lose Nostr keys, customer support burden | Custody by another name. | Education + Nostr NIP-26 delegation + hardware signers (Coldcard NIP-26) |
| **Real-money leveraged products** | Higher engagement, more revenue | Far higher legal risk. Margin calls require liquidation engine = custody. | Spot YES/NO only |
| **In-app chat / social feed** | Engagement, community | Moderation burden + spam | Link to Nostr clients; users discuss markets in their existing Nostr feed |
| **Internal "Hunch official" verified markets** | Increase trust in core markets | Two-tier system inevitably becomes curation | All markets equal at protocol level; "trending" surfaces good ones |
| **Casino games (slots, dice) on the side** | Easy revenue | Different legal regime, dilutes the prediction market focus | None. Stay focused. |
| **"Take-down request" form** | Manage abuse complaints | Centralizes responsibility on Hunch the operator | Frontend operators can hide markets at their UI level (without removing from protocol); document the legal model |
| **Aggressive notifications / push** | Re-engagement, retention | Same surveillance issues | Calm tech. User pulls; we don't push. |
| **Margin / borrowing against positions** | DeFi composability | Requires liquidation engine = custody = trust point | Out of scope. Withdraw to LN + use other Lightning tools. |

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
    └──depends on──> [Anti-spam (social graph filtering)]

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
- **Permissionless creation requires anti-spam BEFORE launch**: Augur's assassination markets showed up within 2 weeks of launch (July 2018). If we don't have social-graph filtering ready, we'll get the same headlines.
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
- [ ] **Anti-spam: social graph filter (mute users not in extended follow graph)** — Augur-lesson prevention
- [ ] **"Mark market as invalid" community signal** — Augur-style invalidity reporting (community curation)
- [ ] **HIP-0..N specs published as Nostr long-form notes + GitHub + Radicle** — Protocol-first

### Add After Validation (v1.x, 3-6 months post-launch)

- [ ] **Multi-oracle FROST k-of-n attestation** — When demand for higher-stake markets exceeds single-oracle trust
- [ ] **P2P Tier 2 matching via Nostr** — When power users ask for non-custodial path during market lifetime
- [ ] **Atomic-swap secondary market on Cashu** — Liquidity boost
- [ ] **Federated mint (FROST multisig)** — Reduce mint operator trust
- [ ] **Nostr DM resolution alerts (NIP-44)** — Opt-in notifications
- [ ] **Multi-language (FR added as v1+, then ES/PT/RU)** — Geographic expansion
- [ ] **Pro UX (hotkeys, dense view)** — Trader UX
- [ ] **Zap-to-bet from Nostr client** — Cross-app UX

### Future Consideration (v2+)

- [ ] **Multi-outcome markets (>YES/NO)** — Schnorr adaptor sig multi-outcome
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
| Anti-spam (social graph filter) | HIGH | MEDIUM | P1 |
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

| Feature | Polymarket | Augur | Manifold | Mostro/Robosats | Hunch (us) |
|---------|------------|-------|----------|-----------------|------------|
| Market creation | Whitelisted proposers (5+ accurate, 6mo, 95%) | Permissionless via REP staking | Permissionless, play-money mostly | N/A (not predictions) | **Permissionless via Nostr event** |
| Oracle | UMA Optimistic | REP-staked reporters | Manifold + creator | N/A | **Multi-oracle marketplace** |
| Resolution time | Most ≤2h, disputes escalate to DVM | ~24-48h | Instant (creator can mark) | N/A | **2-24h depending on oracle (configurable per market)** |
| Custody | USDC in Polymarket CTF contract | ETH in Augur contracts | Internal play-money | None (P2P escrow) | **Cashu mint + DLC (no custody during market lifetime; mint can rug but DLC ensures fund safety at expiry)** |
| KYC | Required for some flows; full KYC for fiat | None | None (Stripe-billed mode) | None | **None** |
| US access | Geo-blocked (since 2022 CFTC) | Not blocked (decentralized) | Geo-restricted in some states | None (Tor) | **Geo-blocked (official frontend)** |
| Fiat | USDC, ACH, debit (US-side) | None | Mana (USD-pegged) | None | **None — Lightning only** |
| Token | None | REP | Mana (not crypto-backed) | None | **None** |
| Mobile | Web + iOS app | Web (Augur app no longer maintained) | Web + iOS app | Robosats: Tor browser; Mostro: Nostr clients | **PWA only (v1)** |
| Dispute mechanism | $750 USDC bond, 2h challenge, escalates to UMA DVM | REP-staked reporting + market invalidation | Creator/admin can void | Nostr-based community arbitration | **Optimistic Schnorr attestation + Nostr challenge + reputation slashing (social, not staked)** |
| Censorship resistance | Low (US-blocked, KYC, USDC freezable) | High (decentralized but unmaintained) | Low (Manifold can delete) | High (Tor + P2P) | **High by design** |
| Liquidity model | CLOB + AMM hybrid | Order book | Internal "house" | P2P maker/taker | **Cashu mint orderbook (Tier 1) + Nostr P2P (Tier 2)** |
| Settlement | On-chain Polygon | On-chain Ethereum | Internal DB | On-chain Bitcoin | **On-chain Bitcoin via DLC** |

## Key UX Patterns to Steal / Adapt

1. **Polymarket's single-buy-box on market page** — Best UX in the space. Adapt directly.
2. **Polymarket's "Resolved" badge with verification link** — Build trust by exposing settlement.
3. **Augur's "Mark as Invalid" flag (NO/YES/INVALID three-way)** — Crucial for permissionless markets. Adapt to "INVALID" as a third Cashu token type that pays out 50/50 if oracle attests invalid.
4. **Manifold's market creation friction (just a textbox + date)** — Lower friction than Polymarket's whitelist gauntlet. Aim for this UX.
5. **Robosats's Tor-only mode toggle** — Simple cypherpunk-friendly UX.
6. **Mostro's order browser UI on Nostr clients** — Patterns for browsing Nostr-event-based orders.
7. **Polymarket's category/topic chips** — Politics, Sports, Crypto, Culture as quick filters.
8. **Polymarket's price chart with bet placement overlay** — Tradingview-lite charts with click-to-bet.

## Anti-spam Strategy for Permissionless Markets

The Augur assassination markets in 2018 showed the failure mode. Our defenses:

1. **Social graph filter by default** — UI only shows markets created by people the user follows or is N hops from on Nostr (configurable: 1-hop, 2-hop, all)
2. **Community "INVALID" flag** — Any oracle can attest "invalid" → market resolves 50/50 (refund) automatically via DLC
3. **Reputation-aware sorting** — Markets sorted by creator's historical resolution accuracy, oracle's accuracy, dispute history
4. **Per-frontend moderation** — `hunch.io` can hide markets from its UI without removing them from the protocol; this delegates "what's tasteful" to the frontend operator
5. **Explicit "Hide" buttons** — User can mute creator, mute topic, or mute markets matching keywords
6. **No "trending" until N participants** — Markets need minimum participation to be discoverable beyond the creator's followers
7. **Slow-roll discovery** — New markets gain visibility gradually based on reactions, follows, time
8. **Operator ToS for hosted frontend** — `hunch.io` explicitly prohibits certain content; protocol stays neutral

## Sources

- [Polymarket Resolution Docs (UMA Oracle)](https://docs.polymarket.com/developers/resolution/UMA)
- [Polymarket Help: How Markets Are Disputed](https://help.polymarket.com/en/articles/13364551-how-are-markets-disputed)
- [Inside UMA Optimistic Oracle (Rocknblock)](https://rocknblock.io/blog/how-prediction-markets-resolution-works-uma-optimistic-oracle-polymarket)
- [Augur Assassination Markets (Newsweek 2018)](https://www.newsweek.com/welcome-augur-cryptocurrency-death-market-where-you-can-bet-donald-trump-1043571)
- [Augur Dark Side (Futurism)](https://futurism.com/augur-assassination-marketplace-decentralized-blockchain)
- Mostro and Robosats public docs (Tor + Nostr P2P UX patterns)
- Manifold Markets product (manifold.markets) — feature parity reference

---
*Feature research for: Bitcoin-native cypherpunk prediction market protocol*
*Researched: 2026-05-27*
