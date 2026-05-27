# Hunch — Claude Code Project Guide

## What This Project Is

Hunch is a **permissionless, cypherpunk prediction market protocol on Bitcoin**. The full vision and current state live in:

- `.planning/PROJECT.md` — project context, requirements, decisions
- `.planning/REQUIREMENTS.md` — v1 requirements with REQ-IDs and traceability
- `.planning/ROADMAP.md` — 4-phase roadmap
- `.planning/research/SUMMARY.md` — executive summary of domain research
- `.planning/STATE.md` — current progress and active phase
- `docs/HIP-*.md` — protocol specifications (to be written in Phase 1)

**Read these before acting** on any non-trivial task.

## Stack at a Glance

- **Backend services (Rust workspace):** `hunch-protocol` (shared types), `hunch-mint` (CDK + NUT-DLC + LDK Node + DLC), `hunch-oracle` (NIP-88 + FROST), `hunch-relay` (nostr-rs-relay), `hunch-matcher` (Tier 2 P2P)
- **Frontend (TypeScript / Next.js):** `apps/hunch-web` — Next.js 15 + Tailwind + shadcn/ui + NDK + cashu-ts + WebLN
- **Build tools:** Cargo workspace + Bun
- **Distribution:** Cloudflare Pages + Tor hidden service + IPFS pin + GitHub + Radicle

See `.planning/research/STACK.md` for full library versions and rationale.

## GSD Workflow

This project uses [Get Shit Done (GSD)](https://github.com/) workflow:

- Use `/gsd-discuss-phase N` before planning a phase to surface ambiguities
- Use `/gsd-plan-phase N` to create the executable plan for phase N
- Use `/gsd-execute-phase N` to run a phase with atomic commits
- Use `/gsd-progress` to check status anytime
- Mode: **YOLO** (auto-approve, commit, then review)
- Granularity: **Coarse** (3-5 broad phases, 1-3 plans each)
- Model profile: **Quality (Opus)** for planning agents
- All quality agents enabled: Research, Plan Check, Verifier, Nyquist Validation

## Cypherpunk Principles (Non-Negotiable)

Apply these in every implementation decision:

1. **Trust the math.** Never introduce a centralized trust point unless absolutely necessary; document it loudly if you must.
2. **No KYC.** Ever. No email, no phone, no analytics linking to identity. Nostr pubkey is the only identifier.
3. **No US.** Geo-block on the official frontend. No US-targeted features.
4. **Open source MIT.** Every line must be readable and forkable. No proprietary closed-source dependencies on critical paths.
5. **Protocol-first.** HIPs (Hunch Improvement Proposals) define the protocol. Code is one implementation; encourage others.
6. **Multi-frontend / multi-mint / multi-oracle.** Never hard-code a single Hunch instance as canonical. Anyone should be able to host their own.
7. **No tokens.** No governance token, no utility token, no incentive token. Bitcoin is the token.
8. **Tor + IPFS first.** Hidden service from day 1; clearnet is a convenience.

## Engineering Principles for This Repo

- **Rust services are critical infrastructure.** Treat them like Bitcoin Core: small changes, big reviews, no clever shortcuts.
- **No custom crypto.** Use `frost-secp256k1-tr`, `secp256k1`, `bdk`, `rust-dlc`, `cdk` as-is. Never roll your own nonces, blinding factors, or signing flows.
- **NUT-DLC is alpha.** Pin versions. Test on signet. Don't ship to mainnet without external audit signoff.
- **Mainnet hardcore is a goal, not a starting state.** Use tiered caps in launch phase (week 1-4: 100k sat cap; month 2-3: 1M sat cap; then uncap) despite the "no caps" target.
- **Logging never leaks user identity.** Cashu blind sigs are useless if logs deanonymize.
- **Reserves proofs published weekly.** Mint operator transparency is non-optional.
- **Settlement is verifiable on-chain.** Every market detail page shows the settlement tx + oracle Schnorr sig.

## Working with Code in This Repo

### Don't

- Don't add Polygon, Solana, USDC, or any non-Bitcoin/non-Cashu payment rail.
- Don't add KYC fields (even "optional" email).
- Don't introduce custodial flows where the operator can rug.
- Don't hide oracle reputation behind clicks — it must be visible at bet time.
- Don't add a centralized matcher engine. Mint orderbook (Tier 1) or P2P Nostr (Tier 2) are the only options.
- Don't write to GitHub Issues only — also mirror to Radicle and Nostr.
- Don't pin a single Cloudflare/Hetzner/Vercel as canonical host.
- Don't add analytics (no GA, no PostHog, no fingerprinting).
- Don't use `git commit --no-verify` to bypass pre-commit hooks unless the user explicitly authorizes.

### Do

- Do verify oracle Schnorr signatures before trusting any attestation.
- Do verify Cashu DLEQ proofs on every token receive (NUT-12 mandatory).
- Do use NIP-07 / NIP-46 for all auth flows.
- Do test new features on Mutinynet before mainnet.
- Do publish HIPs as Markdown + Nostr long-form (NIP-23).
- Do recommend multi-relay setups (NIP-65 outbox model).
- Do include refund timeout fallback in every DLC contract.
- Do use `frost-secp256k1-tr` for Taproot-compatible threshold Schnorr.
- Do mirror commits to GitHub + Radicle.

## Quick Commands

- `/gsd-progress` — Where are we? What's next?
- `/gsd-discuss-phase 1` — Discuss Phase 1 context before planning
- `/gsd-plan-phase 1` — Plan Phase 1
- `/gsd-execute-phase 1` — Execute Phase 1

## Key Reference Documents

| Document | Purpose |
|----------|---------|
| `.planning/PROJECT.md` | Project vision, constraints, key decisions |
| `.planning/REQUIREMENTS.md` | All v1 requirements with REQ-IDs |
| `.planning/ROADMAP.md` | 4-phase plan |
| `.planning/STATE.md` | Current progress |
| `.planning/config.json` | GSD workflow config |
| `.planning/research/STACK.md` | Recommended technologies |
| `.planning/research/FEATURES.md` | Feature landscape |
| `.planning/research/ARCHITECTURE.md` | System design |
| `.planning/research/PITFALLS.md` | Things to avoid |
| `.planning/research/SUMMARY.md` | Research synthesis |
| `docs/HIP-*.md` | Protocol specifications (Phase 1 deliverable) |
| `docs/MANIFESTO.md` | Cypherpunk manifesto (Phase 1 deliverable) |

---
*Hunch — Trust the math.*
