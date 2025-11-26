/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  // For dynamic features - NOT static export
  // Keep server-side rendering for real-time features
  async rewrites() {
    return [
      {
        source: '/api/:path*',
        destination: '/api/:path*',
      },
    ]
  },
  // Enable image optimization
  images: {
    domains: ['localhost'],
    // Add your domain here
  },
  // Environment variables
  env: {
    CUSTOM_KEY: process.env.CUSTOM_KEY,
  },
}

module.exports = nextConfig
