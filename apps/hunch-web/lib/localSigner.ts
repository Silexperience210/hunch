// In-browser Nostr signer — a fallback for devices without a NIP-07 extension (most phones).
// The key lives in this browser's localStorage (same trust model as the Cashu wallet key) and signs
// events with BIP-340 Schnorr via the audited @noble libs. NIP-07 stays the preferred path when an
// extension is present; this just makes Hunch usable on mobile. (NIP-46 remote signing is a future
// upgrade for stronger key isolation.)

import { schnorr } from "@noble/curves/secp256k1.js";
import { bytesToHex, hexToBytes } from "@noble/hashes/utils.js";
import { eventId } from "./verify.ts";
import type { EventTemplate } from "./build.ts";
import type { NostrEvent } from "./hunch.ts";

const NSEC_KEY = "hunch:nostr-sk";

/** Signs a template with an explicit 32-byte secret key (hex). Pure — no storage, so it's testable. */
export function signEventWithKey(t: EventTemplate, secretHex: string): NostrEvent {
  const sk = hexToBytes(secretHex.trim());
  const pubkey = bytesToHex(schnorr.getPublicKey(sk));
  const base = {
    pubkey,
    created_at: Math.floor(Date.now() / 1000),
    kind: t.kind,
    tags: t.tags,
    content: t.content,
  };
  const id = eventId(base);
  const sig = bytesToHex(schnorr.sign(hexToBytes(id), sk));
  return { id, ...base, sig };
}

function genSecret(): string {
  const b = new Uint8Array(32);
  crypto.getRandomValues(b);
  return bytesToHex(b);
}

/** The browser's Nostr identity secret (hex), created on first use and persisted. */
export function localSecret(): string {
  let s = localStorage.getItem(NSEC_KEY);
  if (!s) {
    s = genSecret();
    localStorage.setItem(NSEC_KEY, s);
  }
  return s;
}

/** Replace the local identity with an imported secret (hex). */
export function setLocalSecret(hex: string) {
  localStorage.setItem(NSEC_KEY, hex.trim().toLowerCase());
}

/** The local identity's x-only public key (hex). */
export function localPubkey(): string {
  return bytesToHex(schnorr.getPublicKey(hexToBytes(localSecret())));
}

/** Signs a template with the stored local key. */
export function signLocally(t: EventTemplate): NostrEvent {
  return signEventWithKey(t, localSecret());
}
