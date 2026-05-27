# Pitfalls Research

**Domain:** Bitcoin-native cypherpunk prediction market protocol (DLC + Cashu + Lightning + Nostr)
**Researched:** 2026-05-27
**Confidence:** HIGH (anchored on documented incidents: Polymarket CFTC 2022, Augur 2018, Tornado Cash dev 2023-2025, multiple Cashu/Nostr known issues).

## Critical Pitfalls

### Pitfall 1: CFTC Enforcement Action Like Polymarket (Blockratize, $1.4M, 2022)

**What goes wrong:**
The CFTC asserts event-based binary options contracts are "swaps" under their jurisdiction (CEA). Operating such a market without DCM/SEF registration → cease-and-desist + fines + criminal exposure for operators. Polymarket (Blockratize Inc.) was fined $1.4M and had to wind down all non-compliant markets in January 2022. Polymarket later restructured to obtain CFTC approval for US resumption.

**Why it happens:**
Founders underestimate the breadth of CFTC's "swap" definition. They think "we're a smart contract, we're decentralized" provides immunity. The CFTC explicitly disagreed and went after Blockratize Inc. directly.

**How to avoid:**
1. **No US-domiciled legal entity for Hunch operator.** Use Switzerland, Panama, BVI, or El Salvador-incorporated foundation.
2. **No US-resident operators of the official frontend/mint/oracle.** Founder, contractors, key contributors all non-US (and document this).
3. **Geo-block US on official frontend** (IP-based, Tor exit list, ToS).
4. **No US-targeted marketing.** No US ad spend, no US PR.
5. **Truly decentralized protocol** — anyone can host a frontend, run a mint, run an oracle. Hunch operator never has unique control over markets, funds, or settlement.
6. **Legal counsel BEFORE mainnet launch** — Crypto-specialized firm in chosen jurisdiction (Wachsman in CH, Tony Anderson in BVI, etc.).
7. **Document the absence of custody** rigorously. If operator never touches user funds (DLC + Cashu pattern), CFTC's leverage drops dramatically (Polymarket held USDC, Hunch never holds settled funds).

**Warning signs:**
- US users disclosing US location to support
- Press coverage mentioning specific high-profile US bettors (Polymarket got attention from US bettors discussing them in interviews)
- Subpoenas to frontend host (Cloudflare, registrar)

**Phase to address:**
Phase 0 (legal structure) + Phase 1 (geo-block infrastructure + ToS).

---

### Pitfall 2: Tornado-Cash-Style Developer Prosecution (Storm Conviction, August 2025)

**What goes wrong:**
Even if the protocol is "neutral", US federal prosecutors can convict the developer for "operating an unlicensed money-transmitting business" under 18 U.S.C. § 1960. Roman Storm was convicted in August 2025 (jury deadlocked on money laundering + sanctions, but convicted on § 1960). The knowledge element matters: prosecution argued Storm "continued to provide this service with knowledge that Tornado Cash was transmitting large volumes of criminal proceeds."

**Why it happens:**
US prosecutors creatively apply existing money-transmitter statutes to crypto developers. Open-source defense (Bernstein v. US) is being tested but not universally protective. Mens rea (knowledge of criminal use) is the prosecution's hook.

**How to avoid:**
1. **Maintainer anonymity / pseudonymity where possible.** Satoshi pattern. Not always practical for community building, but consider Nym-style identities.
2. **No US-resident maintainers** — same logic as #1. Storm was arrested in Washington State.
3. **Aggressive ToS that prohibits illegal use** — establishes that operator does NOT have knowledge of criminal proceeds use.
4. **No active screening of users** — paradoxically, doing nothing is better than half-doing KYC. Tornado Cash got hit harder because they did SOME screening.
5. **No direct integration with sanctioned mixers / tumblers / known criminal services.**
6. **Active anti-abuse mechanisms documented** (geo-block, ToS, takedown response) — shows good faith.
7. **Legal defense fund pre-funded** — set aside funds via foundation for potential legal expenses (SPI funded Tornado Cash dev defense with $500K).

**Warning signs:**
- High-profile criminal use case (ransomware, sanctioned entity)
- OFAC sanctioning the protocol itself (Tornado Cash sanctioned Aug 2022)
- Subpoena to operator entity

**Phase to address:**
Phase 0 (legal structure + counsel) + Phase 1 (anti-abuse + ToS).

---

### Pitfall 3: NUT-DLC Spec Instability During Build

**What goes wrong:**
NUT-DLC (PR #128 by conduition, depends on PR #127) is in active discussion, not merged into Cashu protocol. If we build against an unfinalized spec, we may need to rewrite when the spec changes. If we fork CDK and the upstream merges a different design, we're orphaned.

**Why it happens:**
We're at the frontier. NUT-DLC requires both spec maturity AND CDK implementation. As of 2026-05, neither is finalized.

**How to avoid:**
1. **Spike NUT-DLC FIRST** before any production code commits to it. 1-2 weeks of pure prototyping against PR #128.
2. **Contribute upstream actively.** Cashu PR review, Telegram discussion. We want the spec to land in a form we can use.
3. **Design our integration as a CDK extension trait/module**, not a fork. Easier to track upstream.
4. **Fallback plan**: if NUT-DLC isn't ready by launch, ship v1 with "trusted mint, off-chain promise" model (custodial-ish mint) and migrate to NUT-DLC in v1.x. Document this in our marketing materials honestly.
5. **Direct contact with Calle / conduition / Gandlaf** — Nostr DMs, Cashu Telegram. We need to be inside the conversation.

**Warning signs:**
- PR #128 closed without merge
- Cashu protocol team picks a different design
- CDK adds NUT-DLC but with breaking API

**Phase to address:**
Phase 0 (spike NUT-DLC, decide fork vs upstream).

---

### Pitfall 4: Mint Operator Rug During Market Lifetime

**What goes wrong:**
Hunch operates the reference Cashu mint at launch. Between bet placement and market settlement, the mint operator (us) could theoretically:
- Issue YES tokens without backing them with Lightning collateral
- Block users from selling positions
- Refuse to honor atomic swaps
- Disappear with the DLC funding before CET broadcast

This is the same trust issue any Cashu mint has, amplified by per-market collateral.

**Why it happens:**
Cashu's "Chaumian mint" trust model. Mint operator has full control during market lifetime. The DLC protects funds at SETTLEMENT, but not during issuance.

**How to avoid:**
1. **Mint state proofs** (NUT-22 style or custom) — publish periodic proofs that issued tokens are backed by reserves.
2. **Daily / weekly reserves audits** published on Nostr — operator publishes signed proofs of solvency, like Bitfinex used to.
3. **Multiple competing mints** — protocol-level support so users can choose mint per market.
4. **Mint reputation events on Nostr** (kind:30891) — community signals build trust over time.
5. **Path to federated mint** (Phase 2 — FROST multisig operators) so single op can't rug.
6. **Open-source mint code** — anyone can audit.
7. **Mint operator pseudonym + public reputation** — if Hunch the org rugs, they lose all credibility immediately.
8. **Cap initial market sizes during early operation** — even though we said "no caps" in PROJECT.md, prudent early caps reduce blast radius. Revisit after 3-6 months without incident.

**Warning signs:**
- Mint API errors increase suddenly
- Mint operator goes silent on Nostr
- Reserves proofs delayed or missing
- Unusual Lightning channel state

**Phase to address:**
Phase 1 (mint reserves proofs + reputation infrastructure) + Phase 2 (federation).

---

### Pitfall 5: Oracle Lies / Oracle Collusion

**What goes wrong:**
An oracle attests an incorrect outcome. Market resolves wrongly. Losing side has no recourse on a Bitcoin DLC (no smart contract to slash). FROST k-of-n helps but doesn't fully solve — if k operators collude, they can sign anything.

**Why it happens:**
Oracles are humans/machines. Single-oracle markets are trust-anchored on one entity. Even k-of-n FROST can be subverted if k members are compromised or bribed.

**How to avoid:**
1. **FROST k-of-n with high threshold for high-value markets** (e.g., 7-of-10) — collusion requires massive coordination.
2. **Oracle reputation system on Nostr** — past attestation accuracy publicly tracked. Bad attestation = lifetime brand damage.
3. **Dispute period** — 24-48h between attestation and DLC execution. Anyone publishes challenge events (kind:30890); if challenge wins community signal, market resolves to "invalid" (50/50 refund CET).
4. **"Invalid" outcome built into DLC** — most markets should have INVALID as a third possible outcome with 50/50 split, used when the question wasn't actually verifiable or oracle disputes are unresolvable.
5. **Diversity in oracle marketplace** — many competing oracles, market creators choose; bettors refuse markets with sketchy oracles.
6. **Public attestation event format** — oracle signs ALSO over event context (sources, evidence), not just the outcome. Easier to refute false attestations.

**Warning signs:**
- Oracle attests outcomes that contradict public news / on-chain data
- Multiple disputes from independent users
- Oracle suddenly stops communicating
- Pattern of attestations that benefit specific addresses

**Phase to address:**
Phase 1 (single-oracle + dispute period + INVALID outcome) + Phase 2 (FROST k-of-n + advanced reputation).

---

### Pitfall 6: Permissionless Market Abuse (Augur Assassination Market Replay)

**What goes wrong:**
Within 2 weeks of launch, malicious actors create markets like "Will [public figure] be killed in 2026?" — financial incentive for harm, massive PR disaster, regulatory attention, App Store-equivalent deplatforming threats. Augur experienced exactly this in July 2018 with Trump/Bezos/Betty White assassination markets.

**Why it happens:**
Permissionless market creation is a core differentiator. Anti-spam without losing this principle is hard. Augur Foundation said "we have no power to censor" — true at protocol layer, disaster for PR.

**How to avoid:**
1. **Frontend curation at hunch.io level** — explicit ToS prohibits violence/harm markets; frontend hides them via blocklist + ML filter + reports.
2. **Social graph default filter** — UI shows markets only from extended follow graph (1-3 hops). Bad-faith strangers don't appear unless friends-of-friends amplify them.
3. **Community INVALID flag** — built-in mechanism to flag markets as invalid; oracle attests INVALID → 50/50 refund automatically.
4. **Reputation-aware sorting** — by default, no anonymous newbie market reaches "trending" without N participants from established Nostr identities.
5. **No SEO of harmful markets** — robots.txt blocks indexing of markets matching certain patterns on official frontend.
6. **PR response playbook ready** — when (not if) it happens, public statement: "Hunch is a protocol; this is the cost of permissionlessness; our frontend has hidden this market; users can mute, fork, etc."
7. **Distance maintainer from frontend operator** — protocol maintainer is not the curator of any specific frontend.

**Warning signs:**
- Reports / mutes spiking on certain markets
- Press inquiries about specific markets
- Twitter / Nostr screenshots going viral

**Phase to address:**
Phase 1 (anti-spam infrastructure + INVALID outcome + PR playbook ready BEFORE launch).

---

### Pitfall 7: Mainnet Hardcore Launch Without Sufficient Audit

**What goes wrong:**
We launch mainnet without caps. A bug in the DLC contract, mint logic, or FROST signing leaks funds. Headlines: "Hunch loses 50 BTC of user funds in launch week." Project DOA.

**Why it happens:**
Pressure to ship, complexity of Bitcoin primitives, untested combinations (Cashu + DLC integration is novel).

**How to avoid:**
1. **External security audit MANDATORY before mainnet.** Firms with Bitcoin DLC expertise: Block Digital Contracting, Galaxy Audit, Mariko Wakabayashi / Trail of Bits Bitcoin team, Cashu auditors (Calle network). Cost: $50-150K. Worth it.
2. **Extensive Mutinynet testing** — 30-second blocks, real Lightning, real on-chain settlement at no real cost. 2-3 months minimum.
3. **Bug bounty program live BEFORE mainnet** — Vidar / HackenProof / Bugcrowd. Pay top dollar for critical findings.
4. **Tiered launch despite "no caps" goal**: Even though we said no artificial caps, prudent practice is:
   - Week 1-4: invite-only mainnet, max 100k sats per market
   - Month 2-3: open mainnet, max 1M sats per market
   - Month 4+: caps removed
   This is honest engineering, not contradicting cypherpunk goals.
5. **Multi-sig treasury / reserve** — operator's emergency reserve to make affected users whole in catastrophic bug (optional + ethically complex but considered).
6. **Operational monitoring** — anomaly detection on mint reserves, oracle attestations, DLC executions.
7. **Public incident response process documented** before mainnet.

**Warning signs:**
- Audit findings rated High or Critical not remediated
- Bug bounty submissions revealing exploits
- Anomalies in Mutinynet load testing

**Phase to address:**
Phase 1 (audit + Mutinynet) + Phase 2 (bug bounty + tiered launch).

---

### Pitfall 8: Schnorr Nonce Reuse / Cryptographic Implementation Bugs

**What goes wrong:**
Schnorr signatures require unique nonces per signature. Reused nonce = private key recovery. Bug in FROST signing or oracle signing logic could leak operator/oracle keys.

**Why it happens:**
Custom crypto is hard. FROST DKG and signing flows are complex. We're integrating multiple crypto libraries.

**How to avoid:**
1. **Use battle-tested libraries only** — `frost-secp256k1-tr` (ZF Foundation), `secp256k1` (Pieter Wuille). NEVER write our own nonce generation.
2. **Deterministic nonces via BIP-340 / RFC 6979** — most libraries do this by default; verify.
3. **Cryptographic review** — separate from general audit, get a cryptographer specifically on FROST integration (Tim Ruffing, Jonas Nick — Blockstream — both contributed to FROST research).
4. **No hand-rolled crypto** — every signature, every blinding, every DKG ceremony uses upstream library calls.
5. **Property-based fuzzing** — proptest / quickcheck for crypto-adjacent code.
6. **Reproducible builds** — same source = same binary; supply-chain integrity.

**Warning signs:**
- Linter warnings on `rand::random` for crypto-adjacent values
- Custom nonce generation code in PRs
- Compiler upgrades changing crypto behavior
- Dependency updates without re-audit

**Phase to address:**
All phases — continuous vigilance. Mandatory audit at Phase 1 milestone.

---

### Pitfall 9: Censorship via Frontend / Domain / Infrastructure Provider

**What goes wrong:**
Cloudflare drops hunch.io. Registrar pulls the domain. Vercel/Hetzner suspends the deployment. App Store removes any future native app. Hunch becomes inaccessible to mainstream users.

**Why it happens:**
Centralized providers respond to legal pressure, content complaints, or US Treasury sanctions. Polymarket-style products attract this attention.

**How to avoid:**
1. **Multi-host frontend** from day 1: Cloudflare Pages + Vercel + IPFS pin + Tor hidden service + Radicle. Single host loss = no impact.
2. **Diverse registrars** — use Njalla (Sweden, privacy-respecting), 1984 Hosting (Iceland), or similar; multiple TLD spread.
3. **DNS via Handshake / ENS** as backup naming layers — `.hunch.bit` or similar.
4. **Tor hidden service is canonical from launch** — power users use `.onion`; non-Tor frontends are conveniences.
5. **IPFS pinning across multiple providers** — Pinata, web3.storage, plus self-pin.
6. **Frontend code minimized and portable** — anyone can run it. Document deployment in 5 minutes.
7. **No backend dependencies that can be unilaterally disabled** — protocol relies on Nostr relays (many) + Bitcoin (uncensorable) + Cashu mints (multiple). Frontend connects directly.
8. **Crypto wallet integration** — WebLN, Cashu wallets, etc., all work without backend.

**Warning signs:**
- Cloudflare abuse complaint
- Domain registrar inquiry
- Legal letters to hosting providers

**Phase to address:**
Phase 1 (multi-host from day 1) + ongoing.

---

### Pitfall 10: Cold-Start Liquidity / Empty Markets

**What goes wrong:**
Hunch launches. Few markets, no liquidity. Bettors arrive, see nothing to bet on, leave. Markets sit empty. Network effect never bootstraps.

**Why it happens:**
Two-sided market problem. Without bettors, market creators don't bother. Without markets, bettors leave.

**How to avoid:**
1. **Operator-seeded markets at launch** — Hunch operator creates 50-100 well-curated markets on launch day, on topics with known interest (crypto-native predictions are easiest).
2. **Operator-provided liquidity (LP) on first markets** — operator takes both sides of initial markets to ensure there's always a tradable price.
3. **Bootstrap influencer partnerships** — known Bitcoin Nostr personalities create + promote a few markets each. (Marty Bent, Matt Odell, Stacy Herbert, Calle, Gandlaf etc.).
4. **Launch with sport / crypto / culture markets** that have natural community interest (next halving block height, Olympics-style events with neutral oracle availability).
5. **Quick resolution markets first** — markets that resolve in days, not months. Quick wins build trust.
6. **Promote market creation via Nostr zaps** — community members earn Cashu tokens for high-quality market questions.
7. **Cross-promote in Bitcoin Twitter/Nostr/podcasts** — distribute early.

**Warning signs:**
- Week 1: <100 unique users
- Week 4: <10 active markets with >5 participants each
- Markets sit at single-bettor for >24h

**Phase to address:**
Phase 1 launch checklist (operator-seeded markets ready) + Phase 1 marketing strategy.

---

### Pitfall 11: Solo Dev Burnout / Single Point of Failure

**What goes wrong:**
Solo dev maintains everything: protocol, mint, oracle, frontend, infra, community. Burnout. Single bug, single key compromise, single Twitter pile-on can end project momentum.

**Why it happens:**
Ambitious cypherpunk projects, solo founders, no early contributors. Lone-genius pattern.

**How to avoid:**
1. **Open-source from day 1.** Public GitHub + Radicle. Welcome contributors early.
2. **Document everything as you go.** HIPs, ARCHITECTURE.md, deployment guides. Future contributors should be able to ramp.
3. **Modular monorepo** — separate crates that contributors can adopt one at a time.
4. **Public Nostr presence + Discord/Matrix room** — community starts forming pre-launch.
5. **Pay first contributors via grants** (HRF, OpenSats, Spiral, Geyser) — secure 1-2 grants in Phase 1 to fund early contributors.
6. **Key custody plan** — operator keys in hardware (Coldcard, BitBox02), backup strategy with trusted custodian (your lawyer? a multisig with friends?).
7. **Operator continuity plan** — written instructions for what happens if dev disappears (handover key procedures, infra access, community ownership transition).
8. **Cap weekly work hours** — sustainable pace beats burnout sprint.

**Warning signs:**
- Code commits frequency drops
- Issue response time increases
- Public visibility lapses

**Phase to address:**
Phase 0 (community presence, contributor docs) + ongoing.

---

### Pitfall 12: Lightning Liquidity / Channel Management for Mint

**What goes wrong:**
Mint runs out of Lightning inbound or outbound liquidity. Deposits fail, withdrawals fail. Bad UX. Users leave.

**Why it happens:**
Operating Lightning at scale requires active channel management. Mint deposits = inbound; withdrawals = outbound. Imbalance is normal but must be managed.

**How to avoid:**
1. **LSP partnership** — Voltage, Olympus, Greenlight as inbound liquidity provider.
2. **Auto-balance with submarine swaps** — Boltz, Loop integration.
3. **Liquidity monitoring + alerts** — Prometheus metrics on channel balances; alert at thresholds.
4. **Initial liquidity bootstrap** — operator funds multiple channels with reputable nodes (ACINQ, Lightning Labs nodes).
5. **Channel partner diversity** — not all eggs in one basket.
6. **Trampoline payments** for receiving payments to unreachable channels.

**Warning signs:**
- Routing failures > 1% of attempts
- Channel balances 100% in one direction
- Lightning fees increase noticeably

**Phase to address:**
Phase 1 deployment + Phase 2 operations playbook.

---

### Pitfall 13: Relay Censorship + Nostr Spam

**What goes wrong:**
Major relays (Damus, primal, etc.) drop Hunch market events as "spam" or due to legal pressure. Users on those relays can't see markets. Conversely, malicious actors flood Hunch's relay with spam.

**Why it happens:**
Relay operators have their own ToS, content policies, performance limits.

**How to avoid:**
1. **Run own relay** — `relay.hunch.markets` — operator-controlled.
2. **Recommend multi-relay** — UI shows markets fetched from multiple relays; user choice.
3. **NIP-65 (Outbox model)** — author publishes to their outbox relays; consumers fetch from those. Reduces single-relay dependency.
4. **PoW (NIP-13) on market creation** — small CPU cost prevents spam without being barrier to legitimate use.
5. **Bloom-filter-based relay pre-filtering** for content moderation at relay level.
6. **Encourage community relays** with cypherpunk principles.
7. **Pricing relay if needed** — accept zaps to publish (Pleb-friendly cost).

**Warning signs:**
- Relay-side rejection rate increasing
- Discrepancy between markets visible across relays
- Single relay outages affecting visibility

**Phase to address:**
Phase 1 (own relay + multi-relay support).

---

### Pitfall 14: FROST DKG Ceremony Failure

**What goes wrong:**
Multi-oracle FROST setup requires all k-of-n co-oracles to participate in DKG (distributed key generation). One offline or buggy participant kills the ceremony. After setup, key changes (adding/removing oracle members) require full re-DKG.

**Why it happens:**
FROST is round-optimized but still multi-round. Real-world coordination is hard. Participants on different networks, time zones.

**How to avoid:**
1. **Async-friendly DKG implementation** — use a library that supports paused/resumed DKG.
2. **Coordination via Nostr DMs (NIP-44)** — participants communicate end-to-end encrypted.
3. **Robust signing (ROAST)** — extension to FROST that tolerates partial participation in signing rounds (research active in DDK ecosystem).
4. **Test ceremonies on signet first** — never first-time-on-mainnet.
5. **Document playbook with timing, fallbacks, dispute resolution.**
6. **Single-oracle markets are FINE as default** — multi-oracle is for high-value markets where the operational cost is justified.

**Warning signs:**
- Frequent DKG aborts in testing
- Participants offline at signing time
- Coordination delays in production signing

**Phase to address:**
Phase 2 (multi-oracle FROST as add-on capability).

---

### Pitfall 15: Tax Compliance Confusion for Users

**What goes wrong:**
Users in various jurisdictions don't understand their tax obligations on prediction market winnings. They use Hunch, win, get audited, get burned, blame Hunch.

**Why it happens:**
Prediction markets are taxed differently across jurisdictions (gambling vs commodity vs derivative vs not-taxable).

**How to avoid:**
1. **Explicit disclaimers** — "This is not financial advice. Consult your tax advisor."
2. **Tax-exempt by-design where possible** — but realistically not under our control.
3. **Provide tx history export** — users can self-report.
4. **NO tax reporting (1099-style) to any agency** — that would require KYC. Document this.
5. **Educational content (without legal advice)** — point to general tax guides for users in their region.

**Warning signs:**
- Users complaining about tax surprises
- Press scrutiny on tax angle

**Phase to address:**
Phase 1 (UX disclaimers + tx export).

---

## Technical Debt Patterns

Shortcuts that seem reasonable but create long-term problems.

| Shortcut | Immediate Benefit | Long-term Cost | When Acceptable |
|----------|-------------------|----------------|-----------------|
| Hardcode Hunch's mint as default | Easy onboarding | Reinforces centralization; harder to introduce multi-mint UI later | Never — design multi-mint UI from day 1 |
| Skip FROST k-of-n; use single oracle initially | Faster v1 | Single-oracle becomes the trust anchor; harder to evolve | Acceptable for v1 IF reputation + dispute infra are in place |
| Build matcher engine as core | Better orderbook UX | Centralization risk; censorship vector | Never — mint orderbook is fine for Tier 1 |
| Use Cloudflare-only frontend | Easiest deploy | Single host dependency | Acceptable only with IPFS + Tor mirrors live |
| Forgo audit for v1 | Save $50-150K, ship faster | Catastrophic launch bug | Never — audit is mandatory for mainnet |
| Skip Tor hidden service v1 | Less infra to manage | "Cypherpunk" claim weakened | Never — Tor is foundational for credibility |
| Postpone reputation events to v2 | Simpler v1 | Permissionless market spam unmitigated | Never — reputation is the social anti-spam mechanism |
| Use centralized DB as source of truth, Nostr as backup | Simpler dev | Forks impossible; lock-in | Never |
| Allow USD/USDC pricing display | Easier mental model for users | Brings Polymarket-style legal exposure | Acceptable as informational only, not as denomination |

## Integration Gotchas

| Integration | Common Mistake | Correct Approach |
|-------------|----------------|------------------|
| **rust-dlc / DDK** | Treating as stable; not pinning version | DDK is alpha (0.0.17); pin exact version; test on each upgrade |
| **CDK NUT-DLC extension** | Building against unmerged PR | Spike first, contribute upstream, plan fallback to pure CDK if NUT-DLC stalls |
| **LDK Node** | Embedding in WASM without testing browser memory limits | Test WASM build in real browsers; have native fallback |
| **Nostr relays** | Assuming relay always accepts events | Implement retry + multi-relay fanout; bake in NIP-65 outbox |
| **WebLN** | Assuming user has Alby or Mutiny extension | Detect provider; show install prompt; mobile fallback (BOLT-11 QR + manual paste) |
| **Cashu mint API** | Tight coupling to one mint's API | Use Cashu standard NUTs; user can switch mints |
| **frost-secp256k1-tr** | Mixing with non-Taproot FROST variant | Choose Taproot-compat from start (BIP-340 alignment) |
| **Bitcoin Core / electrs** | Hardcoding mempool.space as backend | Run own electrs; mempool.space as fallback |
| **NIP-07 / NIP-46 signers** | Only supporting NIP-07 (browser ext) | Support both — NIP-46 (remote signer) for mobile + hardware signers |
| **Cashu blinded signatures** | Implementing without DLEQ proof verification (NUT-12) | Always verify DLEQ to prevent mint exploit |

## Performance Traps

| Trap | Symptoms | Prevention | When It Breaks |
|------|----------|------------|----------------|
| Loading all markets from Nostr at startup | Frontend cold start 10s+ | Paginated relay queries, local SQLite index | >500 active markets |
| Re-fetching reputation events every render | Browser CPU pegged | Cache reputation events with TTL | >50 oracles tracked |
| Naive WebSocket connection per relay | Browser hits connection limits | NDK connection pool, multiplexed | >5 active relays |
| Mint single-threaded signing | Latency under load | Tokio task spawning, multi-core utilization | >100 concurrent bets |
| DLC contract per market, no UTXO aggregation | Bitcoin tx fees explode at fee spikes | Batch funding txs across markets when possible | Sat/vB > 50 sustained |
| Loading entire DLC contract state on each page view | Frontend memory bloat | Lazy load, query specific outcomes | DLC contracts > 100 |
| Loading all Nostr events into memory at frontend | Memory exhaustion | Stream + virtual scroll | >10k events |

## Security Mistakes

Domain-specific security issues beyond general web security.

| Mistake | Risk | Prevention |
|---------|------|------------|
| Mint operator's Lightning wallet keys on hot machine | Funds drain if server compromised | HSM, hardware-signed transactions, multisig hot/cold split |
| Oracle private key in env var | Key compromise leaks via memory dump | Hardware oracle signer (e.g., Coldcard with PSBT for attestations) |
| FROST share storage in plaintext on disk | Multi-oracle DKG compromise | Encrypted at rest, key-derivation from passphrase |
| Cashu mint private key online accessible | All issuance compromised | Hardware HSM or air-gapped signer |
| Frontend trusting any Nostr event with valid sig | Sybil markets, fake events | Sig verification + reputation/social graph filtering |
| No verification of Cashu DLEQ proofs | Mint can forge signatures | NUT-12 DLEQ mandatory on all token receives |
| User Lightning channels with operator | Custodial relationship | LDK Node on user side; operator only as routing peer optionally |
| Predictable Cashu token IDs / blinding factors | Token theft | Use NUT-spec randomness exclusively |
| Mint operator can see who-paid-what | Privacy leak | Cashu blind sigs mitigate; ensure no logging that defeats this |
| Replay of Nostr events | Duplicate bets, double resolution | Event tag checks, NIP-09 deletion respect, timestamp validation |
| TX malleation in DLC funding | DLC state desync | Use SegWit Taproot only; PSBT discipline |

## UX Pitfalls

| Pitfall | User Impact | Better Approach |
|---------|-------------|-----------------|
| Showing raw sats only | Mental model gap (1000 sats = $???) | Toggleable sat/BTC/USD display; BTC default |
| No "loading" states during Nostr fetch | Users think app is broken | Skeleton screens, optimistic UI |
| Unintelligible error messages from mint | Users abandon | Translate Cashu errors to human language |
| BOLT-11 invoice copy-paste burden | Friction on mobile | WebLN auto-pay, QR for cross-device |
| Hiding oracle reputation behind clicks | Users don't verify trust | Display oracle reputation badge prominently |
| No explanation of "INVALID" outcome | Users don't grok dispute mechanism | Inline education on resolution UI |
| Single resolution check 24h after expiry | Users obsess, refresh constantly | Web push (opt-in) or Nostr DM when resolved |
| Mixing positions across markets without portfolio view | Users lose track | Portfolio page with P&L per market |
| Permissionless market creation friction (too many fields) | Few markets created | Minimum viable form: question + outcome + oracle + expiry |
| Hiding the protocol nature | Users think "Hunch decides" | Onboarding explains: "Hunch operates one frontend; the protocol is open" |

## "Looks Done But Isn't" Checklist

Things that appear complete but are missing critical pieces.

- [ ] **Mainnet launch:** Often missing security audit — verify audit report signed off
- [ ] **Mainnet launch:** Often missing bug bounty live — verify HackerOne/Hexens running
- [ ] **DLC settlement:** Often missing refund timeout testing — verify oracle-disappearance flow on signet
- [ ] **Mint API:** Often missing DLEQ proof verification — audit token receive paths
- [ ] **FROST DKG:** Often missing key rotation / member change docs — write ceremony playbook
- [ ] **Frontend:** Often missing Tor hidden service test — verify all flows work in Tor Browser
- [ ] **Frontend:** Often missing IPFS deployment — verify static export works on web3.storage
- [ ] **Resolution UI:** Often missing on-chain settlement link — verify Bitcoin tx + oracle sig visible
- [ ] **Oracle:** Often missing public attestation history — verify Nostr query returns past attestations
- [ ] **Anti-spam:** Often missing social graph default — verify new users see filtered feed
- [ ] **INVALID outcome:** Often missing in CET construction — verify DLC supports 3-outcome (YES/NO/INVALID)
- [ ] **Geo-block:** Often missing Tor exit list — verify Tor users from US-listed exits are blocked
- [ ] **ToS:** Often missing US restriction language — verify legal counsel signoff
- [ ] **Documentation:** Often missing operator deployment guide — verify someone external can deploy in <1h
- [ ] **Backup / recovery:** Often missing operator key custody plan — verify offline backup tested
- [ ] **Monitoring:** Often missing anomaly alerts — verify Prometheus + Grafana on mint+oracle
- [ ] **Incident response:** Often missing public-facing process — verify status page exists
- [ ] **Multi-language:** Often only EN — verify FR working at minimum

## Recovery Strategies

When pitfalls occur despite prevention, how to recover.

| Pitfall | Recovery Cost | Recovery Steps |
|---------|---------------|----------------|
| CFTC enforcement action | HIGH | 1. Engage US criminal defense counsel + crypto regulatory specialist immediately. 2. Cease US-targeted activity. 3. Cooperate vs litigate decision with counsel. 4. Public statement only after counsel signoff. |
| Tornado-Cash-style charges | HIGH | 1. Defense fund activated (SPI, EFF outreach). 2. Public defense campaign. 3. Open-source defense argument prepared. |
| NUT-DLC spec abandoned | MEDIUM | 1. Fork CDK with our extensions. 2. Document fork rationale. 3. Maintain compatibility layer for future re-merge. |
| Mint operator rug suspected | HIGH | 1. Public Nostr post + community signal. 2. Trigger DLC refund timeouts. 3. Switch to backup mint via UI. 4. Investigate / disclose. |
| Oracle lies on attestation | MEDIUM | 1. Community publishes dispute events. 2. Reputation hit on oracle. 3. Markets resolve invalid via INVALID CET if community signal strong. 4. Future markets exclude this oracle. |
| Augur-style abuse market | MEDIUM | 1. Frontend hides market immediately (curation policy). 2. Public statement with Polymarket-style precedent context. 3. Reputation hit on creator. |
| Mainnet launch bug found | HIGH | 1. Emergency pause via mint operator (refuse new bets). 2. Audit firm engaged for diagnosis. 3. Patch + redeploy + public post-mortem. 4. Affected user reimbursement plan if applicable. |
| Schnorr nonce reuse / key leak | CATASTROPHIC | 1. Immediate operator key rotation. 2. Funds in old keys swept (if possible). 3. Audit cause + public post-mortem. 4. If key affected user funds, reimbursement. |
| Frontend deplatformed | LOW | 1. Switch DNS to backup host. 2. Public announcement. 3. Invest in additional mirrors. |
| Solo dev burnout | MEDIUM | 1. Public announcement of pause. 2. Operator hand-off to community per pre-written plan. 3. Multi-contributor model activated. |
| Lightning liquidity dry | LOW-MEDIUM | 1. Engage LSP for emergency channels. 2. Submarine swap for rebalance. 3. Adjust mint fee schedule temporarily. |

## Pitfall-to-Phase Mapping

How roadmap phases should address these pitfalls.

| Pitfall | Prevention Phase | Verification |
|---------|------------------|--------------|
| CFTC enforcement (Polymarket) | Phase 0 (legal) | Counsel sign-off on legal structure documented |
| Tornado-Cash-style prosecution | Phase 0 (legal) + Phase 1 (anti-abuse ToS) | Counsel sign-off; ToS published |
| NUT-DLC spec instability | Phase 0 (spike) | Working NUT-DLC prototype on signet |
| Mint operator rug | Phase 1 (reserves proofs) + Phase 2 (federation) | Public reserves proofs published weekly |
| Oracle lies | Phase 1 (single-oracle + dispute + INVALID) + Phase 2 (FROST) | Dispute mechanism tested on signet markets |
| Augur-style abuse markets | Phase 1 (anti-spam + curation) | Social graph filter UI live before mainnet |
| Insufficient audit | Phase 1 (audit) | Audit report public, all High/Critical findings resolved |
| Cryptographic implementation bugs | All phases (continuous) | Cryptographic library updates tracked, no custom crypto |
| Censorship via providers | Phase 1 (multi-host) | Tor + IPFS + 2+ CDNs verified working |
| Cold-start liquidity | Phase 1 launch checklist | 50+ markets seeded, operator-provided liquidity for first weeks |
| Solo dev burnout | Phase 0 (community) + ongoing | First external contributor merged within 30 days of public repo |
| Lightning liquidity issues | Phase 1 deployment | LSP partnership confirmed before mainnet |
| Relay censorship / spam | Phase 1 (own relay) | Multi-relay fanout tested; PoW filter active |
| FROST DKG failures | Phase 2 (multi-oracle) | DKG ceremony tested on signet 3+ times |
| Tax compliance UX | Phase 1 (disclaimers) | ToS includes tax disclaimer; tx export available |

## Sources

- [CFTC v. Blockratize / Polymarket settlement (Jan 2022)](https://www.cftc.gov/PressRoom/PressReleases/8478-22)
- [Polymarket CFTC approval to resume US operations (2026)](https://www.thebulldog.law/polymarket-receives-cftc-approval-to-resume-us-operations-after-years-offshore)
- [DOJ press release on Roman Storm Tornado Cash conviction](https://www.justice.gov/usao-sdny/pr/founder-tornado-cash-crypto-mixing-service-convicted-knowingly-transmitting-criminal)
- [Roman Storm verdict analysis (Hodder Law)](https://hodder.law/roman-storm-tornado-cash-verdict-crypto-developers/)
- [Tornado Cash dev liability (Mayer Brown)](https://www.mayerbrown.com/en/insights/publications/2025/08/the-tornado-cash-trials-mixed-verdict-implications-for-developer-liability)
- [Augur assassination markets controversy (Newsweek)](https://www.newsweek.com/welcome-augur-cryptocurrency-death-market-where-you-can-bet-donald-trump-1043571)
- [Augur dark side analysis (Futurism)](https://futurism.com/augur-assassination-marketplace-decentralized-blockchain)
- [Cashu NUT-DLC PR #128](https://github.com/cashubtc/nuts/pull/128)
- [DDK alpha disclaimer](https://github.com/bennyhodl/dlcdevkit)
- [SPI funding Tornado Cash defense](https://www.btcc.com/en-US/square/Cryptonews/880619)

---
*Pitfalls research for: Bitcoin-native cypherpunk prediction market protocol*
*Researched: 2026-05-27*
