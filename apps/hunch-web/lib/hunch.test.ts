import { test } from "node:test";
import assert from "node:assert";
import { createHash } from "node:crypto";
import {
  KIND_MARKET,
  KIND_ORDER,
  marketId,
  parseMarketEvent,
  parseOrderEvent,
  parseAnnounceEvent,
  parseAttestationEvent,
  parseReputationEvent,
  aggregateReputation,
  computeEventId,
  canonicalSerialization,
  type NostrEvent,
  type Reputation,
} from "./hunch.ts";

function marketEvent(): NostrEvent {
  return {
    id: "00".repeat(32),
    pubkey: "cc".repeat(32),
    created_at: 1_700_000_000,
    kind: KIND_MARKET,
    tags: [
      ["d", "btc-100k-eoy-2026"],
      ["oracle", "aa".repeat(32)],
      ["outcomes", "YES,NO,INVALID"],
      ["expiry", "1767139200"],
      ["refund_timeout", "1767744000"],
      ["mint", "https://mint.hunch.markets"],
      ["dlc_contract", `${"bb".repeat(32)}:0`],
      ["category", "crypto"],
      ["t", "bitcoin"],
      ["t", "macro"],
    ],
    content: JSON.stringify({
      question: "Will BTC close above $100k on 2026-12-31?",
      resolution_criteria: "YES if BTC/USD >= 100000 at 23:59 UTC",
      sources: ["https://pro.coinbase.com/markets/BTC-USD"],
      rules_version: "1.0",
    }),
    sig: "00".repeat(64),
  };
}

test("marketId format matches the Rust `<pubkey>:30888:<d>`", () => {
  assert.strictEqual(marketId("cc".repeat(32), "m"), `${"cc".repeat(32)}:30888:m`);
});

test("parseMarketEvent extracts all fields", () => {
  const m = parseMarketEvent(marketEvent());
  assert.ok(m);
  assert.strictEqual(m!.d, "btc-100k-eoy-2026");
  assert.strictEqual(m!.id, `${"cc".repeat(32)}:30888:btc-100k-eoy-2026`);
  assert.deepStrictEqual(m!.outcomes, ["YES", "NO", "INVALID"]);
  assert.strictEqual(m!.expiry, 1767139200);
  assert.strictEqual(m!.refundTimeout, 1767744000);
  assert.strictEqual(m!.mint, "https://mint.hunch.markets");
  assert.strictEqual(m!.category, "crypto");
  assert.deepStrictEqual(m!.topics, ["bitcoin", "macro"]);
  assert.strictEqual(m!.content.question, "Will BTC close above $100k on 2026-12-31?");
});

test("parseMarketEvent rejects wrong kind and missing tags", () => {
  assert.strictEqual(parseMarketEvent({ ...marketEvent(), kind: 1 }), null);
  const noD = marketEvent();
  noD.tags = noD.tags.filter((t) => t[0] !== "d");
  assert.strictEqual(parseMarketEvent(noD), null);
});

test("parseOrderEvent extracts bid/ask order", () => {
  const ev: NostrEvent = {
    id: "00".repeat(32),
    pubkey: "dd".repeat(32),
    created_at: 1,
    kind: KIND_ORDER,
    tags: [
      ["d", `${"cc".repeat(32)}:30888:m`],
      ["market", `${"cc".repeat(32)}:30888:m`],
      ["side", "YES"],
      ["amount", "10000"],
      ["price", "73"],
      ["kind", "bid"],
      ["expires", "1900000000"],
    ],
    content: "",
    sig: "00".repeat(64),
  };
  const o = parseOrderEvent(ev);
  assert.ok(o);
  assert.strictEqual(o!.side, "YES");
  assert.strictEqual(o!.amount, 10000);
  assert.strictEqual(o!.price, 73);
  assert.strictEqual(o!.kind, "bid");
  // bad side / kind rejected
  assert.strictEqual(parseOrderEvent({ ...ev, tags: ev.tags.map((t) => (t[0] === "side" ? ["side", "MAYBE"] : t)) }), null);
});

test("computeEventId matches an independent sha256 of the canonical NIP-01 form", async () => {
  const ev = marketEvent();
  const sha256 = (data: Uint8Array) => new Uint8Array(createHash("sha256").update(data).digest());
  const id = await computeEventId(ev, sha256);
  // Independent recomputation from the canonical string.
  const expected = createHash("sha256").update(canonicalSerialization(ev)).digest("hex");
  assert.strictEqual(id, expected);
  assert.strictEqual(id.length, 64);
  // Deterministic.
  assert.strictEqual(await computeEventId(ev, sha256), id);
});

test("canonical serialization has no insignificant whitespace", () => {
  const s = canonicalSerialization(marketEvent());
  assert.ok(!s.startsWith("[ "));
  assert.ok(!s.includes(", ")); // compact form, like serde_json in hunch-nostr
});

function announceEvent(): NostrEvent {
  return {
    id: "00".repeat(32),
    pubkey: "aa".repeat(32),
    created_at: 1_700_000_000,
    kind: 88,
    tags: [
      ["market", `${"cc".repeat(32)}:30888:btc-100k-eoy-2026`],
      ["nonce", "ab".repeat(32)],
    ],
    content: "Resolves on the Coinbase BTC-USD feed.",
    sig: "00".repeat(64),
  };
}

function attestationEvent(): NostrEvent {
  return {
    id: "00".repeat(32),
    pubkey: "aa".repeat(32),
    created_at: 1_700_000_000,
    kind: 89,
    tags: [
      ["market", `${"cc".repeat(32)}:30888:btc-100k-eoy-2026`],
      ["outcome", "YES"],
      ["sig", "cd".repeat(64)],
    ],
    content: "",
    sig: "00".repeat(64),
  };
}

test("parseAnnounceEvent extracts the nonce R", () => {
  const a = parseAnnounceEvent(announceEvent());
  assert.ok(a);
  assert.strictEqual(a!.nonce, "ab".repeat(32));
  assert.strictEqual(a!.market, `${"cc".repeat(32)}:30888:btc-100k-eoy-2026`);
});

test("parseAnnounceEvent rejects wrong kind and bad nonce length", () => {
  assert.strictEqual(parseAnnounceEvent({ ...announceEvent(), kind: 1 }), null);
  const bad = announceEvent();
  bad.tags = [["market", "m"], ["nonce", "abcd"]];
  assert.strictEqual(parseAnnounceEvent(bad), null);
});

test("parseAttestationEvent extracts the 64-byte signature and outcome", () => {
  const a = parseAttestationEvent(attestationEvent());
  assert.ok(a);
  assert.strictEqual(a!.outcome, "YES");
  assert.strictEqual(a!.signature.length, 128);
});

test("parseAttestationEvent rejects unknown outcome and bad signature length", () => {
  const badOutcome = attestationEvent();
  badOutcome.tags = [["market", "m"], ["outcome", "MAYBE"], ["sig", "cd".repeat(64)]];
  assert.strictEqual(parseAttestationEvent(badOutcome), null);
  const badSig = attestationEvent();
  badSig.tags = [["market", "m"], ["outcome", "YES"], ["sig", "deadbeef"]];
  assert.strictEqual(parseAttestationEvent(badSig), null);
});

function reputationEvent(rater: string, rating: string, createdAt = 1_700_000_000): NostrEvent {
  return {
    id: "00".repeat(32),
    pubkey: rater,
    created_at: createdAt,
    kind: 30891,
    tags: [
      ["d", "aa".repeat(32)],
      ["rating", rating],
    ],
    content: "honest settlement history",
    sig: "00".repeat(64),
  };
}

test("parseReputationEvent extracts rater, subject, rating", () => {
  const r = parseReputationEvent(reputationEvent("bb".repeat(32), "80"));
  assert.ok(r);
  assert.strictEqual(r!.rater, "bb".repeat(32));
  assert.strictEqual(r!.subject, "aa".repeat(32));
  assert.strictEqual(r!.rating, 80);
});

test("parseReputationEvent rejects out-of-range or non-integer ratings", () => {
  assert.strictEqual(parseReputationEvent(reputationEvent("bb".repeat(32), "101")), null);
  assert.strictEqual(parseReputationEvent(reputationEvent("bb".repeat(32), "-1")), null);
  assert.strictEqual(parseReputationEvent(reputationEvent("bb".repeat(32), "x")), null);
});

test("aggregateReputation averages distinct raters, newest claim per rater", () => {
  const reps = [
    parseReputationEvent(reputationEvent("11".repeat(32), "60", 100)),
    parseReputationEvent(reputationEvent("22".repeat(32), "100", 100)),
    // same rater re-rates later — newest (80) wins over 60
    parseReputationEvent(reputationEvent("11".repeat(32), "80", 200)),
  ].filter((r): r is Reputation => r !== null);
  const summary = aggregateReputation(reps);
  assert.ok(summary);
  assert.strictEqual(summary!.count, 2);
  assert.strictEqual(summary!.avg, 90); // (80 + 100) / 2
});

test("aggregateReputation returns null for no claims", () => {
  assert.strictEqual(aggregateReputation([]), null);
});
