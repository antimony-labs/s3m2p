/*
═══════════════════════════════════════════════════════════════════════════════
FILE: learn-theme.js | LEARN/theme/learn-theme.js
PURPOSE: Universal theme manager for all LEARN tutorials
MODIFIED: 2025-12-30
LAYER: LEARN → theme
═══════════════════════════════════════════════════════════════════════════════
*/

// Theme Manager - Site-wide light/dark mode
(function() {
    const THEME_KEY = 'toofoo_theme';

    function getSystemTheme() {
        return window.matchMedia('(prefers-color-scheme: light)').matches ? 'light' : 'dark';
    }

    function getStoredTheme() {
        try {
            return localStorage.getItem(THEME_KEY);
        } catch {
            return null;
        }
    }

    function setTheme(theme) {
        document.documentElement.setAttribute('data-theme', theme);
        try {
            localStorage.setItem(THEME_KEY, theme);
        } catch {}
        window.dispatchEvent(new CustomEvent('themechange', { detail: { theme } }));
    }

    function initTheme() {
        const theme = getStoredTheme() || getSystemTheme();
        setTheme(theme);

        // Listen for system theme changes
        window.matchMedia('(prefers-color-scheme: light)').addEventListener('change', (e) => {
            if (!getStoredTheme()) {
                setTheme(e.matches ? 'light' : 'dark');
            }
        });
    }

    // Global API
    window.toggleTheme = function() {
        const current = document.documentElement.getAttribute('data-theme');
        setTheme(current === 'light' ? 'dark' : 'light');
    };

    window.getCurrentTheme = () => document.documentElement.getAttribute('data-theme') || 'dark';

    // Initialize on load
    initTheme();
})();
