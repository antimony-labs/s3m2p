# Testing Guide

## Overview

This project uses a comprehensive testing strategy with 80%+ coverage targets:
- **Vitest** for unit and integration tests
- **Playwright** for visual regression tests (with WebGL capture)
- **React Testing Library** for component tests

## Running Tests

### All Tests
```bash
npm test
```

### Unit Tests (Fastest)
```bash
npm run test:unit
```

### Integration Tests
```bash
npm run test:integration
```

### Component Tests
```bash
npm run test:components
```

### Coverage Report
```bash
npm run test:coverage
```

Opens HTML report in `coverage/index.html` showing:
- Line, function, branch, statement coverage
- Uncovered code paths
- Per-file breakdown

### Visual Regression Tests
```bash
npm run test:visual
```

Runs Playwright tests that capture WebGL-rendered screenshots and compare against baselines.

### Update Visual Baselines
```bash
npm run test:visual:update
```

Rebuilds the site and updates screenshot baselines. **Use after intentional visual changes.**

### Watch Mode (Development)
```bash
npm run test:watch
```

Runs tests automatically on file changes.

## Test Structure

### Unit Tests (`tests/lib/`)
Test individual functions and calculations in isolation:
- `planetScaling.test.ts` - Scaling calculations (km → AU → scene units)
- `orbitalMechanics.test.ts` - Orbital distances, periods, angles
- `cameraSetup.test.ts` - Camera configuration and controls

### Integration Tests (`tests/integration/`)
Test how components work together:
- `heliosphereScene.test.ts` - Scene creation and initialization
- `hydration.test.tsx` - SSR/hydration correctness
- `researchHero.test.tsx` - Research page (legacy)

### Component Tests (`tests/components/`)
Test React components:
- `Controls.test.tsx` - Simulation controls
- (More to be added)

### Visual Regression Tests (`tests/visual/`)
Playwright tests that capture WebGL rendering:
- `heliosphere-main.spec.ts` - Main visualization
- `research-scene.spec.ts` - Research page (legacy)

## Coverage Thresholds

**Configured in `vitest.config.ts`:**

- ✅ Lines: 80%
- ✅ Functions: 80%
- ✅ Branches: 75%
- ✅ Statements: 80%

**Critical paths target: 90%+**
- Planet/sun scaling calculations
- Orbital mechanics
- Camera setup
- Scene initialization

## Writing Tests

### TDD Approach (Recommended)

1. **Write test first** defining correct behavior
2. **Run test** - it should fail
3. **Implement** the feature
4. **Run test** - it should pass
5. **Refactor** while keeping tests green

Example:
```typescript
// tests/lib/myFeature.test.ts
import { describe, expect, it } from 'vitest';
import { myFunction } from '@/app/lib/myFeature';

describe('myFunction', () => {
  it('calculates correct result', () => {
    const result = myFunction(10);
    expect(result).toBe(20);
  });
});
```

### Visual Regression Tests

When adding new visual tests:

1. **Write test** in `tests/visual/*.spec.ts`
2. **Run with update flag** to create baseline:
   ```bash
   npm run test:visual:update
   ```
3. **Commit baseline images** in `tests/visual/*.spec.ts-snapshots/`
4. **Future runs** will compare against baseline

### Best Practices

1. **Descriptive test names** - Test should read like documentation
2. **One assertion per test** (when possible) - Easier to debug
3. **Test edge cases** - Negative numbers, zero, infinity, null
4. **Mock external dependencies** - Keep tests fast and deterministic
5. **Use deterministic random** - Seed Math.random() for consistency

## Debugging Failed Tests

### Unit/Integration Tests
```bash
# Run specific test file
npm test tests/lib/planetScaling.test.ts

# Run specific test
npm test -- -t "converts km to AU correctly"

# Run with verbose output
npm test -- --reporter=verbose
```

### Visual Tests
```bash
# Show test UI
npx playwright test --ui

# Debug specific test
npx playwright test --debug tests/visual/heliosphere-main.spec.ts

# View trace
npx playwright show-trace trace.zip
```

## CI/CD Integration

Tests run automatically on push via `.git/hooks/pre-push`:
1. Unit + integration tests
2. Build verification
3. Visual tests (if enabled with `ENABLE_VISUAL_TESTS=1`)

**Manual run:**
```bash
./scripts/run-checks.sh
```

## Coverage Goals

### Current Status
- 🟢 Heliosphere model: High coverage
- 🟢 Scaling utilities: 100% (new)
- 🟡 Scene creation: Partial coverage
- 🟡 Components: Needs improvement
- 🔴 Legacy code: Excluded

### Adding Coverage

1. **Identify gaps:**
   ```bash
   npm run test:coverage
   open coverage/index.html
   ```

2. **Write tests** for red/yellow code paths

3. **Verify improvement:**
   ```bash
   npm run test:coverage
   ```

## Troubleshooting

### "WebGL not supported" in tests
- Tests run in happy-dom (no WebGL)
- Mock WebGL context for unit tests
- Use Playwright for actual WebGL testing

### Visual tests flaky
- Ensure `animations: 'disabled'`
- Wait for WebGL initialization
- Pause animations before screenshot
- Use deterministic random seed

### Coverage too low
- Check `coverage/index.html` for uncovered lines
- Add tests for branches and edge cases
- Test error handling paths

## Examples

### Testing a Calculation
```typescript
it('calculates planet size correctly', () => {
  const earthSize = planetVisibleSize(6371); // Earth radius in km
  expect(earthSize).toBeCloseTo(0.0255, 4); // Expected size in scene units
});
```

### Testing React Component
```typescript
it('renders control panel', () => {
  render(<Controls heroRef={mockRef} {...mockProps} />);
  expect(screen.getByLabelText(/Time/i)).toBeInTheDocument();
});
```

### Testing Visual Rendering
```typescript
test('renders heliosphere', async ({ page }) => {
  await page.goto('/');
  await page.waitForSelector('canvas');
  await expect(page).toHaveScreenshot('scene.png');
});
```

## Resources

- [Vitest Documentation](https://vitest.dev)
- [Playwright Documentation](https://playwright.dev)
- [React Testing Library](https://testing-library.com/react)
- [Coverage Reports](./coverage/index.html) (after running tests)
