const { execSync } = require('child_process');

// Capture git info at build time
function getGitInfo() {
  try {
    const commit = execSync('git rev-parse HEAD').toString().trim();
    const branch = execSync('git rev-parse --abbrev-ref HEAD').toString().trim();
    const timestamp = new Date().toISOString();
    return { commit, branch, timestamp };
  } catch (error) {
    console.warn('Could not retrieve git info:', error.message);
    return {
      commit: 'unknown',
      branch: 'unknown',
      timestamp: new Date().toISOString(),
    };
  }
}

const gitInfo = getGitInfo();

/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  output: 'export',
  images: {
    unoptimized: true,
  },
  trailingSlash: false,
  // Exclude portfolio directory from build
  pageExtensions: ['ts', 'tsx', 'js', 'jsx'],
  // Only build files in app directory
  distDir: 'out',
  // Inject git info as public env vars
  env: {
    NEXT_PUBLIC_GIT_COMMIT: gitInfo.commit,
    NEXT_PUBLIC_GIT_BRANCH: gitInfo.branch,
    NEXT_PUBLIC_BUILD_TIME: gitInfo.timestamp,
  },
}

module.exports = nextConfig

