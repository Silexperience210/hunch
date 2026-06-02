// End-to-end for the plain wallet helpers (top-up + withdraw) against a running cdk-mintd.
// Companion to wallet-e2e.ts (which covers the conditional outcome-token flow).
//
// Run (after starting a fakewallet mint, see .github/workflows/web-mint-e2e.yml):
//   HUNCH_MINT_URL=http://127.0.0.1:8085 node --experimental-strip-types scripts/wallet-fns-e2e.ts
//
// Flow: mint plain (unlocked) proofs via a paid Lightning quote (deposit / top-up), then melt them
// to pay a bolt11 invoice (withdraw). On fakewallet both settle automatically.

import { Wallet } from "@cashu/cashu-ts";
import { mintPlain, meltToInvoice } from "../lib/wallet.ts";

const MINT = process.env.HUNCH_MINT_URL ?? "http://127.0.0.1:8085";
const sleep = (ms: number) => new Promise((r) => setTimeout(r, ms));

async function paidQuote(w: Wallet, amount: number) {
  const q: any = await w.createMintQuote("bolt11", { amount, unit: "sat" });
  for (let i = 0; i < 80; i++) {
    const s: any = await w.checkMintQuote("bolt11", q);
    if (s.state === "PAID") break;
    await sleep(250);
  }
  return q;
}

async function main() {
  const w = new Wallet(MINT, { unit: "sat" });
  await w.loadMint();

  // 1) Deposit / top-up: mint plain unlocked proofs.
  const proofs = await mintPlain(w, 10, await paidQuote(w, 10));
  const minted = proofs.reduce((s, p) => s + Number(p.amount), 0);
  if (minted !== 10) throw new Error(`expected 10 sat minted, got ${minted}`);
  console.log(`✔ deposited (minted plain) ${minted} sat in ${proofs.length} proof(s)`);

  // 2) Withdraw: melt the proofs to pay a fresh bolt11 invoice.
  const dest: any = await w.createMintQuote("bolt11", { amount: 5, unit: "sat" });
  const { change, paid, fee, state } = await meltToInvoice(w, proofs, dest.request);
  if (state !== "PAID") throw new Error(`melt not paid: state=${state}`);
  console.log(`✔ withdrew ${paid} sat (fee reserve ${fee}); change ${change.length} proof(s)`);

  console.log(`WALLET FNS E2E OK against ${MINT}`);
}

main().catch((e) => {
  console.error("wallet fns e2e failed:", e);
  process.exit(1);
});
