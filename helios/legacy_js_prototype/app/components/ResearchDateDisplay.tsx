'use client';

import { useState, useEffect } from 'react';

export default function ResearchDateDisplay() {
  const [currentDate, setCurrentDate] = useState<Date>(new Date());

  useEffect(() => {
    const handleDateUpdate = (e: Event) => {
      const customEvent = e as CustomEvent<Date>;
      if (customEvent.detail) {
        setCurrentDate(customEvent.detail);
      }
    };

    // Listen for date updates
    window.addEventListener('research-date-update', handleDateUpdate);
    
    // Also listen for scene ready event to get initial date
    const handleSceneReady = () => {
      // Request initial date from hero component if available
      setTimeout(() => {
        const event = new CustomEvent('research-date-request');
        window.dispatchEvent(event);
      }, 100);
    };
    
    window.addEventListener('research-scene-ready', handleSceneReady);
    
    return () => {
      window.removeEventListener('research-date-update', handleDateUpdate);
      window.removeEventListener('research-scene-ready', handleSceneReady);
    };
  }, []);

  const formatDate = (date: Date): string => {
    const year = date.getFullYear();
    const month = String(date.getMonth() + 1).padStart(2, '0');
    const day = String(date.getDate()).padStart(2, '0');
    return `${year}-${month}-${day}`;
  };

  return (
    <div className="text-2xl md:text-3xl font-mono font-light text-white min-h-[2rem]">
      {formatDate(currentDate)}
    </div>
  );
}

