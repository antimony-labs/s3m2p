# too.foo — Solar Memory Online

A minimal, scientifically accurate, production-ready prelaunch site featuring a WebGL visualization of the heliosphere and solar apex drift.

## Overview

This is a static prelaunch website built with Next.js 14, featuring a fully code-rendered WebGL visualization of the heliosphere. The site respects accessibility preferences, includes robust fallbacks, and is optimized for static hosting on Vercel.

## Tech Stack

- **Framework**: Next.js 14 (App Router, TypeScript)
- **Styling**: Tailwind CSS
- **3D Rendering**: Three.js (WebGL2)
- **Deployment**: Vercel (static export)
- **Build Tools**: TypeScript, PostCSS, Autoprefixer

## Features

- 🌌 **WebGL Visualization**: Real-time rendering of heliosphere, starfield, and solar system
- ♿ **Accessibility**: Full ARIA support, keyboard navigation, prefers-reduced-motion
- 🎨 **Minimal Design**: Cosmic calm aesthetic with dark indigo background and cyan accents
- 📱 **Responsive**: Mobile-first design with collapsible controls
- 🖼️ **Fallbacks**: PNG still images for JS-off and WebGL-unsupported scenarios
- ⚡ **Performance**: Optimized for < 2s FCP, ~60 FPS on 2019 laptops

## Project Structure

```
too.foo/
├── app/
│   ├── components/
│   │   ├── Hero.tsx           # WebGL canvas mount & scene lifecycle
│   │   ├── Controls.tsx       # Time, Direction, Motion controls
│   │   └── ClientWrapper.tsx  # Client-side state container
│   ├── lib/
│   │   ├── apex.ts            # Solar apex direction calculations
│   │   ├── heliosphereScene.ts # Three.js scene creation & management
│   │   ├── motion.ts          # Motion preference helpers
│   │   └── still.ts           # Build-time still generation (TypeScript)
│   ├── globals.css            # Tailwind & custom styles
│   ├── layout.tsx             # Root layout with metadata
│   └── page.tsx               # Main page component
├── public/
│   └── img/
│       ├── heliosphere-still.png  # Generated fallback image
│       └── og.png                  # Generated OG image
├── scripts/
│   └── generate-stills.js     # Build script for still generation
├── next.config.js             # Next.js configuration (static export)
├── tailwind.config.ts         # Tailwind configuration
└── tsconfig.json              # TypeScript configuration
```

## Getting Started

### Prerequisites

- Node.js 18+ 
- npm or yarn

### Installation

```bash
# Install dependencies
npm install

# Generate still images (fallback & OG)
npm run generate-stills

# Start development server
npm run dev
```

### Build

```bash
# Build for production (generates stills automatically)
npm run build

# The output will be in the `out/` directory for static hosting
```

## Controls

The site includes a control dock (bottom-right) with:

- **Time Slider**: Scrub through the solar drift animation (0 → 1)
- **Direction Toggle**: Switch between "Apex →" and "Reverse ←"
- **Reduce Motion**: Disable background motion (respects system preference)
- **Pause Background**: Freeze the animation

All controls are fully keyboard accessible with proper ARIA labels and tooltips.

## Scientific Grounding

The visualization is qualitatively accurate with plausible cues:

- **Solar Apex Direction**: RA ≈ 18h, Dec ≈ +30° (Hercules/Vega region)
- **Ecliptic Tilt**: 23.44° constant
- **Heliosphere Shape**: Modern "blunted/croissant-like" abstraction
- **Starfield**: GPU-instanced points with blackbody color bins and distance-based parallax

Note: The visualization is illustrative and not to scale.

## Accessibility

- ✅ Respects `prefers-reduced-motion` system setting
- ✅ Full ARIA labels and roles
- ✅ Keyboard navigation (Tab, Enter, Space, Esc)
- ✅ Screen reader announcements via `aria-live`
- ✅ High contrast focus indicators
- ✅ Semantic HTML structure

## Performance

- **Target FCP**: < 2 seconds
- **Target FPS**: ~60 FPS on 2019 laptops
- **Draw Calls**: < 10 per frame
- **Star Count**: 3k-8k (configurable)
- **Throttling**: Automatic when tab is backgrounded

## License

AGPL-3.0-or-later • TooFoo Continuum License v0.1

## Credits

Scientific depiction based on modern heliosphere research and solar apex direction approximations. Star colors use blackbody temperature bins. Heliosphere silhouette follows the "blunted/croissant" consensus abstraction.

---

**Note**: This is a prelaunch site. No analytics are included at launch for privacy-first approach.




