/* ═══════════════════════════════════════════════════════════════════════════════
   FILE: theme.js | WELCOME/assets/theme.js
   PURPOSE: Theme management and environment-aware link handling for too.foo
   MODIFIED: 2025-12-14
   ═══════════════════════════════════════════════════════════════════════════════

   Functions:
   - initTheme() - Initialize theme from localStorage or system preference
   - toggleTheme() - Switch between light/dark themes
   - updateSubdomainLinks() - Fix subdomain links for dev vs prod environments
*/

/**
 * Initialize theme on page load
 * Priority: localStorage > system preference > light (default)
 */
function initTheme() {
  const savedTheme = localStorage.getItem('theme');
  const systemPrefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;

  let theme = 'light'; // default

  if (savedTheme) {
    theme = savedTheme;
  } else if (systemPrefersDark) {
    theme = 'dark';
  }

  document.documentElement.setAttribute('data-theme', theme);
}

/**
 * Toggle between light and dark themes
 * Saves preference to localStorage
 */
function toggleTheme() {
  const currentTheme = document.documentElement.getAttribute('data-theme') || 'light';
  const newTheme = currentTheme === 'light' ? 'dark' : 'light';

  document.documentElement.setAttribute('data-theme', newTheme);
  localStorage.setItem('theme', newTheme);
}

/**
 * Update subdomain links based on environment
 * Dev: Use localhost:PORT
 * Prod: Use https://subdomain.too.foo
 */
function updateSubdomainLinks() {
  // Detect environment by hostname
  const host = window.location.hostname;
  const isLocalhost = host === 'localhost' || host === '127.0.0.1';
  const isLocalSubdomain = host.endsWith('.local.too.foo');

  if (!isLocalhost && !isLocalSubdomain) {
    // Production - links already use too.foo subdomains, no changes needed
    return;
  }

  // Development (localhost) - map subdomains to localhost ports
  const portMap = {
    'helios': 8081,
    'chladni': 8082,
    'sensors': 8083,
    'autocrate': 8084,
    'blog': 8085,
    'learn': 8086,
    'emergence': 8089,
    'pll': 8090,
    'power': 8091,
    'ai': 8100,
    'ubuntu': 8101,
    'opencv': 8102,
    'arduino': 8103,
    'esp32': 8104,
    'swarm': 8105,
    'slam': 8106,
    'crm': 8108,
  };

  // Update all links that point to too.foo domains
  document.querySelectorAll('a[href*="too.foo"]').forEach(link => {
    const href = link.getAttribute('href') || '';

    // Root domain
    if (/^https?:\/\/too\.foo\/?$/.test(href)) {
      if (isLocalhost) link.setAttribute('href', 'http://localhost:8080');
      if (isLocalSubdomain) link.setAttribute('href', 'http://welcome.local.too.foo');
      return;
    }

    const match = href.match(/^https?:\/\/([^.]+)\.too\.foo(\/.*)?$/);
    if (!match) return;

    const subdomain = match[1];

    if (isLocalhost) {
      const port = portMap[subdomain];
      if (port) link.setAttribute('href', `http://localhost:${port}`);
      return;
    }

    if (isLocalSubdomain) {
      link.setAttribute('href', `http://${subdomain}.local.too.foo`);
      return;
    }
  });

  // Fix home links (href="/" should go to the landing)
  document.querySelectorAll('a[href="/"]').forEach(link => {
    if (isLocalhost) link.setAttribute('href', 'http://localhost:8080');
    if (isLocalSubdomain) link.setAttribute('href', 'http://welcome.local.too.foo');
  });
}

/**
 * Viewport / aspect ratio classification.
 * We expose it to CSS via:
 * - <html data-vp="mobile-portrait|mobile-landscape|desktop">
 * - CSS vars: --vp-w, --vp-h, --vp-min, --vp-max, --vp-ar
 */
function updateViewportMode() {
  const w = Math.max(1, window.innerWidth || 1);
  const h = Math.max(1, window.innerHeight || 1);
  const ar = w / h;
  const minDim = Math.min(w, h);
  const maxDim = Math.max(w, h);

  let vp = 'desktop';
  if (minDim < 720) {
    vp = ar < 1 ? 'mobile-portrait' : 'mobile-landscape';
  }

  const root = document.documentElement;
  root.setAttribute('data-vp', vp);
  root.style.setProperty('--vp-w', `${w}px`);
  root.style.setProperty('--vp-h', `${h}px`);
  root.style.setProperty('--vp-min', `${minDim}px`);
  root.style.setProperty('--vp-max', `${maxDim}px`);
  root.style.setProperty('--vp-ar', `${ar}`);
}

// ═══════════════════════════════════════════════════════════════════════════════
// INITIALIZATION
// ═══════════════════════════════════════════════════════════════════════════════

// Initialize theme immediately (before DOM loads) to prevent flash
initTheme();

// Set up event listeners when DOM is ready
document.addEventListener('DOMContentLoaded', () => {
  // Theme toggle button
  const themeToggle = document.getElementById('theme-toggle');
  if (themeToggle) {
    themeToggle.addEventListener('click', toggleTheme);
  }

  // Update subdomain links for dev environment
  updateSubdomainLinks();

  // Viewport mode (aspect ratio aware) for responsive scaling
  updateViewportMode();
  let raf = 0;
  const onResize = () => {
    if (raf) cancelAnimationFrame(raf);
    raf = requestAnimationFrame(() => {
      raf = 0;
      updateViewportMode();
    });
  };
  window.addEventListener('resize', onResize, { passive: true });
  window.addEventListener('orientationchange', onResize, { passive: true });

  // Listen for system theme changes
  window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', (e) => {
    // Only update if user hasn't set manual preference
    if (!localStorage.getItem('theme')) {
      const newTheme = e.matches ? 'dark' : 'light';
      document.documentElement.setAttribute('data-theme', newTheme);
    }
  });
});
