import { test } from "node:test";
import assert from "node:assert";
import { buildShareNote, KIND_NOTE, marketUrl } from "./share.ts";

test("marketUrl is absolute and encodes the id", () => {
  const u = marketUrl("abc:30888:btc-100k", "https://x.example/hunch/");
  assert.strictEqual(u, "https://x.example/hunch/market/?id=abc%3A30888%3Abtc-100k");
});

test("open-market note carries question, odds, link and hashtags", () => {
  const n = buildShareNote({ question: "Will BTC top 100k in 2026?", id: "a:30888:m", yes: 62 });
  assert.strictEqual(n.kind, KIND_NOTE);
  assert.ok(n.content.includes("Will BTC top 100k in 2026?"));
  assert.ok(n.content.includes("YES 62%") && n.content.includes("NO 38%"));
  assert.ok(n.content.includes("market/?id=a%3A30888%3Am"));
  assert.ok(n.content.includes("#hunch"));
  assert.ok(n.tags.some((t) => t[0] === "t" && t[1] === "hunch"));
  assert.ok(n.tags.some((t) => t[0] === "r" && t[1].includes("market/?id=")));
});

test("a market with no odds yet says so", () => {
  const n = buildShareNote({ question: "Q?", id: "a:30888:m", yes: null });
  assert.ok(/new market/i.test(n.content));
  assert.ok(!/YES \d+%/.test(n.content)); // no odds line
});

test("settlement note announces the verdict + reasoning", () => {
  const n = buildShareNote({
    question: "Is BTC capped at 21M?",
    url: "https://h/market/?id=x",
    settled: { outcome: "YES", reasoning: "Protocol hard-codes 21M." },
  });
  assert.ok(n.content.includes("✅ Settled: YES"));
  assert.ok(n.content.includes("Protocol hard-codes 21M."));
  assert.ok(n.content.includes("verify the signature yourself"));
  assert.ok(n.content.includes("https://h/market/?id=x"));
});

test("INVALID settlement reads as a refund", () => {
  const n = buildShareNote({ question: "Q?", id: "a:30888:m", settled: { outcome: "INVALID" } });
  assert.ok(/refunded/i.test(n.content));
});
