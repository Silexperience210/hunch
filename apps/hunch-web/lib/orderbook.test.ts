import { test } from "node:test";
import assert from "node:assert";
import { buildOrderBook } from "./orderbook.ts";
import type { Order } from "./hunch.ts";

function o(side: "YES" | "NO", kind: "bid" | "ask", price: number, market = "m"): Order {
  return { author: "a" + price, market, side, amount: 1000, price, kind, expires: 1900000000 };
}

test("bids sort high→low, asks low→high; best prices surfaced", () => {
  const book = buildOrderBook([
    o("YES", "bid", 60),
    o("YES", "bid", 72),
    o("YES", "ask", 80),
    o("YES", "ask", 75),
    o("NO", "bid", 30),
  ]);
  assert.deepStrictEqual(book.yesBids.map((x) => x.price), [72, 60]);
  assert.deepStrictEqual(book.yesAsks.map((x) => x.price), [75, 80]);
  assert.strictEqual(book.bestYesBid, 72);
  assert.strictEqual(book.bestYesAsk, 75);
  assert.strictEqual(book.bestNoBid, 30);
  assert.strictEqual(book.bestNoAsk, undefined);
});

test("scopes to the given market", () => {
  const book = buildOrderBook([o("YES", "bid", 50, "m1"), o("YES", "bid", 99, "m2")], "m1");
  assert.deepStrictEqual(book.yesBids.map((x) => x.price), [50]);
});

test("empty book has no best prices", () => {
  const book = buildOrderBook([]);
  assert.strictEqual(book.bestYesBid, undefined);
  assert.strictEqual(book.yesBids.length, 0);
});
