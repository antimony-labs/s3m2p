/**
 * ═══════════════════════════════════════════════════════════════════════════════
 * FILE: visual.spec.ts | TESTS/specs/visual.spec.ts
 * PURPOSE: Visual regression tests for landing page layout and canvas rendering
 * MODIFIED: 2025-11-27
 * ═══════════════════════════════════════════════════════════════════════════════
 */

import { test, expect } from '@playwright/test';

test.describe('Visual Regression Tests', () => {
  test('landing-layout', async ({ page }) => {
    // Static landing page (no WASM/canvas)
    await page.goto('/');

    // Wait for the header mark + footer dock
    await page.waitForSelector('header .bubble-mark', { state: 'visible' });
    await page.waitForSelector('footer .bubble-dock', { state: 'visible' });

    // Verify core copy + nav affordances exist
    await expect(page.getByRole('heading', { name: 'Antimony Labs' })).toBeVisible();
    await expect(page.locator('footer .bubble-dock').getByText('Tools')).toBeVisible();
    await expect(page.locator('footer .bubble-dock').getByText('Sims')).toBeVisible();
    await expect(page.locator('footer .bubble-dock').getByText('Learn')).toBeVisible();

    // Footer bubbles open panels (CSS :target)
    await page.click('footer .bubble-dock >> text=Tools');
    await expect(page.locator('#panel-tools')).toBeVisible();

    // Take a screenshot and compare with baseline
    await expect(page).toHaveScreenshot('landing-layout.png', {
      maxDiffPixels: 150,
    });
  });

  test('license-page', async ({ page }) => {
    await page.goto('/license.html');
    await expect(page.getByRole('heading', { name: 'License' })).toBeVisible();
    await expect(page.locator('pre.license-text')).toContainText('MIT License');
  });
});
