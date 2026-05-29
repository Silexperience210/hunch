// Publish a signed event to relays (browser WebSocket), mirroring hunch-nostr::relay::publish.

import type { NostrEvent } from "./hunch.ts";

export interface PublishResult {
  relay: string;
  accepted: boolean;
  message: string;
}

/** Sends ["EVENT", ev] to one relay and resolves on its ["OK", id, accepted, msg] (or timeout). */
export function publish(url: string, ev: NostrEvent, timeoutMs = 8000): Promise<PublishResult> {
  return new Promise((resolve) => {
    let settled = false;
    let ws: WebSocket;
    const finish = (accepted: boolean, message: string) => {
      if (settled) return;
      settled = true;
      try {
        ws.close();
      } catch {}
      resolve({ relay: url, accepted, message });
    };
    const timer = setTimeout(() => finish(false, "timeout"), timeoutMs);
    try {
      ws = new WebSocket(url);
    } catch (e) {
      clearTimeout(timer);
      return resolve({ relay: url, accepted: false, message: String(e) });
    }
    ws.onopen = () => ws.send(JSON.stringify(["EVENT", ev]));
    ws.onerror = () => {
      clearTimeout(timer);
      finish(false, "connection error");
    };
    ws.onmessage = (msg) => {
      let m: any;
      try {
        m = JSON.parse(typeof msg.data === "string" ? msg.data : "");
      } catch {
        return;
      }
      if (m[0] === "OK" && m[1] === ev.id) {
        clearTimeout(timer);
        finish(Boolean(m[2]), m[3] ?? "");
      } else if (m[0] === "NOTICE") {
        clearTimeout(timer);
        finish(false, "NOTICE: " + (m[1] ?? ""));
      }
    };
  });
}

/** Publishes to all relays; returns one result each. */
export async function publishAll(urls: string[], ev: NostrEvent, timeoutMs = 8000): Promise<PublishResult[]> {
  return Promise.all(urls.map((u) => publish(u, ev, timeoutMs)));
}
