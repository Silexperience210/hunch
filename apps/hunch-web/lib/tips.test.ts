import { test } from "node:test";
import assert from "node:assert";
import { lnAddressToLnurlp, presetTip } from "./tips.ts";

test("lnAddressToLnurlp builds the .well-known URL", () => {
  assert.strictEqual(lnAddressToLnurlp("hunch@21pay.org"), "https://21pay.org/.well-known/lnurlp/hunch");
  assert.strictEqual(lnAddressToLnurlp(" tips@example.com "), "https://example.com/.well-known/lnurlp/tips");
});

test("lnAddressToLnurlp rejects a malformed address", () => {
  assert.throws(() => lnAddressToLnurlp("notanaddress"));
  assert.throws(() => lnAddressToLnurlp("@domain.com"));
  assert.throws(() => lnAddressToLnurlp("name@"));
});

test("presetTip never exceeds the chosen amount or the available leftover", () => {
  assert.strictEqual(presetTip(0, 1000), 0); // no tip chosen
  assert.strictEqual(presetTip(100, 0), 0); // nothing left
  assert.strictEqual(presetTip(100, 1000), 100); // plenty available → full tip
  const clamped = presetTip(500, 300); // tip > available → clamped below it (fee buffer)
  assert.ok(clamped < 300 && clamped > 0);
  for (const [c, a] of [[500, 3], [50, 50], [21, 1000]] as const) {
    const t = presetTip(c, a);
    assert.ok(t >= 0 && t <= c && t <= a); // invariants
  }
});
