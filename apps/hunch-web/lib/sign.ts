// NIP-07 signing + publishing (browser). The signer is the user's extension (window.nostr) —
// the frontend never holds a private key (CLAUDE.md: NIP-07/46 for all auth; no key custody).

import type { EventTemplate } from "./build.ts";
import type { NostrEvent } from "./hunch.ts";

interface Nip07 {
  getPublicKey(): Promise<string>;
  signEvent(event: { kind: number; created_at: number; tags: string[][]; content: string }): Promise<NostrEvent>;
}

function nip07(): Nip07 {
  const n = (globalThis as any).nostr as Nip07 | undefined;
  if (!n) throw new Error("No NIP-07 signer found. Install a Nostr extension (e.g. Alby, nos2x).");
  return n;
}

export async function getPublicKey(): Promise<string> {
  return nip07().getPublicKey();
}

/** Signs a template via the NIP-07 extension; the extension fills id, pubkey, and sig. */
export async function signTemplate(t: EventTemplate): Promise<NostrEvent> {
  return nip07().signEvent({
    kind: t.kind,
    created_at: Math.floor(Date.now() / 1000),
    tags: t.tags,
    content: t.content,
  });
}
