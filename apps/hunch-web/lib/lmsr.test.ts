import { test } from "node:test";
import assert from "node:assert";
import { cost, costToBuy, maxSubsidy, priceYes } from "./lmsr.ts";

const close = (a: number, b: number, eps = 1e-9) => Math.abs(a - b) < eps;

test("empty market is 50/50 and costs b·ln2", () => {
  assert.ok(close(priceYes(0, 0, 1000), 0.5));
  assert.ok(close(cost(0, 0, 1000), maxSubsidy(1000)));
});

test("buying YES raises the YES price", () => {
  const before = priceYes(0, 0, 1000);
  const after = priceYes(500, 0, 1000);
  assert.ok(after > before && after < 1);
});

test("costToBuy is positive and below the share count", () => {
  const c = costToBuy("YES", 100, 0, 0, 1000);
  assert.ok(c > 0 && c < 100); // pay < 100 sats for a payout of 100 when price < 1
});

test("max subsidy is b·ln2", () => {
  assert.ok(close(maxSubsidy(14400), 14400 * Math.LN2));
});
