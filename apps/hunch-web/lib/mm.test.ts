import { test } from "node:test";
import assert from "node:assert";
import { buildBuyBody } from "./mm.ts";

const L = "02" + "a".repeat(64); // 33-byte compressed hex
const B = "03" + "b".repeat(64);

test("buildBuyBody assembles a valid request", () => {
  const body = buildBuyBody({ market: "x:30888:m", side: "YES", budget: 1000, lock: L, refund: B, locktime: 42 });
  assert.deepStrictEqual(body, {
    market: "x:30888:m",
    side: "YES",
    lock: L,
    refund: B,
    budget: 1000,
    locktime: 42,
  });
});

test("buildBuyBody accepts shares instead of budget", () => {
  const body = buildBuyBody({ market: "x:30888:m", side: "NO", shares: 500, lock: L, refund: B });
  assert.strictEqual(body.shares, 500);
  assert.strictEqual(body.budget, undefined);
});

test("buildBuyBody rejects bad input", () => {
  assert.throws(() => buildBuyBody({ market: "", side: "YES", budget: 1, lock: L, refund: B }));
  assert.throws(() => buildBuyBody({ market: "m", side: "MAYBE" as any, budget: 1, lock: L, refund: B }));
  assert.throws(() => buildBuyBody({ market: "m", side: "YES", budget: 1, lock: "deadbeef", refund: B }));
  assert.throws(() => buildBuyBody({ market: "m", side: "YES", budget: 1, lock: L, refund: "xyz" }));
  assert.throws(() => buildBuyBody({ market: "m", side: "YES", lock: L, refund: B }));
});
