#!/usr/bin/env bash
# verify-repo.sh — confirm cypherpunk repo hygiene
# Checks: LICENSE (MIT), CONTRIBUTING.md, CODE_OF_CONDUCT.md, CLAUDE.md, README.md, Radicle remote
# Exit 0 if all pass; exit 1 with diagnostic on first failure.
set -euo pipefail

cd "$(git rev-parse --show-toplevel)"

fail() { echo "FAIL: $1" >&2; exit 1; }

# 1. LICENSE present, MIT
test -f LICENSE || fail "LICENSE missing"
grep -q "MIT License" LICENSE || fail "LICENSE not MIT"

# 2. CONTRIBUTING.md present with pseudonymity guidance
test -f CONTRIBUTING.md || fail "CONTRIBUTING.md missing"
grep -qi "pseudonym" CONTRIBUTING.md || fail "CONTRIBUTING.md lacks pseudonymity guidance"

# 3. CODE_OF_CONDUCT.md present with Pseudonymity section
test -f CODE_OF_CONDUCT.md || fail "CODE_OF_CONDUCT.md missing"
grep -qi "Pseudonymity" CODE_OF_CONDUCT.md || fail "CODE_OF_CONDUCT.md lacks Pseudonymity section"

# 4. CLAUDE.md present (PROTO-08)
test -f CLAUDE.md || fail "CLAUDE.md missing (PROTO-08)"
grep -q "GSD" CLAUDE.md || fail "CLAUDE.md does not reference GSD workflow"

# 5. README.md present with Mirrors section (PROTO-07)
test -f README.md || fail "README.md missing"
grep -q "## Mirrors" README.md || fail "README.md lacks ## Mirrors section"

# 6. Pseudonym GPG key NOT in tracked files (gitignore must protect it)
if git ls-files | grep -qi "SECRET"; then
  fail "SECRET file is tracked by git — gitignore must protect it"
fi

# 7. Radicle remote (best-effort; warn-only if rad not installed)
if command -v rad >/dev/null 2>&1; then
  if ! git remote -v | grep -q rad; then
    echo "WARN: rad CLI installed but no 'rad' git remote configured" >&2
  fi
fi

echo "PASS: verify-repo.sh — all cypherpunk repo checks green"
