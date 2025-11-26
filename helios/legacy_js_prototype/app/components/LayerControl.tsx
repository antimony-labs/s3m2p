'use client';

import { useState, useEffect, useRef } from 'react';
import { HeroRef } from './Hero';
import { ComponentVisibility } from '../lib/heliosphereScene';

interface LayerControlProps {
  heroRef: React.RefObject<HeroRef>;
}

const LAYER_LABELS: Record<keyof ComponentVisibility, string> = {
  heliosphere: 'Heliosphere Surface',
  helioglow: 'Helioglow (UV)',
  terminationShock: 'Termination Shock',
  bowShock: 'Bow Shock (Theoretical)',
  solarWind: 'Solar Wind Streams',
  interstellarWind: 'Interstellar Wind',
  planets: 'Planets',
  orbits: 'Planetary Orbits',
  moon: 'Moon',
  stars: 'Background Stars',
  famousStars: 'Famous Stars',
  voyagers: 'Voyager Spacecraft',
  distanceMarkers: 'Distance Markers (AU)',
  solarApex: 'Solar Apex Direction',
  labels: 'Object Labels',
  interstellarObjects: 'Interstellar Objects',
  constellations: 'Constellations',
};

const LAYER_GROUPS: Array<{
  name: string;
  keys: Array<keyof ComponentVisibility>;
}> = [
  {
    name: 'Heliosphere',
    keys: ['heliosphere', 'helioglow', 'terminationShock', 'bowShock'],
  },
  {
    name: 'Plasma',
    keys: ['solarWind', 'interstellarWind'],
  },
  {
    name: 'Solar System',
    keys: ['planets', 'orbits', 'moon'],
  },
  {
    name: 'Spacecraft',
    keys: ['voyagers'],
  },
  {
    name: 'Reference',
    keys: ['stars', 'famousStars', 'constellations'],
  },
  {
    name: 'Markers',
    keys: ['distanceMarkers', 'solarApex', 'labels', 'interstellarObjects'],
  },
];

export default function LayerControl({ heroRef }: LayerControlProps) {
  const [layers, setLayers] = useState<ComponentVisibility>({
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
    distanceMarkers: false,
    solarApex: false,
    labels: true,
    interstellarObjects: false,
    constellations: false,
  });
  const [isOpen, setIsOpen] = useState(false);
  const [expandedGroups, setExpandedGroups] = useState<Set<string>>(new Set());
  const menuRef = useRef<HTMLDivElement>(null);
  const [isMobile, setIsMobile] = useState(false);

  // Auto-expand all groups on mobile when menu opens
  useEffect(() => {
    if (typeof window === 'undefined') return;
    const checkMobile = () => {
      setIsMobile(window.innerWidth < 640);
    };
    checkMobile();
    window.addEventListener('resize', checkMobile);
    return () => window.removeEventListener('resize', checkMobile);
  }, []);

  useEffect(() => {
    if (isOpen && isMobile) {
      // Auto-expand all groups on mobile
      setExpandedGroups(new Set(LAYER_GROUPS.map(g => g.name)));
    }
  }, [isOpen, isMobile]);

  useEffect(() => {
    if (heroRef.current) {
      const sceneVisibility = heroRef.current.getVisibility();
      if (sceneVisibility) {
        setLayers(sceneVisibility);
      }
    }
  }, [heroRef]);

  useEffect(() => {
    if (!isOpen) return;

    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key === 'Escape') {
        setIsOpen(false);
      }
    };

    const handleClickOutside = (event: MouseEvent) => {
      if (menuRef.current && !menuRef.current.contains(event.target as Node)) {
        setIsOpen(false);
      }
    };

    document.addEventListener('keydown', handleKeyDown);
    document.addEventListener('mousedown', handleClickOutside);

    return () => {
      document.removeEventListener('keydown', handleKeyDown);
      document.removeEventListener('mousedown', handleClickOutside);
    };
  }, [isOpen]);

  const handleToggle = (key: keyof ComponentVisibility) => {
    const newValue = !layers[key];
    setLayers((prev) => ({ ...prev, [key]: newValue }));
    heroRef.current?.toggleComponent(key, newValue);
  };

  const handleToggleGroup = (groupName: string) => {
    const newExpanded = new Set(expandedGroups);
    if (newExpanded.has(groupName)) {
      newExpanded.delete(groupName);
    } else {
      newExpanded.add(groupName);
    }
    setExpandedGroups(newExpanded);
  };

  const handleSelectAll = () => {
    const allKeys = LAYER_GROUPS.flatMap((g) => g.keys);
    const newLayers = { ...layers };
    allKeys.forEach((key) => {
      newLayers[key] = true;
      heroRef.current?.toggleComponent(key, true);
    });
    setLayers(newLayers);
  };

  const handleSelectNone = () => {
    const allKeys = LAYER_GROUPS.flatMap((g) => g.keys);
    const newLayers = { ...layers };
    allKeys.forEach((key) => {
      newLayers[key] = false;
      heroRef.current?.toggleComponent(key, false);
    });
    setLayers(newLayers);
  };

  const activeCount = Object.values(layers).filter(Boolean).length;
  const totalCount = Object.keys(layers).length;

  const renderGroupControls = () => (
    <div className="space-y-1.5">
      {LAYER_GROUPS.map((group) => (
        <div key={group.name} className="rounded border border-white/10">
          <button
            onClick={() => handleToggleGroup(group.name)}
            className="flex w-full items-center justify-between px-2 py-1.5 sm:px-3 sm:py-2 text-[0.7rem] sm:text-[0.75rem] text-white transition-colors hover:bg-white/10 focus:outline-none focus:ring-2 focus:ring-white/40"
          >
            <span className="font-medium">{group.name}</span>
            <span className="text-lg">{expandedGroups.has(group.name) ? '−' : '+'}</span>
          </button>
          {expandedGroups.has(group.name) && (
            <div className="space-y-1 border-t border-white/10 bg-white/5 p-1.5 sm:p-2">
              {group.keys.map((key) => (
                <button
                  key={key}
                  onClick={(event) => {
                    event.preventDefault();
                    event.stopPropagation();
                    handleToggle(key);
                  }}
                  className={`w-full rounded border px-2 py-1.5 sm:px-3 sm:py-2 text-left text-[0.65rem] sm:text-[0.7rem] text-white transition-colors focus:outline-none focus:ring-2 focus:ring-white/40 ${
                    layers[key]
                      ? 'border-white/40 bg-white/25'
                      : 'border-white/20 bg-white/10 hover:bg-white/20'
                  }`}
                >
                  {LAYER_LABELS[key]}
                </button>
              ))}
            </div>
          )}
        </div>
      ))}
    </div>
  );

  return (
    <div
      ref={menuRef}
      className="relative flex items-center gap-2"
      data-ui="layer-control"
    >
      <button
        onClick={() => setIsOpen((prev) => !prev)}
        className="inline-flex items-center gap-1 sm:gap-2 rounded-full border border-white/20 bg-white/10 px-2 py-0.5 sm:px-3 sm:py-1 text-[0.55rem] sm:text-[0.65rem] uppercase tracking-[0.2em] sm:tracking-[0.25em] text-white transition-colors hover:bg-white/20 focus:outline-none focus:ring-2 focus:ring-white/40"
        aria-expanded={isOpen}
        aria-label="Toggle layer visibility menu"
      >
        Layers
        <span className="rounded-full border border-white/10 bg-black/30 px-1.5 py-0.5 sm:px-2 font-mono text-[0.55rem] sm:text-[0.65rem] text-white/80">
          {activeCount}/{totalCount}
        </span>
      </button>

      {isOpen && (
        <div className="hidden sm:block">
          <div className="absolute right-0 top-full z-40 mt-2 w-80 rounded-xl border border-white/10 bg-black/85 p-4 shadow-xl backdrop-blur">
            <div className="mb-3 flex items-center justify-between">
              <h3 className="text-sm font-semibold text-white">Layer Controls</h3>
              <div className="flex gap-2 text-[0.65rem]">
                <button
                  onClick={handleSelectAll}
                  className="rounded-full border border-white/20 bg-white/10 px-3 py-1 text-white transition-colors hover:bg-white/20 focus:outline-none focus:ring-2 focus:ring-white/40"
                >
                  All
                </button>
                <button
                  onClick={handleSelectNone}
                  className="rounded-full border border-white/20 bg-white/10 px-3 py-1 text-white transition-colors hover:bg-white/20 focus:outline-none focus:ring-2 focus:ring-white/40"
                >
                  None
                </button>
              </div>
            </div>
            {renderGroupControls()}
          </div>
        </div>
      )}

      {isOpen && (
        <div className="sm:hidden">
          <div
            className="fixed inset-0 z-40 bg-black/80 backdrop-blur"
            onClick={() => setIsOpen(false)}
          />
          <div className="fixed inset-x-4 top-[calc(env(safe-area-inset-top,0px)+1rem)] bottom-[calc(env(safe-area-inset-bottom,0px)+1rem)] z-50 flex flex-col rounded-2xl border border-white/20 bg-black/90 shadow-2xl">
            <div className="flex-shrink-0 flex items-center justify-between border-b border-white/10 px-3 py-2">
              <h3 className="text-sm font-semibold text-white">Layer Controls</h3>
              <button
                onClick={() => setIsOpen(false)}
                className="rounded-full border border-white/20 bg-white/10 px-2 py-1 text-xs text-white transition-colors hover:bg-white/20 focus:outline-none focus:ring-2 focus:ring-white/40"
                aria-label="Close layer controls"
              >
                ✕
              </button>
            </div>
            <div className="flex-shrink-0 flex items-center gap-2 px-3 py-2 border-b border-white/10">
              <button
                onClick={handleSelectAll}
                className="flex-1 rounded-full border border-white/20 bg-white/10 px-2 py-1.5 text-xs text-white transition-colors hover:bg-white/20 focus:outline-none focus:ring-2 focus:ring-white/40"
              >
                All
              </button>
              <button
                onClick={handleSelectNone}
                className="flex-1 rounded-full border border-white/20 bg-white/10 px-2 py-1.5 text-xs text-white transition-colors hover:bg-white/20 focus:outline-none focus:ring-2 focus:ring-white/40"
              >
                None
              </button>
            </div>
            <div className="flex-1 overflow-y-auto px-2 py-2">
              {renderGroupControls()}
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
