"use client";

import { useEffect, useMemo, useState } from "react";
import Link from "next/link";
import { parseMarketEvent, parseOrderEvent, parseAttestationEvent, type Market, type Order, KIND_MARKET, KIND_ORDER, KIND_ORACLE_ATTESTATION } from "@/lib/hunch";
import { buildOrderBook, type OrderBook } from "@/lib/orderbook";
import { queryRelays, relaysFromUrl } from "@/lib/relay";
import { verifyEvent } from "@/lib/verify";
import { OddsBar } from "@/components/OddsBar";
import { Input, Select } from "@/components/ui";

type StateFilter = "live" | "open" | "resolved" | "all";
interface Settled { at: number; outcome: string }
const SETTLED_WINDOW = 2 * 24 * 3600; // resolved markets stay on the main list for 2 days

export default function HomePage() {
  const [markets, setMarkets] = useState<Market[]>([]);
  const [books, setBooks] = useState<Map<string, OrderBook>>(new Map());
  const [settled, setSettled] = useState<Map<string, Settled>>(new Map());
  const [status, setStatus] = useState<string>("Loading…");
  const [relays] = useState<string[]>(relaysFromUrl);

  const [q, setQ] = useState("");
  const [topic, setTopic] = useState("");
  const [stateFilter, setStateFilter] = useState<StateFilter>("live");

  useEffect(() => {
    let cancelled = false;
    (async () => {
      // Markets (kind 30888) and orders (kind 38888) in parallel — orders drive the implied-odds bar.
      const [marketEvents, orderEvents, delEvents, attEvents] = await Promise.all([
        queryRelays(relays, { kinds: [KIND_MARKET], limit: 200 }),
        queryRelays(relays, { kinds: [KIND_ORDER], limit: 500 }),
        queryRelays(relays, { kinds: [5], limit: 200 }), // NIP-09 deletions
        queryRelays(relays, { kinds: [KIND_ORACLE_ATTESTATION], limit: 300 }), // settlements
      ]);

      // Settlements: a market is resolved by an attestation (kind 89) from ITS oracle.
      // marketId -> [{by, at, outcome}], newest valid one wins (matched to the market's oracle below).
      const attByMarket = new Map<string, { by: string; at: number; outcome: string }[]>();
      for (const ev of attEvents.filter(verifyEvent)) {
        const a = parseAttestationEvent(ev);
        if (!a) continue;
        const arr = attByMarket.get(a.market) ?? [];
        arr.push({ by: ev.pubkey, at: ev.created_at, outcome: a.outcome });
        attByMarket.set(a.market, arr);
      }

      // NIP-09: a market is deleted if its creator published a kind:5 referencing its `a` coordinate.
      const deleted = new Set<string>();
      for (const ev of delEvents.filter(verifyEvent)) {
        for (const t of ev.tags) {
          if (t[0] === "a" && typeof t[1] === "string") {
            const [k, pk, ...rest] = t[1].split(":");
            if (k === String(KIND_MARKET) && pk === ev.pubkey) deleted.add(`${pk}:${KIND_MARKET}:${rest.join(":")}`);
          }
        }
      }

      const seen = new Set<string>();
      const parsed = marketEvents
        .filter(verifyEvent)
        .map(parseMarketEvent)
        .filter((m): m is Market => m !== null)
        .filter((m) => !deleted.has(m.id))
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

      // A market is settled iff one of its attestations is from its own oracle (newest wins).
      const settledMap = new Map<string, Settled>();
      for (const m of parsed) {
        const valid = (attByMarket.get(m.id) ?? []).filter((a) => a.by === m.oracle).sort((a, b) => b.at - a.at)[0];
        if (valid) settledMap.set(m.id, { at: valid.at, outcome: valid.outcome });
      }

      if (!cancelled) {
        setMarkets(parsed);
        setBooks(nextBooks);
        setSettled(settledMap);
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

  // Activity = total live orders in a market's book; drives the "most active" ranking + badge.
  const activity = useMemo(() => {
    const a = new Map<string, number>();
    for (const [id, b] of books) a.set(id, b.yesBids.length + b.yesAsks.length + b.noBids.length + b.noAsks.length);
    return a;
  }, [books]);

  const now = Math.floor(Date.now() / 1000);
  const filtered = useMemo(() => {
    const needle = q.trim().toLowerCase();
    return markets
      .filter((m) => {
        if (needle && !m.content.question.toLowerCase().includes(needle)) return false;
        if (topic && m.category !== topic && !m.topics.includes(topic)) return false;
        const s = settled.get(m.id);
        const freshlySettled = s ? now - s.at <= SETTLED_WINDOW : false;
        if (stateFilter === "resolved") return !!s; // all settled markets
        if (stateFilter === "open") return !s && m.expiry > now;
        if (stateFilter === "live") return !s || freshlySettled; // open + resolved < 2 days
        return true; // "all"
      })
      // Most active first, then soonest to close.
      .sort((x, y) => (activity.get(y.id) ?? 0) - (activity.get(x.id) ?? 0) || x.expiry - y.expiry);
  }, [markets, q, topic, stateFilter, now, activity, settled]);

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
          <option value="live">live</option>
          <option value="open">open</option>
          <option value="resolved">resolved</option>
          <option value="all">all</option>
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
        {filtered.map((m, i) => {
          const expired = m.expiry <= now;
          const book = books.get(m.id);
          const acts = activity.get(m.id) ?? 0;
          const hot = acts > 0 && i === 0; // most active = first (sorted by activity)
          return (
            <Link
              key={m.id}
              href={`/market?id=${encodeURIComponent(m.id)}`}
              className="market-card block rounded p-3"
              style={hot ? { borderColor: "var(--accent)" } : undefined}
            >
              <div className="flex items-start justify-between gap-2">
                <div className="font-bold text-sm">{m.content.question}</div>
                {acts > 0 && (
                  <span className="text-xs whitespace-nowrap" style={{ color: "var(--accent)" }}>
                    🔥 {acts} order{acts === 1 ? "" : "s"}
                  </span>
                )}
              </div>
              <div className="mt-2">
                <OddsBar book={book} compact />
              </div>
              <div style={{ color: "var(--muted)" }} className="text-xs mt-2 flex gap-3 flex-wrap">
                {hot && <span style={{ color: "var(--accent)" }}>most active</span>}
                {(() => {
                  const s = settled.get(m.id);
                  return s ? (
                    <span style={{ color: "var(--accent)" }}>settled: {s.outcome}</span>
                  ) : (
                    <span style={{ color: expired ? "var(--muted)" : "var(--accent)" }}>{expired ? "expired" : "open"}</span>
                  );
                })()}
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
