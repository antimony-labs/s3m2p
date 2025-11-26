'use client';

const TECH_STACK = [
  { name: 'Next.js', version: '14.2.33', url: 'https://nextjs.org' },
  { name: 'React', version: '18.3.1', url: 'https://react.dev' },
  { name: 'Three.js', version: '0.181.0', url: 'https://threejs.org' },
  { name: 'TypeScript', version: '5.7.2', url: 'https://www.typescriptlang.org' },
  { name: 'Tailwind CSS', version: '3.4.17', url: 'https://tailwindcss.com' },
  { name: 'Vitest', version: '4.0.7', url: 'https://vitest.dev' },
  { name: 'Playwright', version: '1.56.1', url: 'https://playwright.dev' },
];

export default function Footer() {
  return (
    <footer
      className="fixed inset-x-0 bottom-0 z-20 pointer-events-none"
      style={{ paddingBottom: 'calc(env(safe-area-inset-bottom, 0px) + 0.5rem)' }}
    >
      <div className="px-2 sm:px-4 lg:px-6">
        <div className="mx-auto max-w-6xl">
          <div className="pointer-events-auto rounded-xl sm:rounded-2xl border border-white/10 bg-black/55 backdrop-blur px-2 py-1 sm:px-4 sm:py-3">
            <div className="flex flex-col gap-1 sm:flex-row sm:items-center sm:justify-between sm:gap-3">
              {/* Hide tech stack on mobile to maximize screen space */}
              <div className="hidden sm:flex flex-col gap-1">
                <span className="text-[0.5rem] uppercase tracking-[0.35em] text-white/40">
                  Tech Stack
                </span>
                <div className="flex flex-wrap gap-1.5 sm:gap-2">
                  {TECH_STACK.map((tech) => (
                    <a
                      key={tech.name}
                      href={tech.url}
                      target="_blank"
                      rel="noopener noreferrer"
                      className="group inline-flex items-baseline gap-1 rounded-full border border-white/10 bg-white/5 px-2 py-0.5 text-white/70 transition-colors hover:border-white/20 hover:bg-white/15 hover:text-white"
                      title={`${tech.name} v${tech.version}`}
                    >
                      <span className="text-[0.6rem] font-medium group-hover:text-white">
                        {tech.name}
                      </span>
                      <span className="font-mono text-[0.5rem] text-white/40 group-hover:text-white/60">
                        {tech.version}
                      </span>
                    </a>
                  ))}
                </div>
              </div>

              {/* Compact license on mobile */}
              <div className="text-[0.45rem] uppercase tracking-[0.25em] text-white/30 sm:text-[0.5rem] sm:tracking-[0.3em] sm:text-right">
                <div className="sm:block">AGPL-3.0-or-later</div>
                <div className="hidden sm:block">TooFoo Continuum License v0.1</div>
                <div className="sm:hidden">AGPL-3.0</div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </footer>
  );
}


