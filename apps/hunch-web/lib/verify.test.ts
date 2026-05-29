import { test } from "node:test";
import assert from "node:assert";
import { verifyEvent, eventId } from "./verify.ts";
import type { NostrEvent } from "./hunch.ts";

// A real kind:30888 event signed by the Rust `hunch` CLI (hunch-nostr), key
// 5f80…c3a0. This is a cross-language vector: TS must accept what Rust signed.
const RUST_SIGNED: NostrEvent = {
  content: '{"question":"vector","resolution_criteria":"","sources":[],"rules_version":"1.0"}',
  created_at: 1780072520,
  id: "373e70eb52b0063a0f32e974b947cb6488bac79da58381ca372e9997751ef860",
  kind: 30888,
  pubkey: "d1cf61ee61a17d034d4e38e81a91b9703294df384fe90dba5149bb1c8da4a680",
  sig: "6732b2a6da27e8c5b829d316bb9c5ae62c29b20a73fc31b310056225524f80056e42839a343af68fbbbeda79e61cb3d3f8cebbfb116219f3c41f338a7b9393ed",
  tags: [
    ["d", "vec-test"],
    ["oracle", "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"],
    ["outcomes", "YES,NO,INVALID"],
    ["expiry", "1767139200"],
    ["refund_timeout", "1767744000"],
    ["mint", "https://m.ex"],
    ["dlc_contract", "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb:0"],
  ],
};

test("recomputed id matches the Rust-signed event id", () => {
  assert.strictEqual(eventId(RUST_SIGNED), RUST_SIGNED.id);
});

test("verifyEvent accepts a genuine Rust-signed event (cross-language)", () => {
  assert.strictEqual(verifyEvent(RUST_SIGNED), true);
});

test("verifyEvent rejects tampered content", () => {
  const ev = { ...RUST_SIGNED, content: RUST_SIGNED.content.replace("vector", "evil") };
  assert.strictEqual(verifyEvent(ev), false);
});

test("verifyEvent rejects a tampered signature", () => {
  const ev = { ...RUST_SIGNED, sig: "00" + RUST_SIGNED.sig.slice(2) };
  assert.strictEqual(verifyEvent(ev), false);
});

test("verifyEvent rejects a relabeled pubkey", () => {
  const ev = { ...RUST_SIGNED, pubkey: "bb".repeat(32) };
  assert.strictEqual(verifyEvent(ev), false);
});
