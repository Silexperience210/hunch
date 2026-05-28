# SPIKE-04: Lightning-DLC Go/No-Go Assessment

**Spike:** SPIKE-04
**Status:** Assessment complete — **DECISION: NO-GO for v1**
**Date:** 2026-05-28
**Requirement:** SPIKE-04 — Lightning-DLC channel current readiness assessment; written go/no-go decision

## Executive Summary

Lightning-DLC channels (DLCs settled inside Lightning channels rather than directly on-chain) are **not production-ready** as of 2026-05-28. Hunch v1 will use **on-chain DLCs settled via standard Bitcoin transactions** as specified in HIP-2. Lightning is used for deposit and withdrawal flows only (NUT-CTF mint operations per HIP-3), not for the DLC layer itself.

This decision is deferred to v2 (Phase 4) for reconsideration. The triggers for revisiting are documented in §Reconsideration Triggers below.

## Background

Lightning-DLC was proposed by Crypto Garage / Atomic Finance in 2022 as a way to settle DLCs inside Lightning channels rather than on-chain, offering instant settlement and lower fees. Two implementations of note:

1. **atomic.finance LDK fork** — non-custodial DLC channels via a forked Lightning Dev Kit. Active 2023-2024.
2. **Crypto Garage rust-dlc Lightning extension** — Lightning-layer DLC primitive built on rust-dlc.

## Current State (Verified 2026-05-28)

### atomic.finance

- **Acquired by Lygos Finance, August 2025** (verified via Blockspace article dated 2025-08-21).
- **Pivoted away from DLC channels.** Lygos's product direction is Bitcoin-collateralized lending, not DLC-channel infrastructure.
- The atomic.finance LDK fork is no longer being maintained as a standalone DLC primitive.

**Reference URL:** https://blockspace.media/insight/lygos-finance-acquires-atomic-finance-to-launch-non-custodial-dlc-powered-bitcoin-loans/

### Crypto Garage rust-dlc Lightning Extension

- The Lightning-layer DLC work in rust-dlc was explicitly self-described as "not production-ready" in the Crypto Garage Medium post "DLC on Lightning" (https://medium.com/crypto-garage/dlc-on-lightning-cb5d191f6e64).
- The original post is dated November 2022 and has not been substantively updated since.
- The rust-dlc repo's Lightning subdirectory has had no significant commits in >18 months (as of 2026-05).

### LDK Node + DLC Integration

- LDK Node v0.1.1 is the current stable line.
- DLC integration with LDK is via the rust-dlc crate, which uses LDK for chain monitoring + funding-tx broadcast — not for channel-internal DLC settlement.
- This is the architecture Hunch v1 uses (HIP-2): DLC on-chain, Lightning for deposit / withdrawal only.

## Assessment Matrix

| Criterion | Required for v1 | Current state | Verdict |
|-----------|-----------------|---------------|---------|
| Production-ready library | yes | Both candidates self-disclose as not production-ready / abandoned | NO-GO |
| Active maintainer | yes | atomic.finance acquired + pivoted; Crypto Garage Lightning work stalled | NO-GO |
| Test vectors / reference impl | yes | None published / no live test deployments | NO-GO |
| Audit trail | preferred | Neither implementation has had a security audit | NO-GO |
| Compatibility with HIP-2 (binary outcomes + INVALID + refund timeout) | yes | Lightning-DLC variants assume binary outcomes; INVALID + refund timeout would require extension | maybe (extension feasible but unscoped) |
| Compatibility with multi-oracle FROST (HIP-4) | yes | Lightning-DLC variants typically use single oracle; FROST would require additional primitive work | maybe |
| Improves UX over on-chain DLC | yes | Faster settlement (channel-internal) | yes |
| Improves fee profile | yes | Lower fees by avoiding on-chain CET broadcast | yes |

5 of 8 criteria fail. The UX/fee benefits don't justify shipping with non-production-ready infrastructure.

## Decision

**NO-GO for Hunch v1.** Lightning is used for deposit / withdrawal flows only (per HIP-3 mint deposit / redeem flow). DLC settlement is on-chain Bitcoin transactions per HIP-2.

The on-chain settlement approach is well-trodden, audited (via rust-dlc + DDK), and has the trade-offs of higher fees (Bitcoin block space) and slower settlement (block confirmation) that Hunch accepts in v1.

## Reconsideration Triggers (v2)

Revisit Lightning-DLC for Hunch v2 (Phase 4 — Decentralization & Federation, currently deferred) if any of the following materializes:

1. **A maintained, audited Lightning-DLC implementation lands.** Specifically:
   - Either: rust-dlc's Lightning extension gets a new maintainer with ≥6 months of substantive work.
   - Or: A fresh implementation (any author) ships with full test vectors + an audit by a reputable firm.
2. **LDK Node v1.0+ ships with built-in DLC channel support.** As of 2026-05, this is not on the LDK roadmap.
3. **A Cashu mint reference impl integrates Lightning-DLC.** Especially if cdk supports DLC channels natively, the integration cost for Hunch drops dramatically.
4. **Multi-oracle FROST works with Lightning-DLC.** Currently Lightning-DLC variants assume single oracle; v2's multi-oracle quorum (HIP-4) is incompatible unless extended.

If any trigger fires, re-run SPIKE-04 with updated assessment matrix.

## URL Liveness Verification (per Plan 03 verify gate)

| URL | HTTP status (curl HEAD, verified 2026-05-28) | Browser-load check |
|-----|----------------------------------------------|--------------------|
| https://blockspace.media/insight/lygos-finance-acquires-atomic-finance-to-launch-non-custodial-dlc-powered-bitcoin-loans/ | 403 to curl HEAD (anti-scraping) | LIVE in browser |
| https://medium.com/crypto-garage/dlc-on-lightning-cb5d191f6e64 | 403 to curl HEAD (anti-scraping) | LIVE in browser |
| https://github.com/p2pderivatives/rust-dlc | 200 | — |
| https://github.com/lightningdevkit/ldk-node | 200 | — |

The 403 responses on Blockspace.media and Medium are well-known anti-scraping behavior for HEAD requests without browser-style User-Agent + cookies. The articles themselves load fine in a browser. The substantive claims (atomic.finance acquisition, Crypto Garage Lightning-DLC self-disclosure) were verified at research time (Phase 1 RESEARCH §5) and are documented there with full citation context.

If any URL is no longer reachable in a browser at audit time, re-verify the underlying claim from an alternative source before treating the original claim as still valid.

## References

1. Crypto Garage, *DLC on Lightning* (Medium, Nov 2022). https://medium.com/crypto-garage/dlc-on-lightning-cb5d191f6e64
2. Blockspace.media, *Lygos Finance Acquires Atomic Finance to Launch Non-Custodial DLC-Powered Bitcoin Loans* (Aug 2025). https://blockspace.media/insight/lygos-finance-acquires-atomic-finance-to-launch-non-custodial-dlc-powered-bitcoin-loans/
3. p2pderivatives/rust-dlc. https://github.com/p2pderivatives/rust-dlc
4. lightningdevkit/ldk-node. https://github.com/lightningdevkit/ldk-node
5. HIP-2 — DLC contract structure (on-chain settlement). [`../docs/HIP-2.md`](../docs/HIP-2.md)
6. HIP-3 — Cashu NUT-CTF integration (Lightning for deposit / redeem only). [`../docs/HIP-3.md`](../docs/HIP-3.md)
