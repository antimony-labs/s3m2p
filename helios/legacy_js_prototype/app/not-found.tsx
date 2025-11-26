import NotFoundExperience from './components/NotFoundExperience';
import { generateNotFoundTelemetry } from './lib/notFoundTelemetry';

export default function NotFound() {
  const telemetry = generateNotFoundTelemetry();
  return <NotFoundExperience initialTelemetry={telemetry} />;
}
