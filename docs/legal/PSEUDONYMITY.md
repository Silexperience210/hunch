# Hunch Maintainer Pseudonymity Plan (Private Operational)

```
Status:        STRAWMAN — internal operational document; counsel review optional
Last updated:  2026-05-28
Version:       draft-0.1
Audience:      Hunch maintainer (Silex) + foundation board + counsel
Distribution:  PRIVATE — committed to repo for project record but contains no real-identity disclosure; references operational practices only
```

> A **public-facing** version of this document exists at `PSEUDONYMITY-public.md` containing principles only. This file documents operational practices for the maintainer's own reference.

## Goal

Maintain the project pseudonym (Silex, key `0xF777C5B8`) as a credible long-term identity across all project-facing surfaces, while preserving the maintainer's real-name privacy. The pseudonymity must be load-bearing in the legal sense — i.e., the project must function operationally without ever requiring the maintainer to expose their real identity.

## Lessons from Precedent

Per CONTEXT.md decision D-05 and RESEARCH.md §6, the playbook is informed by:

1. **Bright-line rules from prior crypto enforcement** — maintainers operating under pseudonym have been most defensible when they:
   - Do not take fees from operations
   - Do not exercise unilateral control over operational decisions
   - Do not operate any custodial frontend
   - Do not make statements that could be construed as commanding operational behavior
2. **Polymarket / Coplan example** — Coplan operated openly. Hunch chooses the opposite. The trade-off: less ability to fundraise from open-market sources, more durability against personal-targeting.
3. **Tornado Cash / Storm and adjacent** — even pseudonymous-leaning maintainers have been pursued. Pseudonymity does not guarantee non-pursuit; it raises the cost of pursuit.

The trade-off is explicit: the Hunch maintainer accepts the operational friction of full pseudonymity to maximize protocol durability.

## Operational Practices

### 1. Commits and Code

- **Per-repo git config:** every Hunch repo sets `user.name "Silex"` and `user.email "silex@hunch.markets"` before any commit. Verified by checking `git log --format="%an <%ae>"`.
- **Signed commits:** all maintainer commits are GPG-signed with the project pseudonym key (`0xF777C5B8`). The secret key (`Silex_0xF777C5B8_SECRET.asc`) is `.gitignore`d in every repo + stored on encrypted external media + a paper backup.
- **No real-name in any file:** verified via grep on every commit. No real email, no real legal name, no identifying employer / location strings.
- **No real-name in commit messages:** never reference real-name colleagues, employers, conference talks attended, geographic specifics.

### 2. GitHub / Hosting Accounts

- **Project GitHub account:** `Silexperience210` is the project's GitHub identity. The associated email is a project pseudonym email (not the real maintainer's personal email).
- **2FA on hosting accounts:** TOTP + recovery codes stored separately from the primary device.
- **No GitHub profile real-name field:** the name field is "Silex" only. No bio referencing real geography, employer, or affiliations.
- **Codeberg / Radicle:** same pseudonym, separate account.

### 3. Communications

- **Nostr DM:** primary inbound channel for project communication. HUNCH_NSEC is the project pseudonym key (Plan 02 Task 2 deliverable). Stored on hardware-backed device + paper backup.
- **Email:** PGP-encrypted email to `<pseudonym>@protonmail.com` (TBD). The mailbox is accessed only via Tor; never via the maintainer's primary IP address.
- **No phone calls from personal phone:** if a counsel call requires voice, use Signal Voice (registered to a burner number) or counsel-arranged voice channel.
- **No physical meetings under pseudonym:** counsel meetings happen via secure video / encrypted channel; in-person meetings only with counsel (under engagement scoping NDA).

### 4. Banking / Finance

- **No real-name bank accounts associated with project funds.** The operating entity's banking is in the entity's legal name (with nominee director arrangements for public records). The pseudonymous maintainer is the beneficial owner identified to counsel + bank under KYC, not in public filings.
- **No personal credit card for project expenses.** All project expenses go through the operating-entity card or are paid in Lightning / on-chain Bitcoin from the project pseudonym wallet.

### 5. Key Genesis Hygiene

- The current project pseudonym GPG key (`Silex_0xF777C5B8_SECRET.asc`) was generated on a device that may have prior real-name activity. **Future hardening (Phase 2):** rotate to a fresh pseudonym key generated on a dedicated, never-personal device. Old key signs the new key's introduction for continuity.
- HUNCH_NSEC (Nostr pseudonym key, Plan 02 Task 2) MUST be generated on a clean device with no prior real-name activity. Document the generation device + circumstances in the maintainer's private opsec log.

### 6. Operational Boundaries (Cypherpunk-Maintainer Posture)

- **Do not take fees from mint operations.** The reference mint is run by the operating entity; the maintainer does not personally receive transaction fees.
- **Do not exercise unilateral protocol control.** Protocol upgrades happen via HIP process. The maintainer proposes, but others (independent implementers, external reviewers, oracle operators) decide.
- **Do not custody user funds.** The protocol's no-custody design means the maintainer never holds bettor funds.
- **Do not make operational commands.** Public statements describe what the protocol enables, not what users should do.

These boundaries are operational, not just rhetorical. They are the defensible-posture core of the pseudonymity plan.

## Continuity Plan

If the pseudonymous maintainer becomes unavailable (illness, legal action, personal circumstances):

1. **Foundation board** (post-LEGAL-02 entity formation) inherits operational continuity for the operating entity.
2. **Independent contributors** continue protocol development under their own pseudonyms or real names.
3. **The HUNCH_NSEC + GPG keys** are backed up (paper + encrypted external media) and may be passed to a designated successor identified to counsel ahead of time. Successor signs continuity statement under the original pseudonym after handover.
4. **The protocol survives** because it is open-source MIT and multi-implementation. No single maintainer is required for protocol continuity; only for the reference deployment's specific updates.

The continuity plan is private — never published as the public statement, because doing so would create a target for adversarial action against the successor.

## Doxxing Response (cross-reference PR_PLAYBOOK §G)

The maintainer's pseudonym may be publicly linked to a real identity by a third party at any time. Per CONTEXT.md D-05 + CODE_OF_CONDUCT.md §Pseudonymity, the project does NOT engage with doxxing — neither to confirm nor to deny linkage.

Operational consequence: the maintainer continues normal project operations under the pseudonym regardless of any external linkage claims. Community-norm enforcement (ban from project channels) applies to anyone who publishes doxxing content in project spaces.

## Counsel Coordination

LEGAL-02 counsel will be informed of:
- The pseudonymity scope and the maintainer's KYC-to-counsel-only arrangement
- The continuity plan (under separate engagement letter terms)
- The boundary rules described above (so counsel can advise on whether any operational change risks crossing them)

Counsel sign-off on this document is **optional** but encouraged. If counsel signs off as PDF, it lands in `signoff/` (gitignored per CONTEXT.md D-05).

## References

- CONTEXT.md D-05 — Full pseudonymity scope
- CONTEXT.md D-08 — Stack locked (project pseudonym key referenced in workspace.package.authors)
- CODE_OF_CONDUCT.md §Pseudonymity — Doxxing prohibition
- CONTRIBUTING.md §Pseudonymity — Per-repo git config requirement
- PR_PLAYBOOK.md §G — Doxxing response scenario
- PSEUDONYMITY-public.md — Public-facing pseudonymity statement (principles only)
- engagement-letter-status.md — Counsel engagement tracking
- RESEARCH.md §6 — Legal Foundation deep-dive
