"use client";

import { Suspense, useCallback, useEffect, useState } from "react";
import { useSearchParams } from "next/navigation";
import Link from "next/link";
import {
  aggregateReputation,
  KIND_MARKET,
  KIND_ORDER,
  parseMarketEvent,
  parseOrderEvent,
  type Dispute,
  type Market,
  type OracleAnnounce,
  type OracleAttestation,
  type Order,
  type ReputationSummary,
} from "@/lib/hunch";
import { buildOrderBook, type OrderBook } from "@/lib/orderbook";
import { DEFAULT_RELAYS, queryRelays } from "@/lib/relay";
import { fetchAnnounce, fetchAttestation, fetchReputation } from "@/lib/oracle";
import { fetchDisputes } from "@/lib/disputes";
import { buildDisputeTemplate, buildOrderTemplate, buildReputationTemplate } from "@/lib/build";
import { signTemplate } from "@/lib/sign";
import { publishAll } from "@/lib/publish";
import { verifyEvent } from "@/lib/verify";

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

function OrderForm({ market, onPosted }: { market: string; onPosted: () => void }) {
  const [side, setSide] = useState<"YES" | "NO">("YES");
  const [kind, setKind] = useState<"bid" | "ask">("bid");
  const [amount, setAmount] = useState("10000");
  const [price, setPrice] = useState("50");
  const [status, setStatus] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);
  const field = { background: "var(--card)", border: "1px solid var(--border)", color: "var(--fg)" } as const;

  async function post() {
    setBusy(true);
    setStatus(null);
    try {
      const template = buildOrderTemplate({
        market,
        side,
        kind,
        amount: Number(amount),
        price: Number(price),
        expires: Math.floor(Date.now() / 1000) + 30 * 24 * 3600,
      });
      const signed = await signTemplate(template);
      const results = await publishAll(DEFAULT_RELAYS, signed);
      const ok = results.filter((r) => r.accepted).length;
      setStatus(`Published to ${ok}/${results.length} relays.`);
      onPosted();
    } catch (e) {
      setStatus("Error: " + (e as Error).message);
    } finally {
      setBusy(false);
    }
  }

  return (
    <section className="flex flex-col gap-2" style={{ borderTop: "1px solid var(--border)", paddingTop: 16 }}>
      <div className="font-bold">Post an order</div>
      <div className="flex gap-2 flex-wrap items-center text-sm">
        <select style={field} className="px-2 py-2 rounded" value={side} onChange={(e) => setSide(e.target.value as any)}>
          <option value="YES">YES</option>
          <option value="NO">NO</option>
        </select>
        <select style={field} className="px-2 py-2 rounded" value={kind} onChange={(e) => setKind(e.target.value as any)}>
          <option value="bid">bid (buy)</option>
          <option value="ask">ask (sell)</option>
        </select>
        <input style={field} className="px-2 py-2 rounded w-28" value={amount} onChange={(e) => setAmount(e.target.value)} placeholder="amount sat" />
        <input style={field} className="px-2 py-2 rounded w-24" value={price} onChange={(e) => setPrice(e.target.value)} placeholder="price" />
        <button onClick={post} disabled={busy} className="px-4 py-2 rounded font-bold" style={{ background: "var(--accent)", color: "#000" }}>
          {busy ? "Signing…" : "Sign & post"}
        </button>
      </div>
      {status && <p style={{ color: "var(--muted)" }} className="text-xs">{status}</p>}
    </section>
  );
}

function Row({ label, value }: { label: string; value: string }) {
  return (
    <div className="flex gap-2">
      <span style={{ color: "var(--muted)" }} className="w-28 shrink-0">{label}</span>
      <span className="break-all">{value}</span>
    </div>
  );
}

/** Deep-link to /bet with everything the wallet needs pre-filled (mint + nonce when known). */
function betHref(market: Market, announce: OracleAnnounce | null): string {
  const q = new URLSearchParams({ id: market.id, oracle: market.oracle, mint: market.mint });
  if (announce) q.set("nonce", announce.nonce);
  return `/bet?${q.toString()}`;
}

/** The settlement: the oracle's resolved outcome + the Schnorr signature (CLAUDE.md: always visible). */
function SettlementBanner({ s }: { s: OracleAttestation }) {
  const resolved = s.outcome === "INVALID" ? "INVALID (refunds)" : s.outcome;
  return (
    <section className="flex flex-col gap-1 rounded p-3" style={{ border: "1px solid var(--accent)" }}>
      <div className="text-sm font-bold" style={{ color: "var(--accent)" }}>
        Settled: {resolved}
      </div>
      <div className="text-xs" style={{ color: "var(--muted)" }}>
        Oracle Schnorr signature (verify it yourself — relays are untrusted):
      </div>
      <code className="text-xs break-all">{s.signature}</code>
    </section>
  );
}

/** Lets the signed-in user rate an oracle (HIP-5 kind 30891), then refreshes the summary. */
function RateOracleForm({ oracle, market, onRated }: { oracle: string; market: string; onRated: () => void }) {
  const [score, setScore] = useState("50");
  const [note, setNote] = useState("");
  const [status, setStatus] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);
  const field = { background: "var(--card)", border: "1px solid var(--border)", color: "var(--fg)" } as const;

  async function submit() {
    setBusy(true);
    setStatus(null);
    try {
      const n = Number(score);
      if (!Number.isInteger(n) || n < -100 || n > 100) throw new Error("score must be an integer -100..100");
      const template = buildReputationTemplate({ subject: oracle, scope: "oracle", score: n, market, note: note.trim() });
      const signed = await signTemplate(template);
      const results = await publishAll(DEFAULT_RELAYS, signed);
      const ok = results.filter((r) => r.accepted).length;
      setStatus(`Published to ${ok}/${results.length} relays.`);
      onRated();
    } catch (e) {
      setStatus("Error: " + (e as Error).message);
    } finally {
      setBusy(false);
    }
  }

  return (
    <div className="flex gap-2 flex-wrap items-center text-sm">
      <input style={field} className="px-2 py-1 rounded w-24" value={score} onChange={(e) => setScore(e.target.value)} placeholder="-100..100" />
      <input style={field} className="px-2 py-1 rounded flex-1 min-w-[160px]" value={note} onChange={(e) => setNote(e.target.value)} placeholder="note (optional)" />
      <button onClick={submit} disabled={busy} className="px-3 py-1 rounded" style={field}>
        {busy ? "Signing…" : "Rate oracle"}
      </button>
      {status && <span style={{ color: "var(--muted)" }} className="text-xs w-full">{status}</span>}
    </div>
  );
}

/** Market metadata + oracle status/reputation + settlement, from the kind:30888 / 88 / 89 / 30891 events. */
function MarketMeta({ id }: { id: string }) {
  const [market, setMarket] = useState<Market | null>(null);
  const [announce, setAnnounce] = useState<OracleAnnounce | null>(null);
  const [settlement, setSettlement] = useState<OracleAttestation | null>(null);
  const [rep, setRep] = useState<ReputationSummary | null>(null);

  // Reputation can be refreshed independently (after the user rates the oracle).
  const loadRep = useCallback(async (oracle: string) => {
    const claims = await fetchReputation(DEFAULT_RELAYS, oracle);
    setRep(aggregateReputation(claims));
  }, []);

  useEffect(() => {
    let cancelled = false;
    (async () => {
      // id == `<creator>:30888:<d>` — query the creator's market event and verify it.
      const [creator, , ...rest] = id.split(":");
      const d = rest.join(":");
      if (!creator || !d) return;
      const events = await queryRelays(DEFAULT_RELAYS, { kinds: [KIND_MARKET], authors: [creator], "#d": [d], limit: 5 });
      const m = events.filter(verifyEvent).map(parseMarketEvent).find((x): x is Market => x !== null && x.id === id);
      if (cancelled || !m) return;
      setMarket(m);
      const [a, s] = await Promise.all([
        fetchAnnounce(DEFAULT_RELAYS, m.oracle, m.id),
        fetchAttestation(DEFAULT_RELAYS, m.oracle, m.id),
        loadRep(m.oracle),
      ]);
      if (!cancelled) {
        setAnnounce(a);
        setSettlement(s);
      }
    })();
    return () => {
      cancelled = true;
    };
  }, [id, loadRep]);

  if (!market) return null;

  const repText = rep
    ? `${rep.avg >= 0 ? "+" : ""}${rep.avg} (range -100..100, ${rep.count} rater${rep.count === 1 ? "" : "s"})`
    : "no ratings yet";

  return (
    <div className="flex flex-col gap-4">
      <div className="font-bold">{market.content.question}</div>
      {settlement && <SettlementBanner s={settlement} />}
      <section className="flex flex-col gap-1 text-sm">
        <Row label="oracle" value={market.oracle} />
        <Row label="reputation" value={repText} />
        <Row label="mint" value={market.mint} />
        <Row label="dlc_contract" value={market.dlcContract} />
        <Row label="expiry" value={new Date(market.expiry * 1000).toISOString()} />
        <Row label="resolution" value={market.content.resolution_criteria || "—"} />
        <Row label="oracle nonce" value={announce ? `committed (${announce.nonce.slice(0, 16)}…)` : "not announced yet"} />
      </section>
      <RateOracleForm oracle={market.oracle} market={market.id} onRated={() => loadRep(market.oracle)} />
      <Link
        href={betHref(market, announce)}
        className="self-start text-sm px-4 py-2 rounded font-bold"
        style={{ background: "var(--accent)", color: "#000" }}
      >
        Bet →
      </Link>
      <Disputes market={market.id} attestationId={settlement?.eventId} />
    </div>
  );
}

const CLAIM_CATEGORIES = ["oracle_misread", "source_unavailable", "ambiguous_criteria", "premature", "other"];

/** Lists disputes for a market (kind 30890) and lets the signed-in user contest the attestation. */
function Disputes({ market, attestationId }: { market: string; attestationId?: string }) {
  const [disputes, setDisputes] = useState<Dispute[]>([]);
  const [claim, setClaim] = useState(CLAIM_CATEGORIES[0]);
  const [evidence, setEvidence] = useState("");
  const [status, setStatus] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);
  const field = { background: "var(--card)", border: "1px solid var(--border)", color: "var(--fg)" } as const;

  const load = useCallback(async () => {
    setDisputes(await fetchDisputes(DEFAULT_RELAYS, market));
  }, [market]);

  useEffect(() => {
    load();
  }, [load]);

  async function submit() {
    setBusy(true);
    setStatus(null);
    try {
      if (!attestationId) throw new Error("nothing to dispute yet — the oracle has not attested");
      const template = buildDisputeTemplate({
        market,
        attestation: attestationId,
        claim: claim.trim(),
        evidence: evidence.trim() || undefined,
      });
      const signed = await signTemplate(template);
      const results = await publishAll(DEFAULT_RELAYS, signed);
      const ok = results.filter((r) => r.accepted).length;
      setStatus(`Published to ${ok}/${results.length} relays.`);
      load();
    } catch (e) {
      setStatus("Error: " + (e as Error).message);
    } finally {
      setBusy(false);
    }
  }

  return (
    <section className="flex flex-col gap-2" style={{ borderTop: "1px solid var(--border)", paddingTop: 16 }}>
      <div className="font-bold">Disputes</div>
      {disputes.length === 0 ? (
        <p className="text-xs" style={{ color: "var(--muted)" }}>
          No disputes raised.
        </p>
      ) : (
        <ul className="flex flex-col gap-2 text-sm">
          {disputes.map((d) => (
            <li key={d.disputer} className="flex flex-col gap-1 rounded p-2" style={{ border: "1px solid var(--border)" }}>
              <div className="flex gap-2 flex-wrap">
                <span style={{ color: "var(--accent)" }}>{d.claim}</span>
                <span style={{ color: "var(--muted)" }}>· {d.disputer.slice(0, 8)}…</span>
                <span style={{ color: "var(--muted)" }}>· re {d.attestation.slice(0, 10)}…</span>
              </div>
              {d.evidence && <div className="text-xs break-all">{d.evidence}</div>}
            </li>
          ))}
        </ul>
      )}
      {attestationId ? (
        <div className="flex gap-2 flex-wrap items-center text-sm">
          <select style={field} className="px-2 py-1 rounded" value={claim} onChange={(e) => setClaim(e.target.value)}>
            {CLAIM_CATEGORIES.map((c) => (
              <option key={c} value={c}>
                {c}
              </option>
            ))}
          </select>
          <input style={field} className="px-2 py-1 rounded flex-1 min-w-[200px]" value={evidence} onChange={(e) => setEvidence(e.target.value)} placeholder="evidence (url / event id / note)" />
          <button onClick={submit} disabled={busy} className="px-3 py-1 rounded" style={field}>
            {busy ? "Signing…" : "Raise dispute"}
          </button>
        </div>
      ) : (
        <p className="text-xs" style={{ color: "var(--muted)" }}>
          A dispute contests the oracle attestation — available once the market is settled.
        </p>
      )}
      {status && (
        <span style={{ color: "var(--muted)" }} className="text-xs">
          {status}
        </span>
      )}
    </section>
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
      const orders = events
        .filter(verifyEvent) // untrusted relays — only verified orders enter the book
        .map(parseOrderEvent)
        .filter((o): o is Order => o !== null);
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
        <Link href="/" className="text-sm">
          ← markets
        </Link>
        <h1 className="text-sm mt-2 break-all" style={{ color: "var(--muted)" }}>
          {id}
        </h1>
      </div>

      {id && <MarketMeta id={id} />}

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

      {id && <OrderForm market={id} onPosted={load} />}
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
