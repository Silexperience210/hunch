// Fetches a mint's HIP-1 announce (kind 30892) — its endpoint, reserves-proof, and accepted oracles.
//
// The announce is mint-wide (no market tag): the mint sets its `d` tag to its mint id, which relays
// index, so we filter by `#d`. Each event is Schnorr-verified (relays are untrusted); newest wins.

import { queryRelays, type RelayFilter } from "./relay.ts";
import { verifyEvent } from "./verify.ts";
import { KIND_MINT_ANNOUNCE, parseMintAnnounceEvent, type MintAnnounce } from "./hunch.ts";

/** Fetches the newest verified announce from the mint identified by `mintId`. */
export async function fetchMintAnnounce(
  relays: string[],
  mintId: string,
  limit = 50,
): Promise<MintAnnounce | null> {
  const filter: RelayFilter = { kinds: [KIND_MINT_ANNOUNCE], "#d": [mintId], limit };
  const events = await queryRelays(relays, filter);
  let best: MintAnnounce | null = null;
  let bestAt = -1;
  for (const ev of events) {
    if (!verifyEvent(ev)) continue;
    const m = parseMintAnnounceEvent(ev);
    if (!m || m.mintId !== mintId) continue;
    if (ev.created_at > bestAt) {
      best = m;
      bestAt = ev.created_at;
    }
  }
  return best;
}
