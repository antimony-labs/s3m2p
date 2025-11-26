/**
 * Hydration Error Detection Test
 * 
 * This test actually runs the app in a browser and checks for React hydration errors
 * in the console. This catches issues that unit tests miss because they don't simulate
 * the actual Next.js SSR → hydration flow.
 * 
 * Enhanced to:
 * - Test all critical routes
 * - Capture ALL console errors (not just hydration-related)
 * - Generate detailed error reports
 * - Check for React Error Boundaries
 */

import { test, expect } from '@playwright/test';

// Hydration error patterns to detect
const HYDRATION_ERROR_PATTERNS = [
  /hydration/i,
  /Hydration/i,
  /425/,
  /418/,
  /423/,
  /did not match/i,
  /server-rendered HTML/i,
  /Minified React error/i,
];

// Routes to test
const ROUTES_TO_TEST = [
  { path: '/', name: 'Home' },
  { path: '/research', name: 'Research' },
  { path: '/heliosphere-demo', name: 'Heliosphere Demo' },
];

interface ConsoleMessage {
  type: string;
  text: string;
  url: string;
}

function isHydrationError(text: string): boolean {
  return HYDRATION_ERROR_PATTERNS.some(pattern => pattern.test(text));
}

test.describe('Hydration Error Detection', () => {
  for (const route of ROUTES_TO_TEST) {
    test(`should not have React hydration errors on ${route.name} page (${route.path})`, async ({ page }) => {
      const consoleMessages: ConsoleMessage[] = [];
      const hydrationErrors: ConsoleMessage[] = [];
      const allErrors: ConsoleMessage[] = [];

      // Capture ALL console messages
      page.on('console', (msg) => {
        const text = msg.text();
        const type = msg.type();
        const message: ConsoleMessage = {
          type,
          text,
          url: page.url(),
        };
        
        consoleMessages.push(message);
        
        if (type === 'error' || type === 'warning') {
          allErrors.push(message);
          
          if (isHydrationError(text)) {
            hydrationErrors.push(message);
          }
        }
      });

      // Capture page errors (unhandled exceptions)
      page.on('pageerror', (error) => {
        const message: ConsoleMessage = {
          type: 'pageerror',
          text: error.message,
          url: page.url(),
        };
        consoleMessages.push(message);
        allErrors.push(message);
        
        if (isHydrationError(error.message)) {
          hydrationErrors.push(message);
        }
      });

      // Navigate to the page
      await page.goto(route.path, { waitUntil: 'networkidle', timeout: 30000 });

      // Wait for React to hydrate (longer wait for complex pages)
      await page.waitForTimeout(3000);

      // Check for React Error Boundaries
      const errorBoundary = await page.evaluate(() => {
        // Check for React DevTools error overlay
        const errorOverlay = document.querySelector('[data-react-error-boundary]');
        if (errorOverlay) {
          return errorOverlay.textContent || 'React Error Boundary triggered';
        }
        return null;
      });

      if (errorBoundary) {
        hydrationErrors.push({
          type: 'error',
          text: `React Error Boundary: ${errorBoundary}`,
          url: page.url(),
        });
      }

      // Report results
      if (hydrationErrors.length > 0) {
        console.error(`\n❌ HYDRATION ERRORS DETECTED on ${route.name} (${route.path}):\n`);
        hydrationErrors.forEach((error, index) => {
          console.error(`${index + 1}. [${error.type.toUpperCase()}]`);
          console.error(`   ${error.text}\n`);
        });
        
        console.error(`📋 All console errors/warnings (${allErrors.length} total):\n`);
        allErrors.forEach((error, index) => {
          console.error(`${index + 1}. [${error.type.toUpperCase()}]`);
          console.error(`   ${error.text}\n`);
        });
      } else if (allErrors.length > 0) {
        console.log(`\nℹ️  ${route.name} page has ${allErrors.length} non-hydration console error(s)/warning(s) (not failing test)`);
      }

      // Assert no hydration errors
      expect(hydrationErrors).toHaveLength(0);
    });
  }

  test('should not have hydration errors across all routes', async ({ page }) => {
    const allHydrationErrors: ConsoleMessage[] = [];

    // Capture console messages
    page.on('console', (msg) => {
      const text = msg.text();
      if ((msg.type() === 'error' || msg.type() === 'warning') && isHydrationError(text)) {
        allHydrationErrors.push({
          type: msg.type(),
          text,
          url: page.url(),
        });
      }
    });

    // Visit all routes sequentially
    for (const route of ROUTES_TO_TEST) {
      await page.goto(route.path, { waitUntil: 'networkidle', timeout: 30000 });
      await page.waitForTimeout(2000);
    }

    if (allHydrationErrors.length > 0) {
      console.error('\n❌ HYDRATION ERRORS DETECTED ACROSS ALL ROUTES:\n');
      allHydrationErrors.forEach((error, index) => {
        console.error(`${index + 1}. [${error.type.toUpperCase()}] ${error.url}`);
        console.error(`   ${error.text}\n`);
      });
    }

    expect(allHydrationErrors).toHaveLength(0);
  });
});

