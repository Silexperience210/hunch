# Mint-as-Market-Maker — Settlement via Operator Reserve (Path 2)

**Status:** design / scoping. The pricing + fee + settlement-accounting engine is built and tested
(`hunch-mm`); this document frames the remaining plumbing that turns the computed rake into real sats
on the operator's mint.

## 1. The gap today

The current 1-click bet is **1:1 escrow**, not an odds-based bet:

- Bettor deposits `cost` sats → cdk-mintd mints `cost` worth of tokens P2PK-locked to `L_X = B + S_X`.
- Oracle attests X → bettor redeems with `l_X = b + s_X` → gets **`cost` back**. Losers' tokens are
  locked to the non-attested outcome and become unredeemable.

So winners only recover their stake; there are no winnings. A real market pays a winner
`shares` (> `cost`), funded by the losing side + the maker's liquidity. `hunch-mm` already prices
this (`quote_buy`), tracks inventory/rake (`apply_buy`), and computes the settlement P&L
(`settle`). What's missing is **issuing the winning token worth `shares`, not `cost`.**

## 2. Mechanism — MM-mediated buy, "issue at odds"

The operator runs **both** the mint (cdk-mintd) and the market maker. A buy becomes MM-mediated:

1. **Quote.** Bettor asks to buy `shares` of side X on a market. The MM prices it via the pool:
   `cost = fair + fee` (fee = the rake).
2. **Pay the MM.** The MM returns a Lightning invoice (operator LNbits) for `cost`, or accepts ecash.
3. **Issue at odds.** On payment, the MM mints **`shares`** worth of X-locked tokens from cdk-mintd —
   it pays the `shares`-sat mint quote **from its own reserve**, fronting `shares − cost`. The tokens
   are P2PK-locked to the **bettor's** `L_X = B + S_X` (the MM pays the quote; the *output* is locked
   to the bettor — a standard NUT-11 mint with `pubkey = L_X`, already proven in `wallet-e2e.ts`).
4. **Deliver.** The MM hands the `shares` X-locked tokens to the bettor (ecash).
5. **Record.** The MM calls `pool.apply_buy(X, shares)` → `q_X += shares`, `collected += cost`,
   `realized_rake += fee`.

At **settlement**:

- **X wins:** the bettor redeems their `shares` X-locked tokens with `l_X` → gets `shares` sats.
  Real winnings, paid from the reserve. ✅
- **X loses:** the bettor's tokens are dead; they lost `cost`. ✅
- **INVALID:** refund path (NUT-11 `locktime` + `refund` key = bettor) returns `cost`. ✅

The bettor's trustless guarantee at settlement is unchanged: only the oracle's attested outcome
unlocks the winning tokens (DLC lock `L_X`), and the tokens are real Cashu the operator cannot
un-issue.

## 3. Reserve economics

Because the operator **is** the mint, money flows through one wallet:

- Everything bettors pay (`collected`) enters the reserve.
- At settlement, winners redeem `payout = q_winner` out of the reserve.
- **Operator net = `collected − payout = mm_pnl`** — exactly what `Pool::settle` returns.

Worst-case draw-down is bounded: `mm_pnl ≥ rake − b·ln2`. So the operator must keep **`b·ln2` sats of
its own working capital** in reserve per active market (the max it can be down on a one-sided
settlement). A **hedged book (`q_yes == q_no`) returns exactly the rake with zero directional risk**
(proven live). Everything else nets out; the seed is recovered at settlement plus/minus the swing.

**Sizing:** depth `b` sets both liquidity quality and capital at risk (`b·ln2`). Start small
(e.g. `b = 10_000` → ~6.9k sat at risk/market), scale with volume and the tiered-launch caps.

## 4. Trust model & cypherpunk mitigations

This is **custodial-during-market** — the operator holds funds and is the explicit counterparty
(the "house"). That is what HIP-3 already accepts. Mitigations (all per `CLAUDE.md`):

- **Reserves proofs published** (mint announce kind 30892 `reserves_proof`, weekly) — non-optional.
- **Settlement stays trustless**: the oracle Schnorr attestation is the only thing that unlocks
  winners; the operator can't change outcomes, only refuse service *before* settlement.
- **Real, held tokens**: once issued, winning tokens are the bettor's; the operator can't claw back.
- **Multi-mint**: the operator is one mint among many; bad behavior → bettors leave.
- **Logging never deanonymizes**: the MM sees position sizes per ephemeral bettor pubkey `B`, never
  identity. No analytics, no identity linkage (`CLAUDE.md`).

## 5. ⚠️ Regulatory note (read PITFALLS.md)

Being the explicit counterparty/bookmaker is materially different from running neutral P2P infra: it
looks like *operating a betting/derivatives venue*. This raises the CFTC-enforcement and §1960
exposure flagged in `.planning/research/PITFALLS.md`. The offshore-entity / no-US-ops / maintainer
pseudonymity constraints are **more** important under Path 2, not less. Scope the legal posture with
counsel before running a funded operator MM on mainnet at scale.

## 6. Components to build

- [ ] **MM buy service** (extend `hunch-mm` or a sibling crate): HTTP `POST /buy {market, side,
      shares|budget, bettor_pubkey}` → quote → LNbits invoice → on-paid mint `shares` X-locked tokens
      (cdk-mintd) paying from reserve → deliver tokens → `apply_buy`. Reuses `hunch-dlc` for `L_X`,
      a cdk-mint HTTP client (cf. `crates/hunch-mint/tests/e2e_mint.rs`), and an LNbits client.
- [ ] **Reserve guard**: track reserve balance; refuse a buy whose worst-case draw-down would exceed
      available working capital; expose a `reserve` status endpoint.
- [ ] **Seed flow**: operator funds `b·ln2` working capital per market (LNbits → reserve).
- [ ] **Settlement reconcile**: after the oracle attests, run `settle`, verify reserve == expected,
      surface `mm_pnl`. Sweep realized rake to the operator's main wallet.
- [ ] **Frontend wiring**: `AmmPanel`'s 1-click "Bet" calls `/buy` (MM-mediated) instead of the
      direct `deposit→mintLocked`; keep the direct path as a fallback. Show "the mint is the
      counterparty" + the fee (already shown) + a link to reserves proof.
- [ ] **Reserves proof publication**: periodic kind 30892 update.

## 7. Already done (this engine)

- `pool.rs` — LMSR pricing + maker fee (rake) + inventory; `quote_buy`, `apply_buy`,
  `shares_for_budget`, `max_subsidy`, **`settle`** (payout + `mm_pnl`).
- `store.rs` — persistent per-market ledger (seed / fills / rake).
- `main.rs` — CLI `seed | quote | buy | rake | settle | list`.
- Frontend `lib/amm.ts` is fee-aware and the panel shows the maker fee.

The engine already answers, for any market, **exactly how much the MM must front and how much it
keeps**. Path 2 is wiring that ledger to cdk-mintd issuance + the operator reserve.
