import type { Metadata } from 'next';
import dynamic from 'next/dynamic';

// Make entire page client-only to prevent all hydration errors
// This page relies on browser APIs and client-side state
const ResearchPageClient = dynamic(
  () => import('./ResearchPageClient'),
  { 
    ssr: false,
    loading: () => (
      <main className="min-h-screen bg-black text-white flex items-center justify-center">
        <div className="text-center">
          <div className="mb-4">
            <div className="inline-block h-12 w-12 animate-spin rounded-full border-4 border-solid border-cyan-400 border-r-transparent"></div>
          </div>
          <p className="text-xl">Loading Sun-Centric Heliosphere...</p>
        </div>
      </main>
    ),
  }
);

export const metadata: Metadata = {
  title: 'Heliosphere Research Visualization — Scientific Accuracy',
  description: 'Sun-centric, dataset-driven visualization of the heliosphere across the Sun\'s entire lifetime (0-12 Gyr)',
  openGraph: {
    title: 'Sun-Centric Heliosphere Visualization',
    description: 'Scientifically accurate heliosphere simulation with precomputed datasets',
    images: ['/img/heliosphere-research.png'],
  },
  twitter: {
    card: 'summary_large_image',
    title: 'Sun-Centric Heliosphere Visualization',
    description: 'Scientifically accurate heliosphere simulation with precomputed datasets',
    images: ['/img/heliosphere-research.png'],
  },
};

// Note: With static export (output: 'export'), we can't use 'export const dynamic'
// Instead, we rely on dynamic() imports with ssr: false to prevent SSR
// The page will be statically generated as an empty shell that loads client components
export default function ResearchPage() {
  return <ResearchPageClient />;
}
