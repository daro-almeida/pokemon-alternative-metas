import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  /* config options here */
  images: {
    remotePatterns: [new URL('https://play.pokemonshowdown.com/sprites/**')], 
  },
};

export default nextConfig;
