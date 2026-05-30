"use client";

import { Suspense, useRef, useState } from "react";
import { useSearchParams } from "next/navigation";
import Link from "next/link";
import type { Wallet } from "@cashu/cashu-ts";
import { compressedPubkey, outcomeLockKey, outcomeUnlockSecret, randomBettorSecret } from "@/lib/dlc";
import { connect, depositQuote, mintLocked, payWithWebln, redeem, waitPaid } from "@/lib/wallet";
import { fetchAnnounce, fetchAttestation } from "@/lib/oracle";
import { DEFAULT_RELAYS } from "@/lib/relay";

const field = { background: "var(--card)", border: "1px solid var(--border)", color: "var(--fg)" } as const;
const REFUND_LOCKTIME = Math.floor(Date.now() / 1000) + 90 * 24 * 3600; // 90 days

function BetView() {
  const params = useSearchParams();
  const [mintUrl, setMintUrl] = useState(params.get("mint") || "http://127.0.0.1:8085");
  const [market, setMarket] = useState(params.get("id") ?? "");
  const [oracle, setOracle] = useState(params.get("oracle") ?? "");
  const [nonce, setNonce] = useState(params.get("nonce") ?? "");
  const [relays, setRelays] = useState(DEFAULT_RELAYS.join(", "));
  const [outcome, setOutcome] = useState<"YES" | "NO">("YES");
  const [amount, setAmount] = useState("100");
  const [secret, setSecret] = useState("");
  const [invoice, setInvoice] = useState("");
  const [attestationSig, setAttestationSig] = useState("");
  const [status, setStatus] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);

  const wallet = useRef<Wallet | null>(null);
  const quote = useRef<any>(null);
  const proofsKey = `hunch:proofs:${market}:${outcome}`;

  function log(s: string) {
    setStatus(s);
  }
  async function guard(fn: () => Promise<void>) {
    setBusy(true);
    try {
      await fn();
    } catch (e) {
      log("Error: " + (e as Error).message);
    } finally {
      setBusy(false);
    }
  }

  const bettorPub = secret ? safe(() => compressedPubkey(secret)) : "";

  function relayList(): string[] {
    return relays.split(",").map((s) => s.trim()).filter(Boolean);
  }

  async function fetchNonce() {
    await guard(async () => {
      if (!oracle.trim() || !market.trim()) throw new Error("Set the oracle pubkey and market id first.");
      log("Fetching the oracle announce (kind:88) from relays…");
      const a = await fetchAnnounce(relayList(), oracle.trim(), market.trim());
      if (!a) throw new Error("No verified announce found for this oracle + market.");
      setNonce(a.nonce);
      log(`✔ Nonce R = ${a.nonce.slice(0, 16)}… loaded from the oracle announce.`);
    });
  }

  async function fetchAtt() {
    await guard(async () => {
      if (!oracle.trim() || !market.trim()) throw new Error("Set the oracle pubkey and market id first.");
      log("Fetching the oracle attestation (kind:89) from relays…");
      const a = await fetchAttestation(relayList(), oracle.trim(), market.trim());
      if (!a) throw new Error("No verified attestation found yet — the market may be unresolved.");
      setAttestationSig(a.signature);
      if (a.outcome === "YES" || a.outcome === "NO") setOutcome(a.outcome);
      log(`✔ Settlement: oracle attested ${a.outcome}. Signature loaded — redeem if it matches your position.`);
    });
  }

  async function deposit() {
    await guard(async () => {
      if (!secret) throw new Error("Generate a wallet key first.");
      const B = compressedPubkey(secret);
      const lock = outcomeLockKey(B, oracle.trim(), nonce.trim(), market.trim(), outcome);
      const w = await connect(mintUrl.trim());
      wallet.current = w;
      const { quote: q, invoice: inv } = await depositQuote(w, Number(amount));
      quote.current = q;
      setInvoice(inv);
      log(`Lock L_${outcome} = ${lock.slice(0, 16)}…  Pay the invoice, then "Pay & mint".`);
    });
  }

  async function payAndMint() {
    await guard(async () => {
      const w = wallet.current;
      if (!w || !quote.current) throw new Error("Run Deposit first.");
      try {
        await payWithWebln(invoice);
      } catch {
        log("WebLN unavailable — pay the invoice manually, then click again to continue.");
      }
      await waitPaid(w, quote.current);
      const B = compressedPubkey(secret);
      const lock = outcomeLockKey(B, oracle.trim(), nonce.trim(), market.trim(), outcome);
      const proofs = await mintLocked(w, Number(amount), quote.current, lock, B, REFUND_LOCKTIME);
      localStorage.setItem(proofsKey, JSON.stringify(proofs));
      log(`✔ Minted ${proofs.length} ${outcome} proof(s), locked to the oracle outcome. Saved locally.`);
    });
  }

  async function doRedeem() {
    await guard(async () => {
      const raw = localStorage.getItem(proofsKey);
      if (!raw) throw new Error("No saved proofs for this market/outcome.");
      if (!attestationSig.trim()) throw new Error("Paste the oracle's kind:89 attestation signature.");
      const w = wallet.current ?? (await connect(mintUrl.trim()));
      const spend = outcomeUnlockSecret(secret, attestationSig.trim());
      const fresh = await redeem(w, JSON.parse(raw), spend);
      const total = fresh.reduce((s: number, p: any) => s + p.amount, 0);
      log(`✔ Redeemed! ${total} sat of unlocked proofs. The outcome resolved ${outcome}.`);
    });
  }

  return (
    <div className="flex flex-col gap-3 max-w-2xl">
      <Link href="/" className="text-sm">← markets</Link>
      <h1 className="font-bold">Bet (mint conditional tokens)</h1>
      <p style={{ color: "var(--muted)" }} className="text-xs">
        Mints Cashu tokens locked to the oracle outcome (NUT-11 P2PK to L = B + S_X). Spendable only
        if the oracle attests your outcome; reclaimable after the refund timeout. Your wallet key
        stays in this browser.
      </p>

      <div className="flex gap-2 items-center">
        <button onClick={() => setSecret(randomBettorSecret())} className="px-3 py-2 text-sm rounded" style={field}>
          Generate wallet key
        </button>
        <span style={{ color: "var(--muted)" }} className="text-xs break-all">
          {bettorPub ? `B = ${bettorPub.slice(0, 20)}…` : "no key yet"}
        </span>
      </div>

      <input style={field} className="px-3 py-2 text-sm rounded" placeholder="mint url" value={mintUrl} onChange={(e) => setMintUrl(e.target.value)} />
      <input style={field} className="px-3 py-2 text-sm rounded" placeholder="market id (creator:30888:slug)" value={market} onChange={(e) => setMarket(e.target.value)} />
      <input style={field} className="px-3 py-2 text-sm rounded" placeholder="oracle pubkey (x-only hex)" value={oracle} onChange={(e) => setOracle(e.target.value)} />
      <input style={field} className="px-3 py-2 text-sm rounded" placeholder="relays (comma-separated)" value={relays} onChange={(e) => setRelays(e.target.value)} />
      <div className="flex gap-2">
        <input style={field} className="px-3 py-2 text-sm rounded flex-1" placeholder="oracle nonce R (x-only hex, from the kind:88 announce)" value={nonce} onChange={(e) => setNonce(e.target.value)} />
        <button onClick={fetchNonce} disabled={busy} className="px-3 py-2 text-sm rounded whitespace-nowrap" style={field}>
          fetch
        </button>
      </div>
      <div className="flex gap-2">
        <select style={field} className="px-2 py-2 text-sm rounded" value={outcome} onChange={(e) => setOutcome(e.target.value as "YES" | "NO")}>
          <option value="YES">YES</option>
          <option value="NO">NO</option>
        </select>
        <input style={field} className="px-3 py-2 text-sm rounded w-32" placeholder="amount sat" value={amount} onChange={(e) => setAmount(e.target.value)} />
      </div>

      <div className="flex gap-2 flex-wrap">
        <button onClick={deposit} disabled={busy} className="px-4 py-2 text-sm rounded font-bold" style={{ background: "var(--accent)", color: "#000" }}>
          1. Deposit
        </button>
        <button onClick={payAndMint} disabled={busy || !invoice} className="px-4 py-2 text-sm rounded font-bold" style={field}>
          2. Pay & mint
        </button>
      </div>

      {invoice && (
        <div className="text-xs break-all" style={{ color: "var(--muted)" }}>
          invoice: {invoice}
        </div>
      )}

      <section className="flex flex-col gap-2" style={{ borderTop: "1px solid var(--border)", paddingTop: 12 }}>
        <div className="font-bold text-sm">Redeem after settlement</div>
        <div className="flex gap-2">
          <input style={field} className="px-3 py-2 text-sm rounded flex-1" placeholder="oracle attestation signature (kind:89 sig hex)" value={attestationSig} onChange={(e) => setAttestationSig(e.target.value)} />
          <button onClick={fetchAtt} disabled={busy} className="px-3 py-2 text-sm rounded whitespace-nowrap" style={field}>
            fetch
          </button>
        </div>
        <button onClick={doRedeem} disabled={busy} className="self-start px-4 py-2 text-sm rounded font-bold" style={{ background: "var(--accent)", color: "#000" }}>
          Redeem
        </button>
      </section>

      {status && <p className="text-xs break-all" style={{ color: "var(--muted)" }}>{status}</p>}
    </div>
  );
}

function safe(fn: () => string): string {
  try {
    return fn();
  } catch {
    return "";
  }
}

export default function BetPage() {
  return (
    <Suspense fallback={<p style={{ color: "var(--muted)" }}>Loading…</p>}>
      <BetView />
    </Suspense>
  );
}
