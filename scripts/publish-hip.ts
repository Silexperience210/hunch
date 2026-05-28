#!/usr/bin/env bun
/**
 * publish-hip.ts — NIP-23 long-form publisher for HIPs (kind 30023)
 *
 * Usage:
 *   HUNCH_NSEC="nsec1..." bun scripts/publish-hip.ts docs/HIP-0.md
 *
 * Publishes the HIP markdown content as a kind:30023 replaceable Nostr event
 * to a fixed set of public relays plus the project relay (once Phase 2 deploys
 * relay.hunch.markets). Appends the resulting naddr1 reference to the HIP file
 * as a footer for round-trip verifiability.
 *
 * Replaceable identifier (`d` tag) is the HIP slug: `hip-N` from the filename.
 */

import { readFile, appendFile } from "node:fs/promises";
import { basename } from "node:path";
import NDK, { NDKEvent, NDKPrivateKeySigner } from "@nostr-dev-kit/ndk";
import { nip19 } from "nostr-tools";

const RELAYS = [
  "wss://relay.damus.io",
  "wss://nos.lol",
  "wss://relay.nostr.band",
  "wss://relay.hunch.markets", // Phase 2 deployment; ignore connection failure here
];

interface HIPFront {
  hip: string;
  title: string;
  status: string;
  type: string;
}

function parseHipFrontmatter(md: string): HIPFront {
  // HIPs use a fenced text block at top: ```HIP:0...```
  const fence = md.match(/```[\s\S]*?```/);
  if (!fence) throw new Error("HIP frontmatter fence not found");
  const body = fence[0];
  const get = (k: string) => {
    const m = body.match(new RegExp(`^${k}:\\s*(.+)$`, "m"));
    return m ? m[1].trim() : "";
  };
  return {
    hip: get("HIP"),
    title: get("Title"),
    status: get("Status"),
    type: get("Type"),
  };
}

async function main() {
  const filePath = process.argv[2];
  if (!filePath) {
    console.error("Usage: bun scripts/publish-hip.ts <path/to/HIP-N.md>");
    process.exit(1);
  }

  const nsecHex = process.env.HUNCH_NSEC;
  if (!nsecHex) {
    console.error("HUNCH_NSEC environment variable required (project Nostr pseudonym key)");
    console.error("Generate with: bun scripts/generate-keypair.ts");
    process.exit(1);
  }

  const md = await readFile(filePath, "utf8");
  const front = parseHipFrontmatter(md);
  if (!front.hip) {
    console.error(`Could not parse HIP frontmatter in ${filePath}`);
    process.exit(1);
  }

  // Decode nsec to raw secret (NDKPrivateKeySigner accepts either form)
  let secretHex: string;
  if (nsecHex.startsWith("nsec1")) {
    const decoded = nip19.decode(nsecHex);
    if (decoded.type !== "nsec") throw new Error("Not an nsec");
    secretHex = decoded.data as string;
  } else {
    secretHex = nsecHex;
  }

  const signer = new NDKPrivateKeySigner(secretHex);
  const ndk = new NDK({ explicitRelayUrls: RELAYS, signer });
  await ndk.connect();

  const event = new NDKEvent(ndk);
  event.kind = 30023;
  event.created_at = Math.floor(Date.now() / 1000);
  event.content = md;
  event.tags = [
    ["d", `hip-${front.hip}`],
    ["title", front.title],
    ["summary", `Hunch Improvement Proposal ${front.hip} — ${front.title} (Status: ${front.status})`],
    ["t", "hunch"],
    ["t", "hip"],
    ["t", `hunch-hip-${front.hip}`],
  ];

  await event.sign(signer);
  const published = await event.publish();
  console.log(`Published HIP-${front.hip} to ${published.size} relay(s)`);

  // naddr1 reference for the HIP footer
  const pubkey = (await signer.user()).pubkey;
  const naddr = nip19.naddrEncode({
    identifier: `hip-${front.hip}`,
    pubkey,
    kind: 30023,
    relays: RELAYS,
  });

  const footer = `\n\n---\n\n**Nostr publication:** \`${naddr}\` (NIP-23 long-form, kind:30023, replaceable)\n`;
  await appendFile(filePath, footer);
  console.log(`Appended naddr1 footer to ${basename(filePath)}: ${naddr}`);
  process.exit(0);
}

main().catch((err) => {
  console.error("Publish failed:", err);
  process.exit(1);
});
