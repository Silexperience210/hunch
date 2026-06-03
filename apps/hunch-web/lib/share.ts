// Nostr social broadcast — turn a market (or its settlement) into a standard kind:1 note that any
// Nostr client renders and any relay accepts. This is Hunch's distribution layer: the protocol
// events (kind 30888/89…) live on Hunch relays (public relays reject those kinds), but a plain note
// linking to the market spreads everywhere. Notes are signed by the user's own key (NIP-07 / local /
// Amber) and opt-in — never auto-spam. The settlement note carries the oracle's verdict + reasoning
// so a resolved market broadcasts its result with a verifiable receipt (link back to the signed kind:89).
//
// buildShareNote / marketUrl are pure + offline-tested; signing & publishing reuse sign.ts/publish.ts.

import type { EventTemplate } from "./build.ts";

/** Standard short text note (NIP-01). */
export const KIND_NOTE = 1;

/** Canonical public site for share links. Override with NEXT_PUBLIC_SITE_URL at build time. */
export const SITE_URL =
  (typeof process !== "undefined" && process.env?.NEXT_PUBLIC_SITE_URL) ||
  "https://silexperience210.github.io/hunch";

/** Public relays with open writes (probed live) to broadcast notes to — Hunch is multi-relay by
 * design (CLAUDE.md). Paid/restricted relays (nostr.wine, nostr.land, nsec.app) are intentionally
 * omitted; they reject anonymous writes. */
export const BROADCAST_RELAYS = [
  "wss://relay.damus.io",
  "wss://nos.lol",
  "wss://relay.primal.net",
  "wss://offchain.pub",
  "wss://nostr.mom",
  "wss://nostr-pub.wellorder.net",
  "wss://relay.mostr.pub",
  "wss://nostr.bitcoiner.social",
  "wss://relay.nostr.net",
  "wss://relay.21pay.org",
];

/** Absolute, shareable URL to a market's page. */
export function marketUrl(id: string, site: string = SITE_URL): string {
  return `${site.replace(/\/+$/, "")}/market/?id=${encodeURIComponent(id)}`;
}

export interface ShareOpts {
  question: string;
  /** Absolute URL to the market (defaults to building one from `id` if given). */
  url?: string;
  id?: string;
  /** Implied P(YES) as a 0..100 percent, or null/undefined when there are no odds yet. */
  yes?: number | null;
  /** When set, the note announces the settlement instead of an open market. */
  settled?: { outcome: string; reasoning?: string } | null;
}

function clampPct(n: number): number {
  return Math.max(0, Math.min(100, Math.round(n)));
}

/** Build a kind:1 note announcing a market or its settlement. Pure. */
export function buildShareNote(o: ShareOpts): EventTemplate {
  const url = o.url ?? (o.id ? marketUrl(o.id) : SITE_URL);
  const q = o.question.trim();
  let content: string;

  const tagline = "#hunch #bitcoin #predictions";

  if (o.settled) {
    const oc = o.settled.outcome.toUpperCase();
    const head = oc === "INVALID" ? "♻️ Settled: INVALID — bets refunded" : `✅ Settled: ${oc}`;
    const why = o.settled.reasoning?.trim();
    content = [
      head,
      `🎲 ${q}`,
      "",
      why || null,
      why ? "" : null,
      "🔏 Oracle-signed — verify the signature yourself:",
      `👉 ${url}`,
      "",
      tagline,
    ]
      .filter((l): l is string => l !== null)
      .join("\n");
  } else {
    const odds =
      o.yes === null || o.yes === undefined
        ? "New market — be the first to set the odds."
        : `YES ${clampPct(o.yes)}%  ·  NO ${clampPct(100 - o.yes)}%`;
    content = [
      `🎲 ${q}`,
      "",
      odds,
      "",
      "Bet in Bitcoin ecash — no KYC, no account.",
      `👉 ${url}`,
      "",
      tagline,
    ].join("\n");
  }

  return {
    kind: KIND_NOTE,
    tags: [
      ["t", "hunch"],
      ["t", "bitcoin"],
      ["t", "predictionmarket"],
      ["r", url],
    ],
    content,
  };
}
