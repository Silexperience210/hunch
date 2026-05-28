# NUT-CTF Maintainer Outreach Log

**Spike:** SPIKE-01
**Updated:** 2026-05-28 (initial — outreach not yet sent)

## Outreach Records

| # | Target            | Channel       | Date sent | Channel ID / event   | Response          | Response date | Outcome |
|---|-------------------|---------------|-----------|----------------------|-------------------|---------------|---------|
| 1 | joemphilips       | GitHub PR #337| PENDING   | —                    | none yet          | —             | —       |
| 2 | conduition        | Nostr DM      | PENDING   | —                    | none yet          | —             | —       |
| 3 | Calle             | Nostr DM      | PENDING   | —                    | none yet          | —             | —       |

## How to Send

1. **Generate HUNCH_NSEC** (Plan 02 Task 2 deliverable) — required for Nostr DMs under project pseudonym
2. **Get GitHub repo URL live** — done (Plan 01); https://github.com/Silexperience210/hunch returns 200
3. **Look up target npubs**:
   - conduition: GitHub profile or cashubtc Telegram
   - Calle: https://cashubtc.org/ author block, or cashu.me social channels
4. **Send via Nostr client** (Damus / Amethyst / nostrudel) using HUNCH_NSEC
5. **For GitHub PR #337 comment**: comment as `Silexperience210` (the GitHub account already authenticated and is the repo owner)
6. **Log here**: update the table above with date + channel ID (Nostr event ID or GitHub comment URL)

## Next Steps

After 3-week wait window (target 2026-06-18):

1. If ≥1 maintainer confirms #337 architecture → mark SPIKE-01 VALIDATED in PATH-A-VALIDATION.md
2. If no responses → mark INCONCLUSIVE, proceed with SPIKE-02 against #337 HEAD with explicit unilateral-assumption note in HIP-3
3. If a maintainer signals re-scope → trigger CONTEXT.md update + HIP-3 corrigendum
