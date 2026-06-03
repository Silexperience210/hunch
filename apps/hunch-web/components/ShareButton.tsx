"use client";

// "Share on Nostr" — signs a kind:1 note (buildShareNote) with the user's key and broadcasts it to
// a wide set of public relays, so a market (or its settlement) spreads across every Nostr client.
// Opt-in and user-signed: never auto-spam. Reused on the market page, home cards, and /create.

import { useState } from "react";
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
  const [busy, setBusy] = useState(false);

  async function go() {
    setBusy(true);
    setStatus(null);
    try {
      const signed = await signTemplate(buildShareNote(share));
      const results = await publishAll(BROADCAST_RELAYS, signed);
      const ok = results.filter((r) => r.accepted).length;
      setStatus(ok > 0 ? `✓ Broadcast to ${ok}/${results.length} relays` : "No relay accepted it — try again");
    } catch (e) {
      setStatus("Error: " + (e as Error).message);
    } finally {
      setBusy(false);
    }
  }

  return (
    <span className="inline-flex items-center gap-2">
      <Button size={size} onClick={go} disabled={busy} title="Publish a signed note to public Nostr relays">
        {busy ? "Signing…" : `🟣 ${label}`}
      </Button>
      {status && (
        <span className="text-xs" style={{ color: "var(--muted)" }}>
          {status}
        </span>
      )}
    </span>
  );
}
