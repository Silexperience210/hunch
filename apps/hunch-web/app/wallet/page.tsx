"use client";

import { useEffect, useRef, useState } from "react";
import Link from "next/link";
import type { Proof, Wallet } from "@cashu/cashu-ts";
import { compressedPubkey, randomBettorSecret } from "@/lib/dlc";
import { connect, depositQuote, meltToInvoice, mintPlain, payWithWebln, waitPaid } from "@/lib/wallet";
import { copyText } from "@/lib/clipboard";
import { Alert, Button, Card, Input } from "@/components/ui";

const SECRET_KEY = "hunch:wallet-secret";
const MINT_KEY = "hunch:mint-url";
const PROOFS_PREFIX = "hunch:proofs:";
const SIGNET_MINT = "https://mint-signet.21pay.org";
const MAINNET_MINT = "https://mint-mainnet.21pay.org";
const DEFAULT_MINT = SIGNET_MINT;

const balanceKey = (mint: string) => `hunch:cashu:${mint}`;

/** cashu-ts proof amounts can be number, string (HTTP JSON), bigint, or an Amount ({ value }). */
function toNum(a: unknown): number {
  if (typeof a === "number") return a;
  if (typeof a === "string") return Number(a);
  if (typeof a === "bigint") return Number(a);
  const v = (a as { value?: unknown })?.value;
  return typeof v === "bigint" ? Number(v) : Number(v ?? 0);
}
const sumSat = (proofs: { amount: unknown }[]) => proofs.reduce((s, p) => s + toNum(p.amount), 0);

interface Position {
  market: string;
  outcome: string;
  sat: number;
  count: number;
}
type Status = { msg: string; kind: "info" | "ok" | "error" } | null;

/** Locked outcome positions, scanned from localStorage (`hunch:proofs:<market>:<outcome>`). */
function readPositions(): Position[] {
  const out: Position[] = [];
  for (let i = 0; i < localStorage.length; i++) {
    const key = localStorage.key(i);
    if (!key || !key.startsWith(PROOFS_PREFIX)) continue;
    const rest = key.slice(PROOFS_PREFIX.length); // `<market>:<outcome>`; market itself has colons
    const cut = rest.lastIndexOf(":");
    if (cut < 0) continue;
    try {
      const proofs = JSON.parse(localStorage.getItem(key) ?? "[]") as { amount: number }[];
      if (proofs.length) out.push({ market: rest.slice(0, cut), outcome: rest.slice(cut + 1), sat: sumSat(proofs), count: proofs.length });
    } catch {
      /* skip */
    }
  }
  return out.sort((a, b) => b.sat - a.sat);
}

function loadBalance(mint: string): Proof[] {
  try {
    return JSON.parse(localStorage.getItem(balanceKey(mint)) ?? "[]");
  } catch {
    return [];
  }
}
function saveBalance(mint: string, proofs: Proof[]) {
  localStorage.setItem(balanceKey(mint), JSON.stringify(proofs));
}

export default function WalletPage() {
  const [secret, setSecret] = useState("");
  const [pubkey, setPubkey] = useState("");
  const [mint, setMint] = useState(DEFAULT_MINT);
  const [balance, setBalance] = useState<Proof[]>([]);
  const [positions, setPositions] = useState<Position[]>([]);

  const [depositAmount, setDepositAmount] = useState("1000");
  const [invoice, setInvoice] = useState("");
  const [withdrawInvoice, setWithdrawInvoice] = useState("");
  const [importKey, setImportKey] = useState("");

  const [status, setStatus] = useState<Status>(null);
  const [busy, setBusy] = useState(false);
  const [waiting, setWaiting] = useState(false);
  const [copied, setCopied] = useState("");
  const wallet = useRef<Wallet | null>(null);
  const quote = useRef<any>(null);

  function log(msg: string, kind: "info" | "ok" | "error" = "info") {
    setStatus({ msg, kind });
  }
  async function guard(fn: () => Promise<void>) {
    setBusy(true);
    try {
      await fn();
    } catch (e) {
      log("Error: " + (e as Error).message, "error");
    } finally {
      setBusy(false);
    }
  }

  // Boot: load (or create) the key + mint, then read balances. Runs once.
  useEffect(() => {
    let s = localStorage.getItem(SECRET_KEY) ?? "";
    if (!s) {
      s = randomBettorSecret();
      localStorage.setItem(SECRET_KEY, s);
    }
    const m = localStorage.getItem(MINT_KEY) || DEFAULT_MINT;
    setSecret(s);
    setMint(m);
  }, []);

  // Re-derive pubkey + balances whenever the key or mint changes.
  useEffect(() => {
    if (!secret) return;
    try {
      setPubkey(compressedPubkey(secret));
    } catch {
      setPubkey("");
    }
  }, [secret]);
  useEffect(() => {
    if (!mint) return;
    setBalance(loadBalance(mint));
    setPositions(readPositions());
  }, [mint]);

  function persistMint(m: string) {
    setMint(m);
    localStorage.setItem(MINT_KEY, m);
  }

  async function copy(text: string, label: string, id = label) {
    try {
      await copyText(text);
      setCopied(id);
      setTimeout(() => setCopied((c) => (c === id ? "" : c)), 1500);
      log(`✔ ${label} copied.`, "ok");
    } catch {
      log("Copy failed — select the text manually.", "error");
    }
  }

  function regenerateKey() {
    if (positions.length && !confirm("You still hold outcome tokens under the current key — back up the backup key first, they stay redeemable only with it. Generate a new key anyway?")) return;
    const s = randomBettorSecret();
    localStorage.setItem(SECRET_KEY, s);
    setSecret(s);
    log("New wallet key generated. Old tokens remain spendable only with the previous key.");
  }

  function useImportedKey() {
    const k = importKey.trim().toLowerCase();
    if (!/^[0-9a-f]{64}$/.test(k)) {
      log("Import a 64-char hex secret key.", "error");
      return;
    }
    localStorage.setItem(SECRET_KEY, k);
    setSecret(k);
    setImportKey("");
    log("✔ Imported your wallet key.", "ok");
  }

  // Deposit is one step for the user: get the invoice, then we watch for payment and credit the
  // tokens automatically — no second click. WebLN auto-pays if a provider is present.
  async function startDeposit() {
    const amt = Number(depositAmount);
    if (!Number.isFinite(amt) || amt <= 0) {
      log("Enter a deposit amount in sat.", "error");
      return;
    }
    setBusy(true);
    try {
      const w = await connect(mint.trim());
      wallet.current = w;
      const { quote: q, invoice: inv } = await depositQuote(w, amt);
      quote.current = q;
      setInvoice(inv);
      log("Scan or pay the invoice — your tokens are credited automatically.");
      autoCredit(w, q, amt); // background: poll for payment, then mint + credit
      try {
        await payWithWebln(inv); // auto-pay if Alby/WebLN is available; ignore otherwise
      } catch {
        /* no WebLN — user pays manually, autoCredit still catches it */
      }
    } catch (e) {
      log("Error: " + (e as Error).message, "error");
    } finally {
      setBusy(false);
    }
  }

  // Polls the mint until the invoice is paid, then mints + credits — runs in the background so the
  // rest of the wallet stays usable. ~10 min window.
  async function autoCredit(w: Wallet, q: any, amt: number) {
    setWaiting(true);
    try {
      await waitPaid(w, q, 600);
      const fresh = await mintPlain(w, amt, q);
      const next = [...loadBalance(mint), ...fresh];
      saveBalance(mint, next);
      setBalance(next);
      setInvoice("");
      quote.current = null;
      log(`✔ Credited ${sumSat(fresh)} sat — balance ${sumSat(next)} sat.`, "ok");
    } catch {
      log("Still waiting for the payment — re-deposit if the invoice expired.", "info");
    } finally {
      setWaiting(false);
    }
  }

  async function withdraw() {
    await guard(async () => {
      const inv = withdrawInvoice.trim();
      if (!inv.toLowerCase().startsWith("ln")) throw new Error("Paste a bolt11 Lightning invoice (lnbc…/lntbs…).");
      const proofs = loadBalance(mint);
      if (proofs.length === 0) throw new Error("No balance to withdraw on this mint.");
      const w = wallet.current ?? (await connect(mint.trim()));
      wallet.current = w;
      const { change, paid, fee } = await meltToInvoice(w, proofs, inv);
      saveBalance(mint, change);
      setBalance(change);
      setWithdrawInvoice("");
      log(`✔ Withdrew ${paid} sat (fee reserve ${fee}). Remaining ${sumSat(change)} sat.`, "ok");
    });
  }

  const balSat = sumSat(balance);
  const lockedTotal = positions.reduce((s, p) => s + p.sat, 0);

  return (
    <div className="flex flex-col gap-6 max-w-2xl">
      <section className="flex flex-col gap-1">
        <h1 className="font-bold text-2xl">Wallet</h1>
        <p style={{ color: "var(--muted)" }} className="text-sm">
          A Cashu wallet that lives entirely in this browser — key and tokens are in local storage,
          never on a server. Top up over Lightning, withdraw to any invoice. No custody.
        </p>
      </section>

      {/* Network quick-switch — Signet (test sats) vs Mainnet (real sats). */}
      <div className="flex gap-2 items-center text-sm">
        <span className="text-xs" style={{ color: "var(--muted)" }}>network</span>
        <Button size="sm" variant={mint === SIGNET_MINT ? "primary" : "secondary"} onClick={() => persistMint(SIGNET_MINT)}>
          Signet (test)
        </Button>
        <Button size="sm" variant={mint === MAINNET_MINT ? "primary" : "secondary"} onClick={() => persistMint(MAINNET_MINT)}>
          Mainnet (real sats)
        </Button>
      </div>

      {/* Balance + deposit / withdraw */}
      <Card className="flex flex-col gap-4 p-4">
        <div className="flex items-baseline justify-between">
          <span className="text-xs" style={{ color: "var(--muted)" }}>
            balance · {mint === SIGNET_MINT ? "signet (test)" : mint === MAINNET_MINT ? "mainnet (real)" : mint.replace(/^https?:\/\//, "")}
          </span>
          <span className="text-2xl font-bold" style={{ color: "var(--accent)" }}>{balSat} sat</span>
        </div>

        <div className="flex flex-col gap-2">
          <span className="text-xs" style={{ color: "var(--muted)" }}>deposit (top up over Lightning)</span>
          <div className="flex gap-2 items-center">
            <Input className="w-32" inputMode="numeric" value={depositAmount} onChange={(e) => setDepositAmount(e.target.value)} />
            <span className="text-sm" style={{ color: "var(--muted)" }}>sat</span>
            <Button variant="primary" onClick={startDeposit} disabled={busy || waiting}>
              {waiting ? "Waiting for payment…" : "Deposit"}
            </Button>
          </div>
          {invoice && (
            <Card className="flex flex-col gap-2 p-3">
              <div className="flex items-center gap-2">
                <span className="text-sm font-bold">Lightning invoice</span>
                {waiting && <span className="text-xs" style={{ color: "var(--accent)" }}>⏳ waiting for payment — auto-credits</span>}
                <Button size="sm" onClick={() => copy(invoice, "Invoice", "inv")}>{copied === "inv" ? "✓ Copied" : "Copy"}</Button>
                <a href={`lightning:${invoice}`} className="field px-3 py-1 text-xs rounded">Open wallet</a>
              </div>
              {/* eslint-disable-next-line @next/next/no-img-element */}
              <img
                alt="invoice QR"
                width={200}
                height={200}
                style={{ background: "#fff", padding: 8, borderRadius: 8, alignSelf: "flex-start" }}
                src={`https://api.qrserver.com/v1/create-qr-code/?size=200x200&data=${encodeURIComponent(invoice.toUpperCase())}`}
              />
              <code className="text-xs break-all" style={{ color: "var(--muted)" }}>{invoice}</code>
            </Card>
          )}
        </div>

        <div className="flex flex-col gap-2">
          <span className="text-xs" style={{ color: "var(--muted)" }}>withdraw (pay a Lightning invoice)</span>
          <div className="flex gap-2">
            <Input className="flex-1" placeholder="bolt11 invoice (lntbs… / lnbc…)" value={withdrawInvoice} onChange={(e) => setWithdrawInvoice(e.target.value)} />
            <Button onClick={withdraw} disabled={busy || balSat === 0}>Withdraw</Button>
          </div>
        </div>
      </Card>

      {/* Outcome positions (locked) */}
      <section className="flex flex-col gap-2">
        <div className="flex items-baseline justify-between">
          <h2 className="font-bold">Outcome positions</h2>
          <span className="text-sm" style={{ color: "var(--muted)" }}>{lockedTotal} sat locked</span>
        </div>
        {positions.length === 0 ? (
          <p className="text-sm" style={{ color: "var(--muted)" }}>
            No outcome tokens yet. <Link href="/" style={{ color: "var(--accent)" }}>Browse markets →</Link>
          </p>
        ) : (
          <ul className="flex flex-col gap-2">
            {positions.map((p) => (
              <Card key={`${p.market}:${p.outcome}`} className="flex items-center justify-between gap-3 p-3">
                <div className="flex flex-col gap-1 min-w-0">
                  <span className="text-sm font-bold">
                    <span style={{ color: p.outcome === "YES" ? "var(--accent)" : "var(--fg)" }}>{p.outcome}</span>
                    <span style={{ color: "var(--muted)" }} className="font-normal"> · {p.sat} sat ({p.count} token{p.count === 1 ? "" : "s"})</span>
                  </span>
                  <span className="text-xs break-all" style={{ color: "var(--muted)" }}>{p.market}</span>
                </div>
                <Link href={`/bet?id=${encodeURIComponent(p.market)}`} className="px-3 py-2 text-xs rounded font-bold whitespace-nowrap inline-block" style={{ background: "var(--accent)", color: "#000" }}>
                  Redeem →
                </Link>
              </Card>
            ))}
          </ul>
        )}
      </section>

      {/* Settings: mint + key */}
      <details className="rounded p-3" style={{ border: "1px solid var(--border)" }}>
        <summary className="text-xs cursor-pointer" style={{ color: "var(--muted)" }}>
          settings — mint &amp; key (use your own)
        </summary>
        <div className="flex flex-col gap-3 mt-3">
          <label className="flex flex-col gap-1">
            <span className="text-xs" style={{ color: "var(--muted)" }}>mint URL (use your own mint)</span>
            <Input value={mint} onChange={(e) => persistMint(e.target.value)} placeholder="https://your-mint…" />
          </label>

          <div className="flex flex-col gap-1">
            <span className="text-xs" style={{ color: "var(--muted)" }}>wallet key</span>
            <code className="text-xs break-all" style={{ color: "var(--accent)" }}>{pubkey || "—"}</code>
            <div className="flex gap-2 flex-wrap mt-1">
              <Button size="sm" onClick={() => copy(secret, "Backup key", "bk")}>{copied === "bk" ? "✓ Copied" : "Copy backup key"}</Button>
              <Button size="sm" onClick={() => copy(pubkey, "Pubkey", "pk")}>{copied === "pk" ? "✓ Copied" : "Copy pubkey"}</Button>
              <Button size="sm" onClick={regenerateKey}>New key</Button>
            </div>
            <p className="text-xs mt-1" style={{ color: "var(--muted)" }}>
              The backup key is the only way to spend these tokens. Anyone with it controls the funds.
            </p>
          </div>

          <label className="flex flex-col gap-1">
            <span className="text-xs" style={{ color: "var(--muted)" }}>import your own key (64-char hex secret)</span>
            <div className="flex gap-2">
              <Input className="flex-1" value={importKey} onChange={(e) => setImportKey(e.target.value)} placeholder="your secret key…" />
              <Button onClick={useImportedKey}>Import</Button>
            </div>
          </label>
        </div>
      </details>

      {status && <Alert kind={status.kind}>{status.msg}</Alert>}
    </div>
  );
}
