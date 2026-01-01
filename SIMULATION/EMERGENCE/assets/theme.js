/* ═══════════════════════════════════════════════════════════════════════════════
   FILE: theme.js | SIMULATION/EMERGENCE/assets/theme.js
   PURPOSE: Theme management + dev link rewriting for too.foo apps (local copy)
   MODIFIED: 2025-12-14
   ═══════════════════════════════════════════════════════════════════════════════ */

/**
 * Initialize theme on page load
 * Priority: localStorage > system preference > light (default)
 */
function initTheme() {
  const savedTheme = localStorage.getItem('theme');
  const systemPrefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;

  let theme = 'light';
  if (savedTheme) theme = savedTheme;
  else if (systemPrefersDark) theme = 'dark';

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
 * - localhost: ports
 * - *.local.too.foo: local subdomains
 */
function updateSubdomainLinks() {
  const host = window.location.hostname;
  const isLocalhost = host === 'localhost' || host === '127.0.0.1';
  const isLocalSubdomain = host.endsWith('.local.too.foo');

  if (!isLocalhost && !isLocalSubdomain) return;

  const portMap = {
    'helios': 8081,
    'chladni': 8082,
    'sensors': 8083,
    'autocrate': 8084,
    'blog': 8085,
    'learn': 8086,
    'arch': 8087,
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
  };

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

  document.querySelectorAll('a[href="/"]').forEach(link => {
    if (isLocalhost) link.setAttribute('href', 'http://localhost:8080');
    if (isLocalSubdomain) link.setAttribute('href', 'http://welcome.local.too.foo');
  });
}

function openDetailsFromHash() {
  const id = (window.location.hash || '').replace('#', '').trim();
  if (!id) return;
  const el = document.getElementById(id);
  if (!el) return;
  if (el.tagName && el.tagName.toLowerCase() === 'details') el.open = true;
}

// Initialize theme ASAP to avoid flash
initTheme();

document.addEventListener('DOMContentLoaded', () => {
  const themeToggle = document.getElementById('theme-toggle');
  if (themeToggle) themeToggle.addEventListener('click', toggleTheme);

  updateSubdomainLinks();
  openDetailsFromHash();
  window.addEventListener('hashchange', openDetailsFromHash);

  window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', (e) => {
    if (!localStorage.getItem('theme')) {
      document.documentElement.setAttribute('data-theme', e.matches ? 'dark' : 'light');
    }
  });
});


