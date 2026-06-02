import type { Metadata } from "next";
import Link from "next/link";
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
          <Link href="/" style={{ color: "var(--accent)" }} className="text-xl font-bold">
            Hunch
          </Link>
          <span style={{ color: "var(--muted)" }} className="text-xs hidden sm:inline">
            prediction markets on Bitcoin · trust the math
          </span>
          <nav className="ml-auto flex items-center gap-4 text-xs">
            <Link href="/" style={{ color: "var(--fg)" }} className="hover:opacity-80">
              markets
            </Link>
            <Link href="/wallet/" style={{ color: "var(--fg)" }} className="hover:opacity-80">
              wallet
            </Link>
            <Link
              href="/create/"
              className="px-3 py-1.5 rounded font-bold"
              style={{ background: "var(--accent)", color: "#000" }}
            >
              + create
            </Link>
          </nav>
        </header>
        <main className="px-6 py-6 max-w-4xl mx-auto">{children}</main>
        <footer style={{ color: "var(--muted)" }} className="px-6 py-8 text-xs max-w-4xl mx-auto">
          No KYC. No tokens. No custody at settlement. Multi-relay, multi-mint, multi-oracle.
        </footer>
      </body>
    </html>
  );
}
