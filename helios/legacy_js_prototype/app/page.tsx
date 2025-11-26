import type { Metadata } from 'next';
import dynamic from 'next/dynamic';

// Make entire page client-only to prevent all hydration errors
// This page relies on browser APIs and client-side state
const HomePageClient = dynamic(
  () => import('./HomePageClient'),
  { 
    ssr: false,
    loading: () => (
      <div className="min-h-screen bg-black text-white flex items-center justify-center">
        <div className="text-center">
          <div className="mb-4">
            <div className="inline-block h-12 w-12 animate-spin rounded-full border-4 border-solid border-cyan-400 border-r-transparent"></div>
          </div>
          <p className="text-xl">Loading...</p>
        </div>
      </div>
    ),
  }
);

export const metadata: Metadata = {
  title: 'too.foo — Solar Memory Online',
  description: 'A minimal prelaunch portal for too.foo — encoding our planet\'s living memory. Uploading before GTA 6.',
  openGraph: {
    title: 'too.foo — Solar Memory Online',
    description: 'A minimal prelaunch portal for too.foo — encoding our planet\'s living memory. Uploading before GTA 6.',
    images: ['/img/og.png'],
  },
  twitter: {
    card: 'summary_large_image',
    title: 'too.foo — Solar Memory Online',
    description: 'A minimal prelaunch portal for too.foo — encoding our planet\'s living memory. Uploading before GTA 6.',
    images: ['/img/og.png'],
  },
};

// Note: With static export (output: 'export'), we can't use 'export const dynamic'
// Instead, we rely on dynamic() imports with ssr: false to prevent SSR
// The page will be statically generated as an empty shell that loads client components
export default function Home() {
  return <HomePageClient />;
}
