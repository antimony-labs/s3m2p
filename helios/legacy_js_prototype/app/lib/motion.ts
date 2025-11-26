export function getPrefersReducedMotion(): boolean {
  if (typeof window === 'undefined') return false;
  return window.matchMedia('(prefers-reduced-motion: reduce)').matches;
}

export function createMotionObserver(
  callback: (reduced: boolean) => void
): () => void {
  if (typeof window === 'undefined') return () => {};
  
  const mediaQuery = window.matchMedia('(prefers-reduced-motion: reduce)');
  const handler = (e: MediaQueryListEvent) => callback(e.matches);
  
  mediaQuery.addEventListener('change', handler);
  callback(mediaQuery.matches);
  
  return () => mediaQuery.removeEventListener('change', handler);
}

export function smoothSeek(curr: number, target: number, dt: number): number {
  const tau = 0.25; // seconds to converge ~63%
  const alpha = 1 - Math.exp(-dt / tau);
  return curr + (target - curr) * alpha;
}

