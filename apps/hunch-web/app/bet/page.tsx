"use client";

import { Suspense, useEffect, useRef, useState } from "react";
import { useSearchParams } from "next/navigation";
import Link from "next/link";
import type { Wallet } from "@cashu/cashu-ts";
import { compressedPubkey, outcomeLockKey, outcomeUnlockSecret, randomBettorSecret } from "@/lib/dlc";
import { connect, depositQuote, mintLocked, payWithWebln, redeem, waitPaid } from "@/lib/wallet";
import { fetchAnnounce, fetchAttestation } from "@/lib/oracle";
import { relaysFromUrl, queryRelays } from "@/lib/relay";
import { KIND_MARKET, parseMarketEvent, type Market } from "@/lib/hunch";
import { verifyEvent } from "@/lib/verify";
import { Alert, Button, Card, Input } from "@/components/ui";

const REFUND_LOCKTIME = Math.floor(Date.now() / 1000) + 90 * 24 * 3600; // 90 days
// The 21pay oracle (runs on the Umbrel) — pre-filled so users don't paste a key. Override via ?oracle=.
const DEFAULT_ORACLE = "b32187c658b01420003049758660e62e4a7dd3daefac42076cd1664adce0e335";
// The Hunch relay (carries oracle announces/attestations) — always queried, even if the field differs.
const HUNCH_RELAY = "wss://relay.21pay.org";

type StatusKind = "info" | "ok" | "error";
type Status = { msg: string; kind: StatusKind } | null;

function BetView() {
  const params = useSearchParams();
  const [mintUrl, setMintUrl] = useState(params.get("mint") || "https://mint-signet.21pay.org");
  const [market, setMarket] = useState(params.get("id") ?? "");
  const [oracle, setOracle] = useState(params.get("oracle") || DEFAULT_ORACLE);
  const [nonce, setNonce] = useState(params.get("nonce") ?? "");
  const [relays, setRelays] = useState(relaysFromUrl().join(", "));
  const [outcome, setOutcome] = useState<"YES" | "NO">("YES");
  const [amount, setAmount] = useState("100");
  const [secret, setSecret] = useState("");
  const [invoice, setInvoice] = useState("");
  const [attestationSig, setAttestationSig] = useState("");
  const [question, setQuestion] = useState("");
  const [status, setStatus] = useState<Status>(null);
  const [busy, setBusy] = useState(false);

  const wallet = useRef<Wallet | null>(null);
  const quote = useRef<any>(null);
  const proofsKey = `hunch:proofs:${market}:${outcome}`;

  function log(msg: string, kind: StatusKind = "info") {
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

  const bettorPub = secret ? safe(() => compressedPubkey(secret)) : "";

  // Relays to query: whatever is in the field, but always including the 21pay relay where the
  // Hunch oracle publishes its announces/attestations — so fetch works even if the field is stale.
  function relayList(): string[] {
    const fromField = relays.split(",").map((s) => s.trim()).filter(Boolean);
    return [...new Set([HUNCH_RELAY, ...fromField])];
  }

  // Wallet key: persisted in the browser so it survives reloads (a "bet in progress" stays
  // redeemable) and is always present — no manual "generate" step before depositing.
  useEffect(() => {
    if (secret) return;
    const KEY = "hunch:wallet-secret";
    let s = localStorage.getItem(KEY);
    if (!s) {
      s = randomBettorSecret();
      localStorage.setItem(KEY, s);
    }
    setSecret(s);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  // Auto-load the oracle nonce R on first render when it wasn't passed in the URL,
  // so users never have to paste it. Silent: failures just leave the field editable.
  const autoTried = useRef(false);
  useEffect(() => {
    if (autoTried.current || nonce || !oracle.trim() || !market.trim()) return;
    autoTried.current = true;
    (async () => {
      try {
        const a = await fetchAnnounce(relayList(), oracle.trim(), market.trim());
        if (a) {
          setNonce(a.nonce);
          log(`✔ Oracle nonce loaded — you're ready to deposit.`, "ok");
        }
      } catch {
        /* leave the field for manual entry */
      }
    })();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  // Pull the market question for context, so the bettor sees what they're betting on rather than a
  // raw `creator:30888:slug` id. Silent: if it can't be fetched the page still works headlessly.
  useEffect(() => {
    if (!market.trim()) return;
    let cancelled = false;
    (async () => {
      const [creator, , ...rest] = market.split(":");
      const d = rest.join(":");
      if (!creator || !d) return;
      const events = await queryRelays(relayList(), { kinds: [KIND_MARKET], authors: [creator], "#d": [d], limit: 5 });
      const m = events
        .filter(verifyEvent)
        .map(parseMarketEvent)
        .find((x): x is Market => x !== null && x.id === market.trim());
      if (!cancelled && m) setQuestion(m.content.question);
    })().catch(() => {
      /* leave the headless title */
    });
    return () => {
      cancelled = true;
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [market]);

  async function copyInvoice() {
    try {
      await navigator.clipboard.writeText(invoice);
      log("✔ Invoice copied to clipboard.", "ok");
    } catch {
      log("Copy failed — select the invoice text manually.", "error");
    }
  }

  async function fetchNonce() {
    await guard(async () => {
      if (!oracle.trim() || !market.trim()) throw new Error("Set the oracle pubkey and market id first.");
      log("Fetching the oracle announce (kind:88) from relays…");
      const a = await fetchAnnounce(relayList(), oracle.trim(), market.trim());
      if (!a) throw new Error("No verified announce found for this oracle + market.");
      setNonce(a.nonce);
      log(`✔ Nonce R = ${a.nonce.slice(0, 16)}… loaded from the oracle announce.`, "ok");
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
      log(`✔ Settlement: oracle attested ${a.outcome}. Signature loaded — redeem if it matches your position.`, "ok");
    });
  }

  async function deposit() {
    await guard(async () => {
      if (!secret) throw new Error("Generate a wallet key first.");
      if (!market.trim()) throw new Error("No market selected — open this page from a market's “Bet →” link.");
      if (!nonce.trim())
        throw new Error("Oracle nonce R not loaded yet — the market may not be announced. Open “advanced” to fetch or enter it.");
      const B = compressedPubkey(secret);
      const lock = outcomeLockKey(B, oracle.trim(), nonce.trim(), market.trim(), outcome);
      const w = await connect(mintUrl.trim());
      wallet.current = w;
      const { quote: q, invoice: inv } = await depositQuote(w, Number(amount));
      quote.current = q;
      setInvoice(inv);
      log(`Lightning invoice ready — pay it, then click “Pay & mint”. (locked to L_${outcome} = ${lock.slice(0, 12)}…)`);
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
      log(`✔ Bet placed — minted ${proofs.length} ${outcome} token(s), locked to the oracle outcome and saved in this browser.`, "ok");
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
      const total = fresh.reduce((s: number, p: any) => s + Number(p.amount), 0);
      // Move the unlocked proofs into the wallet balance (keyed by mint) so they survive reload and
      // become withdrawable, and drop the spent locked position.
      const balKey = `hunch:cashu:${mintUrl.trim()}`;
      const prevBal = JSON.parse(localStorage.getItem(balKey) ?? "[]");
      localStorage.setItem(balKey, JSON.stringify([...prevBal, ...fresh]));
      localStorage.removeItem(proofsKey);
      log(`✔ Redeemed ${total} sat into your wallet — withdraw it any time. The outcome resolved ${outcome}.`, "ok");
    });
  }

  const sideButton = (value: "YES" | "NO") => (
    <Button
      key={value}
      variant={outcome === value ? "primary" : "secondary"}
      size="lg"
      className="flex-1"
      onClick={() => setOutcome(value)}
      aria-pressed={outcome === value}
    >
      {value}
    </Button>
  );

  return (
    <div className="flex flex-col gap-5 max-w-2xl">
      <Link href="/" className="text-sm">← markets</Link>

      <div className="flex flex-col gap-1">
        <h1 className="font-bold text-lg">{question || "Place a bet"}</h1>
        <p style={{ color: "var(--muted)" }} className="text-xs">
          Pick a side, deposit sats over Lightning, and mint Cashu tokens that pay out only if the
          oracle attests your outcome (reclaimable after the refund timeout). No custody — your wallet
          key stays in this browser.
        </p>
        <div className="flex gap-2 items-center text-xs" style={{ color: "var(--muted)" }}>
          <span className="break-all">{bettorPub ? `wallet ${bettorPub.slice(0, 16)}…` : "creating wallet…"}</span>
          <Link href="/wallet/" style={{ color: "var(--accent)" }}>view wallet →</Link>
        </div>
      </div>

      {/* 1 · pick a side + stake */}
      <section className="flex flex-col gap-3">
        <div className="flex flex-col gap-2">
          <span style={{ color: "var(--muted)" }} className="text-xs">your side</span>
          <div className="flex gap-2">{sideButton("YES")}{sideButton("NO")}</div>
        </div>
        <label className="flex flex-col gap-2">
          <span style={{ color: "var(--muted)" }} className="text-xs">stake</span>
          <div className="flex items-center gap-2">
            <Input className="w-32" inputMode="numeric" value={amount} onChange={(e) => setAmount(e.target.value)} />
            <span style={{ color: "var(--muted)" }} className="text-sm">sat</span>
          </div>
        </label>
      </section>

      {/* 2 · deposit & mint */}
      <section className="flex flex-col gap-2">
        <span style={{ color: "var(--muted)" }} className="text-xs">deposit &amp; mint</span>
        <div className="flex gap-2 flex-wrap">
          <Button variant="primary" onClick={deposit} disabled={busy}>
            1. Deposit
          </Button>
          <Button onClick={payAndMint} disabled={busy || !invoice}>
            2. Pay &amp; mint
          </Button>
        </div>
      </section>

      {invoice && (
        <Card className="flex flex-col gap-2 p-3">
          <div className="flex items-center gap-2">
            <span className="text-sm font-bold">Lightning invoice</span>
            <Button size="sm" onClick={copyInvoice}>
              Copy
            </Button>
            <a href={`lightning:${invoice}`} className="field px-3 py-1 text-xs rounded">
              Open wallet
            </a>
          </div>
          {/* eslint-disable-next-line @next/next/no-img-element */}
          <img
            alt="invoice QR"
            width={220}
            height={220}
            style={{ background: "#fff", padding: 8, borderRadius: 8, alignSelf: "flex-start" }}
            src={`https://api.qrserver.com/v1/create-qr-code/?size=220x220&data=${encodeURIComponent(invoice.toUpperCase())}`}
          />
          <code className="text-xs break-all" style={{ color: "var(--muted)" }}>{invoice}</code>
        </Card>
      )}

      {/* 3 · redeem after settlement */}
      <section className="flex flex-col gap-2" style={{ borderTop: "1px solid var(--border)", paddingTop: 16 }}>
        <span style={{ color: "var(--muted)" }} className="text-xs">after settlement</span>
        <div className="font-bold text-sm">Redeem your winnings</div>
        <p style={{ color: "var(--muted)" }} className="text-xs">
          Once the oracle attests, the signature loads automatically. Redeem if {outcome} won (or after
          the refund timeout if the market resolved INVALID).
        </p>
        <div className="flex gap-2 flex-wrap">
          <Button onClick={fetchAtt} disabled={busy}>
            Check settlement
          </Button>
          <Button variant="primary" onClick={doRedeem} disabled={busy}>
            Redeem
          </Button>
        </div>
      </section>

      {/* Protocol knobs — pre-filled with the 21pay defaults, hidden so the page reads like a bet. */}
      <details className="rounded p-3" style={{ border: "1px solid var(--border)" }}>
        <summary className="text-xs cursor-pointer" style={{ color: "var(--muted)" }}>
          advanced — mint, oracle, relays, nonce, signature, wallet key
        </summary>
        <div className="flex flex-col gap-2 mt-3">
          <div className="flex gap-2 items-center">
            <span style={{ color: "var(--muted)" }} className="text-xs break-all">
              {bettorPub ? `wallet ${bettorPub.slice(0, 16)}… (saved in this browser)` : "creating wallet…"}
            </span>
            <Button
              size="sm"
              onClick={() => {
                const s = randomBettorSecret();
                localStorage.setItem("hunch:wallet-secret", s);
                setSecret(s);
                log("New wallet key generated (old tokens stay under the previous key).");
              }}
              title="Generate a fresh wallet key"
            >
              new key
            </Button>
          </div>
          <Input placeholder="mint url" value={mintUrl} onChange={(e) => setMintUrl(e.target.value)} />
          <Input placeholder="market id (creator:30888:slug)" value={market} onChange={(e) => setMarket(e.target.value)} />
          <Input placeholder="oracle pubkey (x-only hex)" value={oracle} onChange={(e) => setOracle(e.target.value)} />
          <Input placeholder="relays (comma-separated)" value={relays} onChange={(e) => setRelays(e.target.value)} />
          <div className="flex gap-2">
            <Input className="flex-1" placeholder="oracle nonce R (x-only hex, from the kind:88 announce)" value={nonce} onChange={(e) => setNonce(e.target.value)} />
            <Button className="whitespace-nowrap" onClick={fetchNonce} disabled={busy}>
              fetch
            </Button>
          </div>
          <div className="flex gap-2">
            <Input className="flex-1" placeholder="oracle attestation signature (kind:89 sig hex)" value={attestationSig} onChange={(e) => setAttestationSig(e.target.value)} />
            <Button className="whitespace-nowrap" onClick={fetchAtt} disabled={busy}>
              fetch
            </Button>
          </div>
        </div>
      </details>

      {status && <Alert kind={status.kind}>{status.msg}</Alert>}
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
