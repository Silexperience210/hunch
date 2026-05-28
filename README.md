# Hunch

> **Permissionless prediction markets on Bitcoin. No KYC. No custody. Trust the math.**

Hunch is a Bitcoin-native, cypherpunk prediction market protocol. Anyone can create a market about any verifiable question — politics, sport, culture, crypto — and bet on the outcome using Lightning. Settlement uses Discreet Log Contracts (DLCs); liquidity flows through competing Cashu mints (NUT-DLC); market discovery and oracles publish on Nostr.

The protocol is neutral. Hunch operates a reference frontend, mint, and oracle, but the protocol survives any single operator.

## Status

🚧 **Phase 1 — Cypherpunk Foundation** — specifications + spikes + legal structure.

Hunch is **pre-mainnet**. Mainnet launch is planned after:

1. External security audit signed off
2. 2-3 months of Mutinynet testing
3. Bug bounty program live
4. Tiered launch (invite → public-with-caps → no-caps)

## The Stack

| Layer | Technology |
|---|---|
| Settlement | Bitcoin DLC ([rust-dlc](https://github.com/p2pderivatives/rust-dlc) / [DDK](https://github.com/bennyhodl/dlcdevkit)) |
| Liquidity | Cashu mint with NUT-DLC ([CDK](https://github.com/cashubtc/cdk)) |
| Lightning | [LDK Node](https://github.com/lightningdevkit/ldk-node) |
| Discovery + oracles + reputation | Nostr ([nostr-sdk](https://github.com/rust-nostr/nostr), NDK) |
| Threshold sigs | [frost-secp256k1-tr](https://crates.io/crates/frost-secp256k1-tr) |
| Frontend | Next.js 15 (static export), Tailwind, shadcn/ui |
| Distribution | Cloudflare Pages + Tor hidden service + IPFS + Radicle |

## Repo Layout

```
hunch/
├── crates/
│   ├── hunch-protocol/    # Shared types + Nostr event schemas + DLC builders
│   ├── hunch-mint/        # Cashu mint + NUT-DLC + LDK Node + DLC backing
│   ├── hunch-oracle/      # NIP-88 publisher + FROST k-of-n coordinator
│   ├── hunch-relay/       # nostr-rs-relay deployment
│   ├── hunch-matcher/     # Optional Tier 2 P2P matcher / indexer
│   └── hunch-cli/         # Operator CLI
├── apps/
│   └── hunch-web/         # Next.js reference frontend
├── docs/
│   ├── HIP-0.md           # Protocol overview & manifesto
│   ├── HIP-1.md           # Nostr event kinds
│   ├── HIP-2.md           # DLC contract structure
│   ├── HIP-3.md           # Cashu NUT-DLC for Hunch
│   ├── HIP-4.md           # Multi-oracle FROST
│   └── HIP-5.md           # Reputation event format
└── .planning/             # GSD workflow artifacts (project context, roadmap)
```

## Principles

1. **Trust the math.** Never introduce a centralized trust point unless absolutely necessary.
2. **No KYC.** Ever. Nostr pubkey is the only identifier.
3. **No US.** Geo-block on the official frontend (`hunch.io`). No US-targeted features.
4. **Open source MIT.** Forks encouraged. Mirrors on GitHub and Radicle.
5. **Protocol-first.** HIPs define the protocol. Code is one implementation; encourage others.
6. **Multi-frontend / multi-mint / multi-oracle.** Anyone should be able to host their own.
7. **No tokens.** No governance token, no utility token. Bitcoin is the token.
8. **Tor + IPFS first.** Hidden service from day 1; clearnet is a convenience.

## Mirrors

Hunch is mirrored across multiple hosts to resist deplatforming (cypherpunk principle 8 — Tor + IPFS first):

- **GitHub (primary):** https://github.com/Silexperience210/hunch — LIVE (published 2026-05-28)
- **Radicle (p2p):** PENDING — install `rad` CLI, then `rad init --name hunch --description "Hunch protocol" --default-branch main --public && git push rad main`. Replace this line with the resulting `rad:z…` ID once initialized.
- **Codeberg (fallback):** PENDING — sign up at codeberg.org, create repo `Silex/hunch`, then `git remote add codeberg git@codeberg.org:Silex/hunch.git && git push codeberg main`
- **Tor hidden service:** Phase 2 deliverable
- **IPFS pin:** Phase 2 deliverable

All mirrors track `main`. Reproducibility check (once mirrors are live):
```bash
git fetch --all && git log --oneline -5 origin/main rad/main codeberg/main
```
should show identical commit hashes across all three.

## Contributing

Open source, contributions welcome. See [CONTRIBUTING.md](./CONTRIBUTING.md) and [CODE_OF_CONDUCT.md](./CODE_OF_CONDUCT.md) for guidelines. Pseudonymous contributions are encouraged; real-name disclosure is never required.

The project follows the [Get Shit Done (GSD)](https://github.com/) workflow for planning and execution. See `.planning/` for the active milestone, roadmap, and requirements.

## License

[MIT](./LICENSE) — see LICENSE file.

---

*Hunch — Trust the math.*
