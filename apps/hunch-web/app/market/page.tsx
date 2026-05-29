"use client";

import { Suspense, useEffect, useState } from "react";
import { useSearchParams } from "next/navigation";
import { KIND_ORDER, parseOrderEvent, type Order } from "@/lib/hunch";
import { buildOrderBook, type OrderBook } from "@/lib/orderbook";
import { DEFAULT_RELAYS, queryRelays } from "@/lib/relay";

function Column({ title, orders }: { title: string; orders: Order[] }) {
  return (
    <div className="flex-1 min-w-[180px]">
      <div style={{ color: "var(--muted)" }} className="text-xs mb-1">
        {title}
      </div>
      {orders.length === 0 ? (
        <div style={{ color: "var(--muted)" }} className="text-xs">
          —
        </div>
      ) : (
        <ul className="text-sm flex flex-col gap-1">
          {orders.map((o) => (
            <li key={o.author + o.price} className="flex justify-between gap-3">
              <span style={{ color: "var(--accent)" }}>{o.price} sat</span>
              <span style={{ color: "var(--muted)" }}>
                {o.amount} · {o.author.slice(0, 8)}…
              </span>
            </li>
          ))}
        </ul>
      )}
    </div>
  );
}

function MarketView() {
  const params = useSearchParams();
  const id = params.get("id") ?? "";
  const [book, setBook] = useState<OrderBook | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  async function load() {
    if (!id) {
      setError("No market id (?id=<creator>:30888:<d>).");
      return;
    }
    setLoading(true);
    setError(null);
    try {
      // Orders carry a single-letter `d` tag == market, so relays can filter by #d.
      const events = await queryRelays(DEFAULT_RELAYS, { kinds: [KIND_ORDER], "#d": [id], limit: 500 });
      const orders = events.map(parseOrderEvent).filter((o): o is Order => o !== null);
      setBook(buildOrderBook(orders, id));
      if (orders.length === 0) setError("No orders for this market yet.");
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  }

  useEffect(() => {
    load();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [id]);

  return (
    <div className="flex flex-col gap-5">
      <div>
        <a href="/" className="text-sm">
          ← markets
        </a>
        <h1 className="text-sm mt-2 break-all" style={{ color: "var(--muted)" }}>
          {id}
        </h1>
      </div>

      <button
        onClick={load}
        disabled={loading}
        className="self-start px-4 py-2 text-sm rounded font-bold"
        style={{ background: "var(--accent)", color: "#000" }}
      >
        {loading ? "Loading…" : "Refresh book"}
      </button>

      {error && (
        <p style={{ color: "var(--muted)" }} className="text-sm">
          {error}
        </p>
      )}

      {book && (
        <div className="flex flex-col gap-6">
          <section>
            <div className="font-bold mb-2">YES</div>
            <div className="flex gap-6 flex-wrap">
              <Column title="bids (buy)" orders={book.yesBids} />
              <Column title="asks (sell)" orders={book.yesAsks} />
            </div>
          </section>
          <section>
            <div className="font-bold mb-2">NO</div>
            <div className="flex gap-6 flex-wrap">
              <Column title="bids (buy)" orders={book.noBids} />
              <Column title="asks (sell)" orders={book.noAsks} />
            </div>
          </section>
        </div>
      )}
    </div>
  );
}

export default function MarketPage() {
  return (
    <Suspense fallback={<p style={{ color: "var(--muted)" }}>Loading…</p>}>
      <MarketView />
    </Suspense>
  );
}
