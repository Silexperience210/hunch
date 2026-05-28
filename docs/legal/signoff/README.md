# Counsel Sign-Off PDF Storage Convention

**Purpose:** Counsel-signed PDFs of legal strawmen (TERMS, PRIVACY, PR_PLAYBOOK, PSEUDONYMITY) land here as they are received from counsel. Per CONTEXT.md decision D-05, these are PRIVATE and must not be committed to the public repo.

## Storage Options (user chooses)

Three storage options per Plan 04 user_setup:

### Option A — Private branch in main repo + age encryption

1. Encrypt the PDF with `age` keyed to the maintainer's pseudonym public key
2. Commit the encrypted PDF to a private branch (e.g., `legal-signoffs`)
3. The private branch is never pushed to the public mirror
4. Decryption key is stored in the maintainer's offline-backup

Pro: Single repo for all artifacts. Con: requires careful git-config to avoid accidental push.

### Option B — Separate private GitHub repo

1. Create a private repo `Silexperience210/hunch-legal-signoffs` (private)
2. PDFs committed unencrypted (the repo's privacy is the protection)
3. Cross-reference from this file by branch/commit hash only

Pro: Clean separation. Con: extra repo to manage; GitHub has visibility into content.

### Option C — Counsel-only retention + SHA-256 hash commitments in public repo

1. Counsel retains the signed PDF
2. Maintainer receives a SHA-256 hash of the signed PDF + a Bitcoin / Lightning transaction timestamping the hash
3. The hash is committed to this directory (publicly) as `<doc>.sha256.txt`
4. The PDF itself is stored only by counsel

Pro: No private PDF exposure at all. Con: requires counsel cooperation; recovery depends on counsel availability.

## Recommended Option

**Option B** is the recommended default — clean separation, low ceremony. Option A is acceptable for solo-maintainer setups. Option C is the most cypherpunk-strict but operationally riskiest.

User picks based on opsec preference + counsel cooperation feasibility.

## What Goes Here

This directory is for:

- `TERMS-v1.0-counsel-signed.pdf.gpg` (or equivalent encrypted form per chosen storage option)
- `PRIVACY-v1.0-counsel-signed.pdf.gpg`
- `PR_PLAYBOOK-v1.0-counsel-signed.pdf.gpg`
- `PSEUDONYMITY-v1.0-counsel-acknowledged.pdf.gpg` (optional)
- `engagement-letter-counsel-firm-name.pdf.gpg`
- `foundation-incorporation-papers.pdf.gpg` (after entity formation)
- `operating-entity-incorporation-papers.pdf.gpg` (after entity formation)
- `bank-account-confirmation.pdf.gpg` (after banking introductions)

## What Does NOT Go Here

- The strawmen themselves (those are public, in the parent `docs/legal/` directory)
- The maintainer's real-name documents (those never enter the repo at all)
- Any document containing real-identity disclosure

## Verifying Storage

When a signed PDF lands:

1. Confirm the document is signed by counsel (PGP signature or X.509 cert).
2. Encrypt or commit-private per chosen storage option.
3. Record the SHA-256 in `PHASE-1-FOLLOWUP.md` adjacent to the corresponding F-row.
4. Commit (publicly) the closure of the corresponding F-row in PHASE-1-FOLLOWUP.md.

## References

- `../PHASE-1-FOLLOWUP.md` — Tracker for sign-off PDF status
- `../engagement-letter-status.md` — Engagement state model
- CONTEXT.md D-05 — Full pseudonymity scope (drives the private-storage requirement)
