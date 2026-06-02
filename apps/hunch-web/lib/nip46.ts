// NIP-46 remote signing (Nostr Connect / "bunker") — the key stays in a separate signer app like
// Amber; this client only sends sign requests over a relay. Stronger isolation than the in-browser
// key. Encryption + protocol come from nostr-tools (audited reference impl) — we don't hand-roll
// NIP-44. An ephemeral client key + the bunker URI are persisted so the session survives reload.

import { BunkerSigner, parseBunkerInput } from "nostr-tools/nip46";
import { bytesToHex, hexToBytes } from "@noble/hashes/utils.js";
import type { EventTemplate } from "./build.ts";
import type { NostrEvent } from "./hunch.ts";

const CLIENT_KEY = "hunch:nip46-client-sk";
const BUNKER_KEY = "hunch:nip46-bunker";

let signer: BunkerSigner | null = null;
let connecting: Promise<BunkerSigner> | null = null;

/** Ephemeral client key for this browser's NIP-46 sessions (not the user's identity). */
function clientSecret(): Uint8Array {
  let s = localStorage.getItem(CLIENT_KEY);
  if (!s) {
    const b = new Uint8Array(32);
    crypto.getRandomValues(b);
    s = bytesToHex(b);
    localStorage.setItem(CLIENT_KEY, s);
  }
  return hexToBytes(s);
}

const onauth = (url: string) => {
  try {
    window.open(url, "_blank");
  } catch {
    /* popup blocked — the signer app should still prompt */
  }
};

export function hasBunker(): boolean {
  return typeof localStorage !== "undefined" && !!localStorage.getItem(BUNKER_KEY);
}

export function clearBunker() {
  localStorage.removeItem(BUNKER_KEY);
  signer = null;
  connecting = null;
}

async function open(uri: string): Promise<BunkerSigner> {
  const bp = await parseBunkerInput(uri.trim());
  if (!bp) throw new Error("Invalid bunker:// URI — copy it from Amber → Connect.");
  const s = BunkerSigner.fromBunker(clientSecret(), bp, { onauth });
  await s.connect();
  signer = s;
  return s;
}

/** Connect to a `bunker://` URI from the signer app, persist it, and return the user's pubkey. */
export async function connectBunker(uri: string): Promise<string> {
  const s = await open(uri);
  const pk = await s.getPublicKey();
  localStorage.setItem(BUNKER_KEY, uri.trim());
  return pk;
}

/** Lazily (re)connect using the stored bunker URI. */
async function getSigner(): Promise<BunkerSigner> {
  if (signer) return signer;
  if (connecting) return connecting;
  const uri = localStorage.getItem(BUNKER_KEY);
  if (!uri) throw new Error("No remote signer connected — connect Amber first.");
  connecting = open(uri).finally(() => {
    connecting = null;
  });
  return connecting;
}

export async function nip46Pubkey(): Promise<string> {
  return (await getSigner()).getPublicKey();
}

export async function nip46Sign(t: EventTemplate): Promise<NostrEvent> {
  const ev = await (await getSigner()).signEvent({
    created_at: Math.floor(Date.now() / 1000),
    kind: t.kind,
    tags: t.tags,
    content: t.content,
  });
  return ev as unknown as NostrEvent;
}
