import { test } from "node:test";
import assert from "node:assert";
import { signEventWithKey } from "./localSigner.ts";
import { verifyEvent } from "./verify.ts";

test("a locally-signed event passes verifyEvent (valid NIP-01 id + BIP-340 sig)", () => {
  const ev = signEventWithKey({ kind: 30888, tags: [["d", "x"], ["oracle", "ab".repeat(32)]], content: "hi" }, "11".repeat(32));
  assert.ok(verifyEvent(ev));
  assert.strictEqual(ev.pubkey.length, 64);
  assert.strictEqual(ev.sig.length, 128);
});

test("tampering breaks verification", () => {
  const ev = signEventWithKey({ kind: 38888, tags: [], content: "x" }, "22".repeat(32));
  assert.ok(verifyEvent({ ...ev, content: "tampered" }) === false);
});
