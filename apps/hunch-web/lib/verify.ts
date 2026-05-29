// Schnorr (BIP-340) event verification — mirrors the Rust `hunch_nostr::verify_event`.
//
// Relays are untrusted: anything read from a relay MUST pass this before being trusted.
// Recompute the NIP-01 id from the event fields, check it equals `ev.id`, then verify the
// BIP-340 signature over that id under `ev.pubkey` (x-only). Uses the audited @noble libs.

import { schnorr } from "@noble/curves/secp256k1.js";
import { sha256 } from "@noble/hashes/sha2.js";
import { bytesToHex, hexToBytes } from "@noble/hashes/utils.js";
import { canonicalSerialization, type NostrEvent } from "./hunch.ts";

/** Recomputes the 32-byte NIP-01 event id (hex). */
export function eventId(ev: Pick<NostrEvent, "pubkey" | "created_at" | "kind" | "tags" | "content">): string {
  return bytesToHex(sha256(new TextEncoder().encode(canonicalSerialization(ev))));
}

/**
 * True iff the event's id matches its fields AND the Schnorr signature verifies under its pubkey.
 * Returns false (never throws) on any malformed field.
 */
export function verifyEvent(ev: NostrEvent): boolean {
  try {
    const id = sha256(new TextEncoder().encode(canonicalSerialization(ev)));
    if (bytesToHex(id) !== ev.id) return false;
    return schnorr.verify(hexToBytes(ev.sig), id, hexToBytes(ev.pubkey));
  } catch {
    return false;
  }
}
