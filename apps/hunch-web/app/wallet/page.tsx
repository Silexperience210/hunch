"use client";

import { useEffect, useState } from "react";
import Link from "next/link";
import { compressedPubkey, randomBettorSecret } from "@/lib/dlc";
import { Alert, Button, Card } from "@/components/ui";

const SECRET_KEY = "hunch:wallet-secret";
const PROOFS_PREFIX = "hunch:proofs:";

interface Position {
  market: string;
  outcome: string;
  sat: number;
  count: number;
}

/** Reads the locally-stored Cashu wallet: the key + every minted outcome position. Everything lives
 *  in this browser's localStorage — no server, no custody (CLAUDE.md). */
function readWallet(): { secret: string; pubkey: string; positions: Position[] } {
  let secret = localStorage.getItem(SECRET_KEY) ?? "";
  if (!secret) {
    secret = randomBettorSecret();
    localStorage.setItem(SECRET_KEY, secret);
  }
  let pubkey = "";
  try {
    pubkey = compressedPubkey(secret);
  } catch {
    /* malformed key */
  }

  const positions: Position[] = [];
  for (let i = 0; i < localStorage.length; i++) {
    const key = localStorage.key(i);
    if (!key || !key.startsWith(PROOFS_PREFIX)) continue;
    const rest = key.slice(PROOFS_PREFIX.length); // `<market>:<outcome>`, market itself has colons
    const cut = rest.lastIndexOf(":");
    if (cut < 0) continue;
    const market = rest.slice(0, cut);
    const outcome = rest.slice(cut + 1);
    try {
      const proofs = JSON.parse(localStorage.getItem(key) ?? "[]") as { amount: number }[];
      const sat = proofs.reduce((s, p) => s + (p.amount || 0), 0);
      if (proofs.length) positions.push({ market, outcome, sat, count: proofs.length });
    } catch {
      /* skip unreadable entry */
    }
  }
  positions.sort((a, b) => b.sat - a.sat);
  return { secret, pubkey, positions };
}

export default function WalletPage() {
  const [pubkey, setPubkey] = useState("");
  const [secret, setSecret] = useState("");
  const [positions, setPositions] = useState<Position[]>([]);
  const [status, setStatus] = useState<{ msg: string; kind: "info" | "ok" | "error" } | null>(null);

  function refresh() {
    const w = readWallet();
    setSecret(w.secret);
    setPubkey(w.pubkey);
    setPositions(w.positions);
  }

  useEffect(() => {
    refresh();
  }, []);

  const total = positions.reduce((s, p) => s + p.sat, 0);

  async function copy(text: string, label: string) {
    try {
      await navigator.clipboard.writeText(text);
      setStatus({ msg: `✔ ${label} copied to clipboard.`, kind: "ok" });
    } catch {
      setStatus({ msg: "Copy failed — select the text manually.", kind: "error" });
    }
  }

  function newKey() {
    if (positions.length && !confirm("You still hold tokens under the current key. They stay redeemable only with the OLD key — back it up first. Generate a new key anyway?")) {
      return;
    }
    const s = randomBettorSecret();
    localStorage.setItem(SECRET_KEY, s);
    refresh();
    setStatus({ msg: "New wallet key generated. Old tokens remain spendable only with the previous key.", kind: "info" });
  }

  return (
    <div className="flex flex-col gap-6 max-w-2xl">
      <section className="flex flex-col gap-1">
        <h1 className="font-bold text-2xl">Wallet</h1>
        <p style={{ color: "var(--muted)" }} className="text-sm">
          Your Cashu wallet lives entirely in this browser — the key and your outcome tokens are in
          local storage, never on a server. No custody. Back up the key if you hold tokens.
        </p>
      </section>

      <Card className="flex flex-col gap-2 p-4">
        <div className="text-xs" style={{ color: "var(--muted)" }}>wallet key (Nostr/secp256k1)</div>
        <code className="text-sm break-all" style={{ color: "var(--accent)" }}>{pubkey || "—"}</code>
        <div className="flex gap-2 flex-wrap">
          <Button size="sm" onClick={() => copy(secret, "Backup key (secret)")}>Copy backup key</Button>
          <Button size="sm" onClick={() => copy(pubkey, "Public key")}>Copy pubkey</Button>
          <Button size="sm" onClick={newKey}>New key</Button>
        </div>
        <p className="text-xs" style={{ color: "var(--muted)" }}>
          The backup key is your only way to spend these tokens — anyone with it controls them. Keep it safe.
        </p>
      </Card>

      <section className="flex flex-col gap-2">
        <div className="flex items-baseline justify-between">
          <h2 className="font-bold">Positions</h2>
          <span className="text-sm" style={{ color: "var(--accent)" }}>{total} sat total</span>
        </div>

        {positions.length === 0 ? (
          <Card className="flex flex-col gap-2 p-4">
            <p className="text-sm" style={{ color: "var(--muted)" }}>
              No tokens yet. Place a bet to mint outcome tokens — they&apos;ll show up here.
            </p>
            <Link href="/" className="text-sm" style={{ color: "var(--accent)" }}>Browse markets →</Link>
          </Card>
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
                <Link
                  href={`/bet?id=${encodeURIComponent(p.market)}`}
                  className="px-3 py-2 text-xs rounded font-bold whitespace-nowrap inline-block"
                  style={{ background: "var(--accent)", color: "#000" }}
                >
                  Redeem →
                </Link>
              </Card>
            ))}
          </ul>
        )}
      </section>

      {status && <Alert kind={status.kind}>{status.msg}</Alert>}
    </div>
  );
}
