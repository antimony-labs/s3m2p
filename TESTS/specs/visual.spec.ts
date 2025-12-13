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
    // Navigate to the paused simulation for stable screenshots
    // intro=false hides the mission overlay for consistent screenshots
    await page.goto('/?paused=true&intro=false');

    // Wait for the canvas to be attached
    await page.waitForSelector('#simulation', { state: 'attached' });

    // Wait for critical UI elements to load (fonts, etc)
    await page.waitForSelector('.monolith');

    // Check for home page bubble labels
    await expect(page.locator('#constellation').getByText('Helios')).toBeVisible();
    await expect(page.locator('#constellation').getByText('Learn')).toBeVisible();
    await expect(page.locator('#constellation').getByText('Tools')).toBeVisible();
    
    // Give the canvas a moment to render
    await page.waitForTimeout(1000);
    
    // Take a screenshot and compare with baseline
    await expect(page).toHaveScreenshot('landing-layout.png', {
      maxDiffPixels: 500, // Increased tolerance for particle variations
    });
  });

  test('canvas-exists', async ({ page }) => {
    // Navigate to the paused simulation with intro hidden
    await page.goto('/?paused=true&intro=false');
    
    // Wait for the canvas to be attached
    const canvas = page.locator('#simulation');
    await canvas.waitFor({ state: 'attached' });
    
    // Check that the canvas has a width greater than 0
    const boundingBox = await canvas.boundingBox();
    expect(boundingBox).not.toBeNull();
    expect(boundingBox!.width).toBeGreaterThan(0);
    expect(boundingBox!.height).toBeGreaterThan(0);
  });
});
