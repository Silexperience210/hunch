// Event templates for the write path (mirrors hunch-cli's build_market / order_tags_with_d).
// Pure tag/content assembly — offline-testable. Signing happens via NIP-07 in `sign.ts`.

import { KIND_MARKET, KIND_ORDER, KIND_REPUTATION, OUTCOMES } from "./hunch.ts";

const SEVEN_DAYS = 7 * 24 * 3600;

export interface EventTemplate {
  kind: number;
  tags: string[][];
  content: string;
}

export interface MarketParams {
  slug: string;
  oracle: string;
  expiry: number;
  refundTimeout?: number;
  mint: string;
  dlcContract: string;
  question: string;
  resolution?: string;
  sources?: string[];
  rulesVersion?: string;
  category?: string;
  image?: string;
  topics?: string[];
}

/** Builds the unsigned kind:30888 market event template. Outcomes are the HIP-2 canonical set. */
export function buildMarketTemplate(p: MarketParams): EventTemplate {
  const refund = p.refundTimeout ?? p.expiry + SEVEN_DAYS;
  const tags: string[][] = [
    ["d", p.slug],
    ["oracle", p.oracle],
    ["outcomes", OUTCOMES.join(",")],
    ["expiry", String(p.expiry)],
    ["refund_timeout", String(refund)],
    ["mint", p.mint],
    ["dlc_contract", p.dlcContract],
  ];
  if (p.category) tags.push(["category", p.category]);
  if (p.image) tags.push(["image", p.image]);
  for (const t of p.topics ?? []) tags.push(["t", t]);

  const content = JSON.stringify({
    question: p.question,
    resolution_criteria: p.resolution ?? "",
    sources: p.sources ?? [],
    rules_version: p.rulesVersion ?? "1.0",
  });
  return { kind: KIND_MARKET, tags, content };
}

export interface OrderParams {
  market: string;
  side: "YES" | "NO";
  amount: number;
  price: number;
  kind: "bid" | "ask";
  expires: number;
}

/** Builds the unsigned kind:38888 order template, with `d` == market (addressable + #d-filterable). */
export function buildOrderTemplate(p: OrderParams): EventTemplate {
  return {
    kind: KIND_ORDER,
    tags: [
      ["market", p.market],
      ["side", p.side],
      ["amount", String(p.amount)],
      ["price", String(p.price)],
      ["kind", p.kind],
      ["expires", String(p.expires)],
      ["d", p.market],
    ],
    content: "",
  };
}
