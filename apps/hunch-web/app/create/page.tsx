"use client";

import { useEffect, useMemo, useState } from "react";
import Link from "next/link";
import { buildMarketTemplate } from "@/lib/build";
import { marketId } from "@/lib/hunch";
import { getPublicKey, setSignerMode, signerMode, signTemplate, usingLocalSigner } from "@/lib/sign";
import { setLocalSecret } from "@/lib/localSigner";
import { clearBunker, connectBunker, startNostrConnect } from "@/lib/nip46";
import { copyText } from "@/lib/clipboard";
import { publishAll } from "@/lib/publish";
import { relaysFromUrl } from "@/lib/relay";
import { Alert, Button, Card, Input, Select, Textarea } from "@/components/ui";

// The 21pay oracle + mint running on the Umbrel — proposed by default so creating a market needs
// only a question + a date. Advanced users can override with their own oracle/mint below.
const DEFAULT_ORACLE = "b32187c658b01420003049758660e62e4a7dd3daefac42076cd1664adce0e335";
const DEFAULT_MINT = "https://mint-signet.21pay.org";
// dlc_contract is required by the protocol but not used in the signet/test deployment yet.
const PLACEHOLDER_DLC = "0000000000000000000000000000000000000000000000000000000000000000:0";

/** Turns a question into a short URL-safe slug. */
function slugify(q: string): string {
  return q
    .toLowerCase()
    .normalize("NFD")
    .replace(/[̀-ͯ]/g, "")
    .replace(/[^a-z0-9]+/g, "-")
    .replace(/^-+|-+$/g, "")
    .slice(0, 48) || "market";
}

export default function CreateMarketPage() {
  const [question, setQuestion] = useState("");
  const [resolution, setResolution] = useState("");
  const [expiry, setExpiry] = useState("");
  const [advanced, setAdvanced] = useState(false);
  const [oracle, setOracle] = useState(DEFAULT_ORACLE);
  const [mint, setMint] = useState(DEFAULT_MINT);
  const [relays, setRelays] = useState(relaysFromUrl().join(", "));
  const [status, setStatus] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);

  // Auto-resolution "rubrique" — friendly presets wired to a data source, so a creator never writes
  // raw connector JSON. "manual" = the oracle decides by hand.
  const [autoKind, setAutoKind] = useState<"manual" | "price" | "onchain" | "weather" | "http" | "custom">("manual");
  const [op, setOp] = useState(">=");
  const [threshold, setThreshold] = useState("");
  const [asset, setAsset] = useState("BTC");
  const [quote, setQuote] = useState("USD");
  const [ocMetric, setOcMetric] = useState("block_height");
  const [wLat, setWLat] = useState("");
  const [wLon, setWLon] = useState("");
  const [wDate, setWDate] = useState("");
  const [wMetric, setWMetric] = useState("precipitation_sum");
  const [hUrl, setHUrl] = useState("");
  const [hPath, setHPath] = useState("");
  const [customSpec, setCustomSpec] = useState("");

  /** Assembles the chosen rubrique into a connector spec JSON string, or undefined for manual. */
  function buildResolutionSpec(): string | undefined {
    const th = Number(threshold);
    const needTh = () => {
      if (!threshold.trim() || !Number.isFinite(th)) throw new Error("Enter a numeric threshold for auto-resolution.");
    };
    switch (autoKind) {
      case "manual":
        return undefined;
      case "price":
        needTh();
        return JSON.stringify({ connector: "price", asset: asset.trim() || "BTC", quote: quote.trim() || "USD", op, threshold: th });
      case "onchain":
        needTh();
        return JSON.stringify({ connector: "onchain", metric: ocMetric, op, threshold: th });
      case "weather": {
        needTh();
        const lat = Number(wLat), lon = Number(wLon);
        if (!Number.isFinite(lat) || !Number.isFinite(lon)) throw new Error("Enter numeric lat/lon for the weather market.");
        if (!/^\d{4}-\d{2}-\d{2}$/.test(wDate.trim())) throw new Error("Enter the weather date as YYYY-MM-DD.");
        return JSON.stringify({ connector: "weather", lat, lon, date: wDate.trim(), metric: wMetric, op, threshold: th });
      }
      case "http": {
        needTh();
        if (!hUrl.trim()) throw new Error("Enter the JSON API URL.");
        const path = hPath.split(/[.,]/).map((s) => s.trim()).filter(Boolean);
        if (path.length === 0) throw new Error("Enter the JSON path to the number (e.g. data.0.score).");
        return JSON.stringify({ connector: "http", url: hUrl.trim(), path, op, threshold: th });
      }
      case "custom": {
        const t = customSpec.trim();
        if (!t) return undefined;
        JSON.parse(t); // throws if invalid
        return t;
      }
    }
  }

  const slug = useMemo(() => `${slugify(question)}-${Math.floor(Date.now() / 1000) % 100000}`, [question]);
  const usingDefaultOracle = oracle.trim() === DEFAULT_ORACLE;

  // Signing identity: extension (NIP-07) when present, else an in-browser key (works on mobile).
  const [signerPub, setSignerPub] = useState("");
  const [isLocal, setIsLocal] = useState(true);
  const [isRemote, setIsRemote] = useState(false);
  const [hasExtension, setHasExtension] = useState(false);
  const [importNsec, setImportNsec] = useState("");
  const [bunkerUri, setBunkerUri] = useState("");
  const [ncUri, setNcUri] = useState("");
  const refreshSigner = () => {
    setIsLocal(usingLocalSigner());
    setIsRemote(signerMode() === "nip46");
    setHasExtension(!!(globalThis as { nostr?: unknown }).nostr);
    getPublicKey().then(setSignerPub).catch(() => setSignerPub(""));
  };
  useEffect(() => {
    refreshSigner();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  async function submit() {
    setBusy(true);
    setStatus(null);
    try {
      if (!question.trim()) throw new Error("Enter a question.");
      const expiryTs = Math.floor(new Date(expiry).getTime() / 1000);
      if (!Number.isFinite(expiryTs)) throw new Error("Pick a close date.");
      if (!/^[0-9a-f]{64}$/i.test(oracle.trim())) throw new Error("Oracle must be a 64-char hex key.");

      const resolutionSpec = buildResolutionSpec();

      const template = buildMarketTemplate({
        slug,
        question: question.trim(),
        oracle: oracle.trim(),
        mint: mint.trim(),
        dlcContract: PLACEHOLDER_DLC,
        expiry: expiryTs,
        resolution: resolution.trim(),
        resolutionSpec,
      });
      const signed = await signTemplate(template);
      const id = marketId(signed.pubkey, slug);
      const relayList = relays.split(",").map((s) => s.trim()).filter(Boolean);
      const results = await publishAll(relayList, signed);
      const ok = results.filter((r) => r.accepted).length;
      if (ok === 0) throw new Error("No relay accepted the market. Check the relay URL.");
      setStatus(`✔ Published to ${ok}/${results.length} relay(s). Market id: ${id}`);
    } catch (e) {
      setStatus("Error: " + (e as Error).message);
    } finally {
      setBusy(false);
    }
  }

  return (
    <div className="flex flex-col gap-4 max-w-2xl">
      <Link href="/" className="text-sm">← markets</Link>
      <h1 className="font-bold">Create a market</h1>
      <p style={{ color: "var(--muted)" }} className="text-xs">
        Ask a yes/no question with a clear deadline. Outcomes are always YES / NO / INVALID.
      </p>

      {/* Signing identity — NIP-07 extension when present, else an in-browser key (works on phones). */}
      <Card className="flex flex-col gap-2 p-3">
        <div className="flex items-center gap-2 flex-wrap text-xs">
          <span style={{ color: "var(--muted)" }}>signing as</span>
          <code className="break-all" style={{ color: "var(--accent)" }}>{signerPub ? `${signerPub.slice(0, 16)}…` : "…"}</code>
          <span style={{ color: "var(--muted)" }}>· {isRemote ? "remote signer (Amber/NIP-46)" : isLocal ? "in-browser key" : "Nostr extension"}</span>
          {hasExtension && !isRemote && (
            <Button size="sm" onClick={() => { setSignerMode(isLocal ? "nip07" : "local"); refreshSigner(); }}>
              {isLocal ? "use extension" : "use in-browser key"}
            </Button>
          )}
          {isRemote && (
            <Button size="sm" onClick={() => { clearBunker(); setSignerMode("auto"); refreshSigner(); }}>
              disconnect signer
            </Button>
          )}
        </div>
        {!isRemote && (
          <div className="flex flex-col gap-1">
            <span className="text-xs" style={{ color: "var(--muted)" }}>
              {isLocal
                ? "No extension needed — key stored in this browser (back it up from Wallet). For stronger security, connect a remote signer:"
                : "Or connect a remote signer (key stays in the app):"}
            </span>
            <div className="flex flex-col gap-2">
              <Button
                onClick={() => {
                  const { uri, connected } = startNostrConnect();
                  setNcUri(uri);
                  setStatus("Open the link in Amber and approve the connection…");
                  connected
                    .then((pk) => {
                      setSignerMode("nip46");
                      setNcUri("");
                      refreshSigner();
                      setStatus(`✔ Remote signer connected: ${pk.slice(0, 16)}…`);
                    })
                    .catch((e) => setStatus("Error: " + (e as Error).message));
                }}
              >
                Connect with Amber
              </Button>
              {ncUri && (
                <div className="flex flex-col gap-2 rounded p-2" style={{ border: "1px solid var(--border)" }}>
                  <div className="flex items-center gap-2 flex-wrap">
                    <a href={ncUri} className="px-3 py-1 text-xs rounded font-bold" style={{ background: "var(--accent)", color: "#000" }}>
                      Open in Amber
                    </a>
                    <Button size="sm" onClick={() => copyText(ncUri).then(() => setStatus("✔ Connect link copied."))}>
                      Copy link
                    </Button>
                    <span className="text-xs" style={{ color: "var(--muted)" }}>waiting for approval…</span>
                  </div>
                  {/* eslint-disable-next-line @next/next/no-img-element */}
                  <img alt="nostrconnect QR" width={160} height={160} style={{ background: "#fff", padding: 8, borderRadius: 8, alignSelf: "flex-start" }} src={`https://api.qrserver.com/v1/create-qr-code/?size=160x160&data=${encodeURIComponent(ncUri)}`} />
                </div>
              )}
              <details>
                <summary className="text-xs cursor-pointer" style={{ color: "var(--muted)" }}>or paste a bunker:// URI</summary>
                <div className="flex gap-2 mt-2">
                  <Input className="flex-1" value={bunkerUri} onChange={(e) => setBunkerUri(e.target.value)} placeholder="bunker://…" />
                  <Button
                    onClick={async () => {
                      if (!bunkerUri.trim().startsWith("bunker://")) { setStatus("Error: paste a bunker:// URI."); return; }
                      setStatus("Connecting to the remote signer…");
                      try {
                        await connectBunker(bunkerUri.trim());
                        setSignerMode("nip46");
                        setBunkerUri("");
                        refreshSigner();
                        setStatus("✔ Remote signer connected (NIP-46).");
                      } catch (e) {
                        setStatus("Error: " + (e as Error).message);
                      }
                    }}
                  >
                    Connect
                  </Button>
                </div>
              </details>
            </div>
          </div>
        )}
      </Card>

      <label className="flex flex-col gap-1">
        <span className="text-sm font-bold">Question</span>
        <Textarea
          rows={2}
          placeholder="Will BTC close above $100k on 2026-12-31?"
          value={question}
          onChange={(e) => setQuestion(e.target.value)}
        />
      </label>

      <label className="flex flex-col gap-1">
        <span className="text-sm font-bold">Resolution criteria</span>
        <Textarea
          rows={2}
          placeholder="YES if BTC/USD ≥ 100000 at 23:59 UTC per Coinbase."
          value={resolution}
          onChange={(e) => setResolution(e.target.value)}
        />
        <span style={{ color: "var(--muted)" }} className="text-xs">
          Be precise — this is what the oracle uses to decide.
        </span>
      </label>

      <label className="flex flex-col gap-1">
        <span className="text-sm font-bold">Closes at</span>
        <Input type="datetime-local" value={expiry} onChange={(e) => setExpiry(e.target.value)} />
      </label>

      {/* Auto-resolution rubrique — pre-wired data sources so the oracle can settle automatically. */}
      <Card className="flex flex-col gap-2 p-3">
        <label className="flex flex-col gap-1">
          <span className="text-sm font-bold">Auto-resolution</span>
          <Select value={autoKind} onChange={(e) => setAutoKind(e.target.value as typeof autoKind)}>
            <option value="manual">Manual — the oracle decides by hand</option>
            <option value="price">Crypto price (Coinbase + Kraken)</option>
            <option value="onchain">Bitcoin on-chain (mempool.space)</option>
            <option value="weather">Weather (open-meteo)</option>
            <option value="http">Custom JSON API</option>
            <option value="custom">Raw spec (advanced)</option>
          </Select>
        </label>

        {autoKind === "price" && (
          <div className="flex gap-2 flex-wrap items-end text-sm">
            <label className="flex flex-col gap-1"><span className="text-xs">asset</span><Input className="w-24" value={asset} onChange={(e) => setAsset(e.target.value)} /></label>
            <label className="flex flex-col gap-1"><span className="text-xs">quote</span><Input className="w-24" value={quote} onChange={(e) => setQuote(e.target.value)} /></label>
            <label className="flex flex-col gap-1"><span className="text-xs">op</span><Select value={op} onChange={(e) => setOp(e.target.value)}><option>&gt;=</option><option>&gt;</option><option>&lt;=</option><option>&lt;</option></Select></label>
            <label className="flex flex-col gap-1"><span className="text-xs">threshold (USD)</span><Input className="w-32" inputMode="decimal" placeholder="100000" value={threshold} onChange={(e) => setThreshold(e.target.value)} /></label>
          </div>
        )}

        {autoKind === "onchain" && (
          <div className="flex gap-2 flex-wrap items-end text-sm">
            <label className="flex flex-col gap-1"><span className="text-xs">metric</span><Select value={ocMetric} onChange={(e) => setOcMetric(e.target.value)}><option value="block_height">block height</option><option value="mempool_count">mempool tx count</option><option value="fee_fastest">fastest fee (sat/vB)</option></Select></label>
            <label className="flex flex-col gap-1"><span className="text-xs">op</span><Select value={op} onChange={(e) => setOp(e.target.value)}><option>&gt;=</option><option>&gt;</option><option>&lt;=</option><option>&lt;</option></Select></label>
            <label className="flex flex-col gap-1"><span className="text-xs">threshold</span><Input className="w-32" inputMode="decimal" placeholder="900000" value={threshold} onChange={(e) => setThreshold(e.target.value)} /></label>
          </div>
        )}

        {autoKind === "weather" && (
          <div className="flex gap-2 flex-wrap items-end text-sm">
            <label className="flex flex-col gap-1"><span className="text-xs">lat</span><Input className="w-24" value={wLat} onChange={(e) => setWLat(e.target.value)} placeholder="48.85" /></label>
            <label className="flex flex-col gap-1"><span className="text-xs">lon</span><Input className="w-24" value={wLon} onChange={(e) => setWLon(e.target.value)} placeholder="2.35" /></label>
            <label className="flex flex-col gap-1"><span className="text-xs">date</span><Input className="w-36" value={wDate} onChange={(e) => setWDate(e.target.value)} placeholder="2026-06-10" /></label>
            <label className="flex flex-col gap-1"><span className="text-xs">metric</span><Select value={wMetric} onChange={(e) => setWMetric(e.target.value)}><option value="precipitation_sum">precipitation</option><option value="temperature_2m_max">max temp</option><option value="temperature_2m_min">min temp</option></Select></label>
            <label className="flex flex-col gap-1"><span className="text-xs">op</span><Select value={op} onChange={(e) => setOp(e.target.value)}><option>&gt;=</option><option>&gt;</option><option>&lt;=</option><option>&lt;</option></Select></label>
            <label className="flex flex-col gap-1"><span className="text-xs">threshold</span><Input className="w-24" inputMode="decimal" placeholder="0" value={threshold} onChange={(e) => setThreshold(e.target.value)} /></label>
          </div>
        )}

        {autoKind === "http" && (
          <div className="flex flex-col gap-2 text-sm">
            <label className="flex flex-col gap-1"><span className="text-xs">JSON API URL</span><Input value={hUrl} onChange={(e) => setHUrl(e.target.value)} placeholder="https://api.example.com/score" /></label>
            <div className="flex gap-2 flex-wrap items-end">
              <label className="flex flex-col gap-1"><span className="text-xs">path to number</span><Input className="w-40" value={hPath} onChange={(e) => setHPath(e.target.value)} placeholder="data.0.score" /></label>
              <label className="flex flex-col gap-1"><span className="text-xs">op</span><Select value={op} onChange={(e) => setOp(e.target.value)}><option>&gt;=</option><option>&gt;</option><option>&lt;=</option><option>&lt;</option></Select></label>
              <label className="flex flex-col gap-1"><span className="text-xs">threshold</span><Input className="w-28" inputMode="decimal" value={threshold} onChange={(e) => setThreshold(e.target.value)} /></label>
            </div>
          </div>
        )}

        {autoKind === "custom" && (
          <Textarea rows={2} placeholder={`{"connector":"price","asset":"BTC","op":">=","threshold":100000}`} value={customSpec} onChange={(e) => setCustomSpec(e.target.value)} />
        )}

        <span className="text-xs" style={{ color: "var(--muted)" }}>
          {autoKind === "manual"
            ? "The oracle resolves this market manually."
            : "The oracle resolves automatically from this source at the deadline. Shown to bettors on the market page."}
        </span>
      </Card>

      <Card className="p-3 flex flex-col gap-1">
        <div className="text-sm">
          Oracle:{" "}
          <span style={{ color: "var(--accent)" }}>
            {usingDefaultOracle ? "21pay oracle (default)" : `${oracle.slice(0, 12)}…`}
          </span>
        </div>
        <div style={{ color: "var(--muted)" }} className="text-xs">
          The oracle publishes the result after the deadline. The 21pay oracle is selected for you —
          you don&apos;t need your own. Verify any oracle&apos;s reputation on the market page before betting.
        </div>
      </Card>

      <button
        type="button"
        onClick={() => setAdvanced((v) => !v)}
        className="self-start text-xs"
        style={{ color: "var(--muted)" }}
      >
        {advanced ? "▾ hide advanced" : "▸ advanced (custom oracle / mint / relays)"}
      </button>

      {advanced && (
        <Card className="flex flex-col gap-2 p-3">
          <label className="flex flex-col gap-1">
            <span className="text-xs">Oracle pubkey (x-only hex, 64 chars)</span>
            <Input value={oracle} onChange={(e) => setOracle(e.target.value)} />
          </label>
          <label className="flex flex-col gap-1">
            <span className="text-xs">Mint URL</span>
            <Input value={mint} onChange={(e) => setMint(e.target.value)} />
          </label>
          <label className="flex flex-col gap-1">
            <span className="text-xs">Relays (comma-separated)</span>
            <Input value={relays} onChange={(e) => setRelays(e.target.value)} />
          </label>
          <label className="flex flex-col gap-1">
            <span className="text-xs">Import a Nostr key (64-char hex) to sign with your own identity</span>
            <div className="flex gap-2">
              <Input className="flex-1" value={importNsec} onChange={(e) => setImportNsec(e.target.value)} placeholder="your secret key…" />
              <Button
                onClick={() => {
                  const k = importNsec.trim().toLowerCase();
                  if (!/^[0-9a-f]{64}$/.test(k)) {
                    setStatus("Error: import a 64-char hex secret key.");
                    return;
                  }
                  setLocalSecret(k);
                  setSignerMode("local");
                  setImportNsec("");
                  refreshSigner();
                  setStatus("✔ Imported your Nostr key — signing with the in-browser identity.");
                }}
              >
                Import
              </Button>
            </div>
          </label>
        </Card>
      )}

      <Button variant="primary" className="self-start" onClick={submit} disabled={busy}>
        {busy ? "Signing…" : "Sign & publish"}
      </Button>

      {status && (
        <Alert kind={status.startsWith("Error") ? "error" : status.startsWith("✔") ? "ok" : "info"}>
          {status}
        </Alert>
      )}
    </div>
  );
}
