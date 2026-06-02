// NIP-46 remote signing (Nostr Connect / "bunker") — the key stays in a separate signer app like
// Amber; this client only sends sign requests over a relay. Encryption + protocol come from
// nostr-tools (audited reference impl) — we don't hand-roll NIP-44.
//
// Two pairing flows:
//  - nostrconnect:// (recommended for Amber): WE generate a URI, the user opens it in Amber.
//  - bunker://: the user pastes a URI produced by the signer.
// After pairing we persist the negotiated BunkerPointer + an ephemeral client key, so the session
// reconnects on reload.

import { BunkerSigner, createNostrConnectURI, parseBunkerInput } from "nostr-tools/nip46";
import { schnorr } from "@noble/curves/secp256k1.js";
import { bytesToHex, hexToBytes } from "@noble/hashes/utils.js";
import type { EventTemplate } from "./build.ts";
import type { NostrEvent } from "./hunch.ts";

const CLIENT_KEY = "hunch:nip46-client-sk";
const BP_KEY = "hunch:nip46-bp"; // negotiated BunkerPointer (JSON) for reconnection

// Relays both the client and the signer (Amber) connect to for the NIP-46 channel.
const NC_RELAYS = ["wss://relay.nsec.app", "wss://relay.21pay.org"];

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

function persist(bp: unknown) {
  localStorage.setItem(BP_KEY, JSON.stringify(bp));
}

export function hasBunker(): boolean {
  return typeof localStorage !== "undefined" && !!localStorage.getItem(BP_KEY);
}

export function clearBunker() {
  localStorage.removeItem(BP_KEY);
  signer = null;
  connecting = null;
}

/** Flow 1 — nostrconnect: build a URI for the user to open in Amber; resolves with their pubkey. */
export function startNostrConnect(): { uri: string; connected: Promise<string> } {
  const sk = clientSecret();
  const clientPubkey = bytesToHex(schnorr.getPublicKey(sk));
  const secret = bytesToHex(crypto.getRandomValues(new Uint8Array(16)));
  const uri = createNostrConnectURI({ clientPubkey, relays: NC_RELAYS, secret, name: "Hunch" });
  const connected = (async () => {
    const s = await BunkerSigner.fromURI(sk, uri, { onauth }, 180_000); // wait up to 3 min for Amber
    signer = s;
    persist(s.bp);
    return s.getPublicKey();
  })();
  return { uri, connected };
}

/** Flow 2 — bunker://: connect to a URI produced by the signer app. */
export async function connectBunker(uri: string): Promise<string> {
  const bp = await parseBunkerInput(uri.trim());
  if (!bp) throw new Error("Invalid bunker:// URI — copy it from the signer, or use nostrconnect.");
  const s = BunkerSigner.fromBunker(clientSecret(), bp, { onauth });
  await s.connect();
  const pk = await s.getPublicKey();
  signer = s;
  persist(bp);
  return pk;
}

/** Lazily (re)connect using the persisted BunkerPointer. */
async function getSigner(): Promise<BunkerSigner> {
  if (signer) return signer;
  if (connecting) return connecting;
  const raw = localStorage.getItem(BP_KEY);
  if (!raw) throw new Error("No remote signer connected — connect a signer first.");
  connecting = (async () => {
    const s = BunkerSigner.fromBunker(clientSecret(), JSON.parse(raw), { onauth });
    await s.connect();
    signer = s;
    return s;
  })().finally(() => {
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
