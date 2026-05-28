# Hunch Terms of Service (Reference Frontend)

```
Status:        STRAWMAN — COUNSEL SIGN-OFF PENDING
Effective:     pending counsel review + entity formation
Last updated:  2026-05-28
Version:       draft-0.1
Applies to:    hunch.io reference frontend (Phase 2+); does NOT bind alternative frontends or independent implementations
```

> **NOT YET IN FORCE.** This document is a counsel-input strawman. Final terms require LEGAL-02 counsel review + sign-off (tracked in `PHASE-1-FOLLOWUP.md`).

## 1. Scope

These Terms govern access to and use of the reference frontend at `hunch.io` (and its Tor / IPFS mirrors) operated by Hunch Operations Ltd. (BVI BC, pending incorporation). They do NOT govern:

- The Hunch protocol itself (open-source, MIT, no centralized terms)
- Alternative frontends operated by third parties
- Alternative mints, oracles, or relays
- Forks of the Hunch reference implementation

The protocol is permissionless; these Terms are an operator-side document covering the reference deployment only.

## 2. Eligibility

By accessing the reference frontend, you represent and warrant that you are:

1. Not a resident or citizen of, or physically present in, the United States of America or any U.S. territory.
2. Not a resident or citizen of any other jurisdiction in which use of prediction-market protocols is prohibited by law (including, without limitation, the EU under MiCA for retail end-users in certain Member States; check local law).
3. Not on any government sanctions list (OFAC SDN, EU consolidated, UK sanctions, UN consolidated).
4. At least 18 years of age (or the local age of majority, whichever is higher).
5. Acting in your individual capacity, not on behalf of any institution that prohibits engagement with non-custodial protocols.

The reference frontend implements IP-based geo-blocks (US + sanctioned jurisdictions) and Tor exit-list blocks where Tor traffic is the source. Circumventing geo-blocks violates these Terms.

## 3. Nature of the Reference Frontend

The reference frontend is a **client interface** to the Hunch protocol. It does not:

- Custody user funds
- Operate as a centralized order book (orders are matched via the mint's Tier 1 mechanism per HIP-3, or peer-to-peer via Nostr per HIP-1 kind:38888)
- Issue any token, security, or financial instrument
- Provide investment advice
- Custody mint keys, oracle keys, or DLC keys

The protocol's settlement happens on Bitcoin (DLC); the protocol's liquidity flows through a Cashu mint (NUT-CTF); the protocol's discovery happens on Nostr. The reference frontend renders the protocol; it does not operate it.

## 4. No Investment Advice

Nothing in the reference frontend or its supporting documentation (HIPs, whitepaper, blog) constitutes investment advice, financial advice, accounting advice, or legal advice. Use the protocol at your own discretion after consulting your own professional advisors.

## 5. Prohibited Markets

The reference frontend will not surface, and may filter from display, markets that:

1. Target a specific real-world individual for harm (e.g., assassination markets per Augur-history lessons documented in `PITFALLS.md`).
2. Reference an outcome that is verifiable only through criminal activity (e.g., "Will X be murdered by date Y" — frontend filter, not a protocol-level filter).
3. Violate applicable law in the operating-entity's jurisdiction (subject to counsel review).
4. Target persons or entities on government sanctions lists.

The Hunch protocol itself remains permissionless. Curation happens at the frontend layer. Users seeking unfiltered markets may use alternative frontends or query the underlying Nostr relays directly.

## 6. No Custody, No Recovery

The reference frontend does not custody user funds. The mint's Cashu tokens are held by the user's wallet (browser-local or external Nostr-aware wallet). The frontend has no key, no escrow, no ability to recover funds the user loses access to.

If a user loses their wallet keys, the frontend cannot recover funds. If a mint stops operating, the Cashu reserve mechanism (HIP-3 + HIP-2 refund timeout) allows on-chain recovery through the underlying DLC — but the user must claim it themselves, or use an alternative mint or operator-tool.

## 7. No Tax Advice

Bets made through the protocol may have tax consequences in your jurisdiction. Hunch Operations Ltd. provides no tax advice and no tax reporting. Users are responsible for their own tax compliance.

## 8. Limitation of Liability

To the maximum extent permitted by applicable law, Hunch Operations Ltd. disclaims all warranties (express or implied) regarding the reference frontend, including warranties of fitness for purpose, merchantability, non-infringement, and uninterrupted operation. In no event shall Hunch Operations Ltd. be liable for any indirect, consequential, special, or incidental damages arising from use of the reference frontend.

This limitation does not exclude or limit liability that cannot be excluded under applicable law (e.g., gross negligence, willful misconduct, fraud).

## 9. Protocol Risks

Users acknowledge:

1. **Oracle risk:** an oracle may sign incorrectly, sign late, or fail to sign at all. Mitigations (HIP-2 INVALID outcome, HIP-2 refund timeout, HIP-4 multi-oracle quorums) exist but do not eliminate this risk.
2. **Mint risk:** while a market is live, the mint operator could theoretically misbehave (e.g., not honor settlement). The DLC settlement layer constrains this risk (HIP-2 funding output cannot be unilaterally spent), but mint operational failures are possible.
3. **DLC risk:** Bitcoin DLC primitives are relatively new. Audits exist on rust-dlc but not on every implementation downstream.
4. **NUT-CTF risk:** Cashu NUT-CTF (PR #337) is in draft. HIP-3 tracks this; Phase 1 SPIKE-02 validates the integration on signet before mainnet exposure.
5. **Lightning risk:** Lightning channel routing failures may delay deposits or withdrawals.
6. **Regulatory risk:** prediction-market regulation varies by jurisdiction and may change. Users are responsible for their own legal compliance.

Hunch undergoes external security audit before mainnet launch (Phase 3). The audit report will be published. Use after that point is still subject to the above protocol risks.

## 10. Open Source

The Hunch protocol and reference implementation are MIT-licensed (see `LICENSE` at https://github.com/Silexperience210/hunch). Anyone may fork, run a competing frontend, run an alternative mint, or operate an independent oracle. These Terms govern only the reference frontend deployment operated by Hunch Operations Ltd.

## 11. Termination

Hunch Operations Ltd. may terminate access to the reference frontend at any time, for any reason, including but not limited to: legal compliance, security investigation, or operational shutdown. Termination of frontend access does NOT affect the user's ability to:

- Access the protocol directly via alternative frontends or the CLI
- Recover Cashu tokens via the mint's standard redemption flow
- Claim DLC refund via the protocol's refund-timeout branch (HIP-2)

## 12. Governing Law

Subject to counsel revision. Working hypothesis: British Virgin Islands law (jurisdiction of operating entity).

## 13. Dispute Resolution

Subject to counsel revision. Working hypothesis: binding arbitration under BVI International Arbitration Centre rules; class actions waived.

## 14. Changes

Hunch Operations Ltd. may revise these Terms by publishing an updated version at the canonical URL. Users are bound by the latest version. Material changes will be announced via the reference frontend at least 14 days before effective date.

## 15. Contact

For Terms-related inquiries:
- Nostr DM: `<npub-TBD — Plan 02 Task 2 generates>`
- PGP-encrypted email: `<pseudonym>@protonmail.com` (TBD, published in SECURITY.md Phase 2)

---

**COUNSEL SIGN-OFF PENDING.** This document is a strawman for counsel review. Final version requires LEGAL-02 counsel sign-off as PDF artifact (tracked in `PHASE-1-FOLLOWUP.md`).
