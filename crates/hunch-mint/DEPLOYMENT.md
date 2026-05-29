# Hunch Mint — Deployment Runbook

The mint's **cryptographic protocol is code-complete and proven** against the real `cashu`
crate (`src/cashu_token.rs`): blind-signature issuance (NUT-00 BDHKE), outcome-conditional
redemption (NUT-11 P2PK to the oracle signature point), and the refund/INVALID branch
(NUT-11 locktime + refund). What remains is **operational** — standing up a real mint with a
Lightning backend. None of it requires a protocol change.

This runbook is the path from the proven crypto to a running signet mint (the HIP-3
Draft→Final gate is a signet end-to-end demo).

## Architecture

```
bettor wallet ──(LN deposit)──▶ cdk-mintd ──(blind sign)──▶ conditional token (P2PK → L_X)
                                   │                              │
                                   ▼                              ▼
                              LDK Node (LN)              oracle kind:89 attestation
                                                                  │
bettor ◀──(LN withdraw)── cdk-mintd ◀──(redeem: sign with l_X = b + s_X)──┘
```

- **cdk-mintd**: the Cashu mint daemon (`cashubtc/cdk`). Vanilla NUT-11 — no Hunch fork. The
  Hunch conditionality is entirely in the token's lock key (`L_X`, see `src/cashu_token.rs`),
  which the mint never needs to understand.
- **LDK Node**: Lightning backend for deposits/withdrawals. cdk supports several backends; LDK
  Node (`ldk-node`) keeps it self-custodial and embeddable.
- **Oracle**: `hunch-oracle` publishes the kind:89 attestation; bettors derive `l_X` and redeem.

## Steps

### 1. Lightning backend (signet)

- Run a signet `bitcoind` (or use a public signet node) and an `ldk-node` instance funded with
  signet coins. Confirm it can create/pay invoices.

### 2. Stand up cdk-mintd

- `cargo install cdk-mintd` (pin the version matching `cashu` in `Cargo.toml`, currently 0.16).
- Copy `cdk-mintd.example.toml` (this directory) to `~/.cdk-mintd/config.toml` and fill it in.
  Mint URL behind a Tor hidden service (CLAUDE.md); `nut11` (P2PK) is on by default.
- Start the daemon; verify `GET /v1/keys` and `/v1/info` advertise NUT-11 support.

#### Backend choices (in order of pre-audit preference)

1. **`fakewallet`** — no Lightning at all. Use for local CI / the in-process demo. Default in the
   example config.
2. **`ldknode` on Mutinynet (signet)** — self-contained signet node; the recommended backend for
   the HIP-3 signet end-to-end gate. No external infra.
3. **Umbrel via LNbits** — reuse the LNbits app already running on your Umbrel (the 21pay
   Cloudflare-tunnel setup). Create a *dedicated* LNbits wallet for the mint and put its
   `lnbits_api` URL + admin/invoice keys in `[lnbits]`. Easiest reuse of existing infra.
4. **Umbrel via LND gRPC** — point `[lnd]` at the node directly.

#### Pulling LND creds off Umbrel (option 4)

Umbrel runs LND. From the node (SSH `umbrel@umbrel.local`):

```
# TLS cert + a macaroon for the chain your node runs:
~/umbrel/app-data/lightning/data/lnd/tls.cert
~/umbrel/app-data/lightning/data/lnd/data/chain/bitcoin/<network>/admin.macaroon
```

Copy both to the mint host, set `cert_file` / `macaroon_file`, and `address = "https://<umbrel-ip>:10009"`
(open/forward LND's gRPC port, or run the mint on the Umbrel LAN).

#### ⚠ Mainnet gate

Umbrel runs **mainnet** by default. CLAUDE.md forbids mainnet for this alpha mint until external
audit signoff. **Pre-audit, use a signet/regtest backend** (option 2, or a signet LNbits/LND).
The conditional-token logic (`src/cashu_token.rs`) is identical on every network — only the
payment rail differs — so a signet demo fully validates the design. Reserve the Umbrel mainnet
node for the post-audit, tiered launch (100k → 1M → uncapped).

### 3. Issue conditional tokens

- The bettor wallet mints at amount A (NUT-04 quote → LN deposit → blind outputs).
- Lock the minted proof to `L_X`: build the NUT-11 secret with
  `hunch_mint::cashu_token::outcome_secret(L_X, refund=B, refund_timeout)`, where
  `L_X = hunch_dlc::outcome_lock_key(B, oracle_xonly, nonce_xonly, market, outcome)`.
- (`issue_via_bdhke` in this crate is the in-process reference for the blind-sign exchange.)

### 4. Settle & redeem

- After expiry, `hunch-oracle attest --market <id> --outcome <X>` publishes kind:89.
- Bettor derives `l_X = hunch_mint::redeem_spend_secret(b, attestation_sig)` and swaps the token
  at the mint, signing with `l_X` (NUT-11 P2PK witness). The mint's standard NUT-11 check passes.
- Withdraw via Lightning (NUT-05 melt).

### 5. Refund path (oracle silence / INVALID)

- After `refund_timeout`, the bettor reclaims via the NUT-11 refund key (`B`) — no mint logic
  beyond honoring NUT-11. Proven in `cashu_token::tests::refund_key_spends_after_locktime`.

## Operator obligations (CLAUDE.md)

- Publish weekly reserves proofs (kind:30892 `reserves_proof` tag): outstanding tokens per
  market, DLC funding outpoints, LN channel balances.
- Tor hidden service from day 1; clearnet optional.
- Logs MUST NOT deanonymize bettors (blind sigs are useless if logs link them).
- Signet only until external audit signoff (CLAUDE.md "NUT-DLC is alpha").

## Status / gates

- ✅ Conditional-token crypto: proven against real cashu (`src/cashu_token.rs`, 10 tests).
- ⬜ cdk-mintd + LDK signet instance (this runbook).
- ⬜ Signet end-to-end demo (deposit → issue → attest → redeem) — HIP-3 Draft→Final gate.
