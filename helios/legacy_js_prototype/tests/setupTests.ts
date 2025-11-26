import '@testing-library/jest-dom/vitest';
import { vi } from 'vitest';

if (!globalThis.requestAnimationFrame) {
  globalThis.requestAnimationFrame = (cb: FrameRequestCallback) =>
    setTimeout(() => cb(performance.now()), 16) as unknown as number;
}

if (!globalThis.cancelAnimationFrame) {
  globalThis.cancelAnimationFrame = (id: number) => clearTimeout(id);
}

if (!('matchMedia' in globalThis)) {
  globalThis.matchMedia = ((query: string): MediaQueryList => ({
    matches: false,
    media: query,
    onchange: null,
    addListener: () => undefined,
    removeListener: () => undefined,
    addEventListener: () => undefined,
    removeEventListener: () => undefined,
    dispatchEvent: () => true
  })) as typeof globalThis.matchMedia;
}

if (!globalThis.ResizeObserver) {
  globalThis.ResizeObserver = class {
    observe() {}
    unobserve() {}
    disconnect() {}
  } as typeof ResizeObserver;
}

Object.defineProperty(HTMLCanvasElement.prototype, 'getContext', {
  value: vi.fn(() => ({})),
  configurable: true
});

HTMLCanvasElement.prototype.getBoundingClientRect = function () {
  return {
    width: 1024,
    height: 768,
    top: 0,
    left: 0,
    bottom: 768,
    right: 1024,
    x: 0,
    y: 0,
    toJSON() {
      return this;
    }
  } as DOMRect;
};
