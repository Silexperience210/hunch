# Hunch Maintainer Pseudonymity — Public Statement

```
Status:        Public-facing statement of principles
Last updated:  2026-05-28
Version:       draft-0.1
Audience:      Public; contributors; users; researchers
```

## Why the Maintainer is Pseudonymous

Hunch operates under pseudonym as a matter of architectural principle, not as a matter of convenience.

The protocol is designed so that no single individual is required for its continued operation. The maintainer's identity should not be required either. By operating openly under a pseudonym, the maintainer demonstrates that the project's durability does not depend on any specific real-name reputation, jurisdiction, or affiliation.

This is the cypherpunk tradition: from Satoshi Nakamoto onward, foundational Bitcoin contributors have shown that protocols can be authored, maintained, and forwarded without disclosure of the author's real identity. Hunch follows that tradition by choice.

## What Pseudonymity Means for Hunch

1. **Contributions are evaluated on their merit**, not on the contributor's credentials. The reference implementation, the HIPs, the audit reports — they speak for themselves.
2. **Community communications are pseudonymous.** Maintainer participation in Nostr, GitHub, Codeberg, and similar channels is via the project pseudonym. No real-name disclosure is expected or invited.
3. **Doxxing is prohibited in project spaces.** Publishing real-identity claims about the maintainer (or any contributor) within project-controlled channels (issues, PRs, Discord / Matrix / Telegram if established, Nostr threads tagged with project hashtags) is grounds for permanent ban under the Code of Conduct.
4. **The project does not confirm or deny external linkage claims.** External speculation about the maintainer's real identity is not engaged with — neither to deny nor to acknowledge — because engaging in either direction creates incentives for repeated attempts.
5. **Contributors are encouraged to operate pseudonymously too.** No real-name disclosure is collected by the project. Per-repo git config is required to use a pseudonym for commit attribution (see `CONTRIBUTING.md`).

## What Pseudonymity Does NOT Mean

1. **The maintainer is not unknown.** Counsel, foundation board (post-LEGAL-02), and banks (under KYC) know the legally-identified individual behind the pseudonym. The pseudonym is a public-facing identity, not a void.
2. **The protocol is not lawless.** The operating entity (Hunch Operations Ltd., BVI BC, pending incorporation) is a registered legal entity. It cooperates with lawful inquiries through counsel.
3. **The pseudonym is not unaccountable.** Maintainer reputation accrues to the pseudonym. Bad commits, bad decisions, abuse of trust — these all damage the pseudonym's reputation, and the pseudonym carries the consequence.
4. **It does not endorse anonymity for everyone.** Pseudonymity is a project-level choice. Users of the protocol are users; they may operate pseudonymously or not. The Code of Conduct prohibits doxxing but does not require any individual to operate under pseudonym.

## Why This Matters for Users

The pseudonymity of the maintainer is one of many ways Hunch distributes trust. No single person can be coerced into shutting down the protocol because no single person is the protocol. Settlement happens on Bitcoin. Liquidity flows through the Cashu mint per HIP-3. Discovery uses Nostr per HIP-1. None of these require the maintainer's direct involvement to function.

Users should evaluate Hunch on:

1. The audit report (Phase 3 deliverable; will be public)
2. The reference implementation source (open under MIT)
3. The HIPs (public, externally reviewed)
4. The track record of the operator entity and the reference mint
5. The reputation of independent oracles

Not on:

1. Who the maintainer "really" is

This is by design.

## References

- CODE_OF_CONDUCT.md §Pseudonymity — Community-norm enforcement
- CONTRIBUTING.md §Pseudonymity — Contributor pseudonymity instructions
- TERMS.md §10 — Open-source nature of the protocol
- PR_PLAYBOOK.md §G — Doxxing response scenario
- HIP-0 — Protocol overview emphasizing protocol > maintainer

---

**This is a public statement of principles. Operational details (key handling, communication channels, continuity planning) are private to the maintainer and counsel; see the private `PSEUDONYMITY.md` for those.**
