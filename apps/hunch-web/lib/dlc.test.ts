import { test } from "node:test";
import assert from "node:assert";
import { signaturePoint, outcomeLockKey, outcomeUnlockSecret } from "./dlc.ts";

// Vectors produced by the Rust `hunch-dlc` (deterministic keys). The TS derivation must match
// the Rust/mint byte-for-byte, or wallet-minted tokens wouldn't be redeemable.
const V = {
  market: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa:30888:m",
  oracleXonly: "d1cf61ee61a17d034d4e38e81a91b9703294df384fe90dba5149bb1c8da4a680",
  nonceXonly: "4c5b9f8c55ddb85ad42af65f82ee19b10cb363abbd3e1d332d6601c6400a42b5",
  bettorSecret: "1111111111111111111111111111111111111111111111111111111111111111",
  bettorCompressed: "034f355bdcb7cc0af728ef3cceb9615d90684bb5b2ca5f859ab0f0b704075871aa",
  sigPointYes: "0327414384645a370da29cab414410b65995d46fcef4d80ba0833d521e0521755d",
  lockYes: "03cd7eec261fa5a46cf24eb6040ad9a34ae3a737cf44b86e44cfff3872c59a971e",
  attestationSig:
    "4c5b9f8c55ddb85ad42af65f82ee19b10cb363abbd3e1d332d6601c6400a42b5455fabbf3640b22b502935431fbc9c5e28d2cf6bdde652ba544a192f49f739b9",
  unlockYes: "5670bcd04751c33c613a465430cdad6f39e3e07ceef763cb655b2a405b084aca",
};

test("signaturePoint matches Rust hunch-dlc", () => {
  assert.strictEqual(signaturePoint(V.oracleXonly, V.nonceXonly, V.market, "YES"), V.sigPointYes);
});

test("outcomeLockKey (L = B + S_X) matches Rust hunch-dlc", () => {
  assert.strictEqual(
    outcomeLockKey(V.bettorCompressed, V.oracleXonly, V.nonceXonly, V.market, "YES"),
    V.lockYes,
  );
});

test("outcomeUnlockSecret (l = b + s_X) matches Rust hunch-dlc", () => {
  assert.strictEqual(outcomeUnlockSecret(V.bettorSecret, V.attestationSig), V.unlockYes);
});

test("a different outcome yields a different lock key", () => {
  const lockNo = outcomeLockKey(V.bettorCompressed, V.oracleXonly, V.nonceXonly, V.market, "NO");
  assert.notStrictEqual(lockNo, V.lockYes);
});

test("outcomeUnlockSecret rejects a bad signature length", () => {
  assert.throws(() => outcomeUnlockSecret(V.bettorSecret, "abcd"));
});
