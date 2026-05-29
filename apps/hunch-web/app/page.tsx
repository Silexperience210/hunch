"use client";

import { useEffect, useState } from "react";
import { KIND_MARKET, parseMarketEvent, type Market } from "@/lib/hunch";
import { DEFAULT_RELAYS, queryRelays } from "@/lib/relay";
import { verifyEvent } from "@/lib/verify";

export default function MarketsPage() {
  const [relaysInput, setRelaysInput] = useState(DEFAULT_RELAYS.join(", "));
  const [markets, setMarkets] = useState<Market[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  async function load() {
    setLoading(true);
    setError(null);
    try {
      const relays = relaysInput.split(",").map((s) => s.trim()).filter(Boolean);
      const events = await queryRelays(relays, { kinds: [KIND_MARKET], limit: 200 });
      const parsed = events
        .filter(verifyEvent) // relays are untrusted — drop forged/tampered events
        .map(parseMarketEvent)
        .filter((m): m is Market => m !== null)
        .sort((a, b) => b.expiry - a.expiry);
      setMarkets(parsed);
      if (parsed.length === 0) setError("No markets found on these relays.");
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  }

  useEffect(() => {
    load();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return (
    <div className="flex flex-col gap-5">
      <div className="flex gap-2 items-center flex-wrap">
        <input
          value={relaysInput}
          onChange={(e) => setRelaysInput(e.target.value)}
          spellCheck={false}
          className="flex-1 min-w-[280px] px-3 py-2 text-sm rounded"
          style={{ background: "var(--card)", border: "1px solid var(--border)", color: "var(--fg)" }}
          placeholder="wss://relay1, wss://relay2"
        />
        <button
          onClick={load}
          disabled={loading}
          className="px-4 py-2 text-sm rounded font-bold"
          style={{ background: "var(--accent)", color: "#000" }}
        >
          {loading ? "Loading…" : "Refresh"}
        </button>
      </div>

      {error && (
        <p style={{ color: "var(--muted)" }} className="text-sm">
          {error}
        </p>
      )}

      <ul className="flex flex-col gap-3">
        {markets.map((m) => (
          <li
            key={m.id}
            className="p-4 rounded"
            style={{ background: "var(--card)", border: "1px solid var(--border)" }}
          >
            <a href={`/market/?id=${encodeURIComponent(m.id)}`} className="font-bold" style={{ color: "var(--fg)" }}>
              {m.content.question}
            </a>
            <div style={{ color: "var(--muted)" }} className="text-xs mt-2 flex flex-wrap gap-x-4 gap-y-1">
              <span>oracle {m.oracle.slice(0, 12)}…</span>
              <span>expiry {new Date(m.expiry * 1000).toISOString().slice(0, 10)}</span>
              <span>mint {m.mint}</span>
              {m.topics.length > 0 && <span>#{m.topics.join(" #")}</span>}
            </div>
            {m.content.resolution_criteria && (
              <div style={{ color: "var(--muted)" }} className="text-xs mt-2">
                {m.content.resolution_criteria}
              </div>
            )}
          </li>
        ))}
      </ul>
    </div>
  );
}
