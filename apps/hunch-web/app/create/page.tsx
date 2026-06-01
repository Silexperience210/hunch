"use client";

import { useMemo, useState } from "react";
import Link from "next/link";
import { buildMarketTemplate } from "@/lib/build";
import { marketId } from "@/lib/hunch";
import { signTemplate } from "@/lib/sign";
import { publishAll } from "@/lib/publish";
import { relaysFromUrl } from "@/lib/relay";

const field = {
  background: "var(--card)",
  border: "1px solid var(--border)",
  color: "var(--fg)",
} as const;

// The 21pay oracle + mint running on the Umbrel — proposed by default so creating a market needs
// only a question + a date. Advanced users can override with their own oracle/mint below.
const DEFAULT_ORACLE = "b32187c658b01420003049758660e62e4a7dd3daefac42076cd1664adce0e335";
const DEFAULT_MINT = "https://mint.21pay.org";
// dlc_contract is required by the protocol but not used in the signet/test deployment yet.
const PLACEHOLDER_DLC = "0000000000000000000000000000000000000000000000000000000000000000:0";

/** Turns a question into a short URL-safe slug. */
function slugify(q: string): string {
  return q
    .toLowerCase()
    .normalize("NFD")
    .replace(/[̀-ͯ]/g, "")
    .replace(/[^a-z0-9]+/g, "-")
    .replace(/^-+|-+$/g, "")
    .slice(0, 48) || "market";
}

export default function CreateMarketPage() {
  const [question, setQuestion] = useState("");
  const [resolution, setResolution] = useState("");
  const [expiry, setExpiry] = useState("");
  const [advanced, setAdvanced] = useState(false);
  const [oracle, setOracle] = useState(DEFAULT_ORACLE);
  const [mint, setMint] = useState(DEFAULT_MINT);
  const [relays, setRelays] = useState(relaysFromUrl().join(", "));
  const [status, setStatus] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);

  const slug = useMemo(() => `${slugify(question)}-${Math.floor(Date.now() / 1000) % 100000}`, [question]);
  const usingDefaultOracle = oracle.trim() === DEFAULT_ORACLE;

  async function submit() {
    setBusy(true);
    setStatus(null);
    try {
      if (!question.trim()) throw new Error("Enter a question.");
      const expiryTs = Math.floor(new Date(expiry).getTime() / 1000);
      if (!Number.isFinite(expiryTs)) throw new Error("Pick a close date.");
      if (!/^[0-9a-f]{64}$/i.test(oracle.trim())) throw new Error("Oracle must be a 64-char hex key.");

      const template = buildMarketTemplate({
        slug,
        question: question.trim(),
        oracle: oracle.trim(),
        mint: mint.trim(),
        dlcContract: PLACEHOLDER_DLC,
        expiry: expiryTs,
        resolution: resolution.trim(),
      });
      const signed = await signTemplate(template);
      const id = marketId(signed.pubkey, slug);
      const relayList = relays.split(",").map((s) => s.trim()).filter(Boolean);
      const results = await publishAll(relayList, signed);
      const ok = results.filter((r) => r.accepted).length;
      if (ok === 0) throw new Error("No relay accepted the market. Check the relay URL.");
      setStatus(`✔ Published to ${ok}/${results.length} relay(s). Market id: ${id}`);
    } catch (e) {
      setStatus("Error: " + (e as Error).message);
    } finally {
      setBusy(false);
    }
  }

  return (
    <div className="flex flex-col gap-4 max-w-2xl">
      <Link href="/" className="text-sm">← markets</Link>
      <h1 className="font-bold">Create a market</h1>
      <p style={{ color: "var(--muted)" }} className="text-xs">
        Ask a yes/no question with a clear deadline. Signed with your Nostr extension (NIP-07) —
        no key custody. Outcomes are always YES / NO / INVALID.
      </p>

      <label className="flex flex-col gap-1">
        <span className="text-sm font-bold">Question</span>
        <textarea
          style={field}
          className="px-3 py-2 text-sm rounded"
          rows={2}
          placeholder="Will BTC close above $100k on 2026-12-31?"
          value={question}
          onChange={(e) => setQuestion(e.target.value)}
        />
      </label>

      <label className="flex flex-col gap-1">
        <span className="text-sm font-bold">Resolution criteria</span>
        <textarea
          style={field}
          className="px-3 py-2 text-sm rounded"
          rows={2}
          placeholder="YES if BTC/USD ≥ 100000 at 23:59 UTC per Coinbase."
          value={resolution}
          onChange={(e) => setResolution(e.target.value)}
        />
        <span style={{ color: "var(--muted)" }} className="text-xs">
          Be precise — this is what the oracle uses to decide.
        </span>
      </label>

      <label className="flex flex-col gap-1">
        <span className="text-sm font-bold">Closes at</span>
        <input style={field} className="px-3 py-2 text-sm rounded" type="datetime-local" value={expiry} onChange={(e) => setExpiry(e.target.value)} />
      </label>

      <div className="rounded p-3 flex flex-col gap-1" style={{ border: "1px solid var(--border)" }}>
        <div className="text-sm">
          Oracle:{" "}
          <span style={{ color: "var(--accent)" }}>
            {usingDefaultOracle ? "21pay oracle (default)" : `${oracle.slice(0, 12)}…`}
          </span>
        </div>
        <div style={{ color: "var(--muted)" }} className="text-xs">
          The oracle publishes the result after the deadline. The 21pay oracle is selected for you —
          you don&apos;t need your own. Verify any oracle&apos;s reputation on the market page before betting.
        </div>
      </div>

      <button
        type="button"
        onClick={() => setAdvanced((v) => !v)}
        className="self-start text-xs"
        style={{ color: "var(--muted)" }}
      >
        {advanced ? "▾ hide advanced" : "▸ advanced (custom oracle / mint / relays)"}
      </button>

      {advanced && (
        <div className="flex flex-col gap-2 rounded p-3" style={{ border: "1px solid var(--border)" }}>
          <label className="flex flex-col gap-1">
            <span className="text-xs">Oracle pubkey (x-only hex, 64 chars)</span>
            <input style={field} className="px-3 py-2 text-sm rounded" value={oracle} onChange={(e) => setOracle(e.target.value)} />
          </label>
          <label className="flex flex-col gap-1">
            <span className="text-xs">Mint URL</span>
            <input style={field} className="px-3 py-2 text-sm rounded" value={mint} onChange={(e) => setMint(e.target.value)} />
          </label>
          <label className="flex flex-col gap-1">
            <span className="text-xs">Relays (comma-separated)</span>
            <input style={field} className="px-3 py-2 text-sm rounded" value={relays} onChange={(e) => setRelays(e.target.value)} />
          </label>
        </div>
      )}

      <button
        onClick={submit}
        disabled={busy}
        className="self-start px-4 py-2 text-sm rounded font-bold"
        style={{ background: "var(--accent)", color: "#000" }}
      >
        {busy ? "Signing…" : "Sign & publish"}
      </button>

      {status && (
        <p style={{ color: "var(--muted)" }} className="text-xs break-all">
          {status}
        </p>
      )}
    </div>
  );
}
