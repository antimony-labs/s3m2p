import { test, expect } from '@playwright/test';

test.describe('Research scene visual regression', () => {
  test('renders a stable heliosphere frame', async ({ page }) => {
    await page.addInitScript((seed: number) => {
      function mulberry32(a: number) {
        return () => {
          let t = (a += 0x6d2b79f5);
          t = Math.imul(t ^ (t >>> 15), t | 1);
          t ^= t + Math.imul(t ^ (t >>> 7), t | 61);
          return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
        };
      }

      const seededRandom = mulberry32(seed);
      Math.random = seededRandom;
    }, 42);

    await page.goto('/research');

    const canvas = page.locator('[data-testid="research-scene-canvas"]');
    await expect(canvas).toBeVisible();
    await expect(canvas).toHaveAttribute('data-scene-ready', 'true', { timeout: 120000 });
    await expect(canvas).toHaveScreenshot('research-canvas.png', {
      animations: 'disabled',
      timeout: 120000
    });
  });
});
