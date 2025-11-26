'use client';

import { useRef, useState, useEffect, useCallback } from 'react';
import ResearchGradeHero, { ResearchHeroRef } from './ResearchGradeHero';
import LayerControl from './LayerControl';
import { ComponentVisibility } from '../lib/ResearchGradeHeliosphereScene';

// Extended layer control for research-grade visualization
interface ResearchLayerControlProps {
  heroRef: React.RefObject<ResearchHeroRef>;
}

function ResearchLayerControl({ heroRef }: ResearchLayerControlProps) {
  const [visibility, setVisibility] = useState<ComponentVisibility>({
    heliosphere: true,
    terminationShock: true,
    heliopause: true,
    bowShock: false,
    solarWind: true,
    interstellarWind: true,
    planets: true,
    orbits: true,
    spacecraft: true,
    trajectories: true,
    stars: true,
    coordinateGrid: false,
    distanceMarkers: true,
    dataOverlay: true
  });

  const handleToggle = (layer: keyof ComponentVisibility) => {
    if (heroRef.current) {
      const newVisibility = !visibility[layer];
      heroRef.current.toggleComponent(layer, newVisibility);
      setVisibility(prev => ({ ...prev, [layer]: newVisibility }));
    }
  };

  return (
    <div className="fixed top-20 left-4 z-30 bg-gray-900/90 backdrop-blur-sm rounded-lg p-4 text-white">
      <h3 className="text-sm font-semibold mb-3 text-gray-300">Layers</h3>
      <div className="space-y-2">
        <div className="text-xs font-semibold text-gray-400 mt-3">Heliosphere</div>
        <label className="flex items-center gap-2 text-sm">
          <input
            type="checkbox"
            checked={visibility.heliopause}
            onChange={() => handleToggle('heliopause')}
            className="rounded"
          />
          <span>Heliopause</span>
        </label>
        <label className="flex items-center gap-2 text-sm">
          <input
            type="checkbox"
            checked={visibility.terminationShock}
            onChange={() => handleToggle('terminationShock')}
            className="rounded"
          />
          <span>Termination Shock</span>
        </label>
        <label className="flex items-center gap-2 text-sm">
          <input
            type="checkbox"
            checked={visibility.bowShock}
            onChange={() => handleToggle('bowShock')}
            className="rounded"
          />
          <span>Bow Shock (Theory)</span>
        </label>
        
        <div className="text-xs font-semibold text-gray-400 mt-3">Plasma Flows</div>
        <label className="flex items-center gap-2 text-sm">
          <input
            type="checkbox"
            checked={visibility.solarWind}
            onChange={() => handleToggle('solarWind')}
            className="rounded"
          />
          <span>Solar Wind</span>
        </label>
        <label className="flex items-center gap-2 text-sm">
          <input
            type="checkbox"
            checked={visibility.interstellarWind}
            onChange={() => handleToggle('interstellarWind')}
            className="rounded"
          />
          <span>Interstellar Medium</span>
        </label>
        
        <div className="text-xs font-semibold text-gray-400 mt-3">Solar System</div>
        <label className="flex items-center gap-2 text-sm">
          <input
            type="checkbox"
            checked={visibility.planets}
            onChange={() => handleToggle('planets')}
            className="rounded"
          />
          <span>Planets</span>
        </label>
        <label className="flex items-center gap-2 text-sm">
          <input
            type="checkbox"
            checked={visibility.orbits}
            onChange={() => handleToggle('orbits')}
            className="rounded"
          />
          <span>Orbits</span>
        </label>
        
        <div className="text-xs font-semibold text-gray-400 mt-3">Spacecraft</div>
        <label className="flex items-center gap-2 text-sm">
          <input
            type="checkbox"
            checked={visibility.spacecraft}
            onChange={() => handleToggle('spacecraft')}
            className="rounded"
          />
          <span>Voyager 1 & 2</span>
        </label>
        <label className="flex items-center gap-2 text-sm">
          <input
            type="checkbox"
            checked={visibility.trajectories}
            onChange={() => handleToggle('trajectories')}
            className="rounded"
          />
          <span>Trajectories</span>
        </label>
        
        <div className="text-xs font-semibold text-gray-400 mt-3">Reference</div>
        <label className="flex items-center gap-2 text-sm">
          <input
            type="checkbox"
            checked={visibility.stars}
            onChange={() => handleToggle('stars')}
            className="rounded"
          />
          <span>Stars</span>
        </label>
        <label className="flex items-center gap-2 text-sm">
          <input
            type="checkbox"
            checked={visibility.coordinateGrid}
            onChange={() => handleToggle('coordinateGrid')}
            className="rounded"
          />
          <span>Coordinate Grid</span>
        </label>
        <label className="flex items-center gap-2 text-sm">
          <input
            type="checkbox"
            checked={visibility.distanceMarkers}
            onChange={() => handleToggle('distanceMarkers')}
            className="rounded"
          />
          <span>Distance Rings</span>
        </label>
      </div>
    </div>
  );
}

export default function ResearchGradeClientWrapper() {
  const heroRef = useRef<ResearchHeroRef>(null);
  const [showInfo, setShowInfo] = useState(true);
  const [currentDate, setCurrentDate] = useState<Date>(new Date());

  // Callback to receive date updates from ResearchGradeHero
  const handleDateChange = useCallback((date: Date) => {
    setCurrentDate(date);
    // Update the date display component via custom event
    window.dispatchEvent(new CustomEvent('research-date-update', { detail: date }));
  }, []);

  // Fire initial date event when component mounts
  useEffect(() => {
    window.dispatchEvent(new CustomEvent('research-date-update', { detail: currentDate }));
  }, []);

  return (
    <>
      {/* Simulation canvas */}
      <div className="absolute inset-0 z-0">
        <ResearchGradeHero ref={heroRef} onDateChange={handleDateChange} />
      </div>
      
      <ResearchLayerControl heroRef={heroRef} />
      
      {/* Information panel */}
      {showInfo && (
        <div className="fixed bottom-28 left-1/2 transform -translate-x-1/2 bg-gray-900/90 backdrop-blur-sm rounded-lg p-6 text-white max-w-2xl z-20">
          <button
            className="absolute top-2 right-2 text-gray-400 hover:text-white"
            onClick={() => setShowInfo(false)}
          >
            ✕
          </button>
          <h2 className="text-xl font-bold mb-3">Research-Grade Heliosphere Simulation</h2>
          <p className="text-sm text-gray-300 mb-3">
            This scientifically accurate visualization shows our Solar System's heliosphere - 
            the vast bubble of solar wind that protects us from interstellar radiation.
          </p>
          <div className="grid grid-cols-2 gap-4 text-sm">
            <div>
              <h3 className="font-semibold text-blue-400 mb-1">Key Features:</h3>
              <ul className="text-gray-300 space-y-1">
                <li>• Real Voyager 1 & 2 trajectories</li>
                <li>• MHD-based heliosphere shape</li>
                <li>• Accurate planetary positions</li>
                <li>• Solar cycle variations</li>
              </ul>
            </div>
            <div>
              <h3 className="font-semibold text-green-400 mb-1">Data Sources:</h3>
              <ul className="text-gray-300 space-y-1">
                <li>• NASA JPL Horizons</li>
                <li>• Voyager mission data</li>
                <li>• IBEX measurements</li>
                <li>• Latest research (2024)</li>
              </ul>
            </div>
          </div>
          <p className="text-xs text-gray-400 mt-3">
            Note: Distances compressed for visibility. 1 AU = Earth-Sun distance.
          </p>
        </div>
      )}
    </>
  );
}
