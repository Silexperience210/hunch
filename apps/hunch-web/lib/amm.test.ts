import { test } from "node:test";
import assert from "node:assert";
import { inventoryForProb, quoteBet, sharesForStake } from "./amm.ts";
import { costToBuy } from "./lmsr.ts";

const close = (a: number, b: number, eps = 1e-6) => Math.abs(a - b) < eps;

test("inventoryForProb is 0 at 50% and prices back to p", () => {
  assert.ok(close(inventoryForProb(0.5, 1000), 0));
  // round-trips through the LMSR price (priceYes(q,0,b) === p)
  const q = quoteBet("YES", 0, 0.7, 1000);
  assert.ok(close(q.priceYes, 0.7, 1e-3));
});

test("a 50/50 market quotes ~0.5 each side and pays out more than you stake", () => {
  const q = quoteBet("YES", 100, 0.5, 10000);
  assert.ok(close(q.priceYes, 0.5, 1e-9) && close(q.priceNo, 0.5, 1e-9));
  assert.ok(q.shares > 100); // price < 1, so 100 sats buys > 100 sats of payout
  assert.ok(q.avgPrice > 0.5 && q.avgPrice < 1); // slippage pushes avg above the spot price
  assert.ok(q.priceAfter > q.priceYes); // buying YES raises the YES price
});

test("YES and NO are symmetric at 50%", () => {
  const y = quoteBet("YES", 250, 0.5, 10000);
  const n = quoteBet("NO", 250, 0.5, 10000);
  assert.ok(close(y.shares, n.shares, 1e-6));
  assert.ok(close(y.avgPrice, n.avgPrice, 1e-9));
});

test("a more likely YES is more expensive (fewer shares per stake)", () => {
  const atEven = quoteBet("YES", 100, 0.5, 10000);
  const atFavored = quoteBet("YES", 100, 0.8, 10000);
  assert.ok(atFavored.priceYes > atEven.priceYes);
  assert.ok(atFavored.shares < atEven.shares); // YES costs more → buy less payout for the same stake
});

test("sharesForStake inverts costToBuy", () => {
  const shares = sharesForStake("YES", 100, 0, 0, 10000);
  assert.ok(close(costToBuy("YES", shares, 0, 0, 10000), 100, 1e-3));
});
