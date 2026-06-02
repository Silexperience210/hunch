import { test } from "node:test";
import assert from "node:assert";
import { buildOrderBook, impliedOdds } from "./orderbook.ts";
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

test("impliedOdds splits the best YES/NO bids and sums to 100", () => {
  const book = buildOrderBook([o("YES", "bid", 72), o("NO", "bid", 27)]);
  const odds = impliedOdds(book);
  assert.deepStrictEqual(odds, { yes: 73, no: 27 }); // 72/(72+27)=0.727 → 73
  assert.strictEqual(odds!.yes + odds!.no, 100);
});

test("impliedOdds uses only the best bid on each side", () => {
  const book = buildOrderBook([o("YES", "bid", 40), o("YES", "bid", 60), o("NO", "bid", 40)]);
  assert.deepStrictEqual(impliedOdds(book), { yes: 60, no: 40 }); // 60/(60+40)
});

test("impliedOdds is null without two-sided demand", () => {
  assert.strictEqual(impliedOdds(buildOrderBook([o("YES", "bid", 50)])), null);
  assert.strictEqual(impliedOdds(buildOrderBook([o("YES", "ask", 50), o("NO", "ask", 50)])), null);
  assert.strictEqual(impliedOdds(buildOrderBook([])), null);
});
