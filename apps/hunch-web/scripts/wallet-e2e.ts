// End-to-end wallet test against a running cdk-mintd, using cashu-ts + our DLC key derivation.
// Mirrors the Rust crates/hunch-mint/tests/e2e_mint.rs, but exercises the *browser wallet* path.
//
// Run (after starting a mint, see .github/workflows/web-mint-e2e.yml):
//   HUNCH_MINT_URL=http://127.0.0.1:8085 node --experimental-strip-types scripts/wallet-e2e.ts
//
// Flow: mint a 1-sat token P2PK-locked to L_YES = B + S_YES, then redeem it by signing with
// l_YES = b + s_YES (derived from the oracle's YES attestation). The mint must accept the YES
// token and reject a NO-locked token signed with the YES key.

import { Wallet } from "@cashu/cashu-ts";
import { outcomeLockKey, outcomeUnlockSecret } from "../lib/dlc.ts";

const MINT = process.env.HUNCH_MINT_URL ?? "http://127.0.0.1:8085";

// Deterministic vectors (same as lib/dlc.test.ts; the oracle attested YES on this market).
const market = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa:30888:m";
const oracleXonly = "d1cf61ee61a17d034d4e38e81a91b9703294df384fe90dba5149bb1c8da4a680";
const nonceXonly = "4c5b9f8c55ddb85ad42af65f82ee19b10cb363abbd3e1d332d6601c6400a42b5";
const bettorSecret = "1111111111111111111111111111111111111111111111111111111111111111";
const bettorCompressed = "034f355bdcb7cc0af728ef3cceb9615d90684bb5b2ca5f859ab0f0b704075871aa";
const attestationSigYes =
  "4c5b9f8c55ddb85ad42af65f82ee19b10cb363abbd3e1d332d6601c6400a42b5455fabbf3640b22b502935431fbc9c5e28d2cf6bdde652ba544a192f49f739b9";

const lockYes = outcomeLockKey(bettorCompressed, oracleXonly, nonceXonly, market, "YES");
const lockNo = outcomeLockKey(bettorCompressed, oracleXonly, nonceXonly, market, "NO");
const spendYes = outcomeUnlockSecret(bettorSecret, attestationSigYes);

const sleep = (ms: number) => new Promise((r) => setTimeout(r, ms));

async function mintLocked(wallet: Wallet, pubkey: string) {
  const quote = await wallet.createMintQuote("bolt11", { amount: 1, unit: "sat" });
  // fakewallet auto-settles, but not instantly — wait for PAID.
  for (let i = 0; i < 80; i++) {
    const status = await wallet.checkMintQuote("bolt11", quote);
    if ((status as { state?: string }).state === "PAID") break;
    await sleep(250);
  }
  return wallet.mintProofs("bolt11", 1, quote, undefined, {
    type: "p2pk",
    options: { pubkey, locktime: 1_900_000_000, refundKeys: [bettorCompressed] },
  });
}

async function main() {
  const wallet = new Wallet(MINT, { unit: "sat" });
  await wallet.loadMint();

  // 1) Mint a YES-locked token and redeem it with the YES attestation key.
  const yesProofs = await mintLocked(wallet, lockYes);
  await wallet.receive(yesProofs, { privkey: spendYes });
  console.log("✔ YES token redeemed");

  // 2) A NO-locked token signed with the YES key must be rejected by the mint.
  const noProofs = await mintLocked(wallet, lockNo);
  let rejected = false;
  try {
    await wallet.receive(noProofs, { privkey: spendYes });
  } catch {
    rejected = true;
  }
  if (!rejected) {
    console.error("✗ FAIL: NO token was redeemed with the YES key");
    process.exit(1);
  }
  console.log("✔ NO token rejected");
  console.log(`WALLET E2E OK against ${MINT}`);
}

main().catch((e) => {
  console.error("wallet e2e failed:", e);
  process.exit(1);
});
