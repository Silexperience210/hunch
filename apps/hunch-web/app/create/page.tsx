"use client";

import { useState } from "react";
import Link from "next/link";
import { buildMarketTemplate } from "@/lib/build";
import { marketId } from "@/lib/hunch";
import { getPublicKey, signTemplate } from "@/lib/sign";
import { publishAll } from "@/lib/publish";
import { DEFAULT_RELAYS } from "@/lib/relay";

const field = {
  background: "var(--card)",
  border: "1px solid var(--border)",
  color: "var(--fg)",
} as const;

export default function CreateMarketPage() {
  const [f, setF] = useState({
    slug: "",
    question: "",
    oracle: "",
    mint: "",
    dlcContract: "",
    expiry: "",
    resolution: "",
    relays: DEFAULT_RELAYS.join(", "),
  });
  const [status, setStatus] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);

  const set = (k: keyof typeof f) => (e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) =>
    setF({ ...f, [k]: e.target.value });

  async function submit() {
    setBusy(true);
    setStatus(null);
    try {
      const expiry = Math.floor(new Date(f.expiry).getTime() / 1000);
      if (!Number.isFinite(expiry)) throw new Error("Invalid expiry date.");
      const template = buildMarketTemplate({
        slug: f.slug.trim(),
        question: f.question.trim(),
        oracle: f.oracle.trim(),
        mint: f.mint.trim(),
        dlcContract: f.dlcContract.trim(),
        expiry,
        resolution: f.resolution.trim(),
      });
      const signed = await signTemplate(template);
      const id = marketId(signed.pubkey, f.slug.trim());
      const relays = f.relays.split(",").map((s) => s.trim()).filter(Boolean);
      const results = await publishAll(relays, signed);
      const ok = results.filter((r) => r.accepted).length;
      setStatus(`Published to ${ok}/${results.length} relays. Market id: ${id}`);
    } catch (e) {
      setStatus("Error: " + (e as Error).message);
    } finally {
      setBusy(false);
    }
  }

  return (
    <div className="flex flex-col gap-3 max-w-2xl">
      <Link href="/" className="text-sm">← markets</Link>
      <h1 className="font-bold">Create a market</h1>
      <p style={{ color: "var(--muted)" }} className="text-xs">
        Signed with your Nostr extension (NIP-07). Anyone can create any market — read the
        resolution criteria before betting. Outcomes are always YES / NO / INVALID.
      </p>

      <input style={field} className="px-3 py-2 text-sm rounded" placeholder="slug (e.g. btc-100k-eoy-2026)" value={f.slug} onChange={set("slug")} />
      <textarea style={field} className="px-3 py-2 text-sm rounded" rows={2} placeholder="question" value={f.question} onChange={set("question")} />
      <textarea style={field} className="px-3 py-2 text-sm rounded" rows={2} placeholder="resolution criteria" value={f.resolution} onChange={set("resolution")} />
      <input style={field} className="px-3 py-2 text-sm rounded" placeholder="oracle pubkey (x-only hex, 64 chars)" value={f.oracle} onChange={set("oracle")} />
      <input style={field} className="px-3 py-2 text-sm rounded" placeholder="mint (url or pubkey)" value={f.mint} onChange={set("mint")} />
      <input style={field} className="px-3 py-2 text-sm rounded" placeholder="dlc_contract (txid:vout)" value={f.dlcContract} onChange={set("dlcContract")} />
      <input style={field} className="px-3 py-2 text-sm rounded" type="datetime-local" value={f.expiry} onChange={set("expiry")} />
      <input style={field} className="px-3 py-2 text-sm rounded" placeholder="relays (comma-separated)" value={f.relays} onChange={set("relays")} />

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
