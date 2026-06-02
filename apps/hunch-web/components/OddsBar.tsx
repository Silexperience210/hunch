// Implied-odds bar: a YES/NO probability bar derived from the order book's best bids.
// Presentational only (no hooks) — the probability is computed by `impliedOdds` in lib/orderbook.
// Implied odds come from on-relay bids, which are untrusted and thin; this is a market signal,
// not a guarantee. Settlement is the oracle attestation, shown separately.

import { impliedOdds, type OrderBook } from "@/lib/orderbook";

export function OddsBar({ book, compact = false }: { book: OrderBook; compact?: boolean }) {
  const odds = impliedOdds(book);

  if (!odds) {
    return (
      <div className="text-xs" style={{ color: "var(--muted)" }}>
        {compact ? "no implied odds yet" : "No implied odds yet — needs a YES and a NO bid in the book."}
      </div>
    );
  }

  const { yes, no } = odds;
  return (
    <div className="flex flex-col gap-1">
      <div
        className="flex w-full overflow-hidden rounded"
        style={{ height: compact ? 6 : 10, background: "var(--border)" }}
        role="img"
        aria-label={`Implied odds: YES ${yes} percent, NO ${no} percent`}
      >
        <div style={{ width: `${yes}%`, background: "var(--accent)" }} />
        <div style={{ width: `${no}%`, background: "var(--muted)" }} />
      </div>
      <div className="flex justify-between text-xs">
        <span style={{ color: "var(--accent)" }}>YES {yes}%</span>
        <span style={{ color: "var(--muted)" }}>NO {no}%</span>
      </div>
    </div>
  );
}
