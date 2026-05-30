// Hunch protocol — TypeScript client core (mirrors the Rust `hunch-protocol` / `hunch-nostr`).
//
// Zero dependencies: pure parsing + the NIP-01 event-id algorithm. Schnorr signature
// verification (browser) uses @noble/curves and lives in `crypto.ts`; this module is the
// dependency-free, offline-testable core.

export const KIND_MARKET = 30888;
export const KIND_ORDER = 38888;
export const KIND_ORACLE_ANNOUNCE = 88;
export const KIND_ORACLE_ATTESTATION = 89;
export const KIND_REPUTATION = 30891;

/** The HIP-2 canonical outcomes, in order. */
export const OUTCOMES = ["YES", "NO", "INVALID"] as const;

export interface NostrEvent {
  id: string;
  pubkey: string;
  created_at: number;
  kind: number;
  tags: string[][];
  content: string;
  sig: string;
}

/** HIP-1 market identifier: `<creator_pubkey>:30888:<d>`. */
export function marketId(creatorPubkey: string, d: string): string {
  return `${creatorPubkey}:${KIND_MARKET}:${d}`;
}

function tagValue(tags: string[][], name: string): string | undefined {
  return tags.find((t) => t[0] === name)?.[1];
}

function tagValues(tags: string[][], name: string): string[] {
  return tags.filter((t) => t[0] === name).map((t) => t[1]).filter((v): v is string => v != null);
}

export interface MarketContent {
  question: string;
  resolution_criteria: string;
  sources: string[];
  rules_version: string;
}

export interface Market {
  id: string;
  creator: string;
  d: string;
  oracle: string;
  outcomes: string[];
  expiry: number;
  refundTimeout: number;
  mint: string;
  dlcContract: string;
  category?: string;
  image?: string;
  topics: string[];
  content: MarketContent;
}

/** Parses a kind:30888 event into a Market, or returns null if malformed. */
export function parseMarketEvent(ev: NostrEvent): Market | null {
  if (ev.kind !== KIND_MARKET) return null;
  const d = tagValue(ev.tags, "d");
  const oracle = tagValue(ev.tags, "oracle");
  const outcomesRaw = tagValue(ev.tags, "outcomes");
  const expiry = tagValue(ev.tags, "expiry");
  const refundTimeout = tagValue(ev.tags, "refund_timeout");
  const mint = tagValue(ev.tags, "mint");
  const dlcContract = tagValue(ev.tags, "dlc_contract");
  if (!d || !oracle || !outcomesRaw || !expiry || !refundTimeout || !mint || !dlcContract) return null;

  let content: MarketContent;
  try {
    content = JSON.parse(ev.content);
  } catch {
    return null;
  }
  if (typeof content?.question !== "string") return null;

  return {
    id: marketId(ev.pubkey, d),
    creator: ev.pubkey,
    d,
    oracle,
    outcomes: outcomesRaw.split(",").map((s) => s.trim()),
    expiry: Number(expiry),
    refundTimeout: Number(refundTimeout),
    mint,
    dlcContract,
    category: tagValue(ev.tags, "category"),
    image: tagValue(ev.tags, "image"),
    topics: tagValues(ev.tags, "t"),
    content: {
      question: content.question,
      resolution_criteria: content.resolution_criteria ?? "",
      sources: Array.isArray(content.sources) ? content.sources : [],
      rules_version: content.rules_version ?? "",
    },
  };
}

export interface Order {
  author: string;
  market: string;
  side: "YES" | "NO";
  amount: number;
  price: number;
  kind: "bid" | "ask";
  expires: number;
}

/** Parses a kind:38888 event into an Order, or returns null if malformed. */
export function parseOrderEvent(ev: NostrEvent): Order | null {
  if (ev.kind !== KIND_ORDER) return null;
  const market = tagValue(ev.tags, "market");
  const side = tagValue(ev.tags, "side");
  const amount = tagValue(ev.tags, "amount");
  const price = tagValue(ev.tags, "price");
  const kind = tagValue(ev.tags, "kind");
  const expires = tagValue(ev.tags, "expires");
  if (!market || !amount || !price || !expires) return null;
  if (side !== "YES" && side !== "NO") return null;
  if (kind !== "bid" && kind !== "ask") return null;
  return {
    author: ev.pubkey,
    market,
    side,
    amount: Number(amount),
    price: Number(price),
    kind,
    expires: Number(expires),
  };
}

export interface OracleAnnounce {
  /** The market id the oracle commits to attest. */
  market: string;
  /** The oracle's announced public nonce R (x-only hex, 32 bytes). */
  nonce: string;
  /** Human-readable resolution note. */
  body: string;
}

/** Parses a kind:88 NIP-88 announce, or returns null if malformed (mirrors `OracleAnnounce::from_event`). */
export function parseAnnounceEvent(ev: NostrEvent): OracleAnnounce | null {
  if (ev.kind !== KIND_ORACLE_ANNOUNCE) return null;
  const market = tagValue(ev.tags, "market");
  const nonce = tagValue(ev.tags, "nonce");
  if (!market || !nonce) return null;
  if (!/^[0-9a-f]{64}$/i.test(nonce)) return null; // 32-byte hex
  return { market, nonce: nonce.toLowerCase(), body: ev.content };
}

export interface OracleAttestation {
  /** The market id this attestation resolves. */
  market: string;
  /** The attested outcome (YES / NO / INVALID). */
  outcome: string;
  /** The DLC attestation signature (BIP-340 with the pre-committed nonce), 64-byte hex. */
  signature: string;
}

/** Parses a kind:89 NIP-88 attestation, or returns null if malformed (mirrors `OracleAttestation::from_event`). */
export function parseAttestationEvent(ev: NostrEvent): OracleAttestation | null {
  if (ev.kind !== KIND_ORACLE_ATTESTATION) return null;
  const market = tagValue(ev.tags, "market");
  const outcome = tagValue(ev.tags, "outcome");
  // The signature lives in the `sig` tag with empty content (see OracleAttestation::to_event_parts).
  const signature = tagValue(ev.tags, "sig")?.trim();
  if (!market || !outcome || !signature) return null;
  if (!(OUTCOMES as readonly string[]).includes(outcome)) return null;
  if (!/^[0-9a-f]{128}$/i.test(signature)) return null; // 64-byte hex
  return { market, outcome, signature: signature.toLowerCase() };
}

/** HIP-5 reputation scopes (mirrors `ReputationScope`). */
export const REPUTATION_SCOPES = ["oracle", "mint", "market_creator", "bettor"] as const;
export type ReputationScope = (typeof REPUTATION_SCOPES)[number];

export interface Reputation {
  /** The author of the claim (event pubkey). */
  rater: string;
  /** The rated pubkey — the `p` tag (x-only hex, 32 bytes). */
  target: string;
  /** What is being rated. */
  scope: ReputationScope;
  /** Score in [-100, +100] (HIP-5 §Score Semantics). */
  score: number;
  /** Optional market this claim is scoped to. */
  market?: string;
  /** Free-form justification (content). */
  note: string;
  /** When the claim was signed (for newest-per-rater dedup). */
  createdAt: number;
}

/** Parses a kind:30891 HIP-5 reputation claim, or null if malformed (mirrors `Reputation::from_event`). */
export function parseReputationEvent(ev: NostrEvent): Reputation | null {
  if (ev.kind !== KIND_REPUTATION) return null;
  const target = tagValue(ev.tags, "p");
  const scope = tagValue(ev.tags, "scope");
  const scoreRaw = tagValue(ev.tags, "score");
  if (!target || !scope || scoreRaw == null) return null;
  if (!/^[0-9a-f]{64}$/i.test(target)) return null;
  if (!(REPUTATION_SCOPES as readonly string[]).includes(scope)) return null;
  const score = Number(scoreRaw);
  if (!Number.isInteger(score) || score < -100 || score > 100) return null;
  return {
    rater: ev.pubkey,
    target: target.toLowerCase(),
    scope: scope as ReputationScope,
    score,
    market: tagValue(ev.tags, "market"),
    note: ev.content,
    createdAt: ev.created_at,
  };
}

export interface ReputationSummary {
  /** Rounded mean score in [-100, +100] across distinct raters. */
  avg: number;
  /** Number of distinct raters. */
  count: number;
}

/** Aggregates claims into a mean score + rater count, keeping the newest claim per rater. */
export function aggregateReputation(reps: Reputation[]): ReputationSummary | null {
  const byRater = new Map<string, Reputation>();
  for (const r of reps) {
    const prev = byRater.get(r.rater);
    if (!prev || r.createdAt > prev.createdAt) byRater.set(r.rater, r);
  }
  if (byRater.size === 0) return null;
  const vals = [...byRater.values()].map((r) => r.score);
  return { avg: Math.round(vals.reduce((a, b) => a + b, 0) / vals.length), count: byRater.size };
}

/**
 * NIP-01 event id: sha256 of the canonical serialization
 * `[0, pubkey, created_at, kind, tags, content]`.
 *
 * `JSON.stringify` produces the same compact, control-char-escaped form serde_json does in
 * `hunch-nostr`, so this id matches the Rust implementation byte-for-byte.
 */
export async function computeEventId(
  ev: Pick<NostrEvent, "pubkey" | "created_at" | "kind" | "tags" | "content">,
  sha256: (data: Uint8Array) => Promise<Uint8Array> | Uint8Array,
): Promise<string> {
  const serialized = JSON.stringify([0, ev.pubkey, ev.created_at, ev.kind, ev.tags, ev.content]);
  const digest = await sha256(new TextEncoder().encode(serialized));
  return [...digest].map((b) => b.toString(16).padStart(2, "0")).join("");
}

/** Canonical NIP-01 serialization string used for the event id (exposed for testing). */
export function canonicalSerialization(
  ev: Pick<NostrEvent, "pubkey" | "created_at" | "kind" | "tags" | "content">,
): string {
  return JSON.stringify([0, ev.pubkey, ev.created_at, ev.kind, ev.tags, ev.content]);
}
