// Live cross-language round-trip: the Rust `hunch` CLI publishes a reputation (kind 30891) and a
// dispute (kind 30890) to a relay; the browser read code (fetchReputation / fetchDisputes) pulls
// them back, Schnorr-verifies, and parses. A pass proves the CLI writer and the web reader agree
// on the wire format byte-for-byte.
//
//   HUNCH_RELAY=ws://127.0.0.1:8099 HUNCH_TARGET=<oracle hex> HUNCH_MARKET=<id> \
//   node --experimental-strip-types scripts/cli-roundtrip-e2e.ts

import { fetchReputation } from "../lib/oracle.ts";
import { fetchDisputes } from "../lib/disputes.ts";
import { aggregateReputation } from "../lib/hunch.ts";

const relay = process.env.HUNCH_RELAY ?? "ws://127.0.0.1:8099";
const target = process.env.HUNCH_TARGET ?? "";
const market = process.env.HUNCH_MARKET ?? "";

function fail(msg: string): never {
  console.error("FAIL:", msg);
  process.exit(1);
}
if (!target || !market) fail("set HUNCH_TARGET and HUNCH_MARKET");

const relays = [relay];

const reps = await fetchReputation(relays, target);
if (reps.length === 0) fail("no reputation claim found for target");
const summary = aggregateReputation(reps);
if (!summary) fail("reputation did not aggregate");
console.log(`reputation OK: avg=${summary.avg} count=${summary.count} (claim score=${reps[0].score}, scope=${reps[0].scope})`);
if (reps[0].scope !== "oracle") fail(`unexpected scope ${reps[0].scope}`);

const disputes = await fetchDisputes(relays, market);
if (disputes.length === 0) fail("no dispute found for market");
const d = disputes[0];
if (d.market !== market) fail(`dispute market mismatch: ${d.market}`);
console.log(`dispute OK: claim=${d.claim} attestation=${d.attestation.slice(0, 12)}… disputer=${d.disputer.slice(0, 8)}…`);

console.log("CLI-ROUNDTRIP E2E OK");
process.exit(0);
