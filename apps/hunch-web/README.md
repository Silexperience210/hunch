# hunch-web

Static frontend for Hunch — lists prediction markets straight from Nostr relays.

- **Next.js 15 static export** (`output: "export"`) → deploy to Cloudflare Pages, IPFS, and a
  Tor hidden service with no backend (CLAUDE.md §Distribution).
- **Dependency-free protocol core** (`lib/hunch.ts`) mirroring the Rust `hunch-protocol` /
  `hunch-nostr` wire format; relay query in `lib/relay.ts` over the browser `WebSocket`.
- No analytics, no KYC fields, no tracking.

## Develop

```sh
npm install
npm run test     # offline: verifies lib/hunch.ts parsing + NIP-01 event id (node --test)
npm run dev      # http://localhost:3000
npm run build    # static export to ./out
```

`npm run test` needs no network or browser — it runs the dependency-free core with Node's
built-in test runner (same assertions as the Rust crates).

## Status

- ✅ Markets list (kind 30888) from relays, parsed + displayed.
- ⬜ Order book view (kind 38888) per market.
- ⬜ Create market / post order via NIP-07 (browser signer) + Schnorr verify via `@noble/curves`.
- ⬜ Cashu wallet (cashu-ts) + WebLN deposit/redeem against a Hunch mint (HIP-3).
