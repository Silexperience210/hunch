# Hunch end-to-end demo (signet)

A full market lifecycle on **signet** — create → bet → settle → redeem — wiring
together every component. **Signet/regtest only** until external audit signoff
(CLAUDE.md). The conditional-token model is network-independent, so a signet run
exercises the exact path mainnet will use.

The Nostr side (relay + oracle + web) runs on any machine and is proven by
`scripts/fetch-attestation-e2e.ts` + the CI `web-mint-e2e` job. The mint side
(`cdk-mintd`) needs Linux/`protoc` and a signet Lightning backend — see
[`crates/hunch-mint/DEPLOYMENT.md`](../crates/hunch-mint/DEPLOYMENT.md).

## Components

| Component        | Command                                   | Notes                                  |
|------------------|-------------------------------------------|----------------------------------------|
| Relay            | `hunch-relay --listen 127.0.0.1:8099`     | or any public relay (e.g. `wss://nos.lol`) |
| Mint             | `cdk-mintd` (signet config)               | see DEPLOYMENT.md; Linux/Umbrel        |
| Oracle           | `hunch-oracle …`                          | keygen / announce / attest             |
| Web              | `cd apps/hunch-web && npm run dev`         | or the deployed static site            |

## 1. Build the Rust binaries

```
cargo build -p hunch-relay -p hunch-oracle -p hunch-cli
```

## 2. Start the relay

```
./target/debug/hunch-relay --listen 127.0.0.1:8099
```

Everything below uses `--relay ws://127.0.0.1:8099`. Swap in a public relay to
test multi-relay discovery.

## 3. Start the mint (signet)

Follow [`DEPLOYMENT.md`](../crates/hunch-mint/DEPLOYMENT.md). For the demo the
LDK-Node + Mutinynet backend is self-contained. Note the mint URL (e.g.
`http://127.0.0.1:8085`). Verify it serves keys:

```
curl http://127.0.0.1:8085/v1/keys
```

## 4. Create the oracle identity

```
OUT=$(./target/debug/hunch-oracle keygen)        # prints: secret: <hex> / pubkey: <hex>
SECRET=$(echo "$OUT" | awk '/^secret:/{print $2}')
ORACLE=$(echo  "$OUT" | awk '/^pubkey:/{print $2}')
```

Keep `$SECRET` offline. `$ORACLE` is the oracle's x-only pubkey.

## 5. Create a market

```
./target/debug/hunch market create \
  --secret <CREATOR_SECRET> \
  --slug btc-100k-eoy-2026 \
  --question "Will BTC close above \$100k on 2026-12-31?" \
  --oracle "$ORACLE" \
  --mint http://127.0.0.1:8085 \
  --expiry 1798675200 \
  --relay ws://127.0.0.1:8099
```

The market id is `<creator_pubkey>:30888:btc-100k-eoy-2026`. Open the web app at
`/market?id=<market-id>` to see the metadata, oracle reputation, and order book.

## 6. Oracle announces (commits the nonce R)

```
./target/debug/hunch-oracle announce \
  --secret "$SECRET" --market "<market-id>" \
  --body "Resolves on the Coinbase BTC-USD feed" \
  --relay ws://127.0.0.1:8099
```

The web market page now shows `oracle nonce: committed (…)` and the **Bet →**
link pre-fills the nonce.

## 7. Bet (mint conditional tokens)

In the web app, **Bet →** → generate a wallet key → **Deposit** → pay the signet
invoice (WebLN or manually) → **Pay & mint**. This mints Cashu tokens locked to
`L_YES = B + S_YES` (bettor key + the oracle's YES signature point). Saved in the
browser.

## 8. Oracle settles

```
./target/debug/hunch-oracle attest \
  --secret "$SECRET" --market "<market-id>" --outcome YES \
  --relay ws://127.0.0.1:8099
```

The nonce store locks R to YES — a second attest for a different outcome is
refused (reusing R would leak the oracle key).

## 9. Redeem

On the market page a **Settled: YES** banner appears with the oracle Schnorr
signature. On the Bet page, **Redeem** auto-fetches the kind:89 attestation,
derives the spend key `l_YES = b + s_YES`, and swaps the locked tokens at the
mint. A losing position can never derive its spend key; INVALID / silence is
recovered via the NUT-11 refund branch after the refund timeout.

## What's proven where

- **Nostr flow (steps 4-9, no mint):** `scripts/fetch-attestation-e2e.ts` runs
  the browser code against a real oracle event on a local relay.
- **Mint redemption:** Rust `tests/e2e_mint.rs` + the web `scripts/wallet-e2e.ts`
  run against a real `cdk-mintd` in CI (`mint-e2e`, `web-mint-e2e`).
- **This demo** is the human-driven composition of those proven parts on signet.
