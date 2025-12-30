# LEARN Theme System

Universal theme CSS and JavaScript for all LEARN tutorials.

## Usage

In your tutorial's `index.html`:

```html
<head>
    <meta charset="UTF-8">
    <title>Your Tutorial</title>

    <!-- Google Fonts -->
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link href="https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@400;500&family=Rajdhani:wght@400;600;700&family=Inter:wght@400;500;600&display=swap" rel="stylesheet">

    <!-- Universal Theme -->
    <link rel="stylesheet" href="../theme/learn-theme.css">
    <script src="../theme/learn-theme.js"></script>

    <!-- Tutorial-specific accent colors -->
    <style>
        :root {
            --accent: #YOUR_COLOR;
            --accent-dim: rgba(YOUR_R, YOUR_G, YOUR_B, 0.2);
            --border: rgba(YOUR_R, YOUR_G, YOUR_B, 0.15);
        }

        :root[data-theme="light"] {
            --accent: #YOUR_LIGHT_COLOR;
            --accent-dim: rgba(YOUR_R, YOUR_G, YOUR_B, 0.15);
            --border: rgba(YOUR_R, YOUR_G, YOUR_B, 0.2);
        }
    </style>

    <!-- Your WASM -->
    <link data-trunk rel="rust" href="Cargo.toml" data-wasm-opt="z" />
</head>
```

## Accent Colors

| Tutorial | Dark Mode | Light Mode |
|----------|-----------|------------|
| UBUNTU | #e95420 | #c44018 |
| SLAM | #64ffda | #00897B |
| ESP32 | #ffaa44 | #cc7722 |
| AI | #00ffaa | #00aa77 |
| GIT | #F05032 | #D02010 |

## Features

- ✅ Dark/light mode with system preference detection
- ✅ localStorage persistence
- ✅ Responsive design (mobile-optimized)
- ✅ Print-friendly styles
- ✅ WCAG AA contrast ratios
- ✅ Consistent typography (Rajdhani/Inter/JetBrains Mono)

## Fixed Issues

1. **Bold text invisible** - Now uses accent color
2. **Partition calculator poor contrast** - Theme-aware colors
3. **Font consistency** - Explicit font-family on all elements
4. **Terminal alignment** - Fixed command hint spacing

## API

Global JavaScript functions:

- `window.toggleTheme()` - Switch between light/dark
- `window.getCurrentTheme()` - Get current theme ('light' or 'dark')

Events:

- `themechange` - Dispatched when theme changes
  ```javascript
  window.addEventListener('themechange', (e) => {
      console.log('Theme changed to:', e.detail.theme);
  });
  ```
