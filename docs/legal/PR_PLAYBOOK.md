# Hunch PR Response Playbook

```
Status:        STRAWMAN — COUNSEL SIGN-OFF PENDING
Last updated:  2026-05-28
Version:       draft-0.1
Audience:      Hunch maintainer (Silex) + foundation board + counsel
Distribution:  private until counsel-reviewed; public version derived later
```

> **NOT YET FINALIZED.** This document is a counsel-input strawman. Final version requires LEGAL-05 counsel sign-off (tracked in `PHASE-1-FOLLOWUP.md`). The playbook is intended for INTERNAL use; a public-facing summary may be derived after counsel review.

## Purpose

When PR / regulatory / law-enforcement / market-abuse events arise, the playbook gives the maintainer a pre-baked response framework so decisions are made with cool-head preparation rather than under acute pressure. Each scenario below specifies: trigger, immediate action, communications template, escalation path, and what NOT to do.

The playbook is intentionally written for the cypherpunk-maintainer posture: pseudonymous, no-custody, no-token, multi-implementation. It reflects the locked decisions in CONTEXT.md and the principles in CLAUDE.md.

## Master Principles (apply across all scenarios)

1. **Read the engagement letter first.** Counsel is the primary advisor; the playbook is preparation, not substitute. Before any external communication beyond a holding response, consult counsel.
2. **No improvisation in public.** Hold messages are pre-drafted. New messages go through counsel. Frustration is not a strategy.
3. **Pseudonymity stays intact.** No statements that link the project pseudonym to real identity, ever. Even under pressure.
4. **Protocol-vs-frontend distinction is the most important sentence.** Hunch the protocol is permissionless and decentralized; Hunch Operations Ltd. is the reference operator. Inquiries should be directed at the appropriate layer.
5. **No statements about other operators.** Hunch only speaks for the reference deployment. Other frontends, mints, oracles speak for themselves.
6. **Document everything.** Every inquiry, every response, every escalation gets a timestamped record in the engagement-letter-status follow-up tracker.

## Scenario A — Market Curation Inquiry (Frontend-Level Filter)

**Trigger:** A user, journalist, regulator, or other party complains that a market on the reference frontend is harmful, illegal, or misleading.

**Immediate action:**

1. Triage the market against the frontend curation policy (see TERMS §5).
2. If the market clearly violates curation policy: filter it from the frontend display. Note: this does NOT remove the market from the underlying Nostr protocol; it only removes it from this specific frontend.
3. Respond to the inquirer within 24 hours using the hold message below.

**Hold message:**

> Thank you for your inquiry. The Hunch protocol is permissionless: any user with a Nostr keypair may publish a market about any verifiable question, and the protocol cannot censor markets at its base layer. The reference frontend operated by Hunch Operations Ltd. applies a curation policy that filters certain market categories from display (see Terms §5); this filter has been applied to the market you flagged. Other frontends operated by third parties may make different curation choices; we cannot control those. We do not facilitate removal of markets from the underlying Nostr protocol.

**Escalation:** None unless the inquirer escalates to legal action. If they do: counsel handles.

**Don't:** Apologize. Promise removal "from the protocol." Discuss other markets. Speculate on motive.

## Scenario B — Government / Law Enforcement Subpoena

**Trigger:** A subpoena, civil-investigative demand, MLAT request, or equivalent arrives at Hunch Operations Ltd. (the BVI operating entity) or at counsel.

**Immediate action:**

1. **Do not respond directly.** Counsel handles all official correspondence.
2. **Do not destroy or alter logs** between receipt of the subpoena and counsel's instructions.
3. **Do not discuss publicly** until counsel says it is safe to do so (typically after the subpoena period or after a public-disclosure waiver).
4. **Document the receipt** in the private engagement-tracker (timestamp, jurisdiction, scope).
5. **Notify the foundation board** (if/when one exists) via the secure channel.

**Hold message (only if forced to acknowledge the existence of an inquiry, never the substance):**

> Hunch Operations Ltd. cooperates with lawful inquiries through counsel within the framework of the laws applicable to the operating entity's jurisdiction. We do not custody user funds, do not collect user identity, and do not retain logs beyond standard operational windows. Specifics of any active matter are not for public discussion.

**Don't:**
- Acknowledge or deny specific subpoenas in public.
- Speculate on the subpoena's purpose.
- Make statements suggesting the maintainer fears, defies, or is willing to cooperate beyond legal obligation.
- Take any action (e.g., shutting down servers) without counsel sign-off.

## Scenario C — Regulatory Inquiry (Pre-Enforcement)

**Trigger:** A regulator (e.g., a national securities regulator, a derivatives regulator, an AML supervisor) sends a non-formal inquiry letter requesting information or a meeting.

**Immediate action:**

1. **Forward to counsel within 24 hours.** Do not respond directly.
2. **Document the inquiry** as above.
3. **Continue normal operations** unless counsel advises otherwise.

**Hold response (after counsel review only):**

> Hunch Operations Ltd. is open to engaging with regulators within the framework of the laws applicable to our operating jurisdiction. We are an open-source protocol stewardship entity; we do not custody user funds, issue tokens, or operate as a financial intermediary. We will respond to your specific inquiries through counsel. Please direct further correspondence to [counsel email + PGP fingerprint].

**Don't:**
- Volunteer information.
- Make commitments about future operations.
- Engage in regulatory dialogue without counsel.
- Issue public statements until counsel approves.

## Scenario D — Maintainer Indictment / Criminal Charges

**Trigger:** A maintainer is indicted, arrested, or formally charged in any jurisdiction.

**Immediate action (catastrophic-event procedure):**

1. **Counsel handles all communications.** Do not respond to media inquiries.
2. **Activate the maintainer-succession plan** documented privately in `PSEUDONYMITY.md` §Continuity. Independent contributors may continue protocol work; the indicted maintainer steps back from public-facing roles.
3. **Do not modify the reference deployment** under duress. Any changes require counsel sign-off + foundation board approval.
4. **Public statement (counsel-drafted only):**

> Hunch Operations Ltd. is aware of the legal action involving an individual associated with the project. We are cooperating with counsel and the appropriate authorities. The Hunch protocol itself is open-source, permissionless, and operates independently of any single individual. Reference deployments continue per their established policies.

**Don't:**
- Speculate on the merits of the charges.
- Make statements that could be construed as obstruction.
- Continue operating without counsel involvement.
- Issue calls to action ("rally support") that could be construed as inciting interference.

## Scenario E — Major Mint or Oracle Failure

**Trigger:** The reference mint stops settlement, an oracle attests incorrectly, or a major operator experiences a security incident.

**Immediate action:**

1. **Communicate clearly within 4 hours via Nostr** under the project npub: what happened, what's affected, what the recovery path is.
2. **Activate the refund-timeout branch** (HIP-2) if relevant: communicate to bettors that refund-claim is available via the DLC's refund branch even if the mint cannot continue operating.
3. **Engage post-mortem within 7 days** with full transparency.

**Public message template (Nostr):**

> [INCIDENT] Hunch reference mint experienced [X] at [timestamp]. Affected markets: [list]. Recovery path: [refund-timeout branch / alternative mint / counsel-led response]. Status updates here. — Hunch maintainer

**Don't:**
- Hide the incident.
- Promise recovery that the mint cannot deliver.
- Blame other operators.
- Make security claims that audit has not validated.

## Scenario F — Deplatforming (DNS / Host / App Store)

**Trigger:** A DNS registrar, hosting provider, CDN, or app store removes Hunch infrastructure.

**Immediate action:**

1. **Activate mirror infrastructure** per CLAUDE.md §Multi-host:
   - Tor hidden service URL
   - IPFS pin (web3.storage or equivalent)
   - Codeberg / Radicle / alternative source mirrors
2. **Communicate the new access points within 6 hours** via Nostr + cached emergency-contact channels.
3. **Engage counsel** for any deplatforming action that appears to be legally motivated (rather than ToS-violation).

**Public message template:**

> [MIRROR UPDATE] The [host/registrar] removed Hunch's [resource]. Hunch protocol and reference frontend remain accessible at: [Tor hidden service URL], [IPFS gateway URL], [alternative CDN URL]. All commits and HIPs are mirrored at: github.com/Silexperience210/hunch, codeberg.org/Silex/hunch, [Radicle ID]. Protocol design ensures continuity beyond any single operator. — Hunch maintainer

**Don't:**
- Beg the host to reverse the decision.
- Speculate on the host's motivation.
- Issue calls to action ("boycott X") that could be construed as inciting harassment.

## Scenario G — Maintainer Doxxing (Pseudonym Compromise)

**Trigger:** The maintainer's pseudonym is publicly linked to their real identity by a third party.

**Immediate action:**

1. **Do not confirm or deny.** Per CONTEXT.md decision D-05, the project does not engage with doxxing; that includes engaging to deny linkage.
2. **Continue normal operations** under the project pseudonym.
3. **Activate community-norm enforcement:** doxxing of any contributor (including maintainers) is grounds for permanent ban in community spaces (CODE_OF_CONDUCT.md §Pseudonymity). Ban the doxxer from project channels.
4. **Engage counsel** if the doxxing appears coordinated, escalating, or accompanied by harassment / threats.
5. **Public message (counsel-reviewed only, optional — silence is often best):**

> The Hunch project operates under pseudonym as a matter of principle, not as a matter of public guessing-games. We do not engage with attempted linkage of the project pseudonym to other identities. Public-facing project operations continue normally. Community-norm enforcement against doxxing applies per Code of Conduct §Pseudonymity.

**Don't:**
- Confirm or deny linkage.
- Engage with the doxxer.
- Modify project pseudonym in response (that legitimizes the doxxing as effective).
- Make personal statements (in any direction) about identity.

## Update Procedure

This playbook is updated as scenarios evolve and after each real-world event. Each update records:
- Date
- Scenario(s) affected
- Reason for update
- Counsel sign-off (PDF artifact in `signoff/`)

## References

- CONTEXT.md D-05 — Full pseudonymity scope
- CONTEXT.md D-04 — Jurisdiction deferred to counsel
- TERMS.md §5 — Frontend curation policy
- PSEUDONYMITY.md — Maintainer pseudonymity operational plan
- engagement-letter-status.md — Counsel engagement state tracker
- RESEARCH.md §6 — Legal Foundation deep-dive (informs scenarios B, C, D)
