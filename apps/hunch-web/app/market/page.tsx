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
  type MintAnnounce,
  type OracleAnnounce,
  type OracleAttestation,
  type Order,
  type ReputationSummary,
} from "@/lib/hunch";
import { buildOrderBook, impliedOdds, type OrderBook } from "@/lib/orderbook";
import { OddsBar } from "@/components/OddsBar";
import { AmmPanel } from "@/components/AmmPanel";
import { ShareButton } from "@/components/ShareButton";
import { relaysFromUrl, queryRelays } from "@/lib/relay";
import { fetchAnnounce, fetchAttestation, fetchReputation } from "@/lib/oracle";
import { fetchMintAnnounce } from "@/lib/mint";
import { fetchDisputes } from "@/lib/disputes";
import { buildDeleteTemplate, buildDisputeTemplate, buildOrderTemplate, buildReputationTemplate } from "@/lib/build";
import { getPublicKey, signTemplate } from "@/lib/sign";
import { publishAll } from "@/lib/publish";
import { verifyEvent } from "@/lib/verify";
import { Alert, Button, Card, Input, Select } from "@/components/ui";

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
      const results = await publishAll(relaysFromUrl(), signed);
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
        <Select value={side} onChange={(e) => setSide(e.target.value as any)}>
          <option value="YES">YES</option>
          <option value="NO">NO</option>
        </Select>
        <Select value={kind} onChange={(e) => setKind(e.target.value as any)}>
          <option value="bid">bid (buy)</option>
          <option value="ask">ask (sell)</option>
        </Select>
        <Input className="w-28" value={amount} onChange={(e) => setAmount(e.target.value)} placeholder="amount sat" />
        <Input className="w-24" value={price} onChange={(e) => setPrice(e.target.value)} placeholder="price" />
        <Button variant="primary" onClick={post} disabled={busy}>
          {busy ? "Signing…" : "Sign & post"}
        </Button>
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

/** Human-readable summary of a market's auto-resolution spec, for transparency at bet time. */
function summarizeSpec(raw?: string): string {
  if (!raw) return "manual (oracle decides)";
  try {
    const s = JSON.parse(raw) as Record<string, unknown>;
    switch (s.connector) {
      case "price":
        return `auto · price ${s.asset}/${s.quote ?? "USD"} ${s.op} ${s.threshold} (${((s.sources as string[]) ?? ["coinbase", "kraken"]).join("+")})`;
      case "weather":
        return `auto · weather ${s.metric ?? "precipitation_sum"} @(${s.lat},${s.lon}) ${s.date} ${s.op} ${s.threshold}`;
      case "onchain":
        return `auto · onchain ${s.metric} ${s.op} ${s.threshold}`;
      case "http":
        return `auto · http ${s.url} ${s.op} ${s.threshold}`;
      case "llm":
        return `auto · AI/LLM verdict (oracle's model)`;
      default:
        return `auto · ${s.connector ?? "custom"}`;
    }
  } catch {
    return "custom spec";
  }
}

/** Deep-link to /bet with everything the wallet needs pre-filled (mint + nonce when known). */
function betHref(market: Market, announce: OracleAnnounce | null, opts?: { side?: "YES" | "NO"; amount?: number }): string {
  const q = new URLSearchParams({ id: market.id, oracle: market.oracle, mint: market.mint });
  if (announce) q.set("nonce", announce.nonce);
  if (opts?.side) q.set("side", opts.side);
  if (opts?.amount && opts.amount > 0) q.set("amount", String(Math.round(opts.amount)));
  return `/bet?${q.toString()}`;
}

/** The settlement: the oracle's resolved outcome + the Schnorr signature (CLAUDE.md: always visible). */
function SettlementBanner({ s }: { s: OracleAttestation }) {
  const resolved = s.outcome === "INVALID" ? "INVALID (refunds)" : s.outcome;
  return (
    <Card accent className="flex flex-col gap-1 p-3">
      <div className="text-sm font-bold" style={{ color: "var(--accent)" }}>
        Settled: {resolved}
      </div>
      {s.evidence && (
        <div className="text-sm" style={{ color: "var(--fg)" }}>
          {s.evidence}
        </div>
      )}
      <div className="text-xs" style={{ color: "var(--muted)" }}>
        Oracle Schnorr signature (verify it yourself — relays are untrusted):
      </div>
      <code className="text-xs break-all">{s.signature}</code>
    </Card>
  );
}

/** Lets the signed-in user rate an oracle (HIP-5 kind 30891), then refreshes the summary. */
function RateOracleForm({ oracle, market, onRated }: { oracle: string; market: string; onRated: () => void }) {
  const [score, setScore] = useState("50");
  const [note, setNote] = useState("");
  const [status, setStatus] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);

  async function submit() {
    setBusy(true);
    setStatus(null);
    try {
      const n = Number(score);
      if (!Number.isInteger(n) || n < -100 || n > 100) throw new Error("score must be an integer -100..100");
      const template = buildReputationTemplate({ subject: oracle, scope: "oracle", score: n, market, note: note.trim() });
      const signed = await signTemplate(template);
      const results = await publishAll(relaysFromUrl(), signed);
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
      <Input className="w-24" value={score} onChange={(e) => setScore(e.target.value)} placeholder="-100..100" />
      <Input className="flex-1 min-w-[160px]" value={note} onChange={(e) => setNote(e.target.value)} placeholder="note (optional)" />
      <Button size="sm" onClick={submit} disabled={busy}>
        {busy ? "Signing…" : "Rate oracle"}
      </Button>
      {status && <span style={{ color: "var(--muted)" }} className="text-xs w-full">{status}</span>}
    </div>
  );
}

/** Market metadata + oracle status/reputation + settlement, from the kind:30888 / 88 / 89 / 30891 events. */
function MarketMeta({ id, anchorProb }: { id: string; anchorProb: number }) {
  const [market, setMarket] = useState<Market | null>(null);
  const [announce, setAnnounce] = useState<OracleAnnounce | null>(null);
  const [settlement, setSettlement] = useState<OracleAttestation | null>(null);
  const [rep, setRep] = useState<ReputationSummary | null>(null);
  const [mintAnnounce, setMintAnnounce] = useState<MintAnnounce | null>(null);
  const [isCreator, setIsCreator] = useState(false);
  const [deleted, setDeleted] = useState(false);
  const [delMsg, setDelMsg] = useState<string | null>(null);

  // Reputation can be refreshed independently (after the user rates the oracle).
  const loadRep = useCallback(async (oracle: string) => {
    const claims = await fetchReputation(relaysFromUrl(), oracle);
    setRep(aggregateReputation(claims));
  }, []);

  useEffect(() => {
    let cancelled = false;
    (async () => {
      // id == `<creator>:30888:<d>` — query the creator's market event and verify it.
      const [creator, , ...rest] = id.split(":");
      const d = rest.join(":");
      if (!creator || !d) return;
      const events = await queryRelays(relaysFromUrl(), { kinds: [KIND_MARKET], authors: [creator], "#d": [d], limit: 5 });
      const m = events.filter(verifyEvent).map(parseMarketEvent).find((x): x is Market => x !== null && x.id === id);
      if (cancelled || !m) return;
      setMarket(m);
      getPublicKey().then((pk) => { if (!cancelled) setIsCreator(pk === m.creator); }).catch(() => {});
      const [a, s, , mint] = await Promise.all([
        fetchAnnounce(relaysFromUrl(), m.oracle, m.id),
        fetchAttestation(relaysFromUrl(), m.oracle, m.id),
        loadRep(m.oracle),
        fetchMintAnnounce(relaysFromUrl(), m.mint),
      ]);
      if (!cancelled) {
        setAnnounce(a);
        setSettlement(s);
        setMintAnnounce(mint);
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
  const reserves = mintAnnounce?.reservesProof;
  const oracleBacked = mintAnnounce ? mintAnnounce.supportedOracles.includes(market.oracle.toLowerCase()) : null;

  return (
    <div className="flex flex-col gap-4">
      <div className="flex items-start justify-between gap-2">
        <div className="font-bold">{market.content.question}</div>
        {isCreator && !deleted && (
          <Button
            size="sm"
            onClick={async () => {
              if (!confirm("Delete this market? Publishes a NIP-09 deletion signed by you (the creator).")) return;
              setDelMsg("Signing deletion…");
              try {
                const signed = await signTemplate(buildDeleteTemplate(market.creator, market.d));
                const res = await publishAll(relaysFromUrl(), signed);
                const ok = res.filter((r) => r.accepted).length;
                setDeleted(true);
                setDelMsg(`✔ Deletion published to ${ok}/${res.length} relay(s). It will disappear from the list.`);
              } catch (e) {
                setDelMsg("Error: " + (e as Error).message);
              }
            }}
          >
            Delete
          </Button>
        )}
      </div>
      {delMsg && <Alert kind={delMsg.startsWith("Error") ? "error" : delMsg.startsWith("✔") ? "ok" : "info"}>{delMsg}</Alert>}
      {settlement && <SettlementBanner s={settlement} />}
      <section className="flex flex-col gap-1 text-sm">
        <Row label="oracle" value={market.oracle} />
        <Row label="reputation" value={repText} />
        <Row label="mint" value={market.mint} />
        <div className="flex gap-2">
          <span style={{ color: "var(--muted)" }} className="w-28 shrink-0">reserves</span>
          {reserves ? (
            /^https?:\/\//.test(reserves) ? (
              <a href={reserves} target="_blank" rel="noreferrer" className="break-all" style={{ color: "var(--accent)" }}>
                {reserves}
              </a>
            ) : (
              <span className="break-all">{reserves}</span>
            )
          ) : (
            <span className="break-all" style={{ color: "var(--muted)" }}>
              no mint announce
            </span>
          )}
        </div>
        {oracleBacked !== null && (
          <Row label="oracle backed" value={oracleBacked ? "yes — mint accepts this oracle" : "⚠ mint does not list this oracle"} />
        )}
        <Row label="dlc_contract" value={market.dlcContract} />
        <Row label="expiry" value={new Date(market.expiry * 1000).toISOString()} />
        <Row label="resolution" value={market.content.resolution_criteria || "—"} />
        <Row label="resolves by" value={summarizeSpec(market.resolutionSpec)} />
        <Row label="oracle nonce" value={announce ? `committed (${announce.nonce.slice(0, 16)}…)` : "not announced yet"} />
      </section>
      <RateOracleForm oracle={market.oracle} market={market.id} onRated={() => loadRep(market.oracle)} />
      {!settlement && (
        <AmmPanel anchorProb={anchorProb} href={(side, amount) => betHref(market, announce, { side, amount })} />
      )}
      <div className="flex items-center gap-3 flex-wrap">
        <Link
          href={betHref(market, announce)}
          className="text-sm px-4 py-2 rounded font-bold"
          style={{ background: "var(--accent)", color: "#000" }}
        >
          Bet (advanced) →
        </Link>
        <ShareButton
          share={{
            question: market.content.question,
            id: market.id,
            yes: settlement ? undefined : Math.round(anchorProb * 100),
            settled: settlement ? { outcome: settlement.outcome, reasoning: settlement.evidence } : null,
          }}
        />
      </div>
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

  const load = useCallback(async () => {
    setDisputes(await fetchDisputes(relaysFromUrl(), market));
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
      const results = await publishAll(relaysFromUrl(), signed);
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
          <Select value={claim} onChange={(e) => setClaim(e.target.value)}>
            {CLAIM_CATEGORIES.map((c) => (
              <option key={c} value={c}>
                {c}
              </option>
            ))}
          </Select>
          <Input className="flex-1 min-w-[200px]" value={evidence} onChange={(e) => setEvidence(e.target.value)} placeholder="evidence (url / event id / note)" />
          <Button size="sm" onClick={submit} disabled={busy}>
            {busy ? "Signing…" : "Raise dispute"}
          </Button>
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

  // The AMM anchors its 1-click price on the book's implied odds (else 50/50).
  const odds = book ? impliedOdds(book) : null;
  const anchorProb = odds ? odds.yes / 100 : 0.5;

  async function load() {
    if (!id) {
      setError("No market id (?id=<creator>:30888:<d>).");
      return;
    }
    setLoading(true);
    setError(null);
    try {
      // Orders carry a single-letter `d` tag == market, so relays can filter by #d.
      const events = await queryRelays(relaysFromUrl(), { kinds: [KIND_ORDER], "#d": [id], limit: 500 });
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

      {id && <MarketMeta id={id} anchorProb={anchorProb} />}

      {book && (
        <section className="flex flex-col gap-1">
          <div className="text-xs" style={{ color: "var(--muted)" }}>
            implied odds (from the order book)
          </div>
          <OddsBar book={book} />
        </section>
      )}

      <Button variant="primary" className="self-start" onClick={load} disabled={loading}>
        {loading ? "Loading…" : "Refresh book"}
      </Button>

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
