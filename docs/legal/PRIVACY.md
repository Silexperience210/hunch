# Hunch Privacy Policy (Reference Frontend)

```
Status:        STRAWMAN — COUNSEL SIGN-OFF PENDING
Effective:     pending counsel review + entity formation
Last updated:  2026-05-28
Version:       draft-0.1
Applies to:    hunch.io reference frontend (Phase 2+); does NOT bind alternative frontends or independent implementations
```

> **NOT YET IN FORCE.** This document is a counsel-input strawman. Final privacy policy requires LEGAL-02 counsel review + sign-off (tracked in `PHASE-1-FOLLOWUP.md`).

## 1. Scope

This Privacy Policy covers the reference frontend at `hunch.io` (and its Tor / IPFS mirrors) operated by Hunch Operations Ltd. (BVI BC, pending incorporation). It does NOT cover:

- The Hunch protocol itself (open-source; no centralized data collection)
- Alternative frontends operated by third parties
- Alternative mints, oracles, or relays
- The Nostr relay network (which is a public bulletin board by design)

## 2. What We Collect

**Nothing beyond what the protocol requires.** Specifically:

1. **Nostr public key (`npub`)** — required to interact with the protocol; used to filter your markets, your bets, your reputation. The public key is, by design, public.
2. **Mint URL preferences** — locally stored in your browser; never sent to us.
3. **Frontend settings** — locally stored in your browser; never sent to us.

We do NOT collect:

- Email addresses
- Phone numbers
- Real names
- Postal addresses
- Government identification documents
- Biometric data
- Browser fingerprints (no fingerprinting libraries deployed)
- Behavioral analytics (no Google Analytics, no PostHog, no Mixpanel, no Plausible)
- Cookies beyond session preference (no tracking cookies, no advertising cookies, no third-party cookies)

## 3. What the Protocol Stores Publicly (Beyond Our Control)

The Hunch protocol uses Nostr relays for discovery, attestation, and reputation. By design, Nostr events are public:

- Markets you create are public on Nostr (HIP-1 kind:30888)
- Orders you place on Tier 2 P2P matching are public on Nostr (HIP-1 kind:38888)
- Reputation attestations you publish are public on Nostr (HIP-1 kind:30891)
- Oracle attestations consumed by the protocol are public on Nostr (HIP-1 kind:89)

Bet placement against the mint is **NOT** automatically public — Cashu mints issue blinded tokens whose linkage to your npub is cryptographically obscured. Mints may have their own internal logs; consult your chosen mint's privacy policy.

## 4. Geo-Blocking

We implement IP-based geo-blocks (US + sanctioned jurisdictions) and Tor exit-list blocks per the ToS. To implement these blocks, your IP address is checked at the moment of request against a geo-IP database. The IP address is NOT stored beyond the duration of that single request.

If a request is blocked, the block is recorded only as an aggregate count (no per-IP log).

## 5. Server Logs

The reference frontend's web server may keep short-term logs for operational debugging:

- IP addresses (rotated daily; max 7-day retention)
- HTTP request paths (no query string; no body)
- Response codes
- User-Agent strings

These logs are NOT linked to npub or to specific user activity. They are retained for security investigation only and rotated as described.

## 6. Cookies and Local Storage

The reference frontend uses:

- **Session-only cookies** for preference storage (theme, locale). Deleted when you close the browser.
- **Local storage** for: your chosen mint URL, your chosen oracle preferences, UI state. Never sent to us.

We do NOT use:

- Persistent tracking cookies
- Third-party cookies
- Advertising pixels
- Fingerprinting scripts

## 7. Third-Party Services

The reference frontend may make requests to:

- **Nostr relays** (for market discovery and event publication) — these are public infrastructure outside our control.
- **Cashu mints** (for token operations) — your chosen mint, not us.
- **Bitcoin nodes / Esplora endpoints** (for DLC contract verification) — public infrastructure outside our control.
- **IPFS gateways** (for backup HIP retrieval) — public infrastructure outside our control.
- **CDN providers** (for static asset delivery, Phase 2 — Cloudflare or equivalent) — see their respective privacy policies.

We do NOT use:

- Google Fonts (we self-host fonts)
- Google reCAPTCHA (we use no CAPTCHAs at all on the reference frontend)
- Cloudflare Insights or similar analytics offerings
- Stripe, PayPal, or any fiat payment processor (no fiat is processed)

## 8. GDPR / CCPA / LGPD / PIPEDA Notes

Because we do not collect personal information beyond what is described above, most personal-data rights frameworks do not apply to us in their typical forms:

- **Right to access:** We have no record of you beyond aggregate request counts and short-term server logs. We cannot provide records that do not exist.
- **Right to deletion:** Same. There is nothing to delete.
- **Right to portability:** Your npub is yours; export it from your Nostr client. Your local storage is in your browser; export it via browser developer tools.
- **Right to rectification:** We have no fields to correct.
- **Right to object to processing:** We do no profiling.

If you are physically present in the EU under MiCA and believe these statements are insufficient, please contact us via the channel below. **We do not represent that the reference frontend is GDPR-compliant for EU users** — geo-blocking + ToS may limit EU retail access (counsel-dependent).

## 9. Children's Privacy

The reference frontend is not directed at children under 18. We do not knowingly collect information from minors. The protocol cannot verify age; users represent under the ToS that they are 18+.

## 10. Data Storage Locations

To the extent we run reference frontend infrastructure:

- **Origin servers:** physical jurisdiction subject to counsel revision (working hypothesis: Switzerland for foundation servers; BVI or fallback for operating-entity servers)
- **CDN edge nodes:** global (Cloudflare or equivalent, subject to counsel review)
- **Tor hidden service:** distributed across Tor onion routing; no fixed jurisdiction
- **IPFS mirror:** distributed across IPFS pinning services

## 11. Security

We use industry-standard security practices for our infrastructure: TLS 1.3+, HSTS, CSP headers, regular OS / dependency updates, principle of least privilege for service accounts.

We have NOT had a security audit of the reference frontend's deployment as of the effective date of this strawman. Audit is Phase 3 deliverable (per ROADMAP); the audit report will be published.

## 12. Breach Notification

In the unlikely event of a security breach affecting the reference frontend, we will publish a public notification at the reference frontend's status page (and via Nostr under the project pseudonym npub) within 72 hours of confirmation. Because we do not collect personal information beyond what is described above, the impact of any breach is bounded.

## 13. Changes

We may revise this Privacy Policy by publishing an updated version at the canonical URL. Material changes will be announced at least 14 days before effective date.

## 14. Contact

For privacy-related inquiries:
- Nostr DM: `<npub-TBD — Plan 02 Task 2 generates>`
- PGP-encrypted email: `<pseudonym>@protonmail.com` (TBD, published in SECURITY.md Phase 2)

---

**COUNSEL SIGN-OFF PENDING.** This document is a strawman for counsel review. Final version requires LEGAL-02 counsel sign-off as PDF artifact (tracked in `PHASE-1-FOLLOWUP.md`).
