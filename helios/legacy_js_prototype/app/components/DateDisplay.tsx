'use client';

import { yearsToSeconds } from '../lib/timeScale';

interface DateDisplayProps {
  year: number;
  speed: number; // years per second
}

export default function DateDisplay({ year, speed }: DateDisplayProps) {
  // Format year in standard date format
  const formatStandardTime = (y: number): string => {
    // If it's a reasonable year (between 1000 and 10000), show as calendar date
    if (y >= 1000 && y < 10000) {
      const wholeYear = Math.floor(y);
      const fraction = y - wholeYear;
      const days = Math.floor(fraction * 365.25);
      const date = new Date(wholeYear, 0, 1);
      date.setDate(date.getDate() + days);
      
      const monthNames = ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec'];
      const month = monthNames[date.getMonth()];
      const day = date.getDate();
      const yearStr = date.getFullYear();
      
      return `${yearStr} ${month} ${day}`;
    }
    // For very large or small years, show in scientific notation or with units
    else if (y >= 10000) {
      if (y >= 1000000) {
        return `${(y / 1000000).toFixed(2)}M years`;
      } else if (y >= 1000) {
        return `${(y / 1000).toFixed(2)}K years`;
      }
      return `${y.toFixed(0)} years`;
    }
    // For very small years, show as days/months
    else if (y < 1) {
      const days = y * 365.25;
      if (days < 1) {
        const hours = days * 24;
        if (hours < 1) {
          const minutes = hours * 60;
          return `${minutes.toFixed(0)} min`;
        }
        return `${hours.toFixed(1)} hours`;
      }
      return `${days.toFixed(0)} days`;
    }
    // Default: show year
    return `Year ${y.toFixed(0)}`;
  };
  
  // Format speed display
  const formatSpeed = (s: number): string => {
    if (s < 1) {
      return `${(s * 365.25).toFixed(1)} days/sec`;
    } else if (s < 365.25) {
      return `${s.toFixed(1)} years/sec`;
    } else if (s < 365250) {
      return `${(s / 365.25).toFixed(1)} centuries/sec`;
    } else {
      return `${(s / 365250).toFixed(1)} millennia/sec`;
    }
  };

  return (
    <div className="fixed top-4 left-1/2 transform -translate-x-1/2 z-50 bg-black/60 backdrop-blur-sm border border-white/20 rounded-lg px-6 py-4 shadow-lg">
      <div className="flex flex-col gap-1 items-center">
        <div className="text-3xl md:text-4xl font-mono font-light text-white">
          {formatStandardTime(year)}
        </div>
        <div className="text-sm text-white/60">
          Speed: {formatSpeed(speed)}
        </div>
      </div>
    </div>
  );
}
