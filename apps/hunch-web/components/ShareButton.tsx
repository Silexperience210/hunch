"use client";

// "Share on Nostr" — signs a kind:1 note (buildShareNote) with the user's key and broadcasts it to
// a wide set of public relays, so a market (or its settlement) spreads across every Nostr client.
// Opt-in and user-signed: never auto-spam. Reused on the market page, home cards, and /create.

import { useState } from "react";
import { nip19 } from "nostr-tools";
import { buildShareNote, BROADCAST_RELAYS, type ShareOpts } from "@/lib/share";
import { signTemplate } from "@/lib/sign";
import { publishAll } from "@/lib/publish";
import { Button } from "@/components/ui";

export function ShareButton({
  share,
  size = "sm",
  label = "Share on Nostr",
}: {
  share: ShareOpts;
  size?: "sm" | "md";
  label?: string;
}) {
  const [status, setStatus] = useState<string | null>(null);
  const [noteId, setNoteId] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);

  async function go() {
    setBusy(true);
    setStatus(null);
    setNoteId(null);
    try {
      const signed = await signTemplate(buildShareNote(share));
      const results = await publishAll(BROADCAST_RELAYS, signed);
      const ok = results.filter((r) => r.accepted).length;
      if (ok > 0) {
        setStatus(`✓ Broadcast to ${ok}/${results.length} relays`);
        try {
          setNoteId(nip19.noteEncode(signed.id));
        } catch {}
      } else {
        setStatus("No relay accepted it — try again");
      }
    } catch (e) {
      setStatus("Error: " + (e as Error).message);
    } finally {
      setBusy(false);
    }
  }

  return (
    <span className="inline-flex items-center gap-2 flex-wrap">
      <Button size={size} onClick={go} disabled={busy} title="Publish a signed note to public Nostr relays">
        {busy ? "Signing…" : `🟣 ${label}`}
      </Button>
      {status && (
        <span className="text-xs" style={{ color: "var(--muted)" }}>
          {status}
        </span>
      )}
      {noteId && (
        <a
          href={`https://njump.me/${noteId}`}
          target="_blank"
          rel="noreferrer"
          className="text-xs"
          style={{ color: "var(--accent)" }}
        >
          open in client ↗
        </a>
      )}
    </span>
  );
}
