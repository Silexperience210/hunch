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
