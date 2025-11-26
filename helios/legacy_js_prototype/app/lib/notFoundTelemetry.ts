import { VoyagerTrajectories } from './physics/SpacecraftTrajectories';

const AU_IN_KM = 149597870.7;
const LIGHT_SPEED_KM_S = 299792.458;
const MINUTES_PER_SECOND = 1 / 60;
const LIGHT_MINUTES_PER_AU = (AU_IN_KM / LIGHT_SPEED_KM_S) * MINUTES_PER_SECOND;

export type NotFoundTelemetry = {
  voyager1DistanceAU: number;
  voyager2DistanceAU: number;
  propagationDelayMin: number;
  signalIntegrity: number;
  heliopauseDriftAU: number;
  barycentricBaselineAU: number;
  isoTimestamp: string;
};

export function generateNotFoundTelemetry(reference: Date = new Date()): NotFoundTelemetry {
  const v1 = VoyagerTrajectories.VOYAGER_1;
  const v2 = VoyagerTrajectories.VOYAGER_2;
  
  const yearsSinceHeliopause = (reference.getTime() - v1.milestones.heliopause.date.getTime()) / (1000 * 60 * 60 * 24 * 365.25);
  const expectedDistance = v1.milestones.heliopause.distance + v1.current.annualMotion * Math.max(yearsSinceHeliopause, 0);
  const heliopauseDrift = v1.current.distance - expectedDistance;

  const signalIntegrity = Math.exp(-v1.current.distance / 220);

  return {
    voyager1DistanceAU: v1.current.distance,
    voyager2DistanceAU: v2.current.distance,
    propagationDelayMin: v1.current.distance * LIGHT_MINUTES_PER_AU,
    signalIntegrity,
    heliopauseDriftAU: heliopauseDrift,
    barycentricBaselineAU: (v1.current.distance + v2.current.distance) / 2,
    isoTimestamp: reference.toISOString()
  };
}
