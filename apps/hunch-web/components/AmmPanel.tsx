"use client";

// 1-click bet panel (AMM). Shows an instant YES/NO price (from lib/amm over the LMSR core) and a
// payout quote with slippage, then routes to the proven /bet deposit→mint→redeem flow with the side
// and stake pre-filled. Liquidity is anchored on the order book's implied odds (else 50/50); the
// order book stays the source of truth for resting limit orders (hybrid AMM+CLOB).

import { useState } from "react";
import Link from "next/link";
import { DEFAULT_DEPTH, quoteBet } from "@/lib/amm";
import { Input } from "@/components/ui";

const pct = (x: number) => `${Math.round(x * 100)}%`;
const sat = (x: number) => Math.round(x).toLocaleString("en-US");

function Side({
  side,
  price,
  shares,
  avgPrice,
  href,
}: {
  side: "YES" | "NO";
  price: number;
  shares: number;
  avgPrice: number;
  href: string;
}) {
  return (
    <Link
      href={href}
      className="flex-1 min-w-[160px] rounded p-3 flex flex-col gap-1 no-underline"
      style={{ border: "1px solid var(--border)", background: "var(--card)" }}
    >
      <div className="flex items-baseline justify-between">
        <span className="font-bold" style={{ color: side === "YES" ? "var(--accent)" : "var(--muted)" }}>
          {side}
        </span>
        <span className="text-lg font-bold">{pct(price)}</span>
      </div>
      <div className="text-xs" style={{ color: "var(--muted)" }}>
        win <span style={{ color: "var(--fg)" }}>{sat(shares)} sat</span> · avg {pct(avgPrice)}
      </div>
      <div className="text-xs mt-1 px-2 py-1 rounded self-start font-bold" style={{ background: "var(--accent)", color: "#000" }}>
        Bet {side} →
      </div>
    </Link>
  );
}

export function AmmPanel({
  anchorProb = 0.5,
  depth = DEFAULT_DEPTH,
  href,
}: {
  anchorProb?: number;
  depth?: number;
  /** Build a /bet deep-link with the side + stake pre-filled. */
  href: (side: "YES" | "NO", amount: number) => string;
}) {
  const [stake, setStake] = useState("1000");
  const amt = Math.max(0, Math.floor(Number(stake) || 0));
  const yes = quoteBet("YES", amt, anchorProb, depth);
  const no = quoteBet("NO", amt, anchorProb, depth);

  return (
    <section className="flex flex-col gap-2" style={{ borderTop: "1px solid var(--border)", paddingTop: 16 }}>
      <div className="flex items-center justify-between gap-2 flex-wrap">
        <div className="font-bold">Instant bet</div>
        <div className="flex items-center gap-2 text-sm">
          <span style={{ color: "var(--muted)" }}>stake</span>
          <Input className="w-28" value={stake} onChange={(e) => setStake(e.target.value)} placeholder="sat" />
          <span style={{ color: "var(--muted)" }}>sat</span>
        </div>
      </div>
      <div className="flex gap-3 flex-wrap">
        <Side side="YES" price={yes.priceYes} shares={yes.shares} avgPrice={yes.avgPrice} href={href("YES", amt)} />
        <Side side="NO" price={no.priceNo} shares={no.shares} avgPrice={no.avgPrice} href={href("NO", amt)} />
      </div>
      <p className="text-xs" style={{ color: "var(--muted)" }}>
        AMM (LMSR) price anchored on current odds — the mint is the market maker, so any market is
        bettable instantly. Larger bets move the price (slippage shown as avg). Resting limit orders
        live in the book below.
      </p>
    </section>
  );
}
