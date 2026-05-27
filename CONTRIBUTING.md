# Contributing to Hunch

Hunch is open source under the MIT license. Contributions are welcome.

> **Status**: This is a draft. The full contribution guide will be refined during Phase 1 (Cypherpunk Foundation).

## Code of Conduct

See [CODE_OF_CONDUCT.md](./CODE_OF_CONDUCT.md).

## How to Contribute

1. **Read the foundational documents first**:
   - [PROJECT.md](./.planning/PROJECT.md) — what Hunch is and why
   - [ROADMAP.md](./.planning/ROADMAP.md) — current phase and goals
   - [REQUIREMENTS.md](./.planning/REQUIREMENTS.md) — v1 requirements
   - [research/SUMMARY.md](./.planning/research/SUMMARY.md) — domain research synthesis
   - [docs/HIP-0.md](./docs/HIP-0.md) — protocol manifesto (Phase 1 deliverable)

2. **Pick an open issue** or propose a new one before starting non-trivial work.

3. **Follow the cypherpunk principles** documented in [CLAUDE.md](./CLAUDE.md).

4. **No US-targeted features.** Mirror the project's legal stance.

5. **Don't introduce closed-source dependencies** on critical paths.

## Development Setup

### Rust services

```bash
# Workspace at root
cargo build
cargo test
cargo clippy
```

### Frontend

```bash
cd apps/hunch-web
bun install
bun dev
```

### Local Bitcoin + Lightning

Use [Polar](https://lightningpolar.com) to spin up a local regtest cluster, or [Mutinynet](https://mutinynet.com) for a public signet with 30-second blocks.

## Commit Style

Follow Conventional Commits:
- `feat:` — new feature
- `fix:` — bug fix
- `docs:` — documentation
- `refactor:` — code restructuring
- `test:` — tests
- `chore:` — tooling, config
- `spec:` — HIP changes

Keep commits atomic. Each commit should be reviewable in isolation.

## Pull Requests

1. Branch from `main` to a feature branch
2. Push your branch
3. Open a PR with a clear description
4. Link to the relevant requirement or HIP
5. Ensure CI passes
6. Address review feedback

## Security

**Do not file security issues publicly.** Email security findings to [TBD — Phase 1 deliverable]. PGP key will be published in `SECURITY.md` (Phase 1 deliverable).

## Communication

- **GitHub Issues** — bugs, feature requests
- **Discord/Matrix** — TBD, link in `README.md`
- **Nostr** — `npub...` (TBD, Phase 1)
- **Radicle mirror** — TBD, Phase 1

---

*Cypherpunks write code.*
