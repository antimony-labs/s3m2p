import { defineConfig, devices } from '@playwright/test';

const PORT = 4173;

export default defineConfig({
  testDir: './tests/visual',
  timeout: 120 * 1000,
  expect: {
    toHaveScreenshot: {
      maxDiffPixelRatio: 0.01
    }
  },
  fullyParallel: false,
  use: {
    baseURL: `http://127.0.0.1:${PORT}`,
    viewport: { width: 1280, height: 720 },
    deviceScaleFactor: 1,
    trace: 'on-first-retry',
    ...devices['Desktop Chrome']
  },
  webServer: {
    command: `npm run start:visual`,
    url: `http://127.0.0.1:${PORT}`,
    reuseExistingServer: !process.env.CI,
    timeout: 120 * 1000
  }
});
