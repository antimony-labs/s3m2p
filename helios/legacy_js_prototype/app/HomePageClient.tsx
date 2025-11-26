'use client';

import dynamic from 'next/dynamic';
import ClientWrapper from './components/ClientWrapper';

// Navigation uses usePathname() which can cause hydration errors
// Make it client-only to prevent SSR hydration mismatches
const Navigation = dynamic(
  () => import('./components/Navigation'),
  { ssr: false }
);

export default function HomePageClient() {
  return (
    <div className="min-h-screen bg-black text-white flex flex-col" style={{ minHeight: 'var(--viewport-height, 100vh)' }}>
      <Navigation />
      <div
        className="fixed inset-0 pointer-events-none z-0"
        style={{
          background:
            'radial-gradient(circle at center, transparent 0%, rgba(0, 0, 0, 0.2) 80%, rgba(0, 0, 0, 0.4) 100%)',
        }}
      />

      <section
        className="relative flex-1"
        style={{ minHeight: 'var(--viewport-height, 100vh)' }}
      >
        <ClientWrapper />

        <noscript>
          <img
            src="/img/heliosphere-still.png"
            alt="Stylized, scientifically-informed heliosphere; apex direction implied."
            className="absolute inset-0 w-full h-full object-cover opacity-50 z-0"
          />
        </noscript>

        <div className="absolute inset-x-4 sm:left-1/2 sm:-translate-x-1/2 bottom-[calc(env(safe-area-inset-bottom,0px)+3.5rem)] sm:bottom-32 z-20 pointer-events-none">
          <p className="text-sm sm:text-base md:text-xl lg:text-2xl text-white/80 max-w-2xl mx-auto text-center drop-shadow-md">
            Uploading before GTA 6. Learning the tech and philosophy to encode our planet's DNA.
          </p>
        </div>
        <p className="sr-only">Illustrative; not to scale.</p>
      </section>
    </div>
  );
}

