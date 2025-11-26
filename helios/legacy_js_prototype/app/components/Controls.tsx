'use client';

import { useEffect, useRef, useState } from 'react';
import { getPrefersReducedMotion, createMotionObserver } from '../lib/motion';
import { HeroRef } from './Hero';
import { linearToLogYear, yearToLinear, formatLogTime, yearsToSeconds } from '../lib/timeScale';

type Direction = 1 | -1;

interface ControlsProps {
  heroRef: React.RefObject<HeroRef>;
  onTimeChange: (time: number) => void;
  onDirectionChange: (direction: Direction) => void;
  onMotionChange: (enabled: boolean) => void;
  onPauseChange: (paused: boolean) => void;
}

const SPEED_PRESETS = [
  { label: '1 Earth Year', shortLabel: '1 yr', value: 1 },
  { label: '1 Solar Cycle', shortLabel: '1 cycle', value: 11 },
  { label: '100 Solar Cycles', shortLabel: '100 cycles', value: 1100 },
  { label: '100K Solar Cycles', shortLabel: '100K cycles', value: 1100000 },
];

const MIN_LOG_YEARS = 0.001;
const MAX_LOG_YEARS = 1500000;

export default function Controls({
  heroRef,
  onTimeChange,
  onDirectionChange,
  onMotionChange,
  onPauseChange,
}: ControlsProps) {
  const [year, setYear] = useState(2024.0);
  const [speedIndex, setSpeedIndex] = useState(1);
  const [direction, setDirection] = useState<Direction>(1);
  const [paused, setPaused] = useState(false);
  const [reduceMotion, setReduceMotion] = useState(false);
  const [announcement, setAnnouncement] = useState('');
  const sliderRef = useRef<HTMLInputElement>(null);
  const currentYearRef = useRef(2024.0);
  const targetYearRef = useRef(2024.0);
  const animationFrameRef = useRef<number | null>(null);

  const [logSliderValue, setLogSliderValue] = useState(
    yearToLinear(2024.0, MIN_LOG_YEARS, MAX_LOG_YEARS),
  );

  const speed = SPEED_PRESETS[speedIndex].value;

  useEffect(() => {
    const preset = SPEED_PRESETS[speedIndex];
    setAnnouncement(
      `Year: ${Math.floor(year)}. Speed: ${preset.label} (${speed.toLocaleString()} years/sec).`,
    );
  }, [year, speed, speedIndex]);

  useEffect(() => {
    const systemReduced = getPrefersReducedMotion();
    setReduceMotion(systemReduced);
    onMotionChange(!systemReduced);

    const cleanup = createMotionObserver((reduced) => {
      setReduceMotion(reduced);
      onMotionChange(!reduced);
      if (reduced) {
        setAnnouncement('Motion off (respects your system setting).');
      }
      heroRef.current?.updateScene(currentYearRef.current, direction, !reduced && !paused);
    });

    return cleanup;
  }, [onMotionChange, heroRef, direction, paused]);

  useEffect(() => {
    if (reduceMotion || paused) {
      if (animationFrameRef.current) {
        cancelAnimationFrame(animationFrameRef.current);
        animationFrameRef.current = null;
      }
      try {
        heroRef.current?.updateScene(currentYearRef.current, direction, false);
      } catch (error) {
        console.error('Error updating scene on pause:', error);
      }
      return;
    }

    let lastFrameTime = performance.now();
    let isRunning = true;

    function animate(currentTime: number) {
      if (!isRunning) {
        animationFrameRef.current = null;
        return;
      }

      try {
        const dt = (currentTime - lastFrameTime) / 1000;
        lastFrameTime = currentTime;

        const current = currentYearRef.current;
        const target = targetYearRef.current;
        let finalYear: number;

        if (Math.abs(current - target) > 0.01) {
          const alpha = Math.min(0.15, dt * 5);
          finalYear = current + (target - current) * alpha;
          currentYearRef.current = finalYear;
          setYear(finalYear);
          onTimeChange(finalYear);
        } else {
          finalYear = current + speed * dt * direction;
          finalYear = Math.max(MIN_LOG_YEARS, Math.min(MAX_LOG_YEARS, finalYear));

          currentYearRef.current = finalYear;
          targetYearRef.current = finalYear;
          setYear(finalYear);

          const newLogValue = yearToLinear(finalYear, MIN_LOG_YEARS, MAX_LOG_YEARS);
          setLogSliderValue(newLogValue);
          onTimeChange(finalYear);
        }

        heroRef.current?.updateScene(finalYear, direction, true);
      } catch (error) {
        console.error('Error in animation loop:', error);
        isRunning = false;
        if (animationFrameRef.current) {
          cancelAnimationFrame(animationFrameRef.current);
          animationFrameRef.current = null;
        }
        return;
      }

      if (isRunning && !reduceMotion && !paused) {
        animationFrameRef.current = requestAnimationFrame(animate);
      }
    }

    animationFrameRef.current = requestAnimationFrame(animate);

    return () => {
      isRunning = false;
      if (animationFrameRef.current) {
        cancelAnimationFrame(animationFrameRef.current);
        animationFrameRef.current = null;
      }
    };
  }, [reduceMotion, paused, direction, speed, onTimeChange, heroRef]);

  const handleYearChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const linearValue = parseFloat(e.target.value);
    const value = linearToLogYear(linearValue, MIN_LOG_YEARS, MAX_LOG_YEARS);
    targetYearRef.current = value;
    currentYearRef.current = value;
    setYear(value);
    setLogSliderValue(linearValue);
    onTimeChange(value);
    heroRef.current?.updateScene(value, direction, !reduceMotion && !paused);
  };

  const handleSpeedChange = (newIndex: number) => {
    setSpeedIndex(newIndex);
    const preset = SPEED_PRESETS[newIndex];
    setAnnouncement(
      `Speed: ${preset.label} (${preset.value.toLocaleString()} years/sec).`,
    );
  };

  const handleDirectionToggle = () => {
    const newDirection = direction === 1 ? -1 : 1;
    setDirection(newDirection);
    onDirectionChange(newDirection);
    setAnnouncement(`Direction: ${newDirection === 1 ? 'Forward' : 'Reverse'}.`);
    heroRef.current?.updateScene(currentYearRef.current, newDirection, !reduceMotion && !paused);
  };

  const handlePauseToggle = () => {
    const newPaused = !paused;
    setPaused(newPaused);
    onPauseChange(newPaused);
    setAnnouncement(newPaused ? 'Paused.' : 'Resumed.');
    try {
      heroRef.current?.updateScene(currentYearRef.current, direction, !reduceMotion && !newPaused);
    } catch (error) {
      console.error('Error updating scene on pause', error);
    }
  };

  const handleReduceMotionToggle = () => {
    if (!getPrefersReducedMotion()) {
      const newReduced = !reduceMotion;
      setReduceMotion(newReduced);
      onMotionChange(!newReduced);
      if (newReduced) {
        setAnnouncement('Motion off (respects your system setting).');
      }
      heroRef.current?.updateScene(currentYearRef.current, direction, !newReduced && !paused);
    }
  };

  const handleSliderKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === 'Escape') {
      sliderRef.current?.blur();
    }
  };

  const motionDisabled = reduceMotion || paused;

  return (
    <div
      className="flex flex-col gap-1.5 sm:gap-3"
      role="region"
      aria-label="Simulation controls"
    >
      <div className="flex flex-col gap-1.5 sm:flex-row sm:items-center sm:gap-3">
        <div className="flex flex-col gap-0.5 sm:flex-1 sm:gap-1">
          <label
            htmlFor="year-slider"
            className="flex items-center justify-between text-[0.6rem] uppercase tracking-[0.2em] text-white/60 sm:text-[0.5rem] sm:tracking-[0.25em]"
          >
            <span>Time</span>
            <span className="ml-2 font-mono text-white/80 tracking-normal text-[0.55rem] sm:text-[0.6rem]">
              {formatLogTime(yearsToSeconds(year))}
            </span>
          </label>
          <input
            id="year-slider"
            ref={sliderRef}
            type="range"
            min="0"
            max="1"
            step="0.001"
            value={logSliderValue}
            onChange={handleYearChange}
            onKeyDown={handleSliderKeyDown}
            disabled={motionDisabled}
            aria-label="Time (logarithmic scale)"
            title="Scrub through time (logarithmic scale)"
            className="h-1 sm:h-1.5 w-full appearance-none rounded-full bg-white/20 disabled:cursor-not-allowed disabled:opacity-50"
            style={{
              background: `linear-gradient(to right, #ffffff 0%, #ffffff ${logSliderValue * 100}%, rgba(255, 255, 255, 0.2) ${logSliderValue * 100}%, rgba(255, 255, 255, 0.2) 100%)`,
            }}
          />
        </div>

        <div className="flex items-center justify-center gap-1 sm:gap-1.5 sm:ml-auto">
          <button
            onClick={handleDirectionToggle}
            disabled={motionDisabled}
            aria-label="Switch travel direction"
            title="Switch travel direction"
            className="rounded border border-white/20 bg-white/10 px-1.5 py-0.5 text-xs sm:px-2 sm:py-1 sm:text-sm text-white transition-colors hover:bg-white/20 focus:outline-none focus:ring-2 focus:ring-white/40 disabled:cursor-not-allowed disabled:opacity-50"
          >
            {direction === 1 ? '▶' : '◀'}
          </button>
          <button
            onClick={handlePauseToggle}
            disabled={reduceMotion}
            aria-label={paused ? 'Resume' : 'Pause'}
            title={paused ? 'Resume' : 'Pause'}
            className="rounded border border-white/20 bg-white/10 px-1.5 py-0.5 text-xs sm:px-2 sm:py-1 sm:text-sm text-white transition-colors hover:bg-white/20 focus:outline-none focus:ring-2 focus:ring-white/40 disabled:cursor-not-allowed disabled:opacity-50"
          >
            {paused ? '▶' : '⏸'}
          </button>
          <button
            onClick={handleReduceMotionToggle}
            disabled={getPrefersReducedMotion()}
            aria-label="Disable background motion"
            title={reduceMotion ? 'Motion off' : 'Disable background motion'}
            className="rounded border border-white/20 bg-white/10 px-1.5 py-0.5 text-[0.55rem] sm:px-2 sm:py-1 sm:text-[0.65rem] text-white transition-colors hover:bg-white/20 focus:outline-none focus:ring-2 focus:ring-white/40 disabled:cursor-not-allowed disabled:opacity-50"
          >
            {reduceMotion ? 'Motion off' : 'Reduce'}
          </button>
        </div>
      </div>

      <div className="flex flex-wrap items-center justify-center sm:justify-start gap-1 sm:gap-2">
        <span className="text-[0.55rem] uppercase tracking-[0.25em] text-white/60 sm:text-[0.5rem] sm:tracking-[0.3em]">
          Speed
        </span>
        <div className="flex flex-wrap items-center justify-center gap-0.5 sm:gap-1.5">
          {SPEED_PRESETS.map((preset, index) => (
            <button
              key={preset.label}
              onClick={() => handleSpeedChange(index)}
              disabled={motionDisabled}
              aria-label={`Set speed to ${preset.label}`}
              title={`${preset.label} (${preset.value.toLocaleString()} years/sec)`}
              className={`rounded-full border px-1.5 py-0.5 text-[0.55rem] sm:px-2 sm:py-1 sm:text-[0.65rem] text-white transition-colors focus:outline-none focus:ring-2 focus:ring-white/40 disabled:cursor-not-allowed disabled:opacity-50 ${
                speedIndex === index
                  ? 'border-white/40 bg-white/25'
                  : 'border-white/20 bg-white/10 hover:bg-white/20'
              }`}
            >
              {preset.shortLabel}
            </button>
          ))}
        </div>
      </div>

      <div aria-live="polite" aria-atomic="true" className="sr-only">
        {announcement}
      </div>
    </div>
  );
}
