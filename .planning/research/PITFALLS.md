# Pitfalls Research (Enriched)

**Domain:** Bitcoin-native cypherpunk prediction market protocol (DLC + Cashu + Lightning + Nostr)
**Researched:** 2026-05-27
**Confidence:** HIGH (anchored on primary sources: CFTC v. Blockratize order [Jan 3, 2022], DOJ Storm verdict [Aug 6, 2025], Trail of Bits FROST DKG disclosure [Feb 2024], Polymarket UMA governance attack [Mar 2025], conduition Cashu disclosures [Jul + Nov 2025], MiCA Art. 143 transitional sunset [1 Jul 2026]).

This document was rewritten on 2026-05-27 to verify every legal claim against primary sources, deepen mitigations, and add pitfalls the first draft missed.

---

## Critical Pitfalls (Existential — can end the project)

### Pitfall 1: CFTC Enforcement Action Like Polymarket (Blockratize, $1.4M, Jan 3, 2022)

**What goes wrong:**
The CFTC asserts that event-based binary options ("Will X happen by date Y?") are off-exchange commodity options + facilities trading swaps. Operating without registration as a Designated Contract Market (DCM) or Swap Execution Facility (SEF) → civil monetary penalty + cease-and-desist + criminal referral.

**Exact legal theory (verified against the CFTC order, [CFTC Docket No. 22-09, filed Jan 3, 2022](https://www.cftc.gov/media/6891/enfblockratizeorder010322/download)):**

Blockratize Inc. (d/b/a Polymarket.com) was charged with violating **four distinct provisions**:

1. **Section 4c(b) of the Commodity Exchange Act** (7 U.S.C. § 6c(b)) — prohibits offering, trading, or confirming commodity options transactions unless compliant with the CEA.
2. **CFTC Regulation 32.2** (17 C.F.R. § 32.2) — implements § 4c(b); requires options on commodities be traded on a DCM unless exempt.
3. **Section 5h(a)(1) of the CEA** (7 U.S.C. § 7b-3(a)(1)) — prohibits operating a facility that brings together multiple participants to trade swaps unless registered as a Swap Execution Facility.
4. **CFTC Regulation 37.3(a)(1)** (17 C.F.R. § 37.3(a)(1)) — SEF registration requirement.

Penalty: **$1,400,000 civil monetary penalty** + cease-and-desist + mandate to wind down all non-compliant markets within a specified period. The order also disclaimed that this resolved only civil exposure; criminal referral was preserved.

**What followed (verified):**

- **November 13, 2024:** FBI raided Polymarket CEO Shayne Coplan's NYC apartment, seizing his phone and electronics, investigating whether Polymarket had violated the 2022 settlement by continuing to allow US user access. ([NBC News](https://www.nbcnews.com/tech/tech-news/fbi-raids-polymarket-ceo-shayne-coplans-apartment-seizes-phone-source-rcna180180))
- **September 2025:** Polymarket acquired **QCX LLC** (a CFTC-licensed Designated Contract Market) and **QC Clearing LLC** for ~$112M, then obtained CFTC no-action relief to use the licensed venue for US activity. ([Blockhead, Sep 4, 2025](https://www.blockhead.co/2025/09/04/cftc-clears-polymarkets-return-to-us-after-three-year-ban/))
- **November 25, 2025:** CFTC issued an **Amended Order of Designation** allowing Polymarket to operate an intermediated trading platform — i.e., users trade through FCMs on a regulated exchange. ([Regulatory Oversight](https://www.regulatoryoversight.com/2025/12/cftc-approval-allows-polymarket-to-reenter-the-u-s-market/))

**Translation for Hunch:** The Polymarket "victory" required (a) acquiring a CFTC-licensed exchange for $112M, (b) operating as an intermediated venue (i.e., abandoning permissionless/non-custodial principles), and (c) accepting full DCM compliance. **This path is incompatible with Hunch's cypherpunk thesis.** Hunch's only viable strategy is to be **so structurally non-US, non-custodial, and protocol-neutral that no operator entity exists for the CFTC to charge.**

**Why founders get caught:**
They underestimate the breadth of § 4c(b) / § 5h(a)(1). They believe "we're a smart contract, we're decentralized" is a defense. The CFTC's response in the Blockratize order explicitly disagreed and went after the corporate entity directly — not the smart contract. Decentralization theater is not a defense.

**How to avoid:**
1. **No US-domiciled legal entity for any Hunch operator role.** Foundation in Switzerland (Stiftung), BVI, Panama, or Liechtenstein. Avoid El Salvador now that the Bitcoin Law has been rolled back under IMF pressure (see Pitfall 17).
2. **No US-resident operators of the official frontend, reference mint, or reference oracle.** Founder, contractors, infrastructure operators all non-US, documented with residency records.
3. **Geo-block US on the official frontend** (IP-based + Tor exit list + ToS attestation at login).
4. **No US-targeted marketing.** No US ad spend, no US conference sponsorship, no US press tour.
5. **Document protocol neutrality rigorously.** HIPs are public specs; reference implementations are one of N possible implementations; forks are explicitly encouraged; no central market registry; no fee captured by Hunch as a per-trade tax.
6. **Document the absence of custody at settlement.** Polymarket held USDC for users; Hunch's DLC + Cashu pattern means the mint operator never has unilateral control over settled funds (the DLC CET path is enforced by the Bitcoin blockchain, signed by the oracle). Make this point explicitly in the legal opinion attached to the project.
7. **Crypto-specialized legal counsel BEFORE mainnet.** MME (Zurich), Bär & Karrer (Zurich), Walkers (BVI), Anderson Legal (Cayman). Cost: $30-80K for the structuring opinion. Worth it.

**Warning signs:**
- US users disclosing US location in support tickets, Discord, or Nostr DMs
- Press coverage mentioning specific high-profile US bettors by name
- Subpoenas to frontend host (Cloudflare, Hetzner, registrar)
- DOJ/CFTC inquiry letters to the foundation
- An FBI raid on a maintainer or contributor (Polymarket precedent: this happened to Coplan **without prior notice or charge**)

**Phase to address:**
Phase 1 (legal structure + counsel) + Phase 2 (geo-block + ToS + protocol neutrality docs).

**Sources:**
- [CFTC v. Blockratize Order, CFTC Docket No. 22-09 (Jan 3, 2022) — PDF](https://www.cftc.gov/media/6891/enfblockratizeorder010322/download)
- [CFTC Press Release 8478-22](https://www.cftc.gov/PressRoom/PressReleases/8478-22)
- [Polymarket Amended Order of Designation (Nov 25, 2025)](https://www.regulatoryoversight.com/2025/12/cftc-approval-allows-polymarket-to-reenter-the-u-s-market/)
- [FBI raid on Coplan (NBC News, Nov 14, 2024)](https://www.nbcnews.com/tech/tech-news/fbi-raids-polymarket-ceo-shayne-coplans-apartment-seizes-phone-source-rcna180180)

---

### Pitfall 2: Tornado-Cash-Style Developer Prosecution (Roman Storm Conviction, Aug 6, 2025)

**What goes wrong:**
Even if the protocol is "neutral" and the developer never custodies funds, US federal prosecutors can charge under **18 U.S.C. § 1960** ("Prohibition of unlicensed money transmitting businesses"). Storm was convicted of **conspiracy to operate an unlicensed money transmitting business** for his role in building and maintaining Tornado Cash.

**§ 1960 elements (verified, [Cornell LII text](https://www.law.cornell.edu/uscode/text/18/1960)):**

> "Whoever knowingly conducts, controls, manages, supervises, directs, or owns all or part of an unlicensed money transmitting business, shall be fined in accordance with this title or imprisoned not more than five years, or both."

An "unlicensed money transmitting business" under § 1960(b)(1) is one that affects interstate or foreign commerce **AND** falls into any of three buckets:
- (A) Operated without a required state license, **whether or not the defendant knew the operation was required to be licensed** (post-Patriot Act amendment — knowledge of the licensing requirement is not an element);
- (B) Fails to comply with federal registration requirements under 31 U.S.C. § 5330; or
- (C) Involves the transportation or transmission of funds **known to the defendant** to have been derived from a criminal offense or intended to support unlawful activity.

**Knowledge standard applied in U.S. v. Storm:**
The prosecution argued under § 1960(b)(1)(C). The DOJ press release framed the conviction as Storm "knowingly transmitting criminal proceeds" — specifically, that Storm continued to operate Tornado Cash "with knowledge that he was transmitting criminal proceeds, including his knowing transmission of hundreds of millions of dollars in criminal proceeds from the Ronin hack, which the FBI publicly attributed to the sanctioned North Korean cybercriminal organization, the Lazarus Group." ([DOJ SDNY press release](https://www.justice.gov/usao-sdny/pr/founder-tornado-cash-crypto-mixing-service-convicted-knowingly-transmitting-criminal))

**Storm's sentence (verified):**

- **Conviction (Aug 6, 2025):** One count of conspiracy to operate an unlicensed money transmitting business (§ 1960). **Maximum statutory exposure: 5 years.** Actual sentence: **not yet pronounced — Storm remains free on bail pending sentencing as of 2026-05.**
- **Hung jury:** The jury **deadlocked** on the two more serious counts: conspiracy to commit money laundering and conspiracy to violate IEEPA sanctions, each carrying up to 20 years.
- **Post-trial:** Defense filed motion for acquittal in Oct 2025. DOJ filed 113-page opposition Nov 2025. Hearing scheduled **April 9, 2026.** DOJ filed letter **March 9, 2026** requesting **retrial on the hung counts** (proposed start: Oct 5 or 12, 2026, ~3 weeks). ([DeFi Education Fund timeline](https://www.defieducationfund.org/us-v-storm-background-timeline/))
- **Appellate posture:** No appeal yet (sentencing has not occurred). Once sentenced, expect direct appeal to the **Second Circuit**, challenging (a) the application of § 1960 to non-custodial protocols, (b) jury instructions on the knowledge element, and (c) First Amendment / Bernstein-style code-as-speech defenses.

**Why this is existentially scary for Hunch:**

Storm did not custody user funds. The Tornado Cash smart contracts were immutable. Storm's role was protocol developer + UI maintainer + relayer operator. The conviction nonetheless attached under § 1960(b)(1)(C) because of **knowledge** that the protocol carried criminal proceeds. Hunch faces an analogous risk: if a publicly known bad actor (sanctioned entity, ransomware operator) uses the Hunch protocol to bet, and Hunch maintainers are aware (via Nostr, press, OFAC list cross-reference), § 1960(b)(1)(C) could be invoked even against fully non-custodial protocol operators.

**How to avoid:**
1. **Maintainer non-US residency, hard.** Storm's prosecution required US jurisdiction (he was a Washington State resident). All key Hunch maintainers should be non-US residents with documented residency, and they should **not travel to the US** while any plausible enforcement risk is active.
2. **Maintainer pseudonymity where workable.** Satoshi pattern. Use Nym-style identities for the most sensitive protocol roles. Frontend operator can be public; protocol core developers can be pseudonymous.
3. **Aggressive ToS prohibiting illegal/sanctioned use.** Establishes constructive notice that the operator does NOT consent to criminal use. Tornado Cash never had a comparable ToS.
4. **No active screening of users (no half-KYC).** Paradoxically, half-doing screening (Tornado Cash's optional compliance tooling) was cited by prosecutors as evidence Storm knew there was a sanctions problem. Either screen everyone (incompatible with cypherpunk) or screen no one (Hunch's chosen path). Document this as a deliberate technical-policy choice.
5. **No direct integration with known sanctioned mixers, ransomware payment rails, or sanctioned exchanges.** If a Cashu mint that integrates with us starts laundering Lazarus Group funds publicly, **delist that mint immediately and document the delisting** — this is the affirmative anti-abuse evidence that distinguishes Hunch from Tornado Cash.
6. **OFAC-list cross-check tool at frontend (informational only).** Frontend can warn (not block) when a counterparty's Bitcoin address shows OFAC overlap — this is the "we tried in good faith" evidence prosecutors look for, **without** triggering the half-screening trap.
7. **Legal defense fund pre-funded.** SPI (Software in the Public Interest) funded ~$500K of Storm's defense. Hunch foundation should set aside a comparable reserve (50 BTC?) earmarked for criminal defense of any indicted maintainer.
8. **EFF / Coin Center / DeFi Education Fund relationship from day 1.** These orgs filed amicus briefs in U.S. v. Storm. Building rapport before any indictment is critical.

**Warning signs:**
- High-profile criminal use case linked to Hunch market (ransomware payout, sanctioned entity bet)
- OFAC sanctioning a Cashu mint that Hunch integrates with (or sanctioning the protocol itself, à la Tornado Cash, Aug 8 2022 — later overturned, but the chilling effect lasted years)
- Subpoena to foundation entity
- FBI visit to any maintainer's home or workplace
- Cloudflare / Hetzner / GitHub serving a US grand jury subpoena targeting Hunch

**Phase to address:**
Phase 1 (legal structure + non-US residency + counsel) + Phase 2 (ToS + anti-abuse evidence + OFAC informational tooling).

**Sources:**
- [18 U.S.C. § 1960 — Cornell LII](https://www.law.cornell.edu/uscode/text/18/1960)
- [DOJ SDNY press release on Storm conviction (Aug 6, 2025)](https://www.justice.gov/usao-sdny/pr/founder-tornado-cash-crypto-mixing-service-convicted-knowingly-transmitting-criminal)
- [US v. Storm background & timeline — DeFi Education Fund](https://www.defieducationfund.org/us-v-storm-background-timeline/)
- [Tornado Cash trial mixed verdict analysis (Mayer Brown, Aug 2025)](https://www.mayerbrown.com/en/insights/publications/2025/08/the-tornado-cash-trials-mixed-verdict-implications-for-developer-liability)
- [Money Laundering Watch analysis (Aug 2025)](https://www.moneylaunderingnews.com/2025/08/tornado-cash-jury-deadlocked-on-most-serious-charges-but-convicted-founder-roman-storm-on-conspiracy-to-operate-an-unlicensed-money-transmitting-business/)

---

### Pitfall 3: EU MiCA + Member-State Gambling Law Exposure (NEW)

**What goes wrong:**
Hunch is non-US, but EU residents will access the protocol. The EU's regulatory map has **two parallel exposures**:

1. **MiCA (Markets in Crypto-Assets, Regulation (EU) 2023/1114).** MiCA's transitional period ends **1 July 2026**. After that date, any entity providing crypto-asset services to EU clients without a MiCA licence is in breach of EU law. ESMA has explicitly stated that MiCA's market abuse regimes apply to **any prediction market using crypto assets** ([ESMA / Norton Rose Fulbright analysis](https://www.nortonrosefulbright.com/en/knowledge/publications/290d594a/the-eus-approach-to-prediction-markets-and-event-contracts)).
2. **National gambling regulators** in each EU member state (France ANJ, Germany GGL, Spain DGOJ, Italy ADM, etc.) each maintain their own licensure regime for "betting" and "gambling." Prediction markets are frequently classified as gambling rather than financial instruments, with the regulator deciding case by case. Spain's regulator has explicitly raised this issue with Polymarket in late 2025. There is no EU-level harmonization.

**Operator exposure for non-EU operators serving EU users:**

MiCA Art. 59 / 60 establish that **any entity offering services to EU clients** (even from outside the EU) is in scope — there is no "we're offshore so we're fine" carve-out. ESMA Guidelines published 17 March 2025 ([ESMA Guidelines 75-453128700-1323](https://www.esma.europa.eu/sites/default/files/2025-03/ESMA75453128700-1323_Guidelines_on_the_conditions_and_criteria_for_the_qualification_of_CAs_as_FIs.pdf)) extend the analysis to whether a crypto-asset is also a financial instrument under MiFID II — in which case BOTH regimes apply.

For prediction market tokens (Cashu YES/NO tokens backed by DLC), the likely classification is:
- **MiCA "other crypto-assets" (Title II)** for the underlying token issuance, OR
- **MiFID II derivative** if the YES/NO claim is viewed as an event-contingent contract (most likely classification).

**Why founders get caught:**

Founders incorrectly assume "we're not in the EU" is a defense. MiCA explicitly captures **active solicitation** of EU customers — and a public website indexed for European search engines, with EU-language marketing, qualifies. Even passive availability can trigger national gambling law in jurisdictions like France (where ANJ has aggressively pursued offshore gambling operators).

**How to avoid:**
1. **Geo-block EU member states with strict gambling regimes on the official frontend.** Minimum block list: **France, Germany, Italy, Spain, Netherlands, Belgium, Portugal**. Use IP geolocation + accepted-language hints.
2. **No EU-language marketing.** Especially no French / German / Italian / Spanish landing pages on the official site. If localized, deploy on a fork operated by a non-Hunch entity.
3. **No MiCA-defined "active solicitation" toward EU users.** No EU conference talks where Hunch is presented as a product (presenting as a *protocol spec* is defensible; presenting as a tradable platform is not).
4. **Switzerland is the recommended HQ jurisdiction** (NOT an EU member, has a clear crypto regime). FINMA's classification is "technology-neutral" — crypto derivatives are treated like derivatives under FMIA + FinSA ([Chambers Switzerland Fintech 2025](https://practiceguides.chambers.com/practice-guides/fintech-2025/switzerland/trends-and-developments)). Switzerland has not issued explicit prediction-market guidance, but the Swiss "same risks, same rules" doctrine likely places prediction markets under derivative-trading rules. **An FMIA exemption or sandbox licence (Geneva or Zug) is the recommended path for a Swiss-domiciled operator** — but this contradicts the cypherpunk "no licence" thesis. **Resolution:** Swiss foundation owns the protocol IP and reference code; no Swiss entity operates the reference mint or oracle (those are operated by individual contributors under personal liability in jurisdictions of their choice, e.g., Panama or Vanuatu).
5. **ToS clause excluding EU users from the official frontend.** Mirror the US exclusion logic.
6. **Tor / IPFS / Radicle distribution provides defensible "we did not actively market to EU users" evidence.** Users who reach Hunch via Tor are actively seeking out a censorship-resistant protocol; this is materially different from an EU resident clicking a Google ad.

**Warning signs:**
- ESMA referencing Hunch in a public statement on prediction markets
- National gambling regulator (ANJ, GGL, DGOJ, ADM) opening an inquiry
- EU-based press calling Hunch a "betting platform" (the wrong frame — we want "open protocol" framing exclusively)
- An EU member-state court issuing an injunction against the domain

**Phase to address:**
Phase 1 (legal structure includes EU exposure analysis) + Phase 2 (geo-block expanded to EU strict-regime states; ToS updated).

**Sources:**
- [Norton Rose Fulbright — EU approach to prediction markets and event contracts (2025-2026)](https://www.nortonrosefulbright.com/en/knowledge/publications/290d594a/the-eus-approach-to-prediction-markets-and-event-contracts)
- [ESMA on MiCA transitional periods end (Apr 2026)](https://www.regulationtomorrow.com/2026/04/esma-statement-on-the-end-of-transitional-periods-under-mica/)
- [ESMA Guidelines 75-453128700-1323 (Mar 2025) — financial instrument qualification](https://www.esma.europa.eu/sites/default/files/2025-03/ESMA75453128700-1323_Guidelines_on_the_conditions_and_criteria_for_the_qualification_of_CAs_as_FIs.pdf)
- [Spain prediction markets regulatory vacuum (Ainvest, May 2026)](https://www.ainvest.com/news/spain-prediction-markets-regulatory-vacuum-agree-fill-2605/)
- [Oxford Business Law Blog — Regulating Prediction Markets in Europe (Mar 2026)](https://blogs.law.ox.ac.uk/oblb/blog-post/2026/03/regulating-prediction-markets-europe-requires-prediction-test)
- [Chambers Fintech Switzerland 2025](https://practiceguides.chambers.com/practice-guides/fintech-2025/switzerland/trends-and-developments)

---

### Pitfall 4: Operator-as-Money-Transmitter Exposure (Even for Cashu Mints) (NEW)

**What goes wrong:**
The Hunch reference Cashu mint accepts Lightning sats (deposits) and pays out Lightning sats (withdrawals). At the FinCEN level, the mint operator could be argued to be a **money transmitter** under [FinCEN's 2013 virtual currency guidance (FIN-2013-G001)](https://www.fincen.gov/resources/statutes-regulations/guidance/application-fincens-regulations-persons-administering) — which states that "a person that creates units of convertible virtual currency and sells those units to another person for real currency or its equivalent is engaged as a business in exchange of currency" and is therefore an MSB.

Even though Cashu is "cryptographically non-custodial" (the user holds the blinded tokens; the mint cannot identify or seize them directly), the Lightning deposit/withdraw flow involves the mint receiving and sending sats. Under FinCEN's analytical framework, **this is money transmission.**

Moreover, the bitcoin-backed reserve held by the mint is **fully custodial in economic terms**, even if cryptographically blinded — ([iscashucustodial.com](https://iscashucustodial.com/) explicitly makes this point). A US-jurisdiction prosecutor reading the FinCEN guidance + the Storm precedent would have a credible § 1960 theory against any US-touching Cashu mint operator.

**Why this is not the same as Pitfall 2:**

Pitfall 2 was about *protocol developers* (Storm wrote code). This pitfall is about *mint operators* (running the Cashu mint as a live service). The exposure is **more direct** — the mint operator literally moves sats on instruction from users, making the § 1960 / FinCEN argument cleaner than it was for Storm.

**How to avoid:**
1. **Reference mint operator is a non-US entity in a jurisdiction with no MSB/MTL equivalent** OR with a clear crypto regime (Switzerland with FinSA exemption, Liechtenstein with TVTG registration, Panama, BVI).
2. **No US users on the reference mint** (geo-block at the mint API layer, not just the frontend — Tor exit list aware).
3. **Mint operator has separate legal entity from foundation.** Foundation owns IP and operates the relay; mint is a separate non-US corp with thin balance sheet (just enough Lightning liquidity to operate). If the mint gets sanctioned, the foundation survives.
4. **Document the cryptographic non-custody:** The mint operator publishes the Cashu protocol guarantee that **the operator cannot tell which user owns which token or which user redeemed which token**. This is a defensible position that the mint is not "transmitting" in the FinCEN sense (no payee identification possible).
5. **Reserves proofs published weekly** establish that the mint is honoring its claim to the underlying sats — useful for both fraud defense and for users who want to verify solvency.
6. **Cashu protocol-level innovation: encourage NUT-XX "non-custodial mint" research** (e.g., Hal Finney's vision via TFTC — non-custodial ecash via secure enclaves [TFTC piece](https://www.tftc.io/non-custodial-ecash-mints-bitcoin-cashu-enclave-hal-finney/)). If a future Cashu NUT lets the mint never custody the underlying sats (via SGX/Nitro enclaves or PSBTs), the MSB argument weakens substantially.
7. **Path to federated mint** (FROST multisig of operators across jurisdictions) means no single mint operator is the "person" engaged in money transmission — the legal theory has to argue against a *federation* of non-US persons, which is much harder.

**Warning signs:**
- FinCEN advisory specifically naming Cashu / ecash mints
- An EU member state classifying Cashu mints as Electronic Money Institutions (EMIs)
- A sister-protocol mint operator (Cashu.me, Macadamia, Gandlaf's mint) being investigated or charged

**Phase to address:**
Phase 1 (mint operator entity structure) + Phase 3 (federated mint design eliminates single-operator MSB exposure).

**Sources:**
- [FinCEN FIN-2013-G001 — application to virtual currency administrators/exchangers](https://www.fincen.gov/resources/statutes-regulations/guidance/application-fincens-regulations-persons-administering)
- [Is Cashu Custodial? — analytical site](https://iscashucustodial.com/)
- [Non-custodial Cashu mints via enclaves — TFTC](https://www.tftc.io/non-custodial-ecash-mints-bitcoin-cashu-enclave-hal-finney/)
- [Cogent Law — Money Transmitter Licensing Crypto Guide](https://cogentlaw.com/money-transmitter-licensing-what-fintechs-and-crypto-companies-need-to-know-article/)

---

### Pitfall 5: Mainnet Hardcore Launch Without Sufficient Audit + Tiered Plan

**What goes wrong:**
We launch mainnet without caps. A bug in the DLC contract logic, mint reserve tracking, FROST signing flow, or Cashu-DLC binding leaks user funds. Headlines: "Hunch loses 50 BTC of user funds in launch week." Project DOA, plus regulatory attention focused on the failure.

**Why it happens:**
Pressure to ship + complexity of Bitcoin primitives + the novel combination (Cashu + DLC + FROST + Lightning + Nostr is unprecedented in production) + the "no caps" cypherpunk goal pulling against prudent engineering.

**Exact tiered launch plan (firm recommendation):**

| Stage | Duration | Per-market cap | Per-user 24h cap | Total mint cap | Gating |
|---|---|---|---|---|---|
| **T0 — Mutinynet validation** | 6-8 weeks | n/a (test sats) | n/a | n/a | All happy-path + adversarial scenarios passing; refund timeout tested with oracle disappearance |
| **T1 — Signet invite beta** | 4 weeks | 100k sat-equivalent | 1M sat-equivalent | 100M sat-equivalent | Invite-only, 50-200 users, ≥10 markets created, all settled correctly |
| **T2 — Mainnet invite beta** | 4 weeks | **100k sat (~$50)** | **500k sat (~$250)** | **50M sat (~$25k)** total mint reserves | Audit signed off; bug bounty live; invite list 200-1000 |
| **T3 — Public mainnet small-cap** | 4-8 weeks | **1M sat (~$500)** | **5M sat (~$2.5k)** | **500M sat (~$250k)** total mint reserves | T2 ran with zero High/Critical incidents; community feedback positive |
| **T4 — Public mainnet medium-cap** | 8-12 weeks | **10M sat (~$5k)** | **25M sat (~$12.5k)** | **5B sat (~$2.5M)** total mint reserves | T3 ran with zero High/Critical incidents; observability stack proven |
| **T5 — Caps removed (true mainnet hardcore)** | indefinite | **none** | **none** | **none** | T4 ran 3+ months without incident; second audit complete |

**Total elapsed time from T0 to T5: ~6-9 months.** This is the honest engineering reality of mainnet hardcore. The "no caps" goal is preserved as T5 — but T2/T3/T4 caps protect users while real-world failure modes surface.

**Audit scope checklist (must cover all of these to be sufficient):**

- [ ] **DLC contract construction** — adapter signature math, CET enumeration, refund tx path, oracle pubkey commitment
- [ ] **Cashu mint code** — NUT-DLC implementation, NUT-12 DLEQ verification, blind signature flow, reserve accounting
- [ ] **NUT-DLC binding** — atomic linkage between Cashu YES/NO token issuance and DLC collateral lock
- [ ] **FROST DKG** — Pedersen DKG implementation (must check coefficient vector length per Trail of Bits Feb 2024 disclosure — see Pitfall 7), participant validation, key share encryption at rest
- [ ] **FROST signing** — nonce generation (must be deterministic per RFC 9591), aggregation, ROAST robustness if implemented
- [ ] **Lightning channel management** — LDK Node version (must be ≥0.4.x post-griefing fix per Pitfall 11), channel state machine, force-close handling
- [ ] **PSBT handling** — all DLC funding txs use Taproot-compatible PSBTs, no malleability path
- [ ] **Oracle Schnorr signing** — BIP-340 compliance, attestation event format (NIP-88), nonce hygiene
- [ ] **Nostr event verification** — sig verification on EVERY event before any trust action, NIP-09 deletion respect, replay prevention via timestamps + event tags
- [ ] **Frontend wallet integration** — WebLN, NIP-07/46, no key material in frontend memory beyond signing scope
- [ ] **Geo-block enforcement** — IP + Tor exit list, ToS acceptance attestation
- [ ] **Reserves proof** — cryptographic correctness of mint's published proofs of solvency
- [ ] **Threat model document** — explicit, signed off by the audit firm

**Audit firm shortlist (verify in Phase 1):**
- **Trail of Bits Bitcoin team** (best track record on threshold sigs — they did the FROST DKG disclosure in Feb 2024)
- **Block Digital Contracting**
- **Cure53** (frontend + crypto integration audits)
- **Quarkslab** (low-level crypto)
- **NCC Group** (general crypto + protocol audits, history with LN)
- **Galaxy Audit / Inference Security** (Bitcoin DLC-specific knowledge)

Budget: **$75-180K** for a single-firm comprehensive audit; **$120-250K** for dual-firm (recommended for T5 → cap removal). Time: 8-14 weeks lead-time before audit start; audit itself 4-8 weeks.

**Bug bounty payout schedule (Bitcoin DLC-specific recommendation):**

| Severity | Definition | Payout (sats) | USD equivalent at $50k/BTC |
|---|---|---|---|
| **Critical** | Direct fund theft, key extraction, FROST share compromise, mint reserve drain, DLC CET forgery | **500M sat (5 BTC)** to **2B sat (20 BTC)** | $250k – $1M |
| **High** | Privilege escalation, settlement manipulation, oracle bypass, denial of service of mint requiring manual recovery | **50M sat – 500M sat** | $25k – $250k |
| **Medium** | Information leak, partial DoS, ToS bypass, geo-block bypass | **5M sat – 50M sat** | $2.5k – $25k |
| **Low** | UI bug with potential security impact, observability gap, documentation flaw with security relevance | **500k sat – 5M sat** | $250 – $2.5k |

Platform: prefer **HackenProof** or direct via Nostr / GitHub Security Advisories (not HackerOne — US-domiciled, KYC for payouts, conflicts with cypherpunk thesis). Hexens has been used by similar projects but charges 20% to coordinate; weigh against direct.

**Other mitigations:**
1. **Multi-sig emergency reserve** held by foundation, 10-20 BTC earmarked for catastrophic-bug reimbursement. **Ethical note:** publishing the existence of this reserve creates an implicit "we'll make users whole" expectation that could itself become a regulatory hook (looks like a deposit guarantee, attracting EMI / banking law). Recommended approach: reserve exists, is NOT publicly advertised; decision to use it is case-by-case based on counsel input.
2. **Operational monitoring** — Prometheus on mint + oracle + Lightning node, alerts on: reserve discrepancy, channel state anomalies, oracle attestation latency, FROST signing failures, Nostr relay write failures.
3. **Public incident response process documented** before mainnet — status page, Nostr-native disclosure channel, time-to-mitigation SLA published.
4. **Reproducible builds** for all Rust services — supply chain integrity verifiable by any user.

**Warning signs:**
- Audit findings rated High or Critical not yet remediated
- Bug bounty submissions revealing exploit paths
- Anomalies in Mutinynet load testing
- Lightning channel force-closes occurring without justification

**Phase to address:**
Phase 2 (audit + Mutinynet + bug bounty live) + Phase 3 (tiered launch through T5).

**Sources:**
- [LDK v0.1.1 release — Duplicate HTLC Force Close griefing fix (nobsbitcoin)](https://www.nobsbitcoin.com/lightning-dev-kit-v0-1-1/)
- [Trail of Bits FROST DKG disclosure (Feb 20, 2024)](https://blog.trailofbits.com/2024/02/20/breaking-the-shared-key-in-threshold-signature-schemes/)
- [ZF FROST remediation announcement](https://zfnd.org/pedersen-dkg-vulnerability-in-frost-distributed-key-generation-successfully-remediated/)
- [Conduition Cashu vulnerability disclosure (Jul 2025)](https://conduition.io/code/cashu-disclosure/)

---

## Severe Pitfalls (Major incident, recoverable)

### Pitfall 6: NUT-DLC Spec Instability During Build

**What goes wrong:**
NUT-DLC (PR #128 by conduition, depends on PR #127) is in active discussion, not merged into Cashu protocol. If we build against an unfinalized spec, we may need to rewrite when the spec changes. If we fork CDK and upstream merges a different design, we're orphaned.

**Why it happens:**
We're at the frontier. NUT-DLC requires both spec maturity AND CDK implementation. As of 2026-05, neither is finalized.

**How to avoid:**
1. **Spike NUT-DLC FIRST** before any production code commits to it. 1-2 weeks pure prototyping against PR #128.
2. **Contribute upstream actively.** Cashu PR review, Telegram discussion. We want the spec to land in a form we can use.
3. **Design our integration as a CDK extension trait/module**, not a fork. Easier to track upstream.
4. **Fallback plan**: if NUT-DLC isn't ready by T2, ship with "trusted mint, off-chain promise" model and migrate to NUT-DLC in v1.x. Document honestly in marketing.
5. **Direct contact with Calle / conduition / Gandlaf** — Nostr DMs, Cashu Telegram.

**Warning signs:**
- PR #128 closed without merge
- Cashu protocol team picks a different design
- CDK adds NUT-DLC but with breaking API

**Phase to address:**
Phase 1 (spike) + Phase 2 (build with fallback plan ready).

**Sources:**
- [Cashu NUT-DLC PR #128](https://github.com/cashubtc/nuts/pull/128)

---

### Pitfall 7: FROST Cryptographic Implementation Attacks (DEEPENED)

**What goes wrong:**
Threshold Schnorr signature schemes (FROST) have known attack surface beyond generic Schnorr nonce reuse. Real-world vulnerabilities have been disclosed in production FROST implementations.

**Documented attacks (verified primary sources):**

1. **Pedersen DKG threshold manipulation (Trail of Bits, Feb 20, 2024)** — A single malicious participant could surreptitiously raise the threshold of the shared key generated using Pedersen DKG, without detection by other participants. **Ten implementations were vulnerable** including the reference FROST implementation maintained by Chelsea Komlo and the Zcash Foundation's `frost-rust` library. If exploited, an attacker could set the threshold higher than the number of issued shares, making the multisig **unspendable** — funds permanently locked. Fix: validate the length of the coefficient commitment vector against `t + 1`. Disclosed Jan 3, 2024; patched in FROST 1.0.0 (ZF Foundation) and contemporaneous releases of `frost-secp256k1`. [Trail of Bits writeup](https://blog.trailofbits.com/2024/02/20/breaking-the-shared-key-in-threshold-signature-schemes/) — [ZF remediation](https://zfnd.org/pedersen-dkg-vulnerability-in-frost-distributed-key-generation-successfully-remediated/).

2. **Wagner / Drijvers k-tree attack on naïve Schnorr multisignature schemes** — FROST is **designed to avoid this** by ensuring the adversary cannot adaptively choose commitments or messages used in the binding factor. **Important:** this is mitigated *in FROST's design* but is a trap for anyone who tries to "simplify" FROST or implement a non-standard variant. [FROST paper, Komlo & Goldberg, eprint 2020/852](https://eprint.iacr.org/2020/852.pdf).

3. **Adaptive corruption (Crites et al.)** — Recent analysis by Crites, Komlo, et al. examines whether FROST is secure when the adversary can adaptively choose which signers to corrupt after seeing the public key and participating in signing. The original FROST paper proved security only in a *static* corruption model. Adaptive security may require a stronger assumption (algebraic group model) or modified construction.

4. **Concurrent / parallel signing sessions** — FROST is provably secure under concurrent signing, BUT this requires correct nonce management. Implementations that leak nonces across sessions, or share nonce state between processes, can be vulnerable to Wagner-style attacks. The `frost-secp256k1-tr` crate handles this correctly; custom wrappers may break the invariants.

5. **Rogue-key attacks on simple Schnorr multisig** — A naïve Schnorr aggregation lets a malicious participant pick its public key as a function of others' keys, then sign solo. **FROST defeats this** via the DKG (no participant chooses their public key independently of the protocol). Again: don't implement variants that drop the DKG.

**Concrete risks for Hunch:**

- If we use `frost-secp256k1-tr` versions BEFORE the Jan 2024 patch (currently we'd be on 2.x or later, but any forks or old vendored copies are risk), we inherit the Pedersen DKG vulnerability.
- If a contributor implements a "lightweight FROST" or tries to shortcut DKG (e.g., trusted dealer instead of distributed generation), they may reintroduce rogue-key or Wagner-style risks.
- If we run signing sessions concurrently across mint + oracle services without proper isolation, nonce state contamination is possible.

**How to avoid:**
1. **Pin `frost-secp256k1-tr` ≥ 2.2.0** (current as of 2026-05; verifies Pedersen DKG fix is in). Update PROJECT.md / STACK.md.
2. **No custom FROST wrappers.** Use upstream calls directly. Document this rule in CLAUDE.md.
3. **Per-process nonce isolation.** Mint and oracle each have their own FROST coordinator process; no shared state.
4. **DKG ceremony playbook with coefficient vector length validation** as an explicit step.
5. **Cryptographic review by FROST specialists** during audit — request specifically Tim Ruffing (Blockstream, ROAST author) or Jonas Nick, or Trail of Bits cryptography team.
6. **Property-based fuzzing on the FROST integration** — proptest scenarios that simulate one malicious participant.
7. **Track upstream FROST research** — adaptive-security paper progress, ChillDKG (BIP-DKG), ROAST robustness extensions.

**Warning signs:**
- FROST crate update with security advisory
- Dependency `cargo audit` warnings on FROST-related crates
- Custom nonce or DKG code introduced by contributors
- Compiler upgrade changes binding factor reproducibility

**Phase to address:**
Phase 1 (spike DKG; audit cryptographer engaged) + Phase 2 (audit covers FROST as a specific scope item) + ongoing maintenance.

**Sources:**
- [Trail of Bits — Breaking the Shared Key in Threshold Signature Schemes (Feb 20, 2024)](https://blog.trailofbits.com/2024/02/20/breaking-the-shared-key-in-threshold-signature-schemes/)
- [ZF Foundation Pedersen DKG remediation announcement](https://zfnd.org/pedersen-dkg-vulnerability-in-frost-distributed-key-generation-successfully-remediated/)
- [FROST paper (Komlo & Goldberg, 2020)](https://eprint.iacr.org/2020/852.pdf)
- [RFC 9591 — FROST](https://datatracker.ietf.org/doc/rfc9591/)
- [ROAST paper (Ruffing et al., 2022) — eprint 2022/550](https://eprint.iacr.org/2022/550.pdf)
- [Blockstream BIP-DKG (ChillDKG)](https://github.com/BlockstreamResearch/bip-frost-dkg)

---

### Pitfall 8: Mint Operator Rug + No Documented Cashu Mint Rugs Yet (DEEPENED)

**What goes wrong:**
The Hunch operator runs the reference Cashu mint. Between bet placement and market settlement, the mint operator could theoretically:
- Issue YES tokens without backing them with Lightning collateral
- Block users from selling positions
- Refuse to honor atomic swaps
- Disappear with DLC funding before CET broadcast

**Current state of Cashu mint risk (verified, May 2026):**

- **No publicly documented Cashu mint rug pulls to date** (as of 2026-05-27 search). This is good news but also misleading — Cashu mints in production are small, with niche audiences. The lack of incidents is partly because the attack surface hasn't been tested at scale.
- **Two recent Cashu vulnerability disclosures (NOT rugs, but operator-impact bugs):**
  - **Jul 2025:** Conduition disclosed vulnerabilities in the Cashu protocol and some Cashu wallets to select Cashu developers. Bounty paid 500 sats, fixes deployed. ([conduition.io](https://conduition.io/code/cashu-disclosure/))
  - **Nov 2025:** Public disclosure of DoS-using-HTLC vulnerability in Cashu, reported Oct 19, 2025, fixed Oct 21, 2025, version 0.18.0 released Oct 28, 2025. Bounty: 100k sats. ([delving-bitcoin archive](https://github.com/jamesob/delving-bitcoin-archive/blob/master/archive/rendered-topics/2025-11-November/2025-11-02-public-disclosure-denial-of-service-using-htlc-in-cashu-id2090.md))
- **The ambient risk remains:** Cashu's Chaumian mint trust model means the mint operator has full control during market lifetime. The DLC protects funds at SETTLEMENT, but not during issuance.

**Why it happens:**
Cashu mints, by design, are trust-required for token *issuance honesty* (the cryptography protects token *unforgeability* but not *backing*). If the mint issues tokens not backed by reserves, users only discover this when they try to redeem.

**How to avoid:**
1. **Mint reserves proofs** (NUT-22-style or custom NUT extension) — publish periodic cryptographic proofs that issued tokens are backed by Lightning reserves.
2. **Daily / weekly reserves audits** published on Nostr — signed proofs of solvency, Bitfinex-style "proof of reserves."
3. **Multiple competing mints** — protocol-level support so users can choose mint per market. Hunch frontend MUST show alternative mints from day 1.
4. **Mint reputation events on Nostr** (kind:30891) — community signals build trust over time. Bad-faith mints get downvoted.
5. **Path to federated mint** (Phase 4 — FROST multisig operators) so single op can't rug.
6. **Open-source mint code** — anyone can audit. Document the deployment so independent mints can spin up.
7. **Mint operator pseudonymity + public reputation** — if Hunch the org rugs, they lose all credibility immediately.
8. **Tiered caps** during early operation (see Pitfall 5) reduce blast radius.
9. **Cross-mint atomic swaps via NUT-04 / NUT-05 Lightning melt+mint** — users can rotate out of a suspect mint quickly.

**Warning signs:**
- Mint API errors increase suddenly
- Mint operator goes silent on Nostr
- Reserves proofs delayed or missing
- Unusual Lightning channel state (forced closes, rebalancing failures)
- Sudden change in mint fee schedule
- Mint operator changes terms unilaterally

**Phase to address:**
Phase 2 (mint reserves proofs + reputation infrastructure) + Phase 4 (federation).

**Sources:**
- [Conduition Cashu vulnerability disclosure (Jul 2025)](https://conduition.io/code/cashu-disclosure/)
- [Cashu HTLC DoS public disclosure (Nov 2, 2025)](https://github.com/jamesob/delving-bitcoin-archive/blob/master/archive/rendered-topics/2025-11-November/2025-11-02-public-disclosure-denial-of-service-using-htlc-in-cashu-id2090.md)
- [Is Cashu Custodial? (iscashucustodial.com)](https://iscashucustodial.com/)

---

### Pitfall 9: Oracle Lies / Oracle Collusion / Bribery Economics (DEEPENED)

**What goes wrong:**
An oracle attests an incorrect outcome. Market resolves wrongly. Losing side has no recourse on a Bitcoin DLC (no smart contract to slash). FROST k-of-n helps but doesn't fully solve — if k operators collude, they can sign anything.

**The 2025 Polymarket UMA precedent (verified, $7M attack):**

Between **March 24 and 25, 2025**, an attacker holding a large UMA token position cast **5 million UMA tokens across three accounts (~25% of total dispute-resolution votes)** to falsely settle the Polymarket market "Will Ukraine agree to Trump's mineral deal before April?" The contract's odds moved from 9% to 100%, resolving as "Yes" — despite no official agreement. **$7M was settled to the attacker's side.** Polymarket called it "unprecedented" and refused refunds. ([CoinDesk](https://www.coindesk.com/markets/2025/03/27/polymarket-uma-communities-lock-horns-after-usd7m-ukraine-bet-resolves), [The Defiant](https://thedefiant.io/news/defi/polymarket-s-usd7m-ukraine-mineral-deal-debacle-traced-to-oracle-whale))

**The bribery economics problem (NEW analysis):**

UMA's failure mode demonstrates a fundamental issue: **token-weighted dispute voting** breaks when the value at stake in the market exceeds the value of the governance token's reputation. If $7M is at stake and the cost of accumulating 25% of UMA voting power is less than $7M of attacker profit minus an acceptable token-price hit, the attack is economically rational.

**Hunch's oracle stake model is reputational, not financial.** This is a deliberate choice (no token, no slashing) — but it raises the question: **what incentivizes oracle truthfulness?**

Answer (Hunch's model): **Reputation-as-business-asset.**

- Oracles announce themselves on Nostr (NIP-88 / kind:31222 oracle-announcement events).
- Their past attestation accuracy is a public, queryable Nostr stream.
- Market creators choose which oracle to use; bettors look at oracle reputation before betting.
- Oracles that build reputation over time can charge attestation fees (e.g., 0.5-1% of market settled value) — this becomes their revenue stream.
- A single false attestation destroys this revenue forever.

**The economic comparison:**

| Attack | UMA / Polymarket model | Hunch reputation model |
|---|---|---|
| Attacker's cost | Buy enough UMA to swing vote (one-time, recoverable on token sale) | Build up oracle reputation over months/years; then burn it (one-shot, non-recoverable) |
| Attacker's gain | Profit on single market | Profit on single market |
| Recovery for attacker | Sell UMA at modest loss; still has the protocol-level voting power for next time | Lost reputation is permanent; cannot oracle again under same identity |
| Defender remedy | Tokenomics adjustments (slow) | Bettors blacklist oracle (instant on next market) |

The key insight: **for Hunch's model to work, the cost of building oracle reputation must exceed the profit from a single false attestation.** This is achieved by:
- **Time-decayed reputation weight:** new oracles have low weight; weight increases with successful attestations and time-without-disputes. (Phase 4 work.)
- **Minimum attestation history before high-value markets:** UI gates markets >X sat to oracles with reputation score ≥Y.
- **Multi-oracle FROST k-of-n for high-value markets:** for markets above a sat threshold, the frontend defaults to a 5-of-7 FROST oracle federation. Bribing 5 reputable oracles is economically infeasible.
- **Dispute window with community challenge:** 24-48h between attestation and DLC execution; anyone publishes challenge events (kind:30890); if challenge wins community signal threshold, market resolves to INVALID (50/50 refund CET).

**INVALID outcome must be built into every DLC.** Most markets should have INVALID as a third CET with 50/50 split, used when the question wasn't actually verifiable or oracle disputes are unresolvable. Augur v2 made INVALID a tradeable outcome and this was correct ([CoinDesk on Augur v2](https://www.coindesk.com/tech/2020/07/28/5-years-after-launch-predictions-market-platform-augur-releases-version-2)).

**How to avoid:**
1. **FROST k-of-n with high threshold for high-value markets** (e.g., 5-of-7 or 7-of-10) — collusion requires massive coordination.
2. **Oracle reputation system on Nostr** with time-decayed weighting.
3. **Dispute period** — 24-48h between attestation and DLC execution.
4. **INVALID outcome built into every DLC** with 50/50 split as fallback.
5. **Diversity in oracle marketplace** — many competing oracles, market creators choose; bettors refuse markets with sketchy oracles.
6. **Public attestation event format** — oracle signs ALSO over event context (sources, evidence URLs, on-chain proofs where applicable), not just the binary outcome.
7. **Cost-of-attack model published** — for each market size tier, document the implied cost of a successful oracle attack. This is honest disclosure.
8. **Oracle insurance pool (Phase 4 exploration):** Oracles voluntarily stake sats into a Cashu-backed pool that pays out to disputed-market victims if oracle is judged false by community signal. Voluntary because slashing is not enforceable on Bitcoin.

**Warning signs:**
- Oracle attests outcomes that contradict public news / on-chain data
- Multiple disputes from independent users
- Oracle suddenly stops communicating
- Pattern of attestations that benefit specific Lightning / on-chain addresses
- An oracle's announced fee schedule decreases dramatically (could indicate desperation or bribe-funded undercutting)

**Phase to address:**
Phase 2 (single-oracle + dispute period + INVALID outcome) + Phase 4 (FROST k-of-n + advanced reputation + cost-of-attack disclosure).

**Sources:**
- [Polymarket UMA oracle manipulation $7M (CoinDesk, Mar 27, 2025)](https://www.coindesk.com/markets/2025/03/27/polymarket-uma-communities-lock-horns-after-usd7m-ukraine-bet-resolves)
- [Orochi Network analysis of UMA attack](https://orochi.network/blog/oracle-manipulation-in-polymarket-2025)
- [The Defiant — $7M Ukraine debacle](https://thedefiant.io/news/defi/polymarket-s-usd7m-ukraine-mineral-deal-debacle-traced-to-oracle-whale)
- [Augur v2 INVALID outcome design (CoinDesk, Jul 2020)](https://www.coindesk.com/tech/2020/07/28/5-years-after-launch-predictions-market-platform-augur-releases-version-2)

---

### Pitfall 10: Permissionless Market Abuse (Augur Assassination Market Replay)

**What goes wrong:**
Within 2 weeks of launch, malicious actors create markets like "Will [public figure] be killed in 2026?" — financial incentive for harm, massive PR disaster, regulatory attention, App Store-equivalent deplatforming threats.

**Augur precedent (verified):**
Augur launched on Ethereum mainnet July 9, 2018. Within weeks, "death pools" / assassination markets appeared on Trump, Bezos, Betty White, Warren Buffett. The Forecast Foundation stated it had "no power to censor, restrict, or curate markets" — true at protocol layer, disaster for PR.

**User numbers post-controversy (verified):** Daily users dropped from **265 in early July 2018 to 37 on August 8, 2018** — a >85% decline in roughly one month. This is the cost of unmitigated permissionless abuse. ([Wikipedia: Augur (software)](https://en.wikipedia.org/wiki/Augur_(software)))

**Also notable: the CFTC in July 2018 publicly noted that Augur contracts resembled binary options under its jurisdiction** — i.e., the assassination markets brought regulatory attention even on a protocol the CFTC otherwise might not have noticed.

**Why it happens:**
Permissionless market creation is a core differentiator. Anti-spam without losing this principle is hard.

**How to avoid:**
1. **Frontend curation at hunch.io level** — explicit ToS prohibits violence/harm markets; frontend hides them via blocklist + ML filter + user reports.
2. **Social graph default filter** — UI shows markets only from extended follow graph (1-3 hops). Bad-faith strangers don't appear unless friends-of-friends amplify them.
3. **Community INVALID flag** — built-in mechanism to flag markets; oracle attests INVALID → 50/50 refund automatically.
4. **Reputation-aware sorting** — by default, no anonymous newbie market reaches "trending" without N participants from established Nostr identities.
5. **No SEO of harmful markets** — robots.txt blocks indexing of markets matching certain patterns on official frontend.
6. **PR response playbook ready** — drafted before launch. Public statement template: "Hunch is a protocol; this is the cost of permissionlessness; our frontend has hidden this market; protocol-level censorship is impossible by design; users can fork, run own frontend, mute."
7. **Distance maintainer from frontend operator** — protocol maintainer is not the curator of any specific frontend. Hunch maintainer can say "I cannot censor the protocol; the frontend operator (legally separate entity) curated this market off the official UI per their ToS."
8. **PoW (NIP-13) on market creation events** — modest CPU cost on a new market prevents bot floods of garbage. Tunable; should not be a barrier to legitimate use.

**Warning signs:**
- Reports / mutes spiking on certain markets
- Press inquiries about specific markets
- Twitter / Nostr screenshots going viral
- Political/regulatory tweet mentioning Hunch by name

**Phase to address:**
Phase 2 (anti-spam infrastructure + INVALID outcome + PR playbook ready BEFORE launch).

**Sources:**
- [Augur (software) — Wikipedia](https://en.wikipedia.org/wiki/Augur_(software))
- [Vice — Assassination markets on Augur (Jul 2018)](https://www.vice.com/en/article/ethereum-assassination-market-augur/)
- [Cointelegraph — Augur assassination markets (Jul 2018)](https://cointelegraph.com/news/blockchain-based-betting-platform-augur-now-features-assassination-markets)

---

### Pitfall 11: Lightning Network Channel Vulnerabilities (NEW + DEEPENED)

**What goes wrong:**
Hunch's mint, oracle, and users all rely on Lightning. 2025 was a brutal year for Lightning security disclosures across all implementations.

**Documented 2025 Lightning vulnerabilities (verified):**

1. **LDK v0.1.x "Duplicate HTLC Force Close Griefing" (early 2025)** — A griefing attack force-closes all of the victim's channels. Discovered by Matt Morehouse. Patched in LDK v0.1.1. ([nobsbitcoin LDK 0.1.1 release notes](https://www.nobsbitcoin.com/lightning-dev-kit-v0-1-1/))
2. **"Irrevocable Fees" vulnerability (mid-2025)** — Could siphon up to 98% of a channel's funds. Affected **Eclair, LDK, and LND**. Mitigations deployed across implementations. ([Bitcoin Protocol News](https://news.bitcoinprotocol.org/lightning-networks-irrevocable-fees-vulnerability-risks-channel-funds/))
3. **Fake Channel DoS in LND (Matt Morehouse disclosure)** — LND performance degraded drastically due to fake-channel DoS; LND stopped responding to requests. Defenses shipped across implementations. ([morehouse.github.io](https://morehouse.github.io/lightning/fake-channel-dos/))
4. **CLN Channel Open Race (Matt Morehouse)** — Concurrent open requests could break CLN state. ([morehouse.github.io](https://morehouse.github.io/lightning/cln-channel-open-race/))
5. **Good Griefing (lingering)** — Bitcoin Magazine notes that "Good Griefing" remains a partially unmitigated vulnerability across LN implementations as of late 2025.

**Why it happens:**
Lightning's protocol is complex; channel state machines are stateful and adversarially exposed; the implementations are still maturing.

**How to avoid:**
1. **Pin LDK Node to most recent patched release** (≥ 0.4.x as of 2026-05; verify before Phase 2 build). Update STACK.md.
2. **Subscribe to LDK + LND + CLN security advisories** (GitHub Security Advisories, LSP partner alerts).
3. **`cargo audit` on every CI run** to catch dependency CVEs.
4. **Multiple LSP relationships** — Voltage, Olympus, Greenlight, Phoenix — don't single-source liquidity.
5. **Channel partner diversity** — don't put all liquidity with one node; partners across ACINQ, Lightning Labs, Voltage, and smaller community nodes.
6. **Watchtower for mint Lightning node** — Eye, Watchtower-eclair, etc. — to handle force-close challenges if mint operator's main node is offline.
7. **Time-locked channel cleanup procedures** — if a channel exhibits abnormal state, operator can force-close defensively rather than waiting.
8. **Anomaly detection on channel state** — Prometheus metrics + alerts on channel balance discontinuity, pending HTLCs > N, force-close events.
9. **No exotic channel types in v1** — stick to anchor outputs + simple taproot if available; avoid PTLCs, Async-Payments, BOLT-12 onion-message-only flows until they're more battle-tested.

**Warning signs:**
- Lightning implementation security advisory published
- LSP partner reports channel anomalies
- Routing failures > 1% of attempts
- Force-close events without operator action

**Phase to address:**
Phase 2 (LDK Node integration with current patched version + multiple LSPs) + ongoing security monitoring.

**Sources:**
- [LDK v0.1.1 — Duplicate HTLC Griefing fix (nobsbitcoin)](https://www.nobsbitcoin.com/lightning-dev-kit-v0-1-1/)
- [Lightning Network Irrevocable Fees Vulnerability (Bitcoin Protocol News, 2025)](https://news.bitcoinprotocol.org/lightning-networks-irrevocable-fees-vulnerability-risks-channel-funds/)
- [Matt Morehouse — DoS Fake Channels](https://morehouse.github.io/lightning/fake-channel-dos/)
- [Matt Morehouse — CLN Channel Open Race](https://morehouse.github.io/lightning/cln-channel-open-race/)
- [Bitcoin Magazine — Good Griefing lingering vulnerability](https://bitcoinmagazine.com/technical/good-griefing-a-lingering-vulnerability-on-lightning-network-that-still-needs-fixing)

---

### Pitfall 12: Schnorr Nonce Reuse / Cryptographic Implementation Bugs (general)

(See also Pitfall 7 for FROST-specific cryptographic risks.)

**What goes wrong:**
Schnorr signatures require unique nonces per signature. Reused nonce = private key recovery. Bug in oracle signing logic could leak operator/oracle keys.

**How to avoid:**
1. **Use battle-tested libraries only** — `frost-secp256k1-tr` (ZF Foundation), `secp256k1` (Pieter Wuille), `bdk`. NEVER write our own nonce generation.
2. **Deterministic nonces via BIP-340 / RFC 6979** — most libraries do this by default; verify.
3. **Cryptographic review** — separate from general audit, get a cryptographer specifically on FROST integration (Tim Ruffing, Jonas Nick, Trail of Bits crypto team).
4. **No hand-rolled crypto** — every signature, every blinding, every DKG ceremony uses upstream library calls.
5. **Property-based fuzzing** — proptest / quickcheck for crypto-adjacent code.
6. **Reproducible builds** — same source = same binary; supply-chain integrity.

**Warning signs:**
- Linter warnings on `rand::random` for crypto-adjacent values
- Custom nonce generation code in PRs
- Compiler upgrades changing crypto behavior
- Dependency updates without re-audit

**Phase to address:**
All phases — continuous vigilance. Mandatory audit at Phase 2 milestone.

---

### Pitfall 13: Censorship via Frontend / Domain / Infrastructure Provider + App Store Deplatforming (DEEPENED)

**What goes wrong:**
Cloudflare drops hunch.io. Registrar pulls the domain. Vercel/Hetzner suspends the deployment. App Store removes any future native app.

**App Store deplatforming patterns (verified primary research):**

1. **Damus (June 2023)** — Apple threatened removal over Lightning "zaps" (in-app digital content tipping with Bitcoin instead of Apple's IAP). Apple cited App Store guideline 3.1.1. **Resolution:** Damus removed zaps-on-posts (treated as digital content payment), retained zaps-at-profile-level (treated as person-to-person). [CoinDesk Damus/Apple resolution](https://www.coindesk.com/tech/2023/06/28/damus-finally-receives-apple-app-store-approval-after-two-week-battle). Jack Dorsey (Damus funder, $5M contribution) publicly criticized the compromise.
2. **Aqua Wallet (Blockstream/Jan3)** — Strategy to remain on iOS: **does not support swaps or buying bitcoin from in-app on iOS launch.** Pure self-custody + receive/send only, no on-ramp/off-ramp inside the app. ([Aqua App Store listing](https://apps.apple.com/us/app/aqua-wallet/id6468594241)).
3. **Blue Wallet** — Lightning + on-chain in one app; remains on iOS by being a wallet (storage/send/receive), not a financial product (no swaps, no in-app buy, no event-betting).
4. **Pattern:** Apple has tolerated self-custodial wallets that don't sell digital content via crypto and don't function as "exchanges" or "betting" platforms. Apple has NOT tolerated tipping-as-content-purchase or in-app trading features.

**Implication for Hunch:** A native iOS Hunch app is essentially **infeasible** under current Apple App Store policies. The app would clearly be:
- A "betting" / "gambling" feature → category restriction or outright ban
- An in-app crypto exchange (sat-to-YES-token swap) → guideline 3.1.5 restriction
- Lightning-based purchase of digital content (YES/NO tokens) → guideline 3.1.1 violation

**Strategy for Hunch:**
- **PWA, not native.** All Hunch frontend functionality via web. Users add to home screen. No App Store dependency.
- **Tor Browser is canonical for iOS users.** No iOS-specific app needed.
- **If native app is ever necessary (Phase 4+):** F-Droid first (Android open store), Android sideload, **no iOS app at all.** Document this decision publicly to set expectation.

**Other infrastructure censorship vectors:**

1. **Cloudflare drops domain** (most common — content complaint or US Treasury sanction).
2. **Registrar pulls TLD** (especially `.io`, `.com`, `.app` — all US-controlled or ICANN-pressurable).
3. **Vercel/Hetzner suspends deployment** (ToS violation, complaint, sanction).
4. **GitHub takes down repo** (DMCA, ToS, sanction — Tornado Cash repos were taken down Aug 2022 then restored after legal challenge).
5. **Apple/Google Play deplatforming.**

**How to avoid:**
1. **Multi-host frontend** from day 1: Cloudflare Pages + Vercel + IPFS pin + Tor hidden service + Radicle. Single host loss = no impact.
2. **Diverse registrars** — Njalla (Sweden), 1984 Hosting (Iceland), Porkbun (US — fallback only). Multiple TLD spread (.markets, .org, .net, .bit via Handshake).
3. **DNS via Handshake / ENS** as backup naming layers — `hunch/` (Handshake), `hunch.eth` (ENS).
4. **Tor hidden service is canonical from launch** — `.onion` is the address-of-record; clearnet domains are conveniences.
5. **IPFS pinning across multiple providers** — Pinata, web3.storage, plus self-pin via Kubo.
6. **Frontend code minimized and portable** — anyone can run it. Document deployment in <5 minutes.
7. **No backend dependencies that can be unilaterally disabled** — protocol relies on Nostr relays (many) + Bitcoin (uncensorable) + Cashu mints (multiple). Frontend connects directly.
8. **GitHub mirror, but Radicle and Codeberg/forgejo mirrors equally active.** If GitHub takes us down, we don't blink.

**Warning signs:**
- Cloudflare abuse complaint
- Domain registrar inquiry
- Legal letters to hosting providers
- App Store rejection letter (would only happen if we ignore the PWA-only advice)

**Phase to address:**
Phase 2 (multi-host from day 1) + ongoing.

**Sources:**
- [Damus zaps Apple App Store removal threat resolution (CoinDesk, Jun 28, 2023)](https://www.coindesk.com/tech/2023/06/28/damus-finally-receives-apple-app-store-approval-after-two-week-battle)
- [Apple guideline 3.1.1 interpretation for Bitcoin tipping (CryptoSlate)](https://cryptoslate.com/damus-says-there-is-nothing-apple-can-do-to-stop-its-bitcoin-lightning-tips/)
- [Aqua Wallet App Store listing (no swaps/buy on iOS)](https://apps.apple.com/us/app/aqua-wallet/id6468594241)
- [Blue Wallet App Store listing](https://apps.apple.com/us/app/bluewallet-bitcoin-wallet/id1376878040)

---

### Pitfall 14: Frontend / IPFS / Tor Deplatform Cascade Recovery Failure (NEW)

**What goes wrong:**
Hunch frontend at hunch.io goes down (Cloudflare drops). IPFS pin loses last replica (the providers we paid stopped serving). Tor hidden service breaks (key compromise or onion routing issue). User now has no way to reach the protocol.

**This is not the same as Pitfall 13** — that one is about *each* host being deplatformed individually. This pitfall is about the **simultaneous failure** of *all* fallbacks, because the multi-host setup wasn't actually independent.

**Real failure modes:**

1. **All IPFS pins paid through the same provider** (Pinata, web3.storage). If the provider account gets suspended, all pins go.
2. **All clearnet domains under the same registrar** (Njalla, Porkbun). Single registrar takedown = no clearnet.
3. **Tor hidden service key on the same server as the mint operator** — server compromise loses everything.
4. **Radicle node was never actually mirrored** — token gesture in docs but never actually deployed.
5. **Bitcoin/Nostr relay setups assumed one specific community relay** — that relay goes down, frontend can't connect.

**Recovery strategy (must be tested before mainnet):**

1. **Documented "recovery from zero" procedure:**
   - User has any one of: a Tor browser + the .onion address, OR a saved IPFS CID + any IPFS gateway, OR a Handshake/ENS name + a HNS/ENS-aware browser, OR a Git clone of the frontend repo + ability to `bun run dev` locally.
   - **Any one of these must be sufficient to access the protocol.** Tested in incident-response drills.
2. **Multiple Tor hidden service descriptors** — secondary `.onion` with a different operator key, published in HIP-0001.
3. **Frontend self-deployment is documented in `README.md`** — anyone can clone + `bun run build` + serve, in <5 minutes.
4. **Frontend has no hard-coded backend.** All endpoints (relays, mints, Esplora) are user-configurable. If hunch.io's defaults all die, users edit settings → still works.
5. **Critical configuration (default relay list, default mint list, default oracle list) published as a Nostr long-form event (NIP-23)** — not as a JSON file on a server. Frontend fetches from Nostr if the server-side config is unreachable.
6. **Pre-funded community treasury for emergency re-deployment** — foundation holds 5-10 BTC earmarked for emergency hosting / domain renewals.
7. **Quarterly recovery drill:** every 90 days, operator simulates total deplatform; verifies a user can still bet using only Tor + saved .onion + community relay.

**Warning signs:**
- IPFS pin counts dropping over time
- Tor descriptor age increasing (hidden service not being re-advertised)
- Community relay reachability decreasing
- DNS propagation issues on backup domains

**Phase to address:**
Phase 2 (recovery procedure documented) + Phase 3 (drill before launch) + ongoing.

---

### Pitfall 15: Cold-Start Liquidity / Empty Markets

**What goes wrong:**
Hunch launches. Few markets, no liquidity. Bettors arrive, see nothing to bet on, leave. Markets sit empty. Network effect never bootstraps.

**Why it happens:**
Two-sided market problem. Without bettors, market creators don't bother. Without markets, bettors leave.

**Augur precedent (verified):** Augur launched July 9, 2018 with 265 daily users; by August 8 (one month later), daily users had dropped to **37**. The cause was a combination of UX issues, assassination market PR, AND fundamental cold-start liquidity. Augur v2 (2020) attempted to address with stable DAI denomination + INVALID outcome + affiliate fees + faster resolution — but never recovered momentum. ([Wikipedia Augur](https://en.wikipedia.org/wiki/Augur_(software)))

**Lessons applied to Hunch:**
1. **Operator-seeded markets at launch** — Hunch operator creates 50-100 well-curated markets on launch day. Topics: Bitcoin halving block height, prominent sports events with neutral oracle availability, crypto market predictions with on-chain data oracles.
2. **Operator-provided liquidity (LP) on first markets** — operator takes both sides of initial markets to ensure tradeable price for first weeks.
3. **Bootstrap influencer partnerships** — Marty Bent, Matt Odell, Stacy Herbert, Calle, Gandlaf, NVK, Junseth, Ben Carman create + promote a few markets each.
4. **Launch with sport / crypto / culture markets** — natural community interest, low PR risk.
5. **Quick resolution markets first** — markets resolve in days, not months. Quick wins build trust.
6. **Promote market creation via Nostr zaps** — community members earn small Cashu token rewards for high-quality market questions.
7. **Cross-promote in Bitcoin Twitter/Nostr/podcasts** before launch.
8. **Polymarket vs Augur lesson:** Polymarket succeeded by being product-obsessed (orderbook + AMM, clean UX, Polygon for low fees). Augur was protocol-obsessed (decentralization theater). **Hunch must be protocol-first but UX-uncompromising.** This is achievable but requires the frontend team to ship at Polymarket-grade quality.

**Warning signs:**
- Week 1: <100 unique users
- Week 4: <10 active markets with >5 participants each
- Markets sit at single-bettor for >24h

**Phase to address:**
Phase 3 (launch checklist) + Phase 3 (marketing strategy).

**Sources:**
- [Augur (software) — Wikipedia](https://en.wikipedia.org/wiki/Augur_(software))
- [Polymarket vs Augur retrospective — PANews](https://www.panewslab.com/en/articles/0e22d6dd-1044-4f29-8074-0eefb0d54195)

---

### Pitfall 16: Solo Dev Burnout / Single Point of Failure

(Unchanged from prior draft — recommendations still hold.)

**What goes wrong:**
Solo dev maintains everything: protocol, mint, oracle, frontend, infra, community. Burnout. Single bug, single key compromise, single Twitter pile-on can end project momentum.

**How to avoid:**
1. **Open-source from day 1** — public GitHub + Radicle + Codeberg.
2. **Document everything as you go** — HIPs, ARCHITECTURE.md, deployment guides.
3. **Modular monorepo** — separate crates that contributors can adopt one at a time.
4. **Public Nostr presence + Discord/Matrix room** — community starts forming pre-launch.
5. **Pay first contributors via grants** — HRF, OpenSats, Spiral, Geyser. Secure 1-2 grants in Phase 1.
6. **Key custody plan** — operator keys in hardware (Coldcard, BitBox02), backup via multisig with trusted friends.
7. **Operator continuity plan** — written instructions if dev disappears (handover key procedures, infra access, community ownership transition).
8. **Cap weekly work hours** — sustainable pace beats burnout sprint.

**Warning signs:**
- Code commit frequency drops
- Issue response time increases
- Public visibility lapses

**Phase to address:**
Phase 1 (community presence + contributor docs) + ongoing.

---

## Moderate Pitfalls (Recoverable, but operationally painful)

### Pitfall 17: El Salvador Founder-Friendliness Has Diminished (NEW)

**What goes wrong:**
Founders pick El Salvador as the operator jurisdiction based on the 2021 Bitcoin Law's reputation, then discover the legal landscape has shifted materially.

**Verified state (May 2026):**

- **January 29, 2025:** El Salvador's Legislative Assembly (controlled by Bukele's New Ideas party) passed reforms by 55-2 vote that **modified 6 articles and repealed 3 articles** of the Bitcoin Law.
- **Key changes (verified, [Reason](https://reason.com/2025/02/03/el-salvador-walks-back-its-bitcoin-law/), [IMF Country Report 25/58](https://www.imf.org/-/media/files/publications/cr/2025/english/1slvea2025001-print-pdf.pdf)):**
  - Bitcoin is **no longer "currency"** but technically remains "legal tender" — a contradictory framing introduced for IMF compliance.
  - Using Bitcoin is now **entirely voluntary** for businesses (previously mandatory acceptance).
  - **Bitcoin can no longer be used to pay taxes or settle government debts.**
  - The government is stepping back from Chivo Wallet involvement.
- **Cause:** $1.4 billion IMF financial assistance package conditions imposed on El Salvador. The IMF explicitly required the rollback of Bitcoin-as-mandatory.
- **Forward-looking:** Per IMF Country Report 25/58, "legal reforms to overhaul the regulation and supervision of crypto-asset activities and markets (including crypto-asset issuers and service providers) will be submitted to Parliament (August 2025 SB), under the guidance of Fund technical assistance." **El Salvador is on a path to crypto regulation under IMF influence.**

**Implication:** El Salvador is no longer the Bitcoin-libertarian sanctuary it was marketed as. Founders setting up there in 2026 face:
- IMF-influenced crypto regulation rollout (expected 2025-2026)
- Political dependence on Bukele's regime — if power shifts, policy could shift further
- Reduced credibility advantage vs. neutral jurisdictions (Switzerland, BVI)

**Recommendation update:** El Salvador moves from "recommended" to "monitor closely." Switzerland (Stiftung) or BVI remain the preferred Hunch foundation jurisdictions. El Salvador could still be acceptable for **individual contributor residency** (low tax + Bitcoin-tolerant culture + visa accessibility) but not for the operating entity.

**Phase to address:**
Phase 1 (legal structure final decision excludes El Salvador as primary).

**Sources:**
- [Reason — El Salvador Walks Back Bitcoin Law (Feb 3, 2025)](https://reason.com/2025/02/03/el-salvador-walks-back-its-bitcoin-law/)
- [IMF Country Report No. 25/58 — El Salvador](https://www.imf.org/-/media/files/publications/cr/2025/english/1slvea2025001-print-pdf.pdf)
- [Digital Watch Observatory — Bitcoin no longer legal tender](https://dig.watch/updates/bitcoin-is-no-longer-legal-tender-in-el-salvador)
- [Americas Quarterly — Bitcoin's Retreat in El Salvador](https://www.americasquarterly.org/article/in-el-salvador-bitcoins-retreat-left-valuable-lessons/)

---

### Pitfall 18: PSBT Smart-Contract-Approval-Scam Equivalents on Bitcoin (NEW)

**What goes wrong:**
The Ethereum world's "token approval scam" — user signs an `approve()` transaction giving a malicious contract unlimited spending rights — has a Bitcoin equivalent via PSBT (Partially Signed Bitcoin Transaction). A user signing a maliciously crafted PSBT can authorize fund transfers they did not intend.

**Real-world precedent (verified):**

- **CertiK has documented PSBT-related vulnerabilities** in production Bitcoin protocols including UniSat Wallet Extension, SwapSats, and Trac's Tap Protocol. ([CertiK PSBT security best practices](https://www.certik.com/resources/blog/exploring-psbt-in-bitcoin-defi-security-best-practices))
- **Pattern:** Frontend or wallet displays a "buy YES" or "sell position" UI, but the PSBT being signed actually authorizes a different (larger, or to a different address) transfer. The wallet may not display the actual output addresses or amounts in a verifiable way.

**Hunch's exposure:**

Hunch users sign PSBTs to fund DLCs, sweep CETs, sell positions via atomic swaps. The user must trust:
1. The Hunch frontend correctly constructed the PSBT.
2. The user's wallet correctly displays what they're signing.
3. No man-in-the-middle modified the PSBT in transit.

A malicious frontend (or compromised dependency in the Hunch frontend supply chain) could craft PSBTs that drain user wallets. A malicious Cashu mint could similarly request signatures on PSBTs that don't match the displayed atomic-swap.

**How to avoid:**
1. **Frontend displays human-readable PSBT decode** — every output, amount, address shown before signing. Don't just say "sign to confirm bet."
2. **Recommended wallets only:** users should sign with wallets that have strong PSBT display (Sparrow, Specter, hardware signers — Coldcard, BitBox02). Discourage signing in browser-only wallets that obscure PSBT contents.
3. **PSBT amount sanity checks:** frontend validates that the PSBT amount matches the displayed bet amount within tolerance.
4. **Address allowlist for known counterparties:** the mint's funding addresses are documented in HIP-0001; frontend warns if PSBT pays to an address NOT in the allowlist.
5. **Reproducible build of frontend** — supply chain integrity for the PSBT construction code is verifiable.
6. **Code-signed releases of frontend** — Hunch maintainer signs releases with PGP / SSH key published in HIP-0001.
7. **Open PSBT inspector tool** — community tool that decodes a Hunch PSBT independently of the Hunch frontend, so paranoid users can cross-check before signing.

**Warning signs:**
- Frontend supply-chain compromise (npm dependency takeover)
- User reports of unexpected sat amounts being deducted
- PSBT signing flow that doesn't show outputs

**Phase to address:**
Phase 2 (frontend PSBT signing UX) + Phase 3 (security audit + reproducible build).

**Sources:**
- [CertiK — Exploring PSBT in Bitcoin DeFi: Security Best Practices](https://www.certik.com/resources/blog/exploring-psbt-in-bitcoin-defi-security-best-practices)

---

### Pitfall 19: PredictIt-Style No-Action Letter Rescission (Lesson Applied to Hunch's Defensible Position)

**What goes wrong:**
PredictIt operated under a 2014 CFTC no-action letter that allowed it to host political prediction markets on a small-scale academic-research basis. **On August 4, 2022, the CFTC's Division of Market Oversight rescinded the no-action letter** and gave Victoria University (PredictIt's operator) until February 15, 2023 to wind down all open contracts.

**The litigation (verified):**
- Users (led by Sun Valley economist Kevin Clarke) filed suit ("Clarke v. CFTC"). District court denied preliminary injunction.
- **July 2023:** The Fifth Circuit reversed and remanded with instructions to enter a preliminary injunction, finding the CFTC's rescission was likely **arbitrary and capricious**. ([Clarke v. CFTC, 5th Cir.](https://law.justia.com/cases/federal/appellate-courts/ca5/22-51124/22-51124-2023-07-21.html))
- **July 2025:** CFTC issued an amended no-action letter allowing PredictIt to continue operating under updated terms.

**Lesson for Hunch:**

PredictIt's exposure was as a **US-domiciled entity operating under a US regulatory grace**. Hunch's chosen path is the opposite — **no US-domiciled entity, no US-granted grace.** This is structurally better because:
- Hunch never relied on a US no-action letter that could be rescinded.
- Hunch's protocol survives any single jurisdiction's enforcement.
- Hunch doesn't need to litigate against the CFTC; it just doesn't transact in the US.

**However, the PredictIt case is instructive in a negative way:**
- Even with an explicit no-action letter, the CFTC could and did try to shut PredictIt down.
- The litigation took 2-3 years to resolve.
- During that time, PredictIt's operations were under threat — users couldn't be confident the markets would settle.

**Hunch's defensible posture:** Document protocol neutrality + non-US operator structure so rigorously that if a US enforcement action ever targets Hunch, the response is "the named defendant has no US nexus; the protocol has no operator; please dismiss for lack of jurisdiction." This is the **Tornado Cash code-is-speech defense, applied preventively rather than reactively.**

**Phase to address:**
Phase 1 (legal structure documentation makes this defense plausible from day 1).

**Sources:**
- [Clarke v. CFTC, 5th Cir. (Jul 2023)](https://law.justia.com/cases/federal/appellate-courts/ca5/22-51124/22-51124-2023-07-21.html)
- [Brookings analysis of PredictIt litigation](https://www.brookings.edu/articles/how-betting-platform-predictits-legal-struggle-could-hamper-regulators-and-hurt-regulated-firms/)
- [Willkie Compliance Concourse — CFTC PredictIt no-action withdrawal](https://complianceconcourse.willkie.com/articles/court-to-cftc-take-no-action-on-predictit-event-contract-no-action-relief/)

---

### Pitfall 20: Lightning Liquidity / Channel Management for Mint

(Unchanged — see prior draft for details.)

**How to avoid:** LSP partnership (Voltage, Olympus, Greenlight, Phoenix); auto-balance via Boltz/Loop; liquidity monitoring + alerts; initial liquidity bootstrap with diverse partners; trampoline payments for receiving.

**Phase to address:** Phase 2 deployment + Phase 3 operations playbook.

---

### Pitfall 21: Relay Censorship + Nostr Spam

(Unchanged — see prior draft for details.)

**How to avoid:** Run own relay; recommend multi-relay; NIP-65 (Outbox model); PoW (NIP-13); bloom-filter pre-filtering; encourage community relays; pricing relay if needed.

**Phase to address:** Phase 2 (own relay + multi-relay support).

---

### Pitfall 22: FROST DKG Ceremony Failure

**What goes wrong:**
Multi-oracle FROST setup requires all k-of-n co-oracles to participate in DKG. One offline or buggy participant kills the ceremony. After setup, key changes (adding/removing oracle members) require full re-DKG.

**How to avoid:**
1. **Async-friendly DKG implementation** — library that supports paused/resumed DKG. ChillDKG (Blockstream BIP-DKG) is the emerging standard.
2. **Coordination via Nostr DMs (NIP-44)** — participants communicate end-to-end encrypted.
3. **Robust signing (ROAST)** — extension to FROST that tolerates partial participation in signing rounds.
4. **Test ceremonies on signet first** — never first-time-on-mainnet.
5. **Document playbook with timing, fallbacks, dispute resolution.**
6. **Single-oracle markets are FINE as default** — multi-oracle is for high-value markets where the operational cost is justified.

**Warning signs:**
- Frequent DKG aborts in testing
- Participants offline at signing time

**Phase to address:** Phase 4 (multi-oracle FROST as add-on capability).

**Sources:**
- [Blockstream ChillDKG (BIP-DKG)](https://github.com/BlockstreamResearch/bip-frost-dkg)
- [ROAST paper](https://eprint.iacr.org/2022/550.pdf)

---

### Pitfall 23: Tax Compliance Confusion for Users

(Unchanged — see prior draft for details.)

**How to avoid:** Explicit disclaimers; tx history export; no tax reporting to any agency (would require KYC); educational content (without legal advice).

**Phase to address:** Phase 2 (UX disclaimers + tx export).

---

## Technical Debt Patterns

Shortcuts that seem reasonable but create long-term problems.

| Shortcut | Immediate Benefit | Long-term Cost | When Acceptable |
|----------|-------------------|----------------|-----------------|
| Hardcode Hunch's mint as default | Easy onboarding | Reinforces centralization; harder to introduce multi-mint UI later | Never — design multi-mint UI from day 1 |
| Skip FROST k-of-n; use single oracle initially | Faster v1 | Single-oracle becomes the trust anchor; harder to evolve | Acceptable for v1 IF reputation + dispute infra are in place |
| Build matcher engine as core | Better orderbook UX | Centralization risk; censorship vector | Never — mint orderbook is fine for Tier 1 |
| Use Cloudflare-only frontend | Easiest deploy | Single host dependency | Acceptable only with IPFS + Tor mirrors live |
| Forgo audit for v1 | Save $75-180K, ship faster | Catastrophic launch bug | Never — audit is mandatory for mainnet |
| Skip Tor hidden service v1 | Less infra to manage | "Cypherpunk" claim weakened | Never — Tor is foundational for credibility |
| Postpone reputation events to v2 | Simpler v1 | Permissionless market spam unmitigated | Never — reputation is the social anti-spam mechanism |
| Use centralized DB as source of truth, Nostr as backup | Simpler dev | Forks impossible; lock-in | Never |
| Allow USD/USDC pricing display | Easier mental model for users | Brings Polymarket-style legal exposure | Acceptable as informational only, not as denomination |
| Skip recovery-from-zero drill | Saves a day | Pitfall 14 cascade not actually tested | Never — drill at least once before mainnet |
| Custom FROST wrapper | Marginal performance gain | Reintroduces Wagner / rogue-key attack surface | Never |
| Allow PSBT signing without human-readable decode | Less UI work | Pitfall 18 PSBT scam vector opens | Never |

---

## Integration Gotchas

| Integration | Common Mistake | Correct Approach |
|-------------|----------------|------------------|
| **rust-dlc / DDK** | Treating as stable; not pinning version | DDK is alpha (0.0.17); pin exact version; test on each upgrade |
| **CDK NUT-DLC extension** | Building against unmerged PR | Spike first, contribute upstream, plan fallback to pure CDK if NUT-DLC stalls |
| **LDK Node** | Embedding in WASM without testing browser memory limits; not pinning post-v0.1.1 patches | Test WASM build in real browsers; pin ≥0.4.x to include Duplicate-HTLC and Irrevocable-Fees fixes |
| **Nostr relays** | Assuming relay always accepts events | Implement retry + multi-relay fanout; bake in NIP-65 outbox |
| **WebLN** | Assuming user has Alby or Mutiny extension | Detect provider; show install prompt; mobile fallback (BOLT-11 QR + manual paste) |
| **Cashu mint API** | Tight coupling to one mint's API | Use Cashu standard NUTs; user can switch mints |
| **frost-secp256k1-tr** | Mixing with non-Taproot FROST variant; not pinning post-Jan 2024 patch | Choose Taproot-compat from start; pin ≥2.2.0 (post-Pedersen-DKG fix) |
| **Bitcoin Core / electrs** | Hardcoding mempool.space as backend | Run own electrs; mempool.space as fallback |
| **NIP-07 / NIP-46 signers** | Only supporting NIP-07 (browser ext) | Support both — NIP-46 (remote signer) for mobile + hardware signers |
| **Cashu blinded signatures** | Implementing without DLEQ proof verification (NUT-12) | Always verify DLEQ to prevent mint exploit |
| **PSBT construction** | Returning to wallet without human-readable decode metadata | Include decode metadata; recommend hardware signer for amounts >X sat |

---

## Performance Traps

(Unchanged from prior draft.)

| Trap | Symptoms | Prevention | When It Breaks |
|------|----------|------------|----------------|
| Loading all markets from Nostr at startup | Frontend cold start 10s+ | Paginated relay queries, local SQLite index | >500 active markets |
| Re-fetching reputation events every render | Browser CPU pegged | Cache reputation events with TTL | >50 oracles tracked |
| Naive WebSocket connection per relay | Browser hits connection limits | NDK connection pool, multiplexed | >5 active relays |
| Mint single-threaded signing | Latency under load | Tokio task spawning, multi-core utilization | >100 concurrent bets |
| DLC contract per market, no UTXO aggregation | Bitcoin tx fees explode at fee spikes | Batch funding txs across markets when possible | Sat/vB > 50 sustained |
| Loading entire DLC contract state on each page view | Frontend memory bloat | Lazy load, query specific outcomes | DLC contracts > 100 |
| Loading all Nostr events into memory at frontend | Memory exhaustion | Stream + virtual scroll | >10k events |

---

## Security Mistakes

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
| Pedersen DKG coefficient vector not validated | Pitfall 7: threshold manipulation, unspendable funds | Validate vector length = t+1 on each participant's contribution |
| PSBT signed without human-readable decode | Pitfall 18: malicious PSBT drains user wallet | Frontend renders full decode; recommend hardware signer |
| Frontend supply chain not reproducible | Pitfall 18: dependency compromise injects malicious PSBT logic | Reproducible builds; PGP-signed releases |

---

## UX Pitfalls

(Unchanged from prior draft.)

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

---

## "Looks Done But Isn't" Checklist

- [ ] **Mainnet launch:** Often missing security audit — verify audit report signed off (and includes FROST DKG coefficient-vector check + LDK Node post-v0.1.1 patches + PSBT decode validation)
- [ ] **Mainnet launch:** Often missing bug bounty live — verify HackenProof / Nostr / Hexens running with documented payout schedule
- [ ] **DLC settlement:** Often missing refund timeout testing — verify oracle-disappearance flow on signet
- [ ] **Mint API:** Often missing DLEQ proof verification — audit token receive paths
- [ ] **FROST DKG:** Often missing key rotation / member change docs — write ceremony playbook
- [ ] **FROST DKG:** Often missing coefficient vector length validation (Trail of Bits Feb 2024 disclosure)
- [ ] **Frontend:** Often missing Tor hidden service test — verify all flows work in Tor Browser
- [ ] **Frontend:** Often missing IPFS deployment — verify static export works on web3.storage + Pinata
- [ ] **Frontend:** Often missing recovery-from-zero drill — verify a user can bet using only Tor + saved .onion + community relay
- [ ] **Resolution UI:** Often missing on-chain settlement link — verify Bitcoin tx + oracle sig visible
- [ ] **Oracle:** Often missing public attestation history — verify Nostr query returns past attestations
- [ ] **Anti-spam:** Often missing social graph default — verify new users see filtered feed
- [ ] **INVALID outcome:** Often missing in CET construction — verify DLC supports 3-outcome (YES/NO/INVALID)
- [ ] **Geo-block:** Often missing Tor exit list — verify Tor users from US-listed exits are blocked
- [ ] **Geo-block:** Often missing EU strict-regime states — verify France/Germany/Italy/Spain blocked
- [ ] **ToS:** Often missing US restriction + EU strict-regime exclusion language — verify legal counsel signoff
- [ ] **Documentation:** Often missing operator deployment guide — verify someone external can deploy in <1h
- [ ] **Backup / recovery:** Often missing operator key custody plan — verify offline backup tested
- [ ] **Monitoring:** Often missing anomaly alerts — verify Prometheus + Grafana on mint+oracle+LN node
- [ ] **Incident response:** Often missing public-facing process — verify status page exists on Nostr long-form + IPFS
- [ ] **Multi-language:** Often only EN — verify FR working at minimum
- [ ] **PSBT decode:** Often missing human-readable display before signing — verify all signing flows
- [ ] **Reproducible build:** Often skipped — verify frontend + Rust services build identically from source
- [ ] **Cashu mint patch level:** Often outdated — verify ≥0.18.0 (post-HTLC DoS fix Oct 2025)
- [ ] **LDK Node patch level:** Often outdated — verify ≥0.4.x (post-Irrevocable-Fees + Duplicate-HTLC fixes)
- [ ] **FROST crate patch level:** Often outdated — verify ≥2.2.0 (post-Pedersen-DKG fix Jan 2024)

---

## Recovery Strategies

| Pitfall | Recovery Cost | Recovery Steps |
|---------|---------------|----------------|
| CFTC enforcement action | HIGH | 1. Engage non-US-based criminal defense + US-based crypto regulatory counsel immediately. 2. Cease US-targeted activity. 3. Public statement only after counsel signoff. 4. Document protocol neutrality + non-US operator structure for jurisdictional challenge. |
| § 1960 / Tornado-Cash-style charges against maintainer | HIGH | 1. Defense fund activated (SPI, EFF, Coin Center, DeFi Education Fund outreach). 2. Maintainer ceases US travel. 3. Public defense campaign. 4. Open-source / code-as-speech defense prepared. |
| MiCA / EU member-state enforcement | MEDIUM | 1. Engage EU-jurisdiction counsel (MME Zurich for Switzerland; local counsel per member state if national gambling regulator involved). 2. Expand geo-block to affected member states. 3. Statement clarifying protocol-not-product framing. |
| NUT-DLC spec abandoned | MEDIUM | 1. Fork CDK with our extensions. 2. Document fork rationale + maintenance commitment. 3. Maintain compatibility layer for future re-merge. |
| Mint operator rug suspected | HIGH | 1. Public Nostr post + community signal. 2. Trigger DLC refund timeouts where possible. 3. Switch to backup mint via UI. 4. Investigate / disclose. |
| Oracle lies on attestation | MEDIUM | 1. Community publishes dispute events. 2. Reputation hit on oracle. 3. Markets resolve INVALID via INVALID CET if community signal strong. 4. Future markets exclude this oracle. |
| Augur-style abuse market | MEDIUM | 1. Frontend hides market immediately (curation policy). 2. Public statement framing Hunch as protocol-not-curator. 3. Reputation hit on creator. |
| Mainnet launch bug found | HIGH | 1. Emergency pause via mint operator (refuse new bets). 2. Audit firm engaged for diagnosis. 3. Patch + redeploy + public post-mortem. 4. Affected user reimbursement if criticality merits + counsel approves. |
| Schnorr nonce reuse / key leak | CATASTROPHIC | 1. Immediate operator key rotation. 2. Funds in old keys swept (if possible). 3. Audit cause + public post-mortem. 4. If user funds affected, reimbursement plan. |
| FROST DKG threshold-raising attack (Trail of Bits Jan 2024 class) | HIGH | 1. Verify share-state recoverable. 2. Initiate emergency re-DKG with corrected coefficient vector validation. 3. Sweep funds from compromised key to new federation. 4. Audit cause. |
| Lightning channel exploit (Irrevocable Fees / Griefing class) | HIGH | 1. Force-close affected channels defensively. 2. Verify funds on-chain. 3. Upgrade LDK Node / LND. 4. Reopen channels with patched implementations. |
| PSBT signing scam (frontend or supply-chain compromise) | HIGH | 1. Revoke compromised frontend release. 2. Issue PGP-signed advisory on Nostr. 3. Audit frontend supply chain. 4. Reimburse affected users if frontend-side fault. |
| Frontend deplatform cascade (Pitfall 14) | LOW-MEDIUM | 1. Execute recovery-from-zero procedure. 2. Switch DNS to backup host. 3. Update Nostr long-form config event. 4. Public announcement via Nostr + community channels. |
| Solo dev burnout | MEDIUM | 1. Public announcement of pause. 2. Operator hand-off to community per pre-written plan. 3. Multi-contributor model activated. |
| Lightning liquidity dry | LOW-MEDIUM | 1. Engage LSP for emergency channels. 2. Submarine swap for rebalance. 3. Adjust mint fee schedule temporarily. |
| App Store deplatform (if native app ever shipped) | LOW (if PWA-only) / HIGH (if iOS-dependent) | 1. PWA fallback active. 2. F-Droid / sideload distribution for Android. 3. Public statement framing as expected outcome (Damus precedent). |

---

## Pitfall-to-Phase Mapping

| Pitfall | Prevention Phase | Verification |
|---------|------------------|--------------|
| 1. CFTC enforcement (Polymarket) | Phase 1 (legal) | Counsel sign-off on legal structure documented; CFTC v. Blockratize legal theory addressed |
| 2. § 1960 Tornado-Cash-style prosecution | Phase 1 (legal) + Phase 2 (anti-abuse ToS) | Counsel sign-off; ToS published; maintainer residency documented |
| 3. EU MiCA + national gambling exposure | Phase 1 (legal) + Phase 2 (geo-block expansion) | Counsel sign-off on EU structure; geo-block includes EU strict-regime states |
| 4. Operator-as-money-transmitter (Cashu mint) | Phase 1 (legal) + Phase 4 (federated mint) | Mint operator entity separate from foundation; federated mint research path documented |
| 5. Insufficient audit | Phase 2 (audit) + Phase 3 (tiered launch) | Audit report public; T0-T5 launch plan executed; bug bounty live |
| 6. NUT-DLC spec instability | Phase 1 (spike) | Working NUT-DLC prototype on signet |
| 7. FROST cryptographic attacks | Phase 1 (spike DKG) + Phase 2 (audit covers FROST scope) + ongoing | FROST crate ≥2.2.0; coefficient vector validation; specialist cryptographer engaged |
| 8. Mint operator rug | Phase 2 (reserves proofs) + Phase 4 (federation) | Reserves proofs published weekly; federated mint design documented |
| 9. Oracle lies / bribery | Phase 2 (single-oracle + dispute + INVALID) + Phase 4 (FROST k-of-n + reputation) | Dispute mechanism tested on signet markets; FROST k-of-n live for high-value markets |
| 10. Augur-style abuse markets | Phase 2 (anti-spam + curation + PR playbook) | Social graph filter UI live before mainnet; PR playbook drafted |
| 11. Lightning channel vulnerabilities | Phase 2 (LDK Node pinned + multi-LSP) + ongoing | LDK Node ≥ 0.4.x; multiple LSP relationships; watchtower active |
| 12. General Schnorr / crypto bugs | All phases (continuous) | Cryptographic library updates tracked, no custom crypto |
| 13. Censorship via providers + App Store | Phase 2 (multi-host + PWA-only) | Tor + IPFS + 2+ CDNs verified working; no native iOS app shipped |
| 14. Deplatform cascade recovery | Phase 2 (recovery docs) + Phase 3 (drill) | Recovery-from-zero drill executed quarterly |
| 15. Cold-start liquidity | Phase 3 launch checklist | 50+ markets seeded; operator-provided liquidity for first weeks; influencer partnerships secured |
| 16. Solo dev burnout | Phase 1 (community) + ongoing | First external contributor merged within 30 days of public repo |
| 17. El Salvador not founder-friendly | Phase 1 (legal jurisdiction choice) | Switzerland/BVI primary, El Salvador excluded for operator entity |
| 18. PSBT signing scams | Phase 2 (PSBT decode UX) + Phase 3 (reproducible builds + signed releases) | All PSBT signing flows show human-readable decode; releases PGP-signed |
| 19. PredictIt-style no-action rescission | Phase 1 (structure makes this defense unnecessary) | No US no-action letter reliance; protocol-neutrality + non-US documented |
| 20. Lightning liquidity dry | Phase 2 (LSP partnership) + Phase 3 (ops playbook) | LSP partnership confirmed before mainnet |
| 21. Relay censorship / spam | Phase 2 (own relay + multi-relay + PoW) | Multi-relay fanout tested; PoW filter active |
| 22. FROST DKG ceremony failures | Phase 4 (multi-oracle) | DKG ceremony tested on signet 3+ times; ChillDKG-compatible |
| 23. Tax compliance UX | Phase 2 (disclaimers + tx export) | ToS includes tax disclaimer; tx export available |

---

## Published Post-Mortems We Can Learn From

### Augur (2018 launch failure + 2020 v2 attempt)

**What happened:**
- Launched July 9, 2018; assassination markets within 2 weeks.
- User count dropped 265 → 37 daily users in one month (-85%).
- CFTC publicly noted Augur contracts resembled binary options under its jurisdiction (Jul 2018).
- Augur v2 (Jul 2020) attempted recovery: DAI denomination, INVALID as tradeable outcome, 24h resolution (vs v1's 7d), affiliate fees, zero-fee orders. Never recovered momentum.

**Lessons applied to Hunch:**
- INVALID outcome built into every DLC from v1 (not added later).
- Frontend curation distinct from protocol neutrality.
- Cold-start liquidity must be operator-funded for first weeks.
- Cypherpunk minimalism + UX uncompromising; "decentralization theater" loses.
- Avoid token-based governance (Augur's REP was a constant distraction).

[Augur (software) — Wikipedia](https://en.wikipedia.org/wiki/Augur_(software))
[CoinDesk Augur v2 launch (Jul 2020)](https://www.coindesk.com/tech/2020/07/28/5-years-after-launch-predictions-market-platform-augur-releases-version-2)
[PANews — veteran retrospective on Augur vs Polymarket](https://www.panewslab.com/en/articles/0e22d6dd-1044-4f29-8074-0eefb0d54195)

### PredictIt (2022-2025 CFTC litigation)

**What happened:**
- Operated under 2014 CFTC no-action letter.
- CFTC rescinded letter Aug 4, 2022, gave until Feb 15, 2023 to wind down.
- Users sued (Clarke v. CFTC); 5th Circuit reversed Jul 2023; preliminary injunction granted.
- Jul 2025: CFTC issued amended no-action letter; PredictIt continues.

**Lessons applied to Hunch:**
- Reliance on US regulatory grace is fragile — Hunch never relies on a no-action letter.
- Protocol design must survive any single jurisdiction's enforcement.
- Documented protocol neutrality from day 1 makes the "no operator to sue" defense plausible.

[Clarke v. CFTC, 5th Cir.](https://law.justia.com/cases/federal/appellate-courts/ca5/22-51124/22-51124-2023-07-21.html)
[Brookings — PredictIt legal struggle](https://www.brookings.edu/articles/how-betting-platform-predictits-legal-struggle-could-hamper-regulators-and-hurt-regulated-firms/)

### Polymarket UMA Oracle Attack (March 2025, $7M)

**What happened:**
- Mar 24-25, 2025: attacker used 5M UMA tokens (25% of dispute-resolution voting power) across 3 accounts.
- Falsely settled "Will Ukraine agree to Trump's mineral deal before April?" — odds moved 9% → 100% → "Yes" despite no deal.
- $7M settled to attacker's side. Polymarket called it "unprecedented" and refused refunds.

**Lessons applied to Hunch:**
- Token-weighted dispute voting fails when market value > governance token reputation cost.
- Hunch's reputation-only model (no token) avoids this specific failure mode but introduces the bribery question (Pitfall 9).
- FROST k-of-n for high-value markets is the structural answer.
- INVALID outcome + dispute window + community challenge must be built into every DLC.
- Cost-of-attack model published per market tier is honest disclosure.

[CoinDesk Polymarket UMA attack](https://www.coindesk.com/markets/2025/03/27/polymarket-uma-communities-lock-horns-after-usd7m-ukraine-bet-resolves)
[Orochi Network analysis](https://orochi.network/blog/oracle-manipulation-in-polymarket-2025)

### DAO Governance Attacks (Beanstalk, etc. — applicable lessons)

**What happened:**
- Beanstalk DAO (Apr 2022): attacker used flash loans to borrow enough governance tokens to pass a malicious proposal, draining $182M in one transaction.
- Average DAO vote takes 8.2 days; blockchain reorgs / bridge drains execute in <10 minutes — on-chain governance is post-mortem, not crisis firewall.

**Lessons applied to Hunch:**
- Hunch has no governance token, no DAO vote — so flash-loan governance attacks don't apply.
- BUT the lesson generalizes: any time-locked decentralization process (FROST DKG, dispute window, multi-oracle coordination) is vulnerable to fast-attack vectors.
- Time-locked dispute periods (24-48h) are a known weak point; documented in Pitfall 9.

[ChainScoreLabs — Why DAO governance fails at crisis management](https://chainscorelabs.com/blog/defi-renaissance-yields-rwas-and-institutional-flows/defi-risk-management-frameworks/why-dao-governance-fails-at-crisis-management)

---

## Sources (Expanded)

### Primary Legal Sources (HIGH confidence)

- **CFTC v. Blockratize Order, CFTC Docket No. 22-09 (Jan 3, 2022)** — [PDF](https://www.cftc.gov/media/6891/enfblockratizeorder010322/download) — Establishes the exact violations: § 4c(b) CEA + Reg 32.2 + § 5h(a)(1) CEA + Reg 37.3(a)(1).
- **CFTC Press Release 8478-22 (Jan 3, 2022)** — [Link](https://www.cftc.gov/PressRoom/PressReleases/8478-22)
- **18 U.S.C. § 1960 (Cornell LII current text)** — [Link](https://www.law.cornell.edu/uscode/text/18/1960) — Statute under which Storm was convicted.
- **DOJ SDNY press release on Storm conviction (Aug 6, 2025)** — [Link](https://www.justice.gov/usao-sdny/pr/founder-tornado-cash-crypto-mixing-service-convicted-knowingly-transmitting-criminal)
- **Clarke v. CFTC, 5th Cir. (Jul 21, 2023)** — [Justia](https://law.justia.com/cases/federal/appellate-courts/ca5/22-51124/22-51124-2023-07-21.html)
- **Polymarket Amended Order of Designation (Nov 25, 2025)** — [Lexology summary](https://www.lexology.com/library/detail.aspx?g=ad6c6730-dde9-48a0-878c-a2da053d884c)
- **IMF Country Report 25/58 — El Salvador (2025)** — [PDF](https://www.imf.org/-/media/files/publications/cr/2025/english/1slvea2025001-print-pdf.pdf)
- **FinCEN FIN-2013-G001 (virtual currency MSB guidance)** — [Link](https://www.fincen.gov/resources/statutes-regulations/guidance/application-fincens-regulations-persons-administering)
- **ESMA Guidelines 75-453128700-1323 (Mar 17, 2025) — financial instrument qualification under MiCA** — [PDF](https://www.esma.europa.eu/sites/default/files/2025-03/ESMA75453128700-1323_Guidelines_on_the_conditions_and_criteria_for_the_qualification_of_CAs_as_FIs.pdf)

### Primary Cryptographic Sources (HIGH confidence)

- **Trail of Bits — Breaking the Shared Key in Threshold Signature Schemes (Feb 20, 2024)** — [Link](https://blog.trailofbits.com/2024/02/20/breaking-the-shared-key-in-threshold-signature-schemes/) — Pedersen DKG vulnerability disclosure.
- **ZF Foundation — Pedersen DKG vulnerability remediation announcement** — [Link](https://zfnd.org/pedersen-dkg-vulnerability-in-frost-distributed-key-generation-successfully-remediated/)
- **FROST paper (Komlo & Goldberg, 2020) — eprint 2020/852** — [PDF](https://eprint.iacr.org/2020/852.pdf)
- **RFC 9591 — FROST (current standard)** — [Link](https://datatracker.ietf.org/doc/rfc9591/)
- **ROAST paper (Ruffing et al., 2022) — eprint 2022/550** — [PDF](https://eprint.iacr.org/2022/550.pdf)
- **Blockstream BIP-DKG (ChillDKG)** — [GitHub](https://github.com/BlockstreamResearch/bip-frost-dkg)

### Documented Incidents (HIGH confidence)

- **Polymarket UMA oracle manipulation $7M (Mar 27, 2025)** — [CoinDesk](https://www.coindesk.com/markets/2025/03/27/polymarket-uma-communities-lock-horns-after-usd7m-ukraine-bet-resolves)
- **Augur assassination markets (Jul 2018)** — [Vice](https://www.vice.com/en/article/ethereum-assassination-market-augur/), [Wikipedia](https://en.wikipedia.org/wiki/Augur_(software))
- **FBI raid on Polymarket CEO Shayne Coplan (Nov 13, 2024)** — [NBC News](https://www.nbcnews.com/tech/tech-news/fbi-raids-polymarket-ceo-shayne-coplans-apartment-seizes-phone-source-rcna180180)
- **Damus Apple App Store zaps removal (Jun 2023)** — [CoinDesk](https://www.coindesk.com/tech/2023/06/28/damus-finally-receives-apple-app-store-approval-after-two-week-battle)
- **Cashu HTLC DoS disclosure (Nov 2, 2025)** — [delving-bitcoin archive](https://github.com/jamesob/delving-bitcoin-archive/blob/master/archive/rendered-topics/2025-11-November/2025-11-02-public-disclosure-denial-of-service-using-htlc-in-cashu-id2090.md)
- **Conduition Cashu vulnerability disclosure (Jul 2025)** — [conduition.io](https://conduition.io/code/cashu-disclosure/)
- **LDK v0.1.1 Duplicate HTLC griefing fix** — [nobsbitcoin](https://www.nobsbitcoin.com/lightning-dev-kit-v0-1-1/)
- **Lightning Network Irrevocable Fees vulnerability (2025)** — [Bitcoin Protocol News](https://news.bitcoinprotocol.org/lightning-networks-irrevocable-fees-vulnerability-risks-channel-funds/)
- **Matt Morehouse — Lightning DoS Fake Channels** — [Link](https://morehouse.github.io/lightning/fake-channel-dos/)
- **El Salvador Bitcoin Law repeal (Jan 29, 2025)** — [Reason](https://reason.com/2025/02/03/el-salvador-walks-back-its-bitcoin-law/)

### Secondary Legal Analysis (MEDIUM confidence)

- **Mayer Brown — Tornado Cash trial implications (Aug 2025)** — [Link](https://www.mayerbrown.com/en/insights/publications/2025/08/the-tornado-cash-trials-mixed-verdict-implications-for-developer-liability)
- **Money Laundering Watch — Storm verdict analysis** — [Link](https://www.moneylaunderingnews.com/2025/08/tornado-cash-jury-deadlocked-on-most-serious-charges-but-convicted-founder-roman-storm-on-conspiracy-to-operate-an-unlicensed-money-transmitting-business/)
- **Norton Rose Fulbright — EU prediction markets approach** — [Link](https://www.nortonrosefulbright.com/en/knowledge/publications/290d594a/the-eus-approach-to-prediction-markets-and-event-contracts)
- **Oxford Business Law Blog — Regulating Prediction Markets in Europe (Mar 2026)** — [Link](https://blogs.law.ox.ac.uk/oblb/blog-post/2026/03/regulating-prediction-markets-europe-requires-prediction-test)
- **Chambers Fintech Switzerland 2025** — [Link](https://practiceguides.chambers.com/practice-guides/fintech-2025/switzerland/trends-and-developments)
- **DeFi Education Fund — US v. Storm timeline** — [Link](https://www.defieducationfund.org/us-v-storm-background-timeline/)

### Domain Analysis (MEDIUM confidence)

- **Is Cashu Custodial?** — [iscashucustodial.com](https://iscashucustodial.com/)
- **CertiK — PSBT security best practices** — [Link](https://www.certik.com/resources/blog/exploring-psbt-in-bitcoin-defi-security-best-practices)
- **TFTC — Non-custodial Cashu mints via enclaves** — [Link](https://www.tftc.io/non-custodial-ecash-mints-bitcoin-cashu-enclave-hal-finney/)
- **Orochi Network — Oracle Manipulation in Polymarket** — [Link](https://orochi.network/blog/oracle-manipulation-in-polymarket-2025)
- **Crypto Economy — Hidden Fragility of Prediction Markets** — [Link](https://crypto-economy.com/the-hidden-fragility-of-prediction-markets-the-data-that-settles-as-the-true-risk/)

---

*Pitfalls research for: Bitcoin-native cypherpunk prediction market protocol*
*Original researched: 2026-05-27 (initial draft)*
*Enriched: 2026-05-27 (this version) — added 6 new pitfalls (#3 MiCA, #4 Cashu mint MTL, #14 deplatform cascade, #17 El Salvador, #18 PSBT scams, #19 PredictIt lesson), verified all legal claims against primary sources, deepened FROST + Lightning sections with documented 2024-2025 incidents, added Augur/PredictIt/Polymarket post-mortems.*
