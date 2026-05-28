# NIPs Repo Kind Reservation PR (Queued)

**Purpose:** HIP-1 reserves Nostr event kinds 30888 (market), 38888 (order), 30890 (dispute), 30891 (reputation), 30892 (mint announce) for Hunch. These kinds are not registered against the NIPs registry (https://github.com/nostr-protocol/nips/blob/master/README.md) as of HIP-1 publication. This document is the draft body for a NIPs-repo Pull Request requesting kind reservation; the PR itself is queued for manual open by the user (no GitHub PR API automation in autonomous environment).

**Created:** 2026-05-28

## Draft PR Body

```text
Title: Reserve event kinds 30888, 38888, 30890, 30891, 30892 for Hunch protocol

## Summary

Reserve the following event kinds for the Hunch protocol
(https://github.com/Silexperience210/hunch), an open-source Bitcoin prediction
market protocol specified by HIP-1 (`docs/HIP-1.md` in the Hunch repo):

| Kind   | Class                       | Name              | Spec link                                                                    |
|--------|-----------------------------|-------------------|------------------------------------------------------------------------------|
| 30888  | Parameterized Replaceable   | Market            | https://github.com/Silexperience210/hunch/blob/main/docs/HIP-1.md#kind-30888 |
| 38888  | Ephemeral                   | Order             | https://github.com/Silexperience210/hunch/blob/main/docs/HIP-1.md#kind-38888 |
| 30890  | Parameterized Replaceable   | Dispute           | https://github.com/Silexperience210/hunch/blob/main/docs/HIP-1.md#kind-30890 |
| 30891  | Parameterized Replaceable   | Reputation        | https://github.com/Silexperience210/hunch/blob/main/docs/HIP-1.md#kind-30891 |
| 30892  | Parameterized Replaceable   | Mint Announce     | https://github.com/Silexperience210/hunch/blob/main/docs/HIP-1.md#kind-30892 |

Hunch also depends on NIP-88 (PR #1681 by benthecarman / conduition) for kinds
88 and 89 (oracle announce / oracle attestation). HIP-1 imports those kinds by
reference and does not re-reserve them.

## Motivation

Hunch is a deployed open protocol with a public spec, reference implementation,
and active development. Reserving kinds in the canonical NIPs registry prevents
silent collisions with future kind allocations and gives ecosystem implementers
(other prediction-market protocols, indexers, frontends) a stable surface.

## Spec Maturity

HIP-1 is in Draft status as of 2026-05-28. The kind allocations are stable
within Hunch but subject to corrigendum if NIP-88 PR #1681 or the underlying
event-kind ranges change before merge. Hunch will track this PR thread and
update both HIP-1 and the Hunch reference implementation if NIPs-repo
maintainers prefer alternative allocations.

## Implementation Status

- Reference Rust types: `crates/hunch-protocol` (Phase 2 deliverable)
- Reference TypeScript types: `apps/hunch-web/lib/protocol/` (Phase 2 deliverable)
- Reference relay configuration: `crates/hunch-relay` (Phase 2 deliverable)
- Tests + test vectors: published with reference impl

## Related Specs

- NIP-88 PR #1681: https://github.com/nostr-protocol/nips/pull/1681 (oracle event kinds)
- NIP-23: https://nips.nostr.com/23 (used by HIP publication channel)
- HIP-1 spec: https://github.com/Silexperience210/hunch/blob/main/docs/HIP-1.md
- HIP-0 protocol overview: https://github.com/Silexperience210/hunch/blob/main/docs/HIP-0.md

## Compatibility

If maintainers prefer different kind numbers, Hunch will accept the reassignment
and issue HIP-1 corrigendum with a 6-month deprecation window for the original
allocations. The reference implementation can dual-emit during the deprecation
window.

## Author

Silex (Hunch protocol maintainer, pseudonymous)
silex@hunch.markets
Nostr: <npub published with Plan 02 Task 2 keygen>
```

## How to Open

1. Fork https://github.com/nostr-protocol/nips
2. Edit `README.md` to add the 5 kind entries in the appropriate tables (kinds table for 38888; parameterized-replaceable kinds table for 30888/30890/30891/30892)
3. Commit + push to fork
4. Open PR against `nostr-protocol/nips:master` with title "Reserve event kinds 30888, 38888, 30890, 30891, 30892 for Hunch protocol" and body above
5. Link the PR back here once opened

## Status

PR not yet opened. Queued for manual open by maintainer. Reasonable timing: Phase 1 Week 4–6, after HIPs are published on Nostr (kind:30023 long-form events) so maintainers can review the published HIPs alongside the PR.
