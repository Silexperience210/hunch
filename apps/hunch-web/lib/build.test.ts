import { test } from "node:test";
import assert from "node:assert";
import { buildMarketTemplate, buildOrderTemplate } from "./build.ts";
import { parseMarketEvent, parseOrderEvent, type NostrEvent } from "./hunch.ts";

// Wrap a template into a fake signed event so we can round-trip it through the parsers.
function asEvent(t: { kind: number; tags: string[][]; content: string }, pubkey = "cc".repeat(32)): NostrEvent {
  return { id: "00".repeat(32), pubkey, created_at: 1, kind: t.kind, tags: t.tags, content: t.content, sig: "00".repeat(64) };
}

test("buildMarketTemplate round-trips through parseMarketEvent", () => {
  const t = buildMarketTemplate({
    slug: "btc-100k",
    oracle: "aa".repeat(32),
    expiry: 1767139200,
    mint: "https://mint.hunch.markets",
    dlcContract: `${"bb".repeat(32)}:0`,
    question: "Will BTC close above $100k?",
    resolution: "YES if >= 100000",
    topics: ["bitcoin"],
    category: "crypto",
  });
  const m = parseMarketEvent(asEvent(t));
  assert.ok(m);
  assert.strictEqual(m!.d, "btc-100k");
  assert.strictEqual(m!.refundTimeout, 1767139200 + 7 * 24 * 3600); // default expiry + 7d
  assert.deepStrictEqual(m!.outcomes, ["YES", "NO", "INVALID"]);
  assert.strictEqual(m!.category, "crypto");
  assert.deepStrictEqual(m!.topics, ["bitcoin"]);
  assert.strictEqual(m!.content.question, "Will BTC close above $100k?");
});

test("buildOrderTemplate has d == market and round-trips through parseOrderEvent", () => {
  const market = `${"cc".repeat(32)}:30888:btc-100k`;
  const t = buildOrderTemplate({ market, side: "YES", amount: 10000, price: 73, kind: "bid", expires: 1900000000 });
  assert.ok(t.tags.some((tag) => tag[0] === "d" && tag[1] === market));
  const o = parseOrderEvent(asEvent(t));
  assert.ok(o);
  assert.strictEqual(o!.side, "YES");
  assert.strictEqual(o!.amount, 10000);
  assert.strictEqual(o!.price, 73);
  assert.strictEqual(o!.kind, "bid");
});
