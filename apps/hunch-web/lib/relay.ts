// Minimal dependency-free Nostr relay client for the browser (mirrors `hunch-nostr::relay`).
// Uses the global WebSocket; no SDK. Query-only (the frontend reads markets/orders).

export interface RelayFilter {
  kinds?: number[];
  authors?: string[];
  ids?: string[];
  limit?: number;
  since?: number;
  until?: number;
  [tag: `#${string}`]: string[] | undefined;
}

/** Opens a one-shot subscription on one relay and resolves with stored events at EOSE/timeout. */
export function queryRelay(url: string, filter: RelayFilter, timeoutMs = 8000): Promise<any[]> {
  return new Promise((resolve) => {
    const events: any[] = [];
    let settled = false;
    let ws: WebSocket;
    const subId = "hunch-" + Math.random().toString(36).slice(2, 10);

    const done = () => {
      if (settled) return;
      settled = true;
      try {
        ws.send(JSON.stringify(["CLOSE", subId]));
        ws.close();
      } catch {}
      resolve(events);
    };

    const timer = setTimeout(done, timeoutMs);

    try {
      ws = new WebSocket(url);
    } catch {
      clearTimeout(timer);
      resolve([]);
      return;
    }

    ws.onopen = () => ws.send(JSON.stringify(["REQ", subId, filter]));
    ws.onerror = () => {
      clearTimeout(timer);
      done();
    };
    ws.onmessage = (msg) => {
      let m: any;
      try {
        m = JSON.parse(typeof msg.data === "string" ? msg.data : "");
      } catch {
        return;
      }
      if (m[0] === "EVENT" && m[1] === subId) events.push(m[2]);
      else if (m[0] === "EOSE" && m[1] === subId) {
        clearTimeout(timer);
        done();
      }
    };
  });
}

/** Queries several relays in parallel and de-duplicates events by id. */
export async function queryRelays(urls: string[], filter: RelayFilter, timeoutMs = 8000): Promise<any[]> {
  const results = await Promise.all(urls.map((u) => queryRelay(u, filter, timeoutMs)));
  const byId = new Map<string, any>();
  for (const evs of results) for (const ev of evs) if (ev?.id) byId.set(ev.id, ev);
  return [...byId.values()];
}

/** Default public relays (override in the UI). Hunch is multi-relay by design (CLAUDE.md). */
export const DEFAULT_RELAYS = ["wss://nos.lol", "wss://relay.damus.io"];

/**
 * Relays to use for reads: the `?relays=` query param (comma-separated wss URLs) if present,
 * else DEFAULT_RELAYS. Lets a deployed static site point at a self-hosted relay with no rebuild.
 */
export function relaysFromUrl(): string[] {
  if (typeof window === "undefined") return DEFAULT_RELAYS;
  const raw = new URLSearchParams(window.location.search).get("relays");
  if (!raw) return DEFAULT_RELAYS;
  const list = raw.split(",").map((s) => s.trim()).filter((s) => s.startsWith("ws"));
  return list.length ? list : DEFAULT_RELAYS;
}
