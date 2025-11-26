import { defineConfig } from 'vitest/config';
import { resolve } from 'node:path';

export default defineConfig({
  test: {
    environment: 'happy-dom',
    globals: true,
    setupFiles: ['./tests/setupTests.ts'],
    include: ['tests/**/*.test.ts', 'tests/**/*.test.tsx'],
    exclude: ['tests/visual/**'], // Playwright tests run separately
    coverage: {
      provider: 'v8',
      reporter: ['text', 'html', 'lcov'],
      reportsDirectory: resolve(__dirname, 'coverage'),
      include: ['app/lib/**', 'app/components/**'],
      exclude: [
        'app/legacy/**',
        '**/*.test.ts',
        '**/*.test.tsx',
        '**/types.ts'
      ],
      // Professional coverage thresholds
      lines: 80,
      functions: 80,
      branches: 75,
      statements: 80,
      // Critical paths require 90%+
      perFile: true
    }
  },
  resolve: {
    alias: {
      '@': resolve(__dirname, '.')
    }
  },
  esbuild: {
    jsx: 'automatic'
  }
});
