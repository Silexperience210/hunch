// Client for the mint-as-market-maker `/buy` service (crates/hunch-mm).
//
// Opt-in: only used when an MM URL is configured (localStorage `hunch:mm-url`). When set, the bettor
// can buy at the MM's LMSR odds — the MM issues `shares` outcome-locked tokens (issue-at-odds) which
// are redeemed after settlement exactly like any outcome token. With no MM URL the default flow
// (deposit → mintLocked) is unchanged.
//
// buildBuyBody is pure + tested; quote/buy are thin fetch wrappers.

const MM_URL_KEY = "hunch:mm-url";

export function mmUrl(): string {
  if (typeof window === "undefined") return "";
  return (localStorage.getItem(MM_URL_KEY) || "").trim();
}

export function setMmUrl(u: string): void {
  if (typeof window !== "undefined") localStorage.setItem(MM_URL_KEY, u.trim());
}

export interface MmQuote {
  shares: number;
  fair: number;
  fee: number;
  cost: number;
  avgPrice: number;
  priceBefore: number;
  priceAfter: number;
}

export interface BuyParams {
  market: string;
  side: "YES" | "NO";
  budget?: number;
  shares?: number;
  /** Bettor's outcome lock L_X = B + S_X (33-byte compressed hex). */
  lock: string;
  /** Bettor's refund key B (33-byte compressed hex). */
  refund: string;
  locktime?: number;
  /** Cashu proofs paying the quoted cost (the MM claims these before issuing). */
  payment?: unknown[];
}

const HEX33 = /^[0-9a-f]{66}$/i;

/** Build + validate the `/buy` (or `/quote`) request body. Pure + tested. */
export function buildBuyBody(p: BuyParams): Record<string, unknown> {
  if (!p.market.trim()) throw new Error("market required");
  if (p.side !== "YES" && p.side !== "NO") throw new Error("side must be YES or NO");
  if (!HEX33.test(p.lock)) throw new Error("lock (L_X) must be 33-byte compressed hex");
  if (!HEX33.test(p.refund)) throw new Error("refund (B) must be 33-byte compressed hex");
  if (!p.budget && !p.shares) throw new Error("budget or shares required");
  const body: Record<string, unknown> = {
    market: p.market.trim(),
    side: p.side,
    lock: p.lock.toLowerCase(),
    refund: p.refund.toLowerCase(),
  };
  if (p.budget) body.budget = p.budget;
  if (p.shares) body.shares = p.shares;
  if (p.locktime) body.locktime = p.locktime;
  if (p.payment && p.payment.length) body.payment = p.payment;
  return body;
}

async function post(url: string, path: string, body: unknown): Promise<any> {
  const res = await fetch(`${url.replace(/\/+$/, "")}${path}`, {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: JSON.stringify(body),
  });
  const j = await res.json().catch(() => ({}));
  if (!res.ok) throw new Error((j as { error?: string }).error || res.statusText);
  return j;
}

/** Fetch a live MM quote (cost + odds) for a stake. */
export async function mmQuoteFetch(
  url: string,
  market: string,
  side: "YES" | "NO",
  budget: number,
): Promise<MmQuote> {
  return post(url, "/quote", { market, side, budget });
}

export type BuyResult =
  | { shares: number; cost: number; fee: number; proofs: unknown[] }
  /** Issuance failed after the payment was claimed — the MM hands the funds back as fresh proofs. */
  | { refunded: true; error: string; refund: unknown[] };

/** Buy at the MM odds: the MM claims the payment then issues `shares` outcome-locked tokens. */
export async function mmBuy(url: string, p: BuyParams): Promise<BuyResult> {
  return post(url, "/buy", buildBuyBody(p));
}
