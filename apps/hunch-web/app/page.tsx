"use client";

import { useEffect, useMemo, useState } from "react";
import Link from "next/link";
import { parseMarketEvent, parseOrderEvent, type Market, type Order, KIND_MARKET, KIND_ORDER } from "@/lib/hunch";
import { buildOrderBook, type OrderBook } from "@/lib/orderbook";
import { queryRelays, relaysFromUrl } from "@/lib/relay";
import { verifyEvent } from "@/lib/verify";
import { OddsBar } from "@/components/OddsBar";
import { Input, Select } from "@/components/ui";

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

  return (
    <div className="flex flex-col gap-6">
      <section className="flex flex-col gap-3 py-2">
        <h1 className="font-bold text-2xl sm:text-3xl">Bet on anything. Settle on Bitcoin.</h1>
        <p style={{ color: "var(--muted)" }} className="text-sm max-w-xl">
          Permissionless prediction markets on Bitcoin. No KYC, no custody, no token — outcomes are
          settled by oracle-signed Bitcoin DLCs and paid in Cashu over Lightning. Trust the math.
        </p>
        <div className="flex gap-2 text-sm">
          <Link href="/create/" className="px-4 py-2 rounded font-bold inline-block" style={{ background: "var(--accent)", color: "#000" }}>
            + Create a market
          </Link>
        </div>
      </section>

      <div className="flex gap-2 flex-wrap items-center text-sm">
        <Input
          className="flex-1 min-w-[200px]"
          placeholder="search question…"
          value={q}
          onChange={(e) => setQ(e.target.value)}
        />
        <Select value={topic} onChange={(e) => setTopic(e.target.value)}>
          <option value="">all topics</option>
          {topics.map((t) => (
            <option key={t} value={t}>
              {t}
            </option>
          ))}
        </Select>
        <Select value={stateFilter} onChange={(e) => setStateFilter(e.target.value as StateFilter)}>
          <option value="all">all</option>
          <option value="open">open</option>
          <option value="expired">expired</option>
        </Select>
      </div>

      {status && (
        <div className="flex flex-col gap-2">
          <p style={{ color: "var(--muted)" }} className="text-sm">
            {status}
          </p>
          {status === "No markets found yet." && (
            <Link href="/create/" className="self-start px-4 py-2 text-sm rounded font-bold inline-block" style={{ background: "var(--accent)", color: "#000" }}>
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
            <Link key={m.id} href={`/market?id=${encodeURIComponent(m.id)}`} className="market-card block rounded p-3">
              <div className="font-bold text-sm">{m.content.question}</div>
              <div className="mt-2">
                <OddsBar book={book} compact />
              </div>
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
