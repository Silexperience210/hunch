#!/usr/bin/env bun
// verify-kind-collisions.ts — fetch NIP registry, verify Hunch kinds do not collide
// outside of intended dependencies (NIP-88 for kinds 88/89, NIP-23 for kind 30023).
// Exit 0 on pass; exit 1 with diagnostic on collision.

const HUNCH_KINDS: Record<number, string> = {
  88: "oracle announce (NIP-88 dependency — expected)",
  89: "oracle attestation (NIP-88 dependency — expected)",
  30023: "HIP long-form (NIP-23 dependency — expected)",
  30888: "market (Hunch HIP-1)",
  38888: "order (Hunch HIP-1)",
  30890: "dispute (Hunch HIP-1)",
  30891: "reputation (Hunch HIP-1)",
  30892: "(reserved by Hunch HIP-1)",
};

const NIP_README_URL = "https://raw.githubusercontent.com/nostr-protocol/nips/master/README.md";

async function main(): Promise<void> {
  console.log("Fetching NIP registry...");
  const res = await fetch(NIP_README_URL);
  if (!res.ok) {
    console.error(`FAIL: could not fetch NIP registry (HTTP ${res.status})`);
    process.exit(1);
  }
  const text = await res.text();

  // NIP registry has lines like "| `30023` | Long-form Content | NIP-23 |" (varies)
  // Match any `<digits>` token in the kinds tables.
  const kindLineRegex = /`(\d+)`/g;
  const registeredKinds = new Set<number>();
  for (const match of text.matchAll(kindLineRegex)) {
    const n = parseInt(match[1], 10);
    if (!Number.isNaN(n)) registeredKinds.add(n);
  }

  let collisions = 0;
  for (const [kind, purpose] of Object.entries(HUNCH_KINDS)) {
    const n = parseInt(kind, 10);
    const isRegistered = registeredKinds.has(n);
    const isExpectedDep = purpose.includes("expected");

    if (isRegistered && !isExpectedDep) {
      console.error(`COLLISION: kind ${n} (${purpose}) is registered in NIP registry`);
      collisions += 1;
    } else if (isRegistered && isExpectedDep) {
      console.log(`OK: kind ${n} (${purpose}) — registered as expected`);
    } else if (!isRegistered && isExpectedDep) {
      console.warn(`WARN: kind ${n} (${purpose}) NOT in registry yet — dependency may be unmerged draft`);
    } else {
      console.log(`OK: kind ${n} (${purpose}) — free for Hunch reservation`);
    }
  }

  if (collisions > 0) {
    console.error(`FAIL: ${collisions} kind collisions found`);
    process.exit(1);
  }
  console.log("PASS: no Hunch kind collisions against NIP registry");
}

main().catch((err) => {
  console.error("ERROR:", err);
  process.exit(1);
});
