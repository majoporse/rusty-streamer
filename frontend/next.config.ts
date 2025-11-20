import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  /* config options here */
  images: {
    remotePatterns: [
      {
        protocol: "https",
        hostname: "encrypted-tbn0.gstatic.com",
        port: "80",
        pathname: "/**",
      },
      {
        protocol: "http",
        hostname: "localhost",
        port: "10000",
        pathname: "/**",
      },
    ],
    dangerouslyAllowLocalIP: true,
  },
};


export default nextConfig;
