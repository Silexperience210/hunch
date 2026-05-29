import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  title: "Hunch — prediction markets on Bitcoin",
  description: "Permissionless, no-KYC prediction markets settled by Bitcoin DLCs. Trust the math.",
};

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <body>
        <header
          style={{ borderBottom: "1px solid var(--border)" }}
          className="px-6 py-4 flex items-baseline gap-3"
        >
          <span style={{ color: "var(--accent)" }} className="text-xl font-bold">
            Hunch
          </span>
          <span style={{ color: "var(--muted)" }} className="text-xs">
            prediction markets on Bitcoin · trust the math
          </span>
        </header>
        <main className="px-6 py-6 max-w-4xl mx-auto">{children}</main>
        <footer style={{ color: "var(--muted)" }} className="px-6 py-8 text-xs max-w-4xl mx-auto">
          No KYC. No tokens. No custody at settlement. Multi-relay, multi-mint, multi-oracle.
        </footer>
      </body>
    </html>
  );
}
