# LEGAL-01: Jurisdiction Comparison Memo

**Status:** STRAWMAN — COUNSEL RECOMMENDATION PENDING (LEGAL-02 input required)
**Drafted:** 2026-05-28
**Author:** Silex (Hunch protocol maintainer, pseudonymous)

> This memo is a working hypothesis for counsel input, NOT a final entity decision. Per CONTEXT.md decision D-04, jurisdiction selection is delegated to the crypto-specialized counsel engaged in LEGAL-02.

## Goal

Identify the offshore jurisdiction (or jurisdictions) where Hunch should incorporate its protocol foundation and (optionally) a separate mint-operator entity. The chosen jurisdiction must:

1. Accept a pseudonymous beneficial owner (or accommodate nominee structures that preserve pseudonymity)
2. Have legal certainty around open-source software development without protocol-token issuance
3. Provide banking access for the foundation to receive donations + pay counsel + auditors
4. Minimize treaty exposure to extraterritorial enforcement actions
5. Have crypto-specialized counsel available in-jurisdiction
6. Have reasonable operational cost (setup + annual maintenance)

## Working Hypothesis (subject to counsel revision)

**Two-entity structure:** Swiss Stiftung (Hunch Foundation) for protocol IP + brand stewardship; BVI BC (Hunch Operations Ltd.) for the reference mint operator. Foundation receives protocol grants and maintains the reference implementation; BVI BC runs the reference mint and assumes operator-side regulatory exposure.

This hypothesis is **subject to counsel revision**. The counsel engaged in LEGAL-02 may recommend Liechtenstein over CH, Cayman over BVI, single-entity over two-entity, or alternatives entirely.

## Comparison Matrix

| Criterion | CH (Stiftung) | BVI BC | Liechtenstein | Panama Foundation | Cayman LLC |
|-----------|---------------|--------|---------------|-------------------|------------|
| Crypto regulatory clarity | Excellent (FINMA, DLT Act 2021) | Good (VASP framework 2020) | Excellent (TVTG 2020) | Mixed (recent reforms) | Good |
| Pseudonymous BO acceptance | Difficult (KYC strict; nominee structures possible) | Possible (nominee director common) | Difficult-to-possible | Possible | Possible |
| Banking access | Excellent (Sygnum, SEBA crypto banks) | Difficult-to-mixed | Good (Bank Frick) | Mixed | Good |
| Counsel availability | High (MME, Bär & Karrer, Kellerhals Carrard) | High (Walkers, Maples, Ogier) | Medium (Marxer, Walch & Schurti) | Medium | High |
| Open-source-friendly | Yes | Yes | Yes | Yes | Yes |
| Foundation-style structure available | Yes (Stiftung) | No (BC only) | Yes (Stiftung) | Yes (Fundación de Interés Privado) | Limited |
| Setup cost (USD est.) | $15K-30K | $3K-8K | $20K-40K | $5K-15K | $8K-15K |
| Annual maintenance (USD est.) | $10K-25K | $3K-8K | $15K-30K | $5K-15K | $5K-12K |
| Treaty exposure to US | Lower (no MLAT for crypto matters; bank secrecy weakened 2016) | Lower (no MLAT-equivalent for crypto, no automatic info exchange beyond CRS) | Lower (TVTG-specific carve-outs) | Mixed (recent reforms increased exposure) | Higher (more treaty integration) |
| Reputational drag in mainstream finance | Low | Medium (offshore stigma) | Low | Higher | Medium |

## Two-Entity Rationale (Working Hypothesis)

A two-entity split lets:

- **Foundation (CH Stiftung)** hold protocol IP, brand assets, maintainer relationships. No operational exposure (does not run a mint, does not custody funds). Lower regulatory burden.
- **Operations (BVI BC)** run the reference mint, hold reserves, settle DLCs. Bears operator-side regulatory exposure (VASP framework). Smaller operational cost; easier to wind down or replace.

This separation matters because:

1. The Foundation can survive even if the Operations entity is forced to wind down (e.g., regulatory action against the reference mint operator). The protocol continues; another mint operator picks up.
2. Personal liability for maintainers is concentrated in the Operations entity (which is BVI-shielded), not the Foundation (which is Swiss-conservative).
3. Counsel + audit costs split sensibly: Foundation legal counsel handles IP/brand; Operations counsel handles VASP/mint compliance.

## Alternative Structures to Consider with Counsel

- **Single CH Stiftung** — simpler, but concentrates operator exposure in Switzerland (high-cost defense if challenged)
- **Single BVI BC** — cheaper, but no foundation-style governance + banking access weaker
- **Liechtenstein Stiftung + BVI BC** — similar to CH+BVI but with TVTG-specific carve-outs for crypto activities
- **Panama Fundación + BVI BC** — cheaper foundation jurisdiction; recent Panama Papers reputational drag
- **Cayman LLC** — single entity, common in DeFi-style projects; higher treaty exposure than BVI

## Operational Considerations for Pseudonymous Maintainer

Per CONTEXT.md decision D-05 (full pseudonymity locked):

1. **Beneficial Owner (BO) disclosure to counsel:** counsel will require KYC on the BO. The chosen counsel MUST accept pseudonymous-but-legally-identified BO (via long-term residence document + Nostr-key cryptographic proof of consistent identity over time). Counsel that requires real-name disclosure is rejected.
2. **Nominee director arrangements:** counsel-provided nominee directors handle public corporate filings. The pseudonymous BO retains beneficial ownership but does not appear in public records. CH and BVI both support this; LI also; Panama partially.
3. **Banking:** Sygnum and SEBA (CH crypto banks) require KYC on BO but accept nominee structures with documented BO behind. Bank Frick (LI) similarly. BVI banking is harder but possible through nominee + offshore bank introducers.
4. **Public communications:** Hunch's pseudonym handles all public-facing communication. Legal entity contracts are signed by nominee directors on behalf of the foundation.

## Counsel Recommendation (PENDING)

> This section is reserved for the counsel engaged in LEGAL-02. After scoping call(s), the counsel records their final recommendation here, including:
>
> - Chosen jurisdiction(s)
> - Specific entity type (Stiftung / BC / LLC / Fundación / etc.)
> - Whether two-entity split is recommended or single-entity is sufficient
> - Specific banking partner(s) approached
> - Specific counsel partner in each jurisdiction (if multi-jurisdiction)
> - Setup timeline + cost estimate
> - Signed engagement letter
>
> **Counsel signature: PENDING**
>
> **Date of recommendation: PENDING**

## References

- Hunch RESEARCH.md §6.1 — Jurisdiction Deep Dive
- Hunch RESEARCH.md §6.2 — Counsel Selection Filters
- Hunch CONTEXT.md decision D-04 — Jurisdiction deferred to counsel
- Hunch CONTEXT.md decision D-05 — Full pseudonymity scope
