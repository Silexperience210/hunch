// Browser Cashu wallet helpers — thin wrappers over cashu-ts, mirroring the proven flow in
// scripts/wallet-e2e.ts (verified live against cdk-mintd in CI). Deposit pays a Lightning invoice
// (WebLN), then mints outcome tokens P2PK-locked to L_X; redeem signs with l_X after attestation.

import { Wallet, type Proof } from "@cashu/cashu-ts";

export async function connect(mintUrl: string): Promise<Wallet> {
  const wallet = new Wallet(mintUrl, { unit: "sat" });
  await wallet.loadMint();
  return wallet;
}

/** Creates a mint quote; returns the quote object and its bolt11 invoice to pay. */
export async function depositQuote(wallet: Wallet, amount: number): Promise<{ quote: any; invoice: string }> {
  const quote: any = await wallet.createMintQuote("bolt11", { amount, unit: "sat" });
  return { quote, invoice: quote.request ?? quote.invoice ?? "" };
}

/** Pays a bolt11 invoice with the user's WebLN provider (Alby, etc.). */
export async function payWithWebln(invoice: string): Promise<void> {
  const webln = (globalThis as any).webln;
  if (!webln) throw new Error("No WebLN provider found. Install Alby or another WebLN wallet.");
  await webln.enable();
  await webln.sendPayment(invoice);
}

/** Waits until the mint marks the quote PAID (or throws after `tries` polls). */
export async function waitPaid(wallet: Wallet, quote: any, tries = 120): Promise<void> {
  for (let i = 0; i < tries; i++) {
    const status: any = await wallet.checkMintQuote("bolt11", quote);
    if (status.state === "PAID") return;
    await new Promise((r) => setTimeout(r, 1000));
  }
  throw new Error("mint quote not paid in time");
}

/** Mints `amount` sat of proofs P2PK-locked to `lockPubkey` (L_X), reclaimable by `refundPubkey`. */
export async function mintLocked(
  wallet: Wallet,
  amount: number,
  quote: any,
  lockPubkey: string,
  refundPubkey: string,
  locktime: number,
): Promise<Proof[]> {
  return wallet.mintProofs("bolt11", amount, quote, undefined, {
    type: "p2pk",
    options: { pubkey: lockPubkey, locktime, refundKeys: [refundPubkey] },
  });
}

/** Redeems P2PK-locked proofs by signing with `spendPrivkey` (l_X). Returns fresh proofs. */
export async function redeem(wallet: Wallet, proofs: Proof[], spendPrivkey: string): Promise<Proof[]> {
  return wallet.receive(proofs, { privkey: spendPrivkey });
}

/** cashu-ts returns amounts as `Amount` objects ({ value: bigint }) in some responses; normalize. */
function amountToNumber(a: unknown): number {
  if (typeof a === "number") return a;
  if (typeof a === "bigint") return Number(a);
  const v = (a as { value?: unknown })?.value;
  return typeof v === "bigint" ? Number(v) : Number(v ?? 0);
}

/** Mints `amount` sat of plain (unlocked) proofs after the quote is paid — a wallet top-up. */
export async function mintPlain(wallet: Wallet, amount: number, quote: any): Promise<Proof[]> {
  return wallet.mintProofs("bolt11", amount, quote);
}

/** Withdraw: melts `proofs` to pay a bolt11 `invoice`. Returns the change proofs (overpayment swapped
 *  back) plus the amount and fee. Proven live in scripts/wallet-fns-e2e.ts. */
export async function meltToInvoice(
  wallet: Wallet,
  proofs: Proof[],
  invoice: string,
): Promise<{ change: Proof[]; paid: number; fee: number; state?: string }> {
  const quote = await wallet.createMeltQuoteBolt11(invoice);
  const paid = amountToNumber((quote as { amount?: unknown }).amount);
  const fee = amountToNumber((quote as { fee_reserve?: unknown }).fee_reserve);
  const res: any = await wallet.meltProofsBolt11(quote, proofs);
  return { change: res.change ?? [], paid, fee, state: res.quote?.state };
}
