"use client";

import { useEffect, useMemo, useState } from "react";
import Link from "next/link";
import { parseMarketEvent, parseOrderEvent, type Market, type Order, KIND_MARKET, KIND_ORDER } from "@/lib/hunch";
import { buildOrderBook, type OrderBook } from "@/lib/orderbook";
import { queryRelays, relaysFromUrl } from "@/lib/relay";
import { verifyEvent } from "@/lib/verify";
import { OddsBar } from "@/components/OddsBar";

type StateFilter = "all" | "open" | "expired";

export default function HomePage() {
  const [markets, setMarkets] = useState<Market[]>([]);
  const [books, setBooks] = useState<Map<string, OrderBook>>(new Map());
  const [status, setStatus] = useState<string>("Loading…");
  const [relays] = useState<string[]>(relaysFromUrl);

  const [q, setQ] = useState("");
  const [topic, setTopic] = useState("");
  const [stateFilter, setStateFilter] = useState<StateFilter>("all");

  useEffect(() => {
    let cancelled = false;
    (async () => {
      // Markets (kind 30888) and orders (kind 38888) in parallel — orders drive the implied-odds bar.
      const [marketEvents, orderEvents] = await Promise.all([
        queryRelays(relays, { kinds: [KIND_MARKET], limit: 200 }),
        queryRelays(relays, { kinds: [KIND_ORDER], limit: 500 }),
      ]);
      const seen = new Set<string>();
      const parsed = marketEvents
        .filter(verifyEvent)
        .map(parseMarketEvent)
        .filter((m): m is Market => m !== null)
        .filter((m) => {
          if (seen.has(m.id)) return false;
          seen.add(m.id);
          return true;
        });

      // Bucket verified orders by market, then build a display book per market.
      const byMarket = new Map<string, Order[]>();
      for (const o of orderEvents.filter(verifyEvent).map(parseOrderEvent)) {
        if (!o) continue;
        const list = byMarket.get(o.market);
        if (list) list.push(o);
        else byMarket.set(o.market, [o]);
      }
      const nextBooks = new Map<string, OrderBook>();
      for (const [mid, orders] of byMarket) nextBooks.set(mid, buildOrderBook(orders, mid));

      if (!cancelled) {
        setMarkets(parsed);
        setBooks(nextBooks);
        setStatus(parsed.length ? "" : "No markets found yet.");
      }
    })();
    return () => {
      cancelled = true;
    };
  }, [relays]);

  // Distinct topics across all markets, for the topic filter.
  const topics = useMemo(() => {
    const set = new Set<string>();
    for (const m of markets) {
      if (m.category) set.add(m.category);
      for (const t of m.topics) set.add(t);
    }
    return [...set].sort();
  }, [markets]);

  const now = Math.floor(Date.now() / 1000);
  const filtered = useMemo(() => {
    const needle = q.trim().toLowerCase();
    return markets.filter((m) => {
      if (needle && !m.content.question.toLowerCase().includes(needle)) return false;
      if (topic && m.category !== topic && !m.topics.includes(topic)) return false;
      if (stateFilter === "open" && m.expiry <= now) return false;
      if (stateFilter === "expired" && m.expiry > now) return false;
      return true;
    });
  }, [markets, q, topic, stateFilter, now]);

  const field = { background: "var(--card)", border: "1px solid var(--border)", color: "var(--fg)" } as const;

  return (
    <div className="flex flex-col gap-4">
      <div className="flex flex-col gap-1">
        <h1 className="font-bold text-lg">Hunch</h1>
        <p style={{ color: "var(--muted)" }} className="text-sm">
          Permissionless prediction markets on Bitcoin. No KYC. No custody. Trust the math.
        </p>
      </div>

      <div className="flex gap-2 flex-wrap items-center text-sm">
        <input
          style={field}
          className="px-3 py-2 rounded flex-1 min-w-[200px]"
          placeholder="search question…"
          value={q}
          onChange={(e) => setQ(e.target.value)}
        />
        <select style={field} className="px-2 py-2 rounded" value={topic} onChange={(e) => setTopic(e.target.value)}>
          <option value="">all topics</option>
          {topics.map((t) => (
            <option key={t} value={t}>
              {t}
            </option>
          ))}
        </select>
        <select style={field} className="px-2 py-2 rounded" value={stateFilter} onChange={(e) => setStateFilter(e.target.value as StateFilter)}>
          <option value="all">all</option>
          <option value="open">open</option>
          <option value="expired">expired</option>
        </select>
      </div>

      {status && (
        <div className="flex flex-col gap-2">
          <p style={{ color: "var(--muted)" }} className="text-sm">
            {status}
          </p>
          {status === "No markets found yet." && (
            <Link
              href="/create/"
              className="self-start px-4 py-2 text-sm rounded font-bold"
              style={{ background: "var(--accent)", color: "#000" }}
            >
              Create the first market →
            </Link>
          )}
        </div>
      )}
      {!status && filtered.length === 0 && (
        <p style={{ color: "var(--muted)" }} className="text-sm">
          No markets match the filters.
        </p>
      )}

      <div className="flex flex-col gap-3">
        {filtered.map((m) => {
          const expired = m.expiry <= now;
          const book = books.get(m.id);
          return (
            <Link
              key={m.id}
              href={`/market?id=${encodeURIComponent(m.id)}`}
              className="block rounded p-3"
              style={{ border: "1px solid var(--border)" }}
            >
              <div className="font-bold text-sm">{m.content.question}</div>
              {book && (
                <div className="mt-2">
                  <OddsBar book={book} compact />
                </div>
              )}
              <div style={{ color: "var(--muted)" }} className="text-xs mt-2 flex gap-3 flex-wrap">
                <span style={{ color: expired ? "var(--muted)" : "var(--accent)" }}>{expired ? "expired" : "open"}</span>
                <span>oracle {m.oracle.slice(0, 12)}…</span>
                <span>expiry {new Date(m.expiry * 1000).toISOString().slice(0, 10)}</span>
                {m.category && <span>· {m.category}</span>}
                {m.topics.map((t) => (
                  <span key={t}>#{t}</span>
                ))}
              </div>
            </Link>
          );
        })}
      </div>
    </div>
  );
}
