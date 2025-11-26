'use client';

import { useRef, useState, useCallback } from 'react';
import { HeroRef } from './Hero';
import Header from './Header';
import Footer from './Footer';
import Simulation from './Simulation';

/**
 * AppShell - Clean three-section architecture
 * 
 * This component orchestrates the three main sections of the application:
 * 1. Simulation (z-0) - WebGL canvas background
 * 2. Header (z-30) - All display data and controls
 * 3. Footer (z-20) - Tech stack and license info
 */
export default function ClientWrapper() {
  const heroRef = useRef<HeroRef>(null);
  const [currentYear, setCurrentYear] = useState(2024.0);

  const handleTimeChange = useCallback((time: number) => {
    setCurrentYear(time);
  }, []);

  const handleDirectionChange = (direction: 1 | -1) => {
    // Direction updates are handled internally by Controls
  };

  const handleMotionChange = (enabled: boolean) => {
    // Motion updates are handled internally by Controls
  };

  const handlePauseChange = (paused: boolean) => {
    // Pause updates are handled internally by Controls
  };

  return (
    <>
      {/* Section 1: Simulation - WebGL canvas background */}
      <Simulation ref={heroRef} />
      
      {/* Section 2: Header - All display data and controls */}
      <Header
        heroRef={heroRef}
        currentYear={currentYear}
        onTimeChange={handleTimeChange}
        onDirectionChange={handleDirectionChange}
        onMotionChange={handleMotionChange}
        onPauseChange={handlePauseChange}
      />
      
      {/* Section 3: Footer - Tech stack and license */}
      <Footer />
    </>
  );
}
