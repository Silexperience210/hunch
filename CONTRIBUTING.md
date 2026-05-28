# Contributing to Hunch

Hunch is open source under the MIT license. Contributions are welcome.

> **Status**: Updated Phase 1 (Cypherpunk Foundation). Pseudonymity discipline is non-negotiable; see below.

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

## Pseudonymity

Hunch is operated under pseudonym. Contributors are **encouraged to operate under pseudonym** as well; real-name disclosure is **never required** and never collected by the project.

All contributor attribution uses Nostr `npub` or pseudonym; do **not** submit PRs with real-name commit authorship. Before pushing, set your per-repo git config:

```bash
git config user.name  "<pseudonym>"
git config user.email "<pseudonym>@hunch.markets"
```

Do **not** edit your global `~/.gitconfig` — set the values per-repo so they apply only to this checkout.

Commits with real-name authorship will be rejected on the public repo.

## Development Setup

### Rust services

```bash
# Workspace at root
cargo build --workspace
cargo test --workspace
cargo clippy --workspace
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

Additional rules:
- Sign all commits under the project pseudonym GPG key (`Silex_0xF777C5B8`). Real-name commits will be rejected on the public repo.
- Keep commits atomic. Each commit should be reviewable in isolation.
- Reference the relevant HIP or REQ-ID when applicable (e.g., `feat(mint): add NUT-CTF redeem path (HIP-3)`).

## Pull Requests

1. Branch from `main` to a feature branch
2. Push your branch
3. Open a PR with a clear description
4. Link to the relevant requirement or HIP
5. Ensure CI passes
6. Address review feedback

## Security

**Do not file security issues publicly.** Report security findings via:

- **Nostr DM** to `npub-TBD` (see SECURITY.md, Phase 2 deliverable), or
- **PGP-encrypted email** to `<pseudonym>@protonmail.com` (TBD — published in SECURITY.md, Phase 2)

The project PGP key is `Silex_0xF777C5B8`; verify the fingerprint via multiple channels before sending sensitive information.

## Communication

- **GitHub Issues** — mirror for triage; canonical issue tracker rotates per-platform
- **Radicle mirror** — TBD, Phase 1 deliverable; non-platform-controlled issue tracking
- **Codeberg mirror** — TBD, Phase 1 deliverable; EU non-Microsoft fallback
- **Nostr** — `npub-TBD` (Phase 2 deliverable, published in SECURITY.md / README.md)

---

*Cypherpunks write code.*
