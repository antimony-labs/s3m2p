# WELCOME - Lightweight Landing Page for too.foo

Simple, fast-loading HTML landing page for Antimony Labs - "too.foo" domain.
No WASM, no build step, pure HTML/CSS/JS.

## Serve & Deploy

```bash
# Development (no build needed!)
cd WELCOME && python3 -m http.server 8080

# Or use any static file server
cd WELCOME && npx serve .

# Deploy: just copy files to hosting (no compilation)
# Files are deployed to Cloudflare Pages as "too-foo" project
```

## Architecture

```
WELCOME/
├── index.html           # Main landing page (mosaic grid)
├── about.html           # Mission/vision (5 languages)
└── assets/
    ├── theme.css        # Shared theme system
    ├── theme.js         # Theme toggle + link management
    └── islands/         # 25 SVG project icons
```

## Purpose

This is the **landing page** for Antimony Labs (too.foo).
- Lightweight: No Rust/WASM, loads instantly
- Responsive: Mobile-first design
- Accessible: Theme toggle (light/dark), system fonts
- Navigation hub: Links to all projects

## Visual Components

### Header
- Animated SVG bubble mark (respects `prefers-reduced-motion`)
- Site name: "too.foo"
- Theme toggle button (☀️ / 🌙)

### Hero Section
- Title: "Antimony Labs"
- Tagline: "Let AI design, humans build."
- Subtitle: Tech stack description

### Mosaic Grid
Apple Watch-like circular layout with 8 main bubbles:
1. **Helios** - Solar system simulation
2. **X** - @LazyShivam (external link)
3. **Blog** - Technical writing
4. **Learn** - Tutorials & courses (expandable)
5. **Simulations** - Interactive demos (expandable)
6. **Tools** - Engineering apps (expandable)
7. **About Me** - LinkedIn profile (external link)
8. **Vision** - Mission statement (links to about.html)

### Expandable Sections
Using `<details>` elements (no JS required):
- **Tools**: PLL, Sensors, AutoCrate, CRM, Power
- **Simulations**: Chladni, **Emergence** (new!), more coming
- **Learn**: AI, Ubuntu, OpenCV, Arduino, ESP32, Swarm, SLAM

## Navigation Links

All links use production URLs (`https://subdomain.too.foo`).
In development, `theme.js` automatically converts them to `localhost:PORT`.

| Link | Destination | Dev URL |
|------|-------------|---------|
| Helios | helios.too.foo | localhost:8081 |
| Blog | blog.too.foo | localhost:8085 |
| Chladni | chladni.too.foo | localhost:8082 |
| Emergence | emergence.too.foo | localhost:8089 |
| PLL | pll.too.foo | localhost:8090 |
| ... | ... | ... |

## Theme System (Shared)

### theme.css
- CSS variables for light/dark themes
- System font stack (no Google Fonts)
- Responsive header component
- Theme toggle button styles
- Utility classes

### theme.js
- Reads `localStorage` and `prefers-color-scheme`
- Toggle between light/dark themes
- Persists user preference
- Auto-converts subdomain links for dev environment

**Usage in other projects**:
```html
<link rel="stylesheet" href="../../WELCOME/assets/theme.css">
<script src="../../WELCOME/assets/theme.js"></script>
```

## Common Tasks

### Adding a new project
1. Add SVG icon to `assets/islands/`
2. Add link in appropriate section of `index.html`:
   - Main mosaic (for major projects)
   - Expandable `<details>` (for category items)
3. Update `SCRIPTS/config.sh` with port mapping
4. Update `Caddyfile` with subdomain routing

### Changing theme colors
Edit `assets/theme.css` CSS variables:
```css
--bg-light: #ffffff;
--bg-dark: #050508;
--accent-light: #0066cc;
--accent-dark: #4d9fff;
```

### Adding multilingual content
Edit `about.html` - add new language section following the pattern:
```html
<div class="vision-content" data-lang="fr" style="display: none;">
  <h2>Un produit pour faire des produits</h2>
  <p>...</p>
</div>
```

## Migration History

**2025-12-14**: Major refactor (Issue #57)
- Removed WASM/Rust simulation (moved to SIMULATION/EMERGENCE)
- Converted to lightweight HTML-only page
- Added shared theme system (theme.css / theme.js)
- Created about.html for mission/vision content
- Deleted: src/*, Cargo.toml, Trunk.toml

**Previous**: Interactive boid simulation with constellation navigation

## Performance

- **Load time**: < 200ms (no compilation, no large JS bundles)
- **Size**: ~100KB total (HTML + CSS + JS + SVGs)
- **Accessibility**: WCAG AA compliant, keyboard navigable
- **SEO**: Fully indexable static HTML

## Testing

```bash
# Visual regression tests
npx playwright test TESTS/specs/visual.spec.ts

# Manual checklist
- [ ] All project links work (both dev and prod)
- [ ] Theme toggle switches correctly
- [ ] Responsive on mobile (mosaic grid adjusts)
- [ ] About page loads and language switcher works
- [ ] No console errors
```

## Deployment

Deployed to **too.foo** (root domain) via Cloudflare Pages.

```bash
# Deploy script
./SCRIPTS/deploy.sh welcome --publish

# CI/CD auto-deploys on push to main branch
```

Output: Static files served directly (no build step).

## Related Projects

- **SIMULATION/EMERGENCE**: Boids ecosystem (the old WELCOME simulation)
- **SIMULATION/CHLADNI**: Wave patterns
- **HELIOS**: Solar system

## Links

- Production: https://too.foo
- Dev: http://localhost:8080 or http://welcome.local.too.foo
- GitHub: https://github.com/Shivam-Bhardwaj/S3M2P
