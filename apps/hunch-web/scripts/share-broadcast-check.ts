// One-off: prove public relays accept a Hunch share note (kind:1). Throwaway key. Manual run:
//   node --experimental-strip-types scripts/share-broadcast-check.ts
import { randomBytes } from "node:crypto";
import { buildShareNote, BROADCAST_RELAYS } from "../lib/share.ts";
import { signEventWithKey } from "../lib/localSigner.ts";
import { publishAll } from "../lib/publish.ts";

const sk = randomBytes(32).toString("hex");
const note = signEventWithKey(
  buildShareNote({ question: "Hunch broadcast self-test — ignore", id: "test:30888:selftest", yes: 50 }),
  sk,
);
console.log("note id:", note.id, "pubkey:", note.pubkey.slice(0, 12) + "…");
const res = await publishAll(BROADCAST_RELAYS, note, 10000);
for (const r of res) console.log(`${r.accepted ? "✓" : "✗"} ${r.relay} ${r.message}`);
const ok = res.filter((r) => r.accepted).length;
console.log(`\nACCEPTED ${ok}/${res.length} public relays`);
process.exit(ok > 0 ? 0 : 1);
