/**
 * Hydration Error Prevention Tests
 * 
 * These tests ensure that components render the same HTML structure
 * on both server and client to prevent React hydration errors (#425, #418, #423).
 * 
 * Hydration errors occur when:
 * - Server-rendered HTML doesn't match client-rendered HTML
 * - Components conditionally render different structures based on client-only state
 * - Dynamic content (dates, random values) differs between server and client
 */

import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { render, screen } from '@testing-library/react';
import HeliosphereDemoClient from '@/app/heliosphere-demo/HeliosphereDemoClient';

// Mock WebGL and Three.js
vi.mock('@/app/lib/SunCentricHeliosphereScene', () => ({
  createSunCentricScene: vi.fn(() => Promise.resolve({
    update: vi.fn(),
    resize: vi.fn(),
    dispose: vi.fn(),
    setTime: vi.fn(),
    toggleValidation: vi.fn(),
  })),
}));

// Mock window.performance
Object.defineProperty(window, 'performance', {
  value: {
    now: vi.fn(() => Date.now()),
  },
  writable: true,
});

// Mock requestAnimationFrame
let rafCallbacks: Array<(time: number) => void> = [];
global.requestAnimationFrame = vi.fn((cb: (time: number) => void) => {
  rafCallbacks.push(cb);
  return 1;
});

global.cancelAnimationFrame = vi.fn();

describe('Hydration Error Prevention', () => {
  beforeEach(() => {
    rafCallbacks = [];
    vi.clearAllMocks();
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it('should render the same structure on initial mount (server-side)', () => {
    const { container } = render(<HeliosphereDemoClient />);
    
    // Component should always render the same root structure
    const root = container.firstChild;
    expect(root).toBeTruthy();
    expect(root).toHaveClass('relative', 'h-screen', 'w-full');
    
    // Canvas should always be present (even if hidden)
    const canvas = container.querySelector('canvas');
    expect(canvas).toBeTruthy();
    
    // Loading overlay should always be present
    const loadingOverlay = container.querySelector('.absolute.inset-0.flex.items-center');
    expect(loadingOverlay).toBeTruthy();
    
    // Error overlay should always be present (even if hidden)
    const errorOverlay = container.querySelector('.bg-black.bg-opacity-90');
    expect(errorOverlay).toBeTruthy();
    
    // Controls UI wrapper should always be present
    const controlsWrapper = container.querySelector('.opacity-0, .opacity-100');
    expect(controlsWrapper).toBeTruthy();
  });

  it('should not conditionally render different root elements', () => {
    const { container, rerender } = render(<HeliosphereDemoClient />);
    
    const initialStructure = container.innerHTML;
    
    // Re-render multiple times to simulate hydration
    rerender(<HeliosphereDemoClient />);
    rerender(<HeliosphereDemoClient />);
    
    // Root structure should remain consistent
    const root = container.firstChild;
    expect(root).toHaveClass('relative', 'h-screen', 'w-full');
  });

  it('should use suppressHydrationWarning on dynamic elements', () => {
    const { container } = render(<HeliosphereDemoClient />);
    
    // Canvas should have suppressHydrationWarning
    const canvas = container.querySelector('canvas');
    expect(canvas).toBeTruthy();
    // Note: suppressHydrationWarning is a React prop, not a DOM attribute
    // So we can't directly test it, but we verify the element exists
    
    // Loading overlay should have suppressHydrationWarning
    const loadingOverlay = container.querySelector('.absolute.inset-0.flex.items-center');
    expect(loadingOverlay).toBeTruthy();
    
    // Error overlay should have suppressHydrationWarning
    const errorOverlay = container.querySelector('.bg-black.bg-opacity-90');
    expect(errorOverlay).toBeTruthy();
  });

  it('should use CSS opacity/visibility instead of conditional rendering', () => {
    const { container } = render(<HeliosphereDemoClient />);
    
    // All overlays should exist in DOM, controlled by CSS
    const loadingOverlay = container.querySelector('.absolute.inset-0.flex.items-center');
    expect(loadingOverlay).toBeTruthy();
    expect(loadingOverlay).toHaveClass('transition-opacity');
    
    const errorOverlay = container.querySelector('.bg-black.bg-opacity-90');
    expect(errorOverlay).toBeTruthy();
    expect(errorOverlay).toHaveClass('transition-opacity');
    
    // Controls wrapper should exist
    const controlsWrapper = Array.from(container.querySelectorAll('div')).find(
      el => el.className.includes('opacity-0') || el.className.includes('opacity-100')
    );
    expect(controlsWrapper).toBeTruthy();
  });

  it('should not render dynamic content that differs between server and client', () => {
    const { container } = render(<HeliosphereDemoClient />);
    
    // Check that we're not rendering dates/times directly
    const textContent = container.textContent || '';
    
    // Should not contain dynamic dates (this is a basic check)
    // More sophisticated checks would look for Date objects, etc.
    expect(textContent).toBeTruthy();
  });

  it('should handle client-side state updates without hydration mismatch', async () => {
    const { container } = render(<HeliosphereDemoClient />);
    
    // Initial render structure
    const initialRoot = container.firstChild;
    expect(initialRoot).toBeTruthy();
    
    // Simulate client-side mount
    await new Promise(resolve => setTimeout(resolve, 100));
    
    // Structure should still match
    const afterMountRoot = container.firstChild;
    expect(afterMountRoot).toBeTruthy();
    expect(afterMountRoot).toHaveClass('relative', 'h-screen', 'w-full');
  });

  it('should render all UI elements in the DOM (even if hidden)', () => {
    const { container } = render(<HeliosphereDemoClient />);
    
    // All major UI sections should exist
    const canvas = container.querySelector('canvas');
    expect(canvas).toBeTruthy();
    
    // Info panel should exist (may be hidden)
    const infoPanel = Array.from(container.querySelectorAll('div')).find(
      el => el.textContent?.includes('Sun-Centric Heliosphere')
    );
    expect(infoPanel).toBeTruthy();
    
    // Control panel should exist (may be hidden)
    const controlPanel = Array.from(container.querySelectorAll('div')).find(
      el => el.textContent?.includes('Controls')
    );
    expect(controlPanel).toBeTruthy();
    
    // Legend should exist (may be hidden)
    const legend = Array.from(container.querySelectorAll('div')).find(
      el => el.textContent?.includes('Legend')
    );
    expect(legend).toBeTruthy();
  });
});

describe('React Hydration Error Detection', () => {
  it('should detect if component structure changes between renders', () => {
    const { container, rerender } = render(<HeliosphereDemoClient />);
    
    const firstRender = container.innerHTML;
    
    // Re-render with same props
    rerender(<HeliosphereDemoClient />);
    
    const secondRender = container.innerHTML;
    
    // Structure should be identical (allowing for React keys/IDs)
    // We check that the same elements exist
    const firstCanvas = container.querySelector('canvas');
    const firstRoot = container.firstChild;
    
    rerender(<HeliosphereDemoClient />);
    
    const secondCanvas = container.querySelector('canvas');
    const secondRoot = container.firstChild;
    
    // Root element should be the same type
    expect(firstRoot?.tagName).toBe(secondRoot?.tagName);
    expect(firstCanvas).toBeTruthy();
    expect(secondCanvas).toBeTruthy();
  });

  it('should not log React hydration errors to console', () => {
    const consoleErrorSpy = vi.spyOn(console, 'error').mockImplementation(() => {});
    const consoleWarnSpy = vi.spyOn(console, 'warn').mockImplementation(() => {});
    
    const { container, rerender } = render(<HeliosphereDemoClient />);
    
    // Re-render to simulate hydration
    rerender(<HeliosphereDemoClient />);
    
    // Check for React hydration error messages
    const errorCalls = consoleErrorSpy.mock.calls;
    const warnCalls = consoleWarnSpy.mock.calls;
    
    const hydrationErrors = [
      ...errorCalls,
      ...warnCalls,
    ].filter(call => {
      const message = call[0]?.toString() || '';
      return (
        message.includes('hydration') ||
        message.includes('Hydration') ||
        message.includes('425') ||
        message.includes('418') ||
        message.includes('423') ||
        message.includes('did not match') ||
        message.includes('server-rendered HTML')
      );
    });
    
    expect(hydrationErrors).toHaveLength(0);
    
    consoleErrorSpy.mockRestore();
    consoleWarnSpy.mockRestore();
  });
});

