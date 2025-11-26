#!/usr/bin/env node

/**
 * Hydration Error Detection Script
 * 
 * This script builds the app, starts a local server, and uses Playwright
 * to check for React hydration errors in the browser console.
 * 
 * Usage:
 *   node scripts/check-hydration.js
 * 
 * Exit codes:
 *   0 - No hydration errors detected
 *   1 - Hydration errors detected or script error
 */

const { execSync, spawn } = require('child_process');
const { promisify } = require('util');
const fs = require('fs');
const path = require('path');

const exec = promisify(require('child_process').exec);

// Routes to check for hydration errors
const ROUTES_TO_CHECK = ['/', '/research', '/heliosphere-demo'];

// Hydration error patterns
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

async function buildApp() {
  console.log('🔨 Building application...');
  try {
    execSync('npm run build', { stdio: 'inherit', cwd: process.cwd() });
    console.log('✅ Build completed successfully\n');
    return true;
  } catch (error) {
    console.error('❌ Build failed:', error.message);
    return false;
  }
}

async function startServer() {
  console.log('🚀 Starting local server...');
  
  // Check if server is already running
  const http = require('http');
  const checkExisting = await new Promise((resolve) => {
    const req = http.get('http://127.0.0.1:4173', (res) => {
      resolve(true); // Server already running
    });
    req.on('error', () => {
      resolve(false); // Server not running
    });
    req.setTimeout(2000, () => {
      req.destroy();
      resolve(false);
    });
    req.end();
  });

  if (checkExisting) {
    console.log('✅ Server already running\n');
    return null; // No process to kill
  }

  const serverProcess = spawn('npm', ['run', 'start:visual'], {
    stdio: 'pipe',
    cwd: process.cwd(),
    shell: true,
  });

  // Wait for server to be ready
  await new Promise((resolve, reject) => {
    let attempts = 0;
    const maxAttempts = 60; // 30 seconds total (500ms * 60)
    const checkReady = setInterval(() => {
      attempts++;
      const req = http.get('http://127.0.0.1:4173', (res) => {
        clearInterval(checkReady);
        console.log('✅ Server is ready\n');
        resolve();
      });
      req.on('error', () => {
        if (attempts >= maxAttempts) {
          clearInterval(checkReady);
          reject(new Error('Server failed to start within 30 seconds'));
        }
      });
      req.setTimeout(1000, () => {
        req.destroy();
        if (attempts >= maxAttempts) {
          clearInterval(checkReady);
          reject(new Error('Server failed to start within 30 seconds'));
        }
      });
      req.end();
    }, 500);
  });

  return serverProcess;
}

async function checkHydrationErrors() {
  console.log('🔍 Checking for hydration errors...\n');
  
  // Import Playwright dynamically (only if available)
  let playwright;
  try {
    playwright = require('@playwright/test');
  } catch (error) {
    console.error('❌ Playwright not found. Install it with: npm install --save-dev @playwright/test');
    console.error('   Then run: npx playwright install chromium');
    console.error('   Or skip hydration check: SKIP_HYDRATION_CHECK=1');
    process.exit(1);
  }

  const { chromium } = playwright;
  
  // Check if browser is installed
  try {
    const browser = await chromium.launch({ headless: true });
  const context = await browser.newContext();
  const page = await context.newPage();

  const hydrationErrors = [];
  const allErrors = [];

  // Capture console messages
  page.on('console', (msg) => {
    const text = msg.text();
    const type = msg.type();
    
    if (type === 'error' || type === 'warning') {
      allErrors.push({ type, text, url: page.url() });
      
      // Check if it's a hydration error
      const isHydrationError = HYDRATION_ERROR_PATTERNS.some(pattern => 
        pattern.test(text)
      );
      
      if (isHydrationError) {
        hydrationErrors.push({ type, text, url: page.url() });
      }
    }
  });

  // Check each route
  for (const route of ROUTES_TO_CHECK) {
    const url = `http://127.0.0.1:4173${route}`;
    console.log(`  Checking ${route}...`);
    
    try {
      await page.goto(url, { waitUntil: 'networkidle', timeout: 30000 });
      
      // Wait a bit for React to hydrate
      await page.waitForTimeout(2000);
      
      // Check for React error boundaries
      const errorBoundary = await page.evaluate(() => {
        return window.__REACT_ERROR_BOUNDARY__ || null;
      });
      
      if (errorBoundary) {
        hydrationErrors.push({
          type: 'error',
          text: 'React Error Boundary triggered',
          url,
        });
      }
    } catch (error) {
      console.error(`  ⚠️  Error loading ${route}:`, error.message);
      allErrors.push({
        type: 'error',
        text: `Failed to load page: ${error.message}`,
        url,
      });
    }
  }

    await browser.close();
    return { hydrationErrors, allErrors };
  } catch (error) {
    if (error.message.includes('Executable doesn\'t exist') || error.message.includes('browserType.launch')) {
      console.error('❌ Playwright browsers not installed.');
      console.error('   Run: npx playwright install chromium');
      console.error('   Or skip hydration check: SKIP_HYDRATION_CHECK=1');
      throw error;
    }
    throw error;
  }
}

async function main() {
  console.log('🧪 Hydration Error Detection Script\n');
  console.log('=' .repeat(50) + '\n');

  // Step 1: Build the app
  const buildSuccess = await buildApp();
  if (!buildSuccess) {
    process.exit(1);
  }

  // Step 2: Start server
  let serverProcess;
  try {
    serverProcess = await startServer();
  } catch (error) {
    console.error('❌ Failed to start server:', error.message);
    process.exit(1);
  }

  // Step 3: Check for hydration errors
  let hydrationErrors = [];
  let allErrors = [];
  
  try {
    const result = await checkHydrationErrors();
    hydrationErrors = result.hydrationErrors;
    allErrors = result.allErrors;
  } catch (error) {
    console.error('❌ Error during hydration check:', error.message);
    serverProcess.kill();
    process.exit(1);
  } finally {
    // Clean up server (only if we started it)
    if (serverProcess) {
      serverProcess.kill();
      console.log('\n🛑 Server stopped');
    }
  }

  // Step 4: Report results
  console.log('\n' + '='.repeat(50));
  console.log('📊 Results\n');

  if (hydrationErrors.length > 0) {
    console.log('❌ HYDRATION ERRORS DETECTED:\n');
    hydrationErrors.forEach((error, index) => {
      console.log(`${index + 1}. [${error.type.toUpperCase()}] ${error.url}`);
      console.log(`   ${error.text}\n`);
    });
    
    console.log(`\n⚠️  Found ${hydrationErrors.length} hydration error(s)`);
    console.log('   Fix these errors before deploying!\n');
    
    // Also show all errors for debugging
    if (allErrors.length > hydrationErrors.length) {
      console.log('📋 All console errors/warnings:\n');
      allErrors.forEach((error, index) => {
        console.log(`${index + 1}. [${error.type.toUpperCase()}] ${error.url}`);
        console.log(`   ${error.text}\n`);
      });
    }
    
    process.exit(1);
  } else {
    console.log('✅ No hydration errors detected!\n');
    
    if (allErrors.length > 0) {
      console.log(`ℹ️  Found ${allErrors.length} non-hydration console error(s)/warning(s):\n`);
      allErrors.forEach((error, index) => {
        console.log(`${index + 1}. [${error.type.toUpperCase()}] ${error.url}`);
        console.log(`   ${error.text}\n`);
      });
    }
    
    console.log('✅ All checks passed!\n');
    process.exit(0);
  }
}

// Handle unhandled errors
process.on('unhandledRejection', (error) => {
  console.error('❌ Unhandled error:', error);
  process.exit(1);
});

// Run the script
main().catch((error) => {
  console.error('❌ Script failed:', error);
  process.exit(1);
});

