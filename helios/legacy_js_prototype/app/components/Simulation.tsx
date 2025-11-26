'use client';

import { useEffect, forwardRef } from 'react';
import Hero, { HeroRef } from './Hero';

const Simulation = forwardRef<HeroRef>((props, ref) => {
  useEffect(() => {
    if (typeof window === 'undefined') return;
    
    const updateViewportHeight = () => {
      const viewport = window.visualViewport;
      const height = viewport?.height ?? window.innerHeight;
      document.documentElement.style.setProperty('--viewport-height', `${height}px`);
    };
    
    updateViewportHeight();
    const handleOrientation = () => updateViewportHeight();
    
    window.addEventListener('resize', updateViewportHeight);
    window.addEventListener('orientationchange', handleOrientation);
    
    const vv = window.visualViewport;
    vv?.addEventListener('resize', updateViewportHeight);
    
    return () => {
      window.removeEventListener('resize', updateViewportHeight);
      window.removeEventListener('orientationchange', handleOrientation);
      vv?.removeEventListener('resize', updateViewportHeight);
    };
  }, []);

  return (
    <div
      className="fixed left-0 right-0 top-0 z-0 pointer-events-none"
      style={{ height: 'var(--viewport-height, 100vh)' }}
    >
      <Hero ref={ref} />
    </div>
  );
});

Simulation.displayName = 'Simulation';

export default Simulation;


