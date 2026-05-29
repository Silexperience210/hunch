// Order-book assembly — pure, offline-testable. Groups parsed orders by side and sorts them
// (bids high→low, asks low→high), and surfaces the best prices. Display-only; matching lives in
// the Rust `hunch-matcher`.

import type { Order } from "./hunch.ts";

export interface OrderBook {
  yesBids: Order[];
  yesAsks: Order[];
  noBids: Order[];
  noAsks: Order[];
  bestYesBid?: number;
  bestYesAsk?: number;
  bestNoBid?: number;
  bestNoAsk?: number;
}

const byPriceDesc = (a: Order, b: Order) => b.price - a.price;
const byPriceAsc = (a: Order, b: Order) => a.price - b.price;

/** Builds an order book from parsed orders, dropping any that don't reference `market`. */
export function buildOrderBook(orders: Order[], market?: string): OrderBook {
  const scoped = market ? orders.filter((o) => o.market === market) : orders;
  const pick = (side: "YES" | "NO", kind: "bid" | "ask") =>
    scoped.filter((o) => o.side === side && o.kind === kind);

  const yesBids = pick("YES", "bid").sort(byPriceDesc);
  const yesAsks = pick("YES", "ask").sort(byPriceAsc);
  const noBids = pick("NO", "bid").sort(byPriceDesc);
  const noAsks = pick("NO", "ask").sort(byPriceAsc);

  return {
    yesBids,
    yesAsks,
    noBids,
    noAsks,
    bestYesBid: yesBids[0]?.price,
    bestYesAsk: yesAsks[0]?.price,
    bestNoBid: noBids[0]?.price,
    bestNoAsk: noAsks[0]?.price,
  };
}
