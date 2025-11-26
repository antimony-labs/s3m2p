'use client';

import { useEffect, useRef, useState, useImperativeHandle, forwardRef } from 'react';
import { createScene, SceneAPI, ComponentVisibility } from '../lib/heliosphereScene';

export type HeroRef = {
  updateScene: (year: number, direction: 1 | -1, motionEnabled: boolean) => void;
  toggleComponent: (component: keyof ComponentVisibility, visible: boolean) => void;
  getVisibility: () => ComponentVisibility;
};

type ViewportSnapshot = {
  width: number;
  height: number;
  devicePixelRatio: number;
};

const getViewportSnapshot = (): ViewportSnapshot => {
  if (typeof window === 'undefined') {
    return { width: 0, height: 0, devicePixelRatio: 1 };
  }
  const viewport = window.visualViewport;
  return {
    width: viewport?.width ?? window.innerWidth,
    height: viewport?.height ?? window.innerHeight,
    devicePixelRatio: window.devicePixelRatio ?? 1,
  };
};

const setViewportCssVar = (height: number) => {
  if (typeof document !== 'undefined') {
    document.documentElement.style.setProperty('--viewport-height', `${height}px`);
  }
};

const Hero = forwardRef<HeroRef>((props, ref) => {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const sceneRef = useRef<SceneAPI | null>(null);
  const [webglSupported, setWebglSupported] = useState(true);
  const [initFailed, setInitFailed] = useState(false);

  useImperativeHandle(ref, () => ({
    updateScene: (year: number, direction: 1 | -1, motionEnabled: boolean) => {
      if (sceneRef.current) {
        sceneRef.current.update(year, direction, motionEnabled);
      }
    },
    toggleComponent: (component: keyof ComponentVisibility, visible: boolean) => {
      if (sceneRef.current) {
        sceneRef.current.toggleComponent(component, visible);
      }
    },
    getVisibility: () => {
      if (sceneRef.current) {
        return sceneRef.current.getVisibility();
      }
      return {
        heliosphere: true,
        helioglow: false,
        terminationShock: true,
        bowShock: false,
        solarWind: true,
        interstellarWind: true,
        planets: true,
        orbits: true,
        moon: true,
        stars: true,
        famousStars: true,
        voyagers: true,
        distanceMarkers: false, // Hidden by default - removed as meaningless artifact
        solarApex: false, // Hidden by default - removed as meaningless artifact
        labels: true,
        interstellarObjects: false, // Hidden by default - removed as meaningless artifacts
        constellations: false,
      };
    },
  }));

  useEffect(() => {
    if (!canvasRef.current) return;

    // Check WebGL support
    const gl = canvasRef.current.getContext('webgl2') || canvasRef.current.getContext('webgl');
    if (!gl) {
      setWebglSupported(false);
      setInitFailed(true);
      return;
    }

    try {
      const initialViewport = getViewportSnapshot();
      const scene = createScene(canvasRef.current, { initialViewport });
      sceneRef.current = scene;

      const handleResize = () => {
        if (!canvasRef.current || !sceneRef.current) return;
        const { width, height } = getViewportSnapshot();
        setViewportCssVar(height);
        canvasRef.current.style.width = `${width}px`;
        canvasRef.current.style.height = `${height}px`;
        sceneRef.current.resize(width || initialViewport.width, height || initialViewport.height);
      };

      setViewportCssVar(initialViewport.height);
      handleResize();
      window.addEventListener('resize', handleResize);
      window.visualViewport?.addEventListener('resize', handleResize);
      
      // Initial render with current year (2024) and no drift
      scene.update(2024.0, 1, false);

      return () => {
        window.removeEventListener('resize', handleResize);
        window.visualViewport?.removeEventListener('resize', handleResize);
        scene.dispose();
        sceneRef.current = null;
      };
    } catch (error) {
      console.error('Failed to initialize WebGL scene:', error);
      setInitFailed(true);
    }
  }, []);

  if (initFailed || !webglSupported) {
    return (
      <div className="absolute inset-0 flex items-center justify-center" role="img" aria-label="Heliosphere visualization fallback">
        <img
          src="/img/heliosphere-still.png"
          alt="Stylized, scientifically-informed heliosphere; apex direction implied. WebGL is not supported or failed to initialize."
          className="w-full h-full object-cover opacity-50"
        />
        <div className="sr-only" role="alert">
          WebGL visualization is not available. Displaying static image fallback.
        </div>
      </div>
    );
  }

  return (
    <canvas
      ref={canvasRef}
      className="absolute inset-0 w-full h-full pointer-events-auto"
      style={{ pointerEvents: 'auto', touchAction: 'none' }}
      aria-hidden="true"
    />
  );
});

Hero.displayName = 'Hero';

export default Hero;
