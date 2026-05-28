# Hunch Manifesto

**Trust the math.**

A prediction market is a financial instrument, but it is also an epistemic instrument. It rewards correctness and punishes confidence-without-evidence. When markets are run honestly, they aggregate dispersed knowledge into prices that beat polls, pundits, and committees at forecasting the world.

But the honest part is hard. Honest markets require: settlement that no operator can reverse, custody that no operator can rug, identity that no operator can leak, and access that no operator can revoke. Existing markets fail at one or more of these. Centralized exchanges custody funds and demand identity. Blockchain protocols entangle settlement with native governance tokens whose holders can re-vote the rules. In both shapes, the market is only as honest as the operator behind it.

Hunch is built so the operator cannot be the variable.

Settlement happens on Bitcoin, not on a database. Discreet Log Contracts mean the operator cannot move funds outside the contracted outcomes. The contract is signed once, the oracle signs the outcome, the math executes. If the operator disappears, the contract still resolves.

Custody is bounded by the math. Cashu mints hold reserves for the duration of a market, not as a permanent treasury. The mint never knows which Nostr public key is betting; it sees only blinded signatures. When the market ends, the mint settles via DLC and the tokens flow back to bettors. No long-term custodianship, no honeypot, no operator-controlled deposit.

Identity is optional. Nostr public keys are the only identifier the protocol requires. Bettors can rotate keys per market if they want; mints cannot link a deposit to a withdrawal across markets; oracles publish under their own pseudonyms. The protocol does not collect names, emails, phone numbers, or biometrics. Anyone telling you they need any of these to bet is lying about what is required.

Access is not the operator's to revoke. Frontends are interchangeable. Mints are interchangeable. Oracles are interchangeable. If a host disappears under pressure, a fork stands up in its place. The reference frontend is one of many; the IPFS pin and Tor hidden service exist so no DNS registrar can erase the project. A user who knows the protocol can bet directly, with no frontend at all.

Hunch is permissionless because permissioned markets are not free markets. Anyone with a Nostr key may post any market about any verifiable question — politics, sport, culture, technology, weather, science. The protocol does not censor. Frontends and indexers may curate; that is their job. The protocol stays neutral.

Hunch is tokenless because tokens are a backdoor to governance capture. There is no Hunch token, no governance vote, no founder allocation. Upgrades happen through HIPs and through independent implementations choosing to adopt them. The unit of account is Bitcoin. Speculators cannot govern what they do not own.

Hunch is open source under MIT, forever. Every line is forkable. Contributor copyright is not collected. The license never changes.

Hunch is pseudonymous by default. Contributors operate under pseudonyms. Maintainers are pseudonymous. The community norm is that doxxing — of maintainers or of any contributor — is grounds for permanent ban. We do not collect, share, or acknowledge real-name disclosures.

These are not features that can be added later. They are the architecture. If a future fork removes them, that fork is no longer Hunch.

**Trust the math.**

— Silex
*Hunch protocol maintainer*
*2026-05-28*
