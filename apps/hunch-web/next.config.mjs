/** @type {import('next').NextConfig} */
const nextConfig = {
  // Fully static export — deployable to Cloudflare Pages, IPFS, and a Tor hidden service
  // with no server (cypherpunk: no backend, forkable, host-anywhere). CLAUDE.md §Distribution.
  output: "export",
  images: { unoptimized: true },
  trailingSlash: true,
};

export default nextConfig;
