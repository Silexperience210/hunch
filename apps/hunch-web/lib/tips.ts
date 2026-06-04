// Opt-in tip to the operator on withdraw — the honest, non-coercive way Hunch's infra (mint
// liquidity, oracle LLM credits, hosting) gets supported. A tip is a Lightning payment to the
// operator's Lightning Address (LNURL-pay), melted from leftover balance *after* the user's own
// withdrawal — never at its expense, and gracefully skipped if anything fails.
//
// lnAddressToLnurlp + presetTip are pure + tested; resolveLnAddressInvoice does the LNURL-pay fetch.

/** Operator Lightning Address tips go to. Override in the browser via localStorage `hunch:tip-address`. */
export const DEFAULT_TIP_ADDRESS = "tips@21pay.org";

export function tipAddress(): string {
  if (typeof window !== "undefined") {
    return (localStorage.getItem("hunch:tip-address") || DEFAULT_TIP_ADDRESS).trim();
  }
  return DEFAULT_TIP_ADDRESS;
}

/** Tip presets shown on the withdraw form (sats). Above the operator LNURL minSendable (210 sat);
 * 2100 = the 21M nod. The LNURL-pay flow enforces the live min/max and skips gracefully otherwise. */
export const TIP_PRESETS = [0, 210, 500, 2100] as const;

/** Clamp a chosen tip to what the leftover balance can actually cover (keeps a small fee buffer). */
export function presetTip(chosen: number, available: number): number {
  if (chosen <= 0 || available <= 0) return 0;
  const buffer = Math.max(2, Math.ceil(available * 0.01)); // leave room for the melt fee reserve
  return Math.max(0, Math.min(chosen, available - buffer));
}

/** Resolve a `name@domain` Lightning Address to its LNURL-pay endpoint URL. Pure. */
export function lnAddressToLnurlp(address: string): string {
  const [name, domain] = address.trim().split("@");
  if (!name || !domain) throw new Error("invalid Lightning address (expected name@domain)");
  return `https://${domain}/.well-known/lnurlp/${name}`;
}

/** Resolve a Lightning Address + amount into a bolt11 invoice via the LNURL-pay flow. */
export async function resolveLnAddressInvoice(address: string, amountSat: number): Promise<string> {
  const meta: any = await (await fetch(lnAddressToLnurlp(address))).json();
  if (meta.status === "ERROR") throw new Error(meta.reason || "LNURL error");
  if (!meta.callback) throw new Error("LNURL-pay endpoint has no callback");
  const amountMsat = amountSat * 1000;
  if (meta.minSendable && amountMsat < meta.minSendable) throw new Error("tip below the operator's minimum");
  if (meta.maxSendable && amountMsat > meta.maxSendable) throw new Error("tip above the operator's maximum");
  const cb = new URL(meta.callback);
  cb.searchParams.set("amount", String(amountMsat));
  const res: any = await (await fetch(cb.toString())).json();
  const invoice = res.pr ?? res.invoice;
  if (!invoice) throw new Error(res.reason || "LNURL-pay returned no invoice");
  return invoice;
}
