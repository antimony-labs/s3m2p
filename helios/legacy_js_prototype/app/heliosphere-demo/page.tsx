/**
 * Sun-Centric Heliosphere Demo Page
 * Showcases the new dataset-driven architecture
 */

import { Metadata } from 'next';
import dynamic from 'next/dynamic';

// Dynamically import with SSR disabled to prevent hydration errors
// This component relies heavily on browser APIs (WebGL, canvas) and should not be server-rendered
const HeliosphereDemoClient = dynamic(
  () => import('./HeliosphereDemoClient'),
  { 
    ssr: false,
    loading: () => (
      <div className="relative h-screen w-full bg-black flex items-center justify-center">
        <div className="text-white text-center">
          <div className="mb-4">
            <div className="inline-block h-12 w-12 animate-spin rounded-full border-4 border-solid border-cyan-400 border-r-transparent"></div>
          </div>
          <p className="text-xl">Loading Sun-Centric Heliosphere...</p>
        </div>
      </div>
    ),
  }
);

export const metadata: Metadata = {
  title: 'Sun-Centric Heliosphere | Demo',
  description: 'Interactive visualization of the heliosphere using precomputed datasets across the Sun\'s lifetime',
};

export default function HeliosphereDemoPage() {
  return (
    <main className="relative min-h-screen w-full bg-black">
      <HeliosphereDemoClient />
    </main>
  );
}

