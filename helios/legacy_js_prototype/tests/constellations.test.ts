import { describe, expect, it } from 'vitest';
import type { Constellation } from '../app/lib/data/Constellations';
import { generateConstellationLines } from '../app/lib/data/Constellations';
import { FAMOUS_STARS } from '../app/lib/starCatalog';

const baseConstellation: Constellation = {
  name: 'Test',
  abbreviation: 'TST',
  stars: [],
  connections: [
    { from: 'Betelgeuse', to: 'Rigel' }
  ],
  center: { ra: 0, dec: 0 }
};

describe('generateConstellationLines', () => {
  it('returns geometry when connected stars exist', () => {
    const geometry = generateConstellationLines(baseConstellation, FAMOUS_STARS);
    expect(geometry).not.toBeNull();

    const positions = geometry!.getAttribute('position');
    expect(positions).toBeDefined();
    expect(positions.count).toBe(2);
  });

  it('returns null if catalog is missing required stars', () => {
    const geometry = generateConstellationLines(
      {
        ...baseConstellation,
        connections: [
          { from: 'Nonexistent Star', to: 'Rigel' }
        ]
      },
      FAMOUS_STARS
    );
    expect(geometry).toBeNull();
  });
});
