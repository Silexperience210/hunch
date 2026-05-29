/** @type {import('next').NextConfig} */
const nextConfig = {
  // Fully static export — deployable to Cloudflare Pages, IPFS, and a Tor hidden service
  // with no server (cypherpunk: no backend, forkable, host-anywhere). CLAUDE.md §Distribution.
  output: "export",
  images: { unoptimized: true },
  trailingSlash: true,
  // For GitHub Pages project subpath (e.g. /hunch). Empty for root hosts (Cloudflare/IPFS/Tor).
  basePath: process.env.NEXT_PUBLIC_BASE_PATH || undefined,
  assetPrefix: process.env.NEXT_PUBLIC_BASE_PATH || undefined,
  // Keep type-checking (real safety); skip lint to avoid an eslint-config setup in CI.
  eslint: { ignoreDuringBuilds: true },
};

export default nextConfig;
