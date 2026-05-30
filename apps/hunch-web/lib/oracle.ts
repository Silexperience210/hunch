// Fetches an oracle's NIP-88 announce (kind 88) and attestation (kind 89) from relays.
//
// The `market` tag is multi-character, so relays don't index it (NIP-01 only indexes
// single-letter tags). We therefore filter by `authors:[oracle] + kind` — which relays DO
// index — and match the market tag client-side. Every event is Schnorr-verified before use
// (relays are untrusted); the newest valid match wins.

import { queryRelays, type RelayFilter } from "./relay.js";
import { verifyEvent } from "./verify.js";
import {
  KIND_ORACLE_ANNOUNCE,
  KIND_ORACLE_ATTESTATION,
  parseAnnounceEvent,
  parseAttestationEvent,
  type NostrEvent,
  type OracleAnnounce,
  type OracleAttestation,
} from "./hunch.js";

/** Picks the verified, market-matching, parseable event with the greatest created_at. */
function newestMatch<T>(
  events: NostrEvent[],
  parse: (ev: NostrEvent) => T | null,
  marketOf: (parsed: T) => string,
  market: string,
): T | null {
  let best: T | null = null;
  let bestAt = -1;
  for (const ev of events) {
    if (!verifyEvent(ev)) continue;
    const parsed = parse(ev);
    if (!parsed || marketOf(parsed) !== market) continue;
    if (ev.created_at > bestAt) {
      best = parsed;
      bestAt = ev.created_at;
    }
  }
  return best;
}

/** Fetches the oracle's latest announce for `market` (carries the nonce R), or null. */
export async function fetchAnnounce(
  relays: string[],
  oraclePubkey: string,
  market: string,
  limit = 200,
): Promise<OracleAnnounce | null> {
  const filter: RelayFilter = { kinds: [KIND_ORACLE_ANNOUNCE], authors: [oraclePubkey], limit };
  const events = await queryRelays(relays, filter);
  return newestMatch(events, parseAnnounceEvent, (a) => a.market, market);
}

/** Fetches the oracle's latest attestation for `market` (the settlement), or null. */
export async function fetchAttestation(
  relays: string[],
  oraclePubkey: string,
  market: string,
  limit = 200,
): Promise<OracleAttestation | null> {
  const filter: RelayFilter = { kinds: [KIND_ORACLE_ATTESTATION], authors: [oraclePubkey], limit };
  const events = await queryRelays(relays, filter);
  return newestMatch(events, parseAttestationEvent, (a) => a.market, market);
}
