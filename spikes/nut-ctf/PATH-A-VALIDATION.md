# SPIKE-01: NUT-CTF Path A Validation

**Spike:** SPIKE-01
**Status:** Maintainer outreach QUEUED (autonomous environment cannot send Nostr DMs / GitHub comments under project pseudonym)
**Date:** 2026-05-28
**Requirement:** SPIKE-01 — NUT-CTF current state validated, fork-vs-upstream decided, maintainer-contact transcripts logged

## Validation Goal

Confirm Path A (NUT-CTF via cashubtc/nuts#337) is the correct architectural direction for Hunch's mint-side multi-bettor primitive. Path B (resurrect cashubtc/nuts#128) and Path C (custodial-promise fallback) are explicitly de-scoped per CONTEXT.md D-01 but referenced here as fallback contingencies.

## Findings (from RESEARCH.md §3)

### PR #128 Status

- **Closed:** 2025-05-20 by thesimplekid (cashubtc maintainer)
- **Closing comment:** "Closing as there is no active work. Please reopen if work continues."
- **Author:** conduition (last commit ~Nov 2024)
- **Conclusion:** Path B is dormant. Reopening would require either conduition resuming work or Hunch maintaining a fork against the closed PR's last commit. Maintenance burden is high; spec drift risk is high.

### PR #337 Status

- **Opened:** 2026-02-07 by joemphilips
- **Title:** "NUTs for Prediction Markets"
- **Architecture:** Conditional Token Framework (CTF) — oracle-agnostic mint that issues conditional tokens whose spendability depends on oracle attestation matching the token's outcome
- **Status:** Active discussion; cashubtc maintainers (thesimplekid, Calle, callebtc) participating
- **Compatibility with Hunch:** Architecturally distinct from the bilateral DLC pattern of PR #128 (mint is no longer the bilateral DLC counterparty in PR #337's framing — instead the mint issues conditional tokens against an externally-funded DLC). Requires HIP-2 + HIP-3 to be structured around this pivot (done in Plan 02).

### Path C (Custodial-Promise Fallback)

If neither A nor B materializes by end of Phase 1, Hunch could ship a custodial-promise variant: the mint promises to pay out per oracle attestation, but the cryptographic primitive does not enforce it. This breaks the no-custody pitch (CLAUDE.md principle 2) and is reserved as last-resort only.

## Maintainer Outreach Queue

The following DMs / GitHub comments are queued. **User must send under project Nostr pseudonym key** (HUNCH_NSEC, generated per Plan 02 Task 2).

### Outreach #1 — joemphilips (PR #337 author)

**Channel:** GitHub PR #337 comment
**Subject:** None (GitHub PR thread)

```text
Hi @joemphilips,

I maintain Hunch (https://github.com/Silexperience210/hunch), a permissionless
Bitcoin prediction market protocol. Our architecture pivots from the closed
PR #128 to PR #337's Conditional Token Framework.

I'm prototyping the NUT-CTF integration on Bitcoin signet (Mutinynet) over
the next 3-4 weeks. A few questions for the maintainer perspective:

1. Is PR #337 still the active architectural direction, or do you see scope
   changes coming?
2. Are there reference implementations or test vectors I can compare against?
3. Would Hunch's spec contribution (HIP-3 in our docs/HIP-3.md) be useful
   feedback to the PR? It currently treats the mint as both DLC counterparty
   AND CTF issuer; I'd value your read on whether that's the right boundary.
4. Is there a path to Hunch maintaining compatibility with upstream as the
   spec evolves (perhaps as an extension package)?

Happy to chat on Nostr (npub-TBD, key gen in progress) or here. No deadline
pressure on your end — Hunch's signet timeline is flexible.

Thanks,
Silex (Hunch protocol maintainer, pseudonymous)
```

### Outreach #2 — conduition (PR #128 author, NUT-DLC originator)

**Channel:** Nostr DM
**Note:** conduition's npub is publicly listed in their GitHub profile

```text
Hi conduition,

I'm Silex, maintaining Hunch (https://github.com/Silexperience210/hunch).

Your PR #128 (NUT-DLC) was the original architectural inspiration for what
we're building. Now that #128 is closed and #337 is the successor, I'd value
your perspective:

1. Do you see #337's CTF approach as a clean architectural improvement, or
   as a different problem (oracle-agnostic vs DLC-bilateral)?
2. Are there pitfalls from your #128 work that #337 doesn't yet address?
3. Would you be open to reviewing HIP-3 (https://github.com/Silexperience210/hunch/blob/main/docs/HIP-3.md)?
   It's our application of #337 to multi-bettor markets via DLC backing.

Honorarium available in sat via Lightning if you'd accept; pseudonym OK.

Thanks,
Silex
```

### Outreach #3 — Calle (cashubtc maintainer)

**Channel:** Nostr DM (Calle's npub: npub1234... — verify from cashubtc.org)

```text
Hi Calle,

Hunch (https://github.com/Silexperience210/hunch) is building on Cashu for
prediction markets. We've pivoted from PR #128 (closed) to PR #337 (NUT-CTF)
and drafted HIP-3 (https://github.com/Silexperience210/hunch/blob/main/docs/HIP-3.md)
specifying the integration.

Two questions for the maintainer perspective:

1. Is PR #337 likely to merge within Q3-Q4 2026, or is it longer-horizon?
2. Would CDK (cashubtc/cdk) accept upstream contributions for the CTF
   primitive as #337 stabilizes, or should Hunch maintain a fork until
   merge?

Looking for go/no-go signal on whether Hunch can build against CTF in our
Phase 2 (next 12-16 weeks) or should plan for the longer wait.

Thanks,
Silex
```

## Decision Tree

```
Outreach response → architectural decision

  joemphilips confirms #337 active + accepts HIP-3 feedback
    → SPIKE-02 builds on #337 as-is
    → HIP-3 transitions Draft → Final on SPIKE-02 success

  joemphilips signals #337 will re-scope significantly
    → Pause HIP-3 Final transition until #337 stabilizes
    → SPIKE-02 builds against a pinned #337 commit; document deltas

  Maintainers signal #337 stalls (PR closes or goes inactive ~3 months)
    → Reopen Path B / Path C decision via CONTEXT.md update
    → Issue HIP-3 corrigendum

  conduition open to reviving #128 fork
    → Path B becomes viable; reopen discussion via CONTEXT.md update

  No maintainer responses within 3 weeks
    → Default to "PR #337 as currently specified" per CONTEXT.md D-01
    → SPIKE-02 builds against the #337 HEAD commit at outreach time
    → Document the unilateral assumption in HIP-3 Backwards Compatibility section
```

## Status

| Step | Status | Note |
|------|--------|------|
| Outreach drafted | ✓ Done | 3 DMs / PR comments queued above |
| Outreach sent | PENDING | User sends from project Nostr pseudonym key (Plan 02 Task 2 generates) |
| Responses logged | PENDING | Append to `spikes/nut-ctf/outreach-log.md` as responses arrive |
| Decision recorded | PENDING | After 3-week wait OR responses received, finalize Path A/B/C in CONTEXT.md |

## Falsification Conditions (per RESEARCH §3 + RESEARCH §7.2)

- **VALIDATED:** at least 1 maintainer response confirms PR #337 architecture; SPIKE-02 signet prototype works end-to-end.
- **INCONCLUSIVE:** No maintainer response within 3 weeks; SPIKE-02 attempts against #337 HEAD; document unilateral assumption.
- **FALSIFIED:** Maintainer signals PR #337 will be closed or re-scoped to incompatibility; trigger Path B/C reconsideration via CONTEXT.md update.
