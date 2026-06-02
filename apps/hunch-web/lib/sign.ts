// Event signing. Prefers a NIP-07 extension (window.nostr) when present; otherwise falls back to an
// in-browser local key so Hunch works on phones with no extension. The user can force either mode.
// (CLAUDE.md prefers NIP-07/46; the local key is the pragmatic mobile fallback — see localSigner.ts.)

import type { EventTemplate } from "./build.ts";
import type { NostrEvent } from "./hunch.ts";
import { localPubkey, signLocally } from "./localSigner.ts";

interface Nip07 {
  getPublicKey(): Promise<string>;
  signEvent(event: { kind: number; created_at: number; tags: string[][]; content: string }): Promise<NostrEvent>;
}

type SignerMode = "auto" | "nip07" | "local";
const MODE_KEY = "hunch:signer";

function extension(): Nip07 | undefined {
  return (globalThis as { nostr?: Nip07 }).nostr;
}

export function signerMode(): SignerMode {
  if (typeof localStorage === "undefined") return "auto";
  return (localStorage.getItem(MODE_KEY) as SignerMode) || "auto";
}

export function setSignerMode(m: SignerMode) {
  localStorage.setItem(MODE_KEY, m);
}

/** Whether the active signer is the in-browser local key (no extension, or forced local). */
export function usingLocalSigner(): boolean {
  const mode = signerMode();
  if (mode === "local") return true;
  if (mode === "nip07") return false;
  return !extension(); // auto: local only when there's no extension
}

export async function getPublicKey(): Promise<string> {
  if (usingLocalSigner()) return localPubkey();
  return extension()!.getPublicKey();
}

/** Signs a template via the active signer (NIP-07 extension, or the in-browser local key). */
export async function signTemplate(t: EventTemplate): Promise<NostrEvent> {
  if (usingLocalSigner()) return signLocally(t);
  const n = extension();
  if (!n) throw new Error("No NIP-07 signer found — switch to the in-browser key.");
  return n.signEvent({
    kind: t.kind,
    created_at: Math.floor(Date.now() / 1000),
    tags: t.tags,
    content: t.content,
  });
}
