/**
 * ═══════════════════════════════════════════════════════════════════════════════
 * FILE: playwright.config.ts | ./playwright.config.ts
 * PURPOSE: Playwright configuration for end-to-end testing and visual regression tests
 * MODIFIED: 2025-12-09
 * ═══════════════════════════════════════════════════════════════════════════════
 */

import { defineConfig, devices } from '@playwright/test';

export default defineConfig({
  testDir: './TESTS/specs',
  outputDir: './TESTS/results',
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  workers: process.env.CI ? 1 : undefined,
  reporter: [['html', { outputFolder: 'TESTS/report' }]],
  use: {
    baseURL: 'http://localhost:8080',
    trace: 'on-first-retry',
  },
  projects: [
    {
      name: 'chromium',
      use: { ...devices['Desktop Chrome'] },
    },
  ],
  webServer: {
    command: 'cd WELCOME && trunk serve',
    url: 'http://localhost:8080',
    reuseExistingServer: !process.env.CI,
    timeout: 120 * 1000,
  },
});

