// Implied-odds bar: a YES/NO probability bar derived from the order book's best bids.
// Presentational only (no hooks) — the probability is computed by `impliedOdds` in lib/orderbook.
// Implied odds come from on-relay bids, which are untrusted and thin; this is a market signal,
// not a guarantee. Settlement is the oracle attestation, shown separately.
//
// Always renders a bar so every market shows the element: with two-sided liquidity it splits
// YES/NO; otherwise it's a flat muted track labelled "—" ("no odds yet").

import { impliedOdds, type OrderBook } from "@/lib/orderbook";

export function OddsBar({ book, compact = false }: { book?: OrderBook | null; compact?: boolean }) {
  const odds = book ? impliedOdds(book) : null;
  const has = odds !== null;
  const yes = odds?.yes ?? 50;
  const no = odds?.no ?? 50;

  return (
    <div className="flex flex-col gap-1">
      <div
        className="flex w-full overflow-hidden rounded"
        style={{ height: compact ? 6 : 10, background: "var(--border)" }}
        role="img"
        aria-label={has ? `Implied odds: YES ${yes} percent, NO ${no} percent` : "No implied odds yet"}
      >
        <div style={{ width: `${yes}%`, background: has ? "var(--accent)" : "transparent" }} />
        <div style={{ width: `${no}%`, background: has ? "var(--muted)" : "transparent" }} />
      </div>
      <div className="flex justify-between text-xs">
        <span style={{ color: has ? "var(--accent)" : "var(--muted)" }}>YES {has ? `${yes}%` : "—"}</span>
        <span style={{ color: "var(--muted)" }}>NO {has ? `${no}%` : "—"}</span>
      </div>
    </div>
  );
}
