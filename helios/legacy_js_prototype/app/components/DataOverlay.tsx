/**
 * Scientific data overlay for heliosphere visualization
 * Displays real-time measurements and information
 */

import React from 'react';
import { Activity, Navigation, Gauge, Thermometer, Magnet, Wind } from 'lucide-react';

interface VoyagerData {
  name: string;
  distance: number; // AU
  velocity: number; // km/s
  lightTime: number; // hours
  position: { lon: number; lat: number }; // degrees
  status: 'active' | 'inactive';
  lastMilestone?: string;
}

interface SolarWindData {
  speed: number; // km/s
  density: number; // particles/cm³
  temperature: number; // K
  pressure: number; // nPa
  magneticField: number; // nT
}

interface DataOverlayProps {
  voyager1: VoyagerData;
  voyager2: VoyagerData;
  solarWind: SolarWindData;
  sunspotNumber: number;
  currentDate: Date;
  showDetails: boolean;
}

export function DataOverlay({
  voyager1,
  voyager2,
  solarWind,
  sunspotNumber,
  currentDate,
  showDetails
}: DataOverlayProps) {
  const formatNumber = (num: number, decimals: number = 1): string => {
    return num.toFixed(decimals);
  };
  
  const formatScientific = (num: number): string => {
    if (num === 0) return '0';
    const exponent = Math.floor(Math.log10(Math.abs(num)));
    const mantissa = num / Math.pow(10, exponent);
    return `${mantissa.toFixed(1)}×10^${exponent}`;
  };
  
  const getSolarCyclePhase = (date: Date): { cycle: number; phase: string } => {
    // Solar cycle calculation (simplified)
    const startCycle24 = new Date('2008-12-01');
    const cycleLength = 11; // years
    const yearsSince = (date.getTime() - startCycle24.getTime()) / (365.25 * 24 * 60 * 60 * 1000);
    const cycle = Math.floor(yearsSince / cycleLength) + 24;
    const phasePercent = (yearsSince % cycleLength) / cycleLength;
    
    let phase = 'Minimum';
    if (phasePercent > 0.2 && phasePercent < 0.4) phase = 'Rising';
    else if (phasePercent > 0.4 && phasePercent < 0.6) phase = 'Maximum';
    else if (phasePercent > 0.6 && phasePercent < 0.8) phase = 'Declining';
    
    return { cycle, phase };
  };
  
  const solarCycle = getSolarCyclePhase(currentDate);
  
  return (
    <div className="fixed inset-x-4 sm:inset-x-auto top-[calc(env(safe-area-inset-top,0px)+1rem)] sm:top-4 sm:right-4 z-30 text-white pointer-events-none">
      <div className="pointer-events-auto w-full sm:w-80 ml-auto space-y-3 sm:space-y-4">
        {/* Main data panel */}
        <div className="bg-gray-900/85 backdrop-blur-md rounded-2xl shadow-2xl p-4 sm:p-5 w-full">
          <h2 className="text-lg font-semibold mb-3 border-b border-gray-700 pb-2">
            Mission Status
          </h2>
          
          {/* Voyager 1 */}
          <div className="mb-4">
            <div className="flex flex-wrap items-center justify-between gap-2 mb-2">
              <h3 className="font-medium text-green-400">Voyager 1</h3>
              <span className={`text-xs px-2 py-1 rounded ${
                voyager1.status === 'active' ? 'bg-green-600' : 'bg-gray-600'
              }`}>
                {voyager1.status.toUpperCase()}
              </span>
            </div>
            
            <div className="grid grid-cols-1 sm:grid-cols-2 gap-2 text-sm">
              <div className="flex items-center gap-1">
                <Navigation size={14} className="text-gray-400" />
                <span className="text-gray-400">Distance:</span>
                <span className="font-mono">{formatNumber(voyager1.distance)} AU</span>
              </div>
              <div className="flex items-center gap-1">
                <Activity size={14} className="text-gray-400" />
                <span className="text-gray-400">Speed:</span>
                <span className="font-mono">{formatNumber(voyager1.velocity)} km/s</span>
              </div>
              <div className="flex items-center gap-1 sm:col-span-2">
                <span className="text-gray-400">Light time:</span>
                <span className="font-mono">{formatNumber(voyager1.lightTime)} hours</span>
              </div>
              {voyager1.lastMilestone && (
                <div className="text-xs text-gray-500 sm:col-span-2">
                  {voyager1.lastMilestone}
                </div>
              )}
            </div>
          </div>
          
          {/* Voyager 2 */}
          <div className="mb-4">
            <div className="flex flex-wrap items-center justify-between gap-2 mb-2">
              <h3 className="font-medium text-cyan-400">Voyager 2</h3>
              <span className={`text-xs px-2 py-1 rounded ${
                voyager2.status === 'active' ? 'bg-green-600' : 'bg-gray-600'
              }`}>
                {voyager2.status.toUpperCase()}
              </span>
            </div>
            
            <div className="grid grid-cols-1 sm:grid-cols-2 gap-2 text-sm">
              <div className="flex items-center gap-1">
                <Navigation size={14} className="text-gray-400" />
                <span className="text-gray-400">Distance:</span>
                <span className="font-mono">{formatNumber(voyager2.distance)} AU</span>
              </div>
              <div className="flex items-center gap-1">
                <Activity size={14} className="text-gray-400" />
                <span className="text-gray-400">Speed:</span>
                <span className="font-mono">{formatNumber(voyager2.velocity)} km/s</span>
              </div>
              <div className="flex items-center gap-1 sm:col-span-2">
                <span className="text-gray-400">Light time:</span>
                <span className="font-mono">{formatNumber(voyager2.lightTime)} hours</span>
              </div>
              {voyager2.lastMilestone && (
                <div className="text-xs text-gray-500 sm:col-span-2">
                  {voyager2.lastMilestone}
                </div>
              )}
            </div>
          </div>
          
          {/* Solar activity */}
          <div className="border-t border-gray-700 pt-3">
            <h3 className="font-medium mb-2">Solar Activity</h3>
            <div className="grid grid-cols-1 sm:grid-cols-2 gap-2 text-sm">
              <div>
                <span className="text-gray-400">Sunspot Number:</span>
                <span className="font-mono ml-1">{formatNumber(sunspotNumber, 0)}</span>
              </div>
              <div>
                <span className="text-gray-400">Cycle:</span>
                <span className="ml-1">{solarCycle.cycle} ({solarCycle.phase})</span>
              </div>
            </div>
            
            {/* Solar cycle progress bar */}
            <div className="mt-2">
              <div className="h-2 bg-gray-700 rounded overflow-hidden">
                <div 
                  className="h-full bg-yellow-500 transition-all duration-500"
                  style={{ width: `${(sunspotNumber / 200) * 100}%` }}
                />
              </div>
              <div className="flex justify-between text-xs text-gray-500 mt-1">
                <span>Min</span>
                <span>Max</span>
              </div>
            </div>
          </div>
        </div>
        
        {/* Detailed measurements panel */}
        {showDetails && (
          <div className="bg-gray-900/85 backdrop-blur-md rounded-2xl shadow-2xl p-4 sm:p-5 w-full">
            <h2 className="text-lg font-semibold mb-3 border-b border-gray-700 pb-2">
              Solar Wind @ 1 AU
            </h2>
            
            <div className="space-y-3">
              <div className="flex items-center justify-between gap-3">
                <div className="flex items-center gap-2">
                  <Wind size={16} className="text-orange-400" />
                  <span className="text-gray-400">Speed</span>
                </div>
                <span className="font-mono">{formatNumber(solarWind.speed)} km/s</span>
              </div>
              
              <div className="flex items-center justify-between gap-3">
                <div className="flex items-center gap-2">
                  <Activity size={16} className="text-blue-400" />
                  <span className="text-gray-400">Density</span>
                </div>
                <span className="font-mono">{formatNumber(solarWind.density)} n/cm³</span>
              </div>
              
              <div className="flex items-center justify-between gap-3">
                <div className="flex items-center gap-2">
                  <Thermometer size={16} className="text-red-400" />
                  <span className="text-gray-400">Temperature</span>
                </div>
                <span className="font-mono">{formatScientific(solarWind.temperature)} K</span>
              </div>
              
              <div className="flex items-center justify-between gap-3">
                <div className="flex items-center gap-2">
                  <Gauge size={16} className="text-green-400" />
                  <span className="text-gray-400">Ram Pressure</span>
                </div>
                <span className="font-mono">{formatNumber(solarWind.pressure)} nPa</span>
              </div>
              
              <div className="flex items-center justify-between gap-3">
                <div className="flex items-center gap-2">
                  <Magnet size={16} className="text-purple-400" />
                  <span className="text-gray-400">Magnetic Field</span>
                </div>
                <span className="font-mono">{formatNumber(solarWind.magneticField)} nT</span>
              </div>
            </div>
            
            {/* Parker spiral angle */}
            <div className="mt-4 pt-3 border-t border-gray-700 space-y-1 text-sm">
              <div className="flex items-center justify-between gap-3">
                <span className="text-gray-400">Parker Spiral Angle @ 1 AU:</span>
                <span className="font-mono">45°</span>
              </div>
              <div className="flex items-center justify-between gap-3">
                <span className="text-gray-400">Alfvén Speed:</span>
                <span className="font-mono">{formatNumber(50 + solarWind.magneticField * 2)} km/s</span>
              </div>
            </div>
          </div>
        )}
        
        {/* Legend */}
        <div className="bg-gray-900/85 backdrop-blur-md rounded-2xl shadow-2xl p-4 sm:p-5 w-full">
          <h3 className="font-medium mb-2">Legend</h3>
          <div className="space-y-1.5 text-sm">
            <div className="flex items-center gap-2">
              <div className="w-3 h-3 bg-orange-500 rounded"></div>
              <span>Termination Shock (~90-200 AU)</span>
            </div>
            <div className="flex items-center gap-2">
              <div className="w-3 h-3 bg-blue-500 rounded"></div>
              <span>Heliopause (~120-350 AU)</span>
            </div>
            <div className="flex items-center gap-2">
              <div className="w-3 h-3 bg-purple-500 rounded"></div>
              <span>Bow Shock (Theoretical ~230 AU)</span>
            </div>
            <div className="flex items-center gap-2">
              <div className="w-3 h-3 bg-green-500 rounded"></div>
              <span>Voyager 1 Trajectory</span>
            </div>
            <div className="flex items-center gap-2">
              <div className="w-3 h-3 bg-cyan-500 rounded"></div>
              <span>Voyager 2 Trajectory</span>
            </div>
          </div>
        </div>
        
        {/* Scale reference */}
        <div className="bg-gray-900/85 backdrop-blur-md rounded-2xl shadow-2xl p-4 sm:p-5 w-full mb-6 sm:mb-0">
          <h3 className="font-medium mb-2">Distance Scale</h3>
          <div className="space-y-1 text-sm">
            <div className="flex flex-wrap items-center justify-between gap-2">
              <span>1 AU</span>
              <span className="text-gray-400">Earth-Sun distance</span>
            </div>
            <div className="flex flex-wrap items-center justify-between gap-2">
              <span>5.2 AU</span>
              <span className="text-gray-400">Jupiter orbit</span>
            </div>
            <div className="flex flex-wrap items-center justify-between gap-2">
              <span>30 AU</span>
              <span className="text-gray-400">Neptune orbit</span>
            </div>
            <div className="flex flex-wrap items-center justify-between gap-2">
              <span>40 AU</span>
              <span className="text-gray-400">Pluto (average)</span>
            </div>
            <div className="flex flex-wrap items-center justify-between gap-2">
              <span>100 AU</span>
              <span className="text-gray-400">Termination shock</span>
            </div>
            <div className="flex flex-wrap items-center justify-between gap-2">
              <span>120+ AU</span>
              <span className="text-gray-400">Interstellar space</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
