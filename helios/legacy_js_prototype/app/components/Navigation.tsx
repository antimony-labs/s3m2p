'use client';

import { useEffect, useState } from 'react';
import Link from 'next/link';
import { usePathname } from 'next/navigation';

export default function Navigation() {
  const pathname = usePathname();
  const [mounted, setMounted] = useState(false);

  // Only use pathname after mount to prevent hydration mismatch
  useEffect(() => {
    setMounted(true);
  }, []);

  // Use empty string during SSR to match initial client render
  const currentPathname = mounted ? pathname : '';

  const links = [
    { href: '/', label: 'Home' },
    { href: '/research', label: 'Research', highlight: true },
  ];

  return (
    <nav className="fixed top-0 left-0 right-0 z-50 bg-black bg-opacity-80 backdrop-blur-sm border-b border-white border-opacity-10">
      <div className="max-w-7xl mx-auto px-2 sm:px-6 lg:px-8">
        <div className="flex items-center justify-between h-10 sm:h-16">
          <div className="flex items-center">
            <Link href="/" className="text-white font-bold text-base sm:text-xl">
              too.foo
            </Link>
          </div>
          
          <div className="flex items-center space-x-2 sm:space-x-4">
            {links.map((link) => (
              <Link
                key={link.href}
                href={link.href}
                className={`px-2 py-1 sm:px-3 sm:py-2 rounded-md text-xs sm:text-sm font-medium transition-colors ${
                  currentPathname === link.href
                    ? 'bg-white bg-opacity-20 text-white'
                    : link.highlight
                    ? 'text-cyan-400 hover:bg-cyan-400 hover:bg-opacity-10'
                    : 'text-gray-300 hover:bg-white hover:bg-opacity-10 hover:text-white'
                }`}
                suppressHydrationWarning
              >
                {link.label}
              </Link>
            ))}
          </div>
        </div>
      </div>
    </nav>
  );
}

