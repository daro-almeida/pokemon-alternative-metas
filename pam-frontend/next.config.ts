import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  /* config options here */
  reactStrictMode: false,
  images: {
    remotePatterns: [new URL("https://play.pokemonshowdown.com/sprites/**")],
  },
};

export default nextConfig;
