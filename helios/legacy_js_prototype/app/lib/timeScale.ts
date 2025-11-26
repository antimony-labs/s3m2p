/**
 * Logarithmic time scale utilities
 * Maps linear slider input (0-1) to logarithmic years
 * Supports time ranges from seconds to billions of years
 */

export const TIME_RANGES = {
  SECONDS: { min: 0, max: 1 },           // 0-1 seconds
  MINUTES: { min: 1, max: 60 },          // 1-60 seconds
  HOURS: { min: 60, max: 86400 },        // 1 minute - 1 day
  DAYS: { min: 86400, max: 31536000 },   // 1 day - 1 year
  YEARS: { min: 31536000, max: 31536000000 }, // 1 year - 1000 years
  MILLENNIA: { min: 31536000000, max: 31536000000000 }, // 1K - 1M years
  EONS: { min: 31536000000000, max: 31536000000000000 }, // 1M - 1B years
};

// Convert linear slider value (0-1) to logarithmic years
export function linearToLogYear(linearValue: number, minYears: number = 0, maxYears: number = 1000000): number {
  if (linearValue <= 0) return minYears;
  if (linearValue >= 1) return maxYears;
  
  // Logarithmic mapping: log10 scale
  const logMin = Math.log10(minYears || 0.0001);
  const logMax = Math.log10(maxYears);
  const logValue = logMin + (logMax - logMin) * linearValue;
  
  return Math.pow(10, logValue);
}

// Convert year to linear slider value (0-1)
export function yearToLinear(year: number, minYears: number = 0, maxYears: number = 1000000): number {
  if (year <= minYears) return 0;
  if (year >= maxYears) return 1;
  
  const logMin = Math.log10(minYears || 0.0001);
  const logMax = Math.log10(maxYears);
  const logYear = Math.log10(year);
  
  return (logYear - logMin) / (logMax - logMin);
}

// Format time with appropriate units
export function formatLogTime(seconds: number): string {
  if (seconds < 1) {
    return `${(seconds * 1000).toFixed(1)} ms`;
  } else if (seconds < 60) {
    return `${seconds.toFixed(2)} sec`;
  } else if (seconds < 3600) {
    return `${(seconds / 60).toFixed(1)} min`;
  } else if (seconds < 86400) {
    return `${(seconds / 3600).toFixed(1)} hours`;
  } else if (seconds < 31536000) {
    return `${(seconds / 86400).toFixed(1)} days`;
  } else if (seconds < 31536000000) {
    return `${(seconds / 31536000).toFixed(2)} years`;
  } else if (seconds < 31536000000000) {
    return `${(seconds / 31536000000).toFixed(2)}K years`;
  } else if (seconds < 31536000000000000) {
    return `${(seconds / 31536000000000).toFixed(2)}M years`;
  } else {
    return `${(seconds / 31536000000000000).toFixed(2)}B years`;
  }
}

// Convert years to seconds (for internal calculations)
export function yearsToSeconds(years: number): number {
  return years * 31536000; // 365.25 days * 24 * 60 * 60
}

// Convert seconds to years
export function secondsToYears(seconds: number): number {
  return seconds / 31536000;
}
