/**
 * Famous stars catalog
 * Coordinates in equatorial (RA, Dec) or galactic (l, b) coordinates
 * Distances in parsecs
 */

export interface StarData {
  name: string;
  ra: number;        // Right Ascension in hours
  dec: number;        // Declination in degrees
  distance: number;  // Distance in parsecs
  magnitude: number; // Apparent magnitude
  color: number;     // RGB color (0xRRGGBB)
  type: string;      // Spectral type
}

export const FAMOUS_STARS: StarData[] = [
  {
    name: 'Vega',
    ra: 18.6156,
    dec: 38.7836,
    distance: 7.68,
    magnitude: 0.03,
    color: 0xaabbff,
    type: 'A0V'
  },
  {
    name: 'Sirius',
    ra: 6.7525,
    dec: -16.7161,
    distance: 2.64,
    magnitude: -1.46,
    color: 0xffffff,
    type: 'A1V'
  },
  {
    name: 'Alpha Centauri',
    ra: 14.6608,
    dec: -60.8350,
    distance: 1.35,
    magnitude: -0.27,
    color: 0xffffcc,
    type: 'G2V'
  },
  {
    name: 'Betelgeuse',
    ra: 5.9195,
    dec: 7.4071,
    distance: 197.0,
    magnitude: 0.45,
    color: 0xff4444,
    type: 'M1-2Ia'
  },
  {
    name: 'Rigel',
    ra: 5.2423,
    dec: -8.2016,
    distance: 264.0,
    magnitude: 0.18,
    color: 0xaaaaff,
    type: 'B8Ia'
  },
  {
    name: 'Arcturus',
    ra: 14.2611,
    dec: 19.1824,
    distance: 11.26,
    magnitude: -0.05,
    color: 0xffaa44,
    type: 'K1.5III'
  },
  {
    name: 'Capella',
    ra: 5.2782,
    dec: 45.9980,
    distance: 13.0,
    magnitude: 0.08,
    color: 0xffffaa,
    type: 'G5III'
  },
  {
    name: 'Altair',
    ra: 19.8463,
    dec: 8.8683,
    distance: 5.13,
    magnitude: 0.76,
    color: 0xaaaaff,
    type: 'A7V'
  },
  {
    name: 'Spica',
    ra: 13.4199,
    dec: -11.1613,
    distance: 80.0,
    magnitude: 0.98,
    color: 0xaaaaff,
    type: 'B1V'
  },
  {
    name: 'Antares',
    ra: 16.4901,
    dec: -26.4320,
    distance: 170.0,
    magnitude: 1.06,
    color: 0xff4444,
    type: 'M1.5Iab'
  },
  {
    name: 'Pollux',
    ra: 7.7553,
    dec: 28.0262,
    distance: 10.34,
    magnitude: 1.14,
    color: 0xffffaa,
    type: 'K0III'
  },
  {
    name: 'Fomalhaut',
    ra: 22.9608,
    dec: -29.6222,
    distance: 7.7,
    magnitude: 1.17,
    color: 0xaaaaff,
    type: 'A3V'
  }
];

/**
 * Convert RA/Dec to Cartesian coordinates
 */
export function raDecToCartesian(ra: number, dec: number, distance: number): [number, number, number] {
  const raRad = (ra / 24) * Math.PI * 2;
  const decRad = (dec * Math.PI) / 180;
  
  const x = distance * Math.cos(decRad) * Math.cos(raRad);
  const y = distance * Math.cos(decRad) * Math.sin(raRad);
  const z = distance * Math.sin(decRad);
  
  return [x, y, z];
}
