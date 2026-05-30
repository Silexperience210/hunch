// Live test for fetchAttestation/fetchAnnounce against a REAL oracle event on a relay.
//
// Drive it after the hunch-oracle daemon has published a kind:89 attestation:
//   HUNCH_RELAY=ws://127.0.0.1:8099 \
//   HUNCH_ORACLE_PUB=<x-only hex> \
//   HUNCH_MARKET=<creator:30888:slug> \
//   HUNCH_OUTCOME=YES \
//   node --experimental-strip-types scripts/fetch-attestation-e2e.ts
//
// It exercises the exact browser code path (relay query → Schnorr verify → parse), so a pass
// proves the bet UI can pull the settlement signature with no hand-pasted hex.

import { fetchAnnounce, fetchAttestation } from "../lib/oracle.ts";
import { outcomeUnlockSecret } from "../lib/dlc.ts";

const relay = process.env.HUNCH_RELAY ?? "ws://127.0.0.1:8099";
const oracle = process.env.HUNCH_ORACLE_PUB ?? "";
const market = process.env.HUNCH_MARKET ?? "";
const expectedOutcome = process.env.HUNCH_OUTCOME ?? "YES";

function fail(msg: string): never {
  console.error("FAIL:", msg);
  process.exit(1);
}

if (!oracle || !market) fail("set HUNCH_ORACLE_PUB and HUNCH_MARKET");

const relays = [relay];
console.log(`querying ${relay} for oracle ${oracle.slice(0, 16)}… market ${market}`);

// kind:88 announce carries the nonce R (optional — only if the oracle announced).
const announce = await fetchAnnounce(relays, oracle, market);
if (announce) {
  console.log(`announce OK: nonce R = ${announce.nonce.slice(0, 16)}…`);
} else {
  console.log("announce: none found (the daemon may have attested without announcing) — continuing");
}

// kind:89 attestation — the thing under test.
const att = await fetchAttestation(relays, oracle, market);
if (!att) fail("fetchAttestation returned null — no verified kind:89 event found");

if (att.market !== market) fail(`market mismatch: ${att.market}`);
if (att.outcome !== expectedOutcome) fail(`outcome ${att.outcome}, expected ${expectedOutcome}`);
if (!/^[0-9a-f]{128}$/.test(att.signature)) fail(`bad signature: ${att.signature}`);

// The fetched signature must drive the spend-key derivation the wallet relies on.
const bettorSecret = "11".repeat(32);
const unlock = outcomeUnlockSecret(bettorSecret, att.signature);
if (!/^[0-9a-f]{64}$/.test(unlock)) fail(`outcomeUnlockSecret produced bad key: ${unlock}`);

console.log(`attestation OK: outcome=${att.outcome} sig=${att.signature.slice(0, 16)}…`);
console.log(`spend-key derivation OK: l_X = ${unlock.slice(0, 16)}…`);
console.log("FETCH-ATTESTATION E2E OK");
process.exit(0);
