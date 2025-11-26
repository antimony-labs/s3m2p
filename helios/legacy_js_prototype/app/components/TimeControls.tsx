/**
 * Time control interface for heliosphere visualization
 * Allows historical, real-time, and predictive modes
 */

import React, { useState, useEffect, useCallback } from 'react';
import { Play, Pause, SkipForward, SkipBack, Calendar, Clock, Zap, Settings } from 'lucide-react';

export type TimeMode = 'historical' | 'realtime' | 'prediction';

interface TimeControlsProps {
  currentDate: Date;
  onDateChange: (date: Date) => void;
  timeSpeed: number; // Days per frame
  onTimeSpeedChange: (speed: number) => void;
  isPlaying: boolean;
  onPlayPause: () => void;
  timeMode: TimeMode;
  onTimeModeChange: (mode: TimeMode) => void;
}

export function TimeControls({
  currentDate,
  onDateChange,
  timeSpeed,
  onTimeSpeedChange,
  isPlaying,
  onPlayPause,
  timeMode,
  onTimeModeChange
}: TimeControlsProps) {
  const [showSettings, setShowSettings] = useState(false);
  const [inputDate, setInputDate] = useState('');
  
  // Format date for display
  const formatDate = useCallback((date: Date): string => {
    return date.toLocaleDateString('en-US', {
      year: 'numeric',
      month: 'long',
      day: 'numeric'
    });
  }, []);
  
  // Format date for input
  const formatDateInput = useCallback((date: Date): string => {
    const year = date.getFullYear();
    const month = String(date.getMonth() + 1).padStart(2, '0');
    const day = String(date.getDate()).padStart(2, '0');
    return `${year}-${month}-${day}`;
  }, []);
  
  useEffect(() => {
    setInputDate(formatDateInput(currentDate));
  }, [currentDate, formatDateInput]);
  
  // Handle date input change
  const handleDateInput = (e: React.ChangeEvent<HTMLInputElement>) => {
    const newDate = new Date(e.target.value);
    if (!isNaN(newDate.getTime())) {
      onDateChange(newDate);
    }
  };
  
  // Skip forward/backward
  const skip = (days: number) => {
    const newDate = new Date(currentDate);
    newDate.setDate(newDate.getDate() + days);
    onDateChange(newDate);
  };
  
  // Predefined speeds (days per frame at 60fps)
  const speeds = [
    { label: '1 day/s', value: 1/60 },
    { label: '1 month/s', value: 30/60 },
    { label: '1 year/s', value: 365.25/60 },
    { label: '11 years/s', value: 11 * 365.25/60 }, // 1 solar cycle per second
    { label: '100 years/s', value: 100 * 365.25/60 },
    { label: '1000 years/s', value: 1000 * 365.25/60 },
  ];
  
  // Key milestones
  const milestones = [
    { label: 'Voyager 1 Launch', date: new Date('1977-09-05') },
    { label: 'Voyager 1 at Jupiter', date: new Date('1979-03-05') },
    { label: 'Voyager 2 at Neptune', date: new Date('1989-08-25') },
    { label: 'V1 Termination Shock', date: new Date('2004-12-16') },
    { label: 'V1 Enters Interstellar Space', date: new Date('2012-08-25') },
    { label: 'V2 Enters Interstellar Space', date: new Date('2018-11-05') },
    { label: 'Today', date: new Date() },
  ];
  
  return (
    <div className="fixed bottom-4 left-1/2 transform -translate-x-1/2 bg-gray-900/90 backdrop-blur-sm rounded-lg shadow-xl p-4 text-white">
      {/* Main controls */}
      <div className="flex items-center gap-4 mb-2">
        {/* Time mode selector */}
        <div className="flex bg-gray-800 rounded-md p-1">
          <button
            className={`px-3 py-1 rounded flex items-center gap-1 transition-colors ${
              timeMode === 'historical' ? 'bg-blue-600' : 'hover:bg-gray-700'
            }`}
            onClick={() => onTimeModeChange('historical')}
            title="Historical Mode"
          >
            <Calendar size={16} />
            <span className="text-sm">History</span>
          </button>
          <button
            className={`px-3 py-1 rounded flex items-center gap-1 transition-colors ${
              timeMode === 'realtime' ? 'bg-blue-600' : 'hover:bg-gray-700'
            }`}
            onClick={() => onTimeModeChange('realtime')}
            title="Real-time Mode"
          >
            <Clock size={16} />
            <span className="text-sm">Live</span>
          </button>
          <button
            className={`px-3 py-1 rounded flex items-center gap-1 transition-colors ${
              timeMode === 'prediction' ? 'bg-blue-600' : 'hover:bg-gray-700'
            }`}
            onClick={() => onTimeModeChange('prediction')}
            title="Prediction Mode"
          >
            <Zap size={16} />
            <span className="text-sm">Future</span>
          </button>
        </div>
        
        {/* Playback controls */}
        <div className="flex items-center gap-2">
          <button
            className="p-2 hover:bg-gray-700 rounded transition-colors"
            onClick={() => skip(-365)}
            title="Back 1 year"
          >
            <SkipBack size={20} />
          </button>
          
          <button
            className="p-3 bg-blue-600 hover:bg-blue-700 rounded-full transition-colors"
            onClick={onPlayPause}
            aria-label={isPlaying ? 'Pause' : 'Play'}
            title={isPlaying ? 'Pause' : 'Play'}
          >
            {isPlaying ? <Pause size={24} /> : <Play size={24} />}
          </button>
          
          <button
            className="p-2 hover:bg-gray-700 rounded transition-colors"
            onClick={() => skip(365)}
            title="Forward 1 year"
          >
            <SkipForward size={20} />
          </button>
        </div>
        
        {/* Current date display */}
        <div className="text-center">
          <div className="text-lg font-semibold">{formatDate(currentDate)}</div>
          <div className="text-xs text-gray-400">
            Day {Math.floor((currentDate.getTime() - new Date('1977-09-05').getTime()) / (24 * 60 * 60 * 1000))} of Voyager Mission
          </div>
        </div>
        
        {/* Speed control */}
        <div className="flex items-center gap-2">
          <span className="text-sm text-gray-400">Speed:</span>
          <select
            className="bg-gray-800 rounded px-2 py-1 text-sm"
            value={timeSpeed}
            onChange={(e) => onTimeSpeedChange(parseFloat(e.target.value))}
          >
            {speeds.map(speed => (
              <option key={speed.value} value={speed.value}>
                {speed.label}
              </option>
            ))}
          </select>
        </div>
        
        {/* Settings button */}
        <button
          className="p-2 hover:bg-gray-700 rounded transition-colors"
          onClick={() => setShowSettings(!showSettings)}
        >
          <Settings size={20} />
        </button>
      </div>
      
      {/* Extended settings panel */}
      {showSettings && (
        <div className="mt-4 pt-4 border-t border-gray-700">
          <div className="grid grid-cols-2 gap-4">
            {/* Date input */}
            <div>
              <label className="block text-sm text-gray-400 mb-1">Jump to date:</label>
              <input
                type="date"
                className="bg-gray-800 rounded px-3 py-1 w-full"
                value={inputDate}
                onChange={handleDateInput}
                min="1900-01-01"
                max="2100-12-31"
              />
            </div>
            
            {/* Milestones */}
            <div>
              <label className="block text-sm text-gray-400 mb-1">Milestones:</label>
              <select
                className="bg-gray-800 rounded px-3 py-1 w-full"
                onChange={(e) => {
                  const milestone = milestones[parseInt(e.target.value)];
                  if (milestone) onDateChange(milestone.date);
                }}
                defaultValue=""
              >
                <option value="" disabled>Select milestone...</option>
                {milestones.map((milestone, index) => (
                  <option key={index} value={index}>
                    {milestone.label}
                  </option>
                ))}
              </select>
            </div>
          </div>
          
          {/* Timeline visualization */}
          <div className="mt-4">
            <div className="relative h-2 bg-gray-700 rounded">
              {/* Progress bar */}
              <div
                className="absolute h-full bg-blue-600 rounded"
                style={{
                  width: `${Math.min(100, Math.max(0,
                    ((currentDate.getTime() - new Date('1977-01-01').getTime()) /
                    (new Date('2030-01-01').getTime() - new Date('1977-01-01').getTime())) * 100
                  ))}%`
                }}
              />
              
              {/* Milestone markers */}
              {milestones.map((milestone, index) => {
                const position = ((milestone.date.getTime() - new Date('1977-01-01').getTime()) /
                  (new Date('2030-01-01').getTime() - new Date('1977-01-01').getTime())) * 100;
                
                if (position >= 0 && position <= 100) {
                  return (
                    <div
                      key={index}
                      className="absolute w-1 h-4 bg-white rounded -top-1 cursor-pointer hover:bg-blue-400"
                      style={{ left: `${position}%` }}
                      onClick={() => onDateChange(milestone.date)}
                      title={milestone.label}
                    />
                  );
                }
                return null;
              })}
            </div>
            
            <div className="flex justify-between text-xs text-gray-400 mt-1">
              <span>1977</span>
              <span>2030</span>
            </div>
          </div>
          
          {/* Info panel */}
          <div className="mt-4 text-sm text-gray-400">
            <div className="flex justify-between">
              <span>Voyager 1 Distance:</span>
              <span className="text-white">
                {(163 + (currentDate.getFullYear() - 2024) * 3.5).toFixed(1)} AU
              </span>
            </div>
            <div className="flex justify-between">
              <span>Voyager 2 Distance:</span>
              <span className="text-white">
                {(136 + (currentDate.getFullYear() - 2024) * 3.2).toFixed(1)} AU
              </span>
            </div>
            <div className="flex justify-between">
              <span>Light Time from Earth:</span>
              <span className="text-white">
                {((163 + (currentDate.getFullYear() - 2024) * 3.5) * 8.3 / 60).toFixed(1)} hours
              </span>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
