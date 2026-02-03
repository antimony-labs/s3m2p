# Mobile Tokens (Type, Spacing, Layout)

Purpose
- Standardize mobile layout and typography across surfaces.
- Keep readability consistent while preserving visual identity.

Breakpoints
- --bp-sm: 360px
- --bp-md: 768px

Spacing scale (8px rhythm with 4px micro step)
- --space-1: 4px
- --space-2: 8px
- --space-3: 12px
- --space-4: 16px
- --space-5: 24px
- --space-6: 32px
- --space-7: 48px

Typography scale
- --font-size-0: 16px
- --font-size-1: 18px
- --font-size-2: 20px
- --font-size-3: 24px
- --font-size-4: 28px
- --font-size-5: 32px

Line height
- --line-height-body: 1.7
- --line-height-heading: 1.2

Layout and sizing
- --content-max: 72ch
- --content-pad: clamp(16px, 4vw, 24px)
- --card-pad: clamp(12px, 3vw, 20px)
- --tap-min: 44px
- --radius-1: 6px
- --radius-2: 10px

Suggested usage (example)
```
:root {
  --space-1: 4px;
  --space-2: 8px;
  --space-3: 12px;
  --space-4: 16px;
  --space-5: 24px;
  --space-6: 32px;
  --space-7: 48px;

  --font-size-0: 16px;
  --font-size-1: 18px;
  --font-size-2: 20px;
  --font-size-3: 24px;
  --font-size-4: 28px;
  --font-size-5: 32px;

  --line-height-body: 1.7;
  --line-height-heading: 1.2;

  --content-max: 72ch;
  --content-pad: clamp(16px, 4vw, 24px);
  --card-pad: clamp(12px, 3vw, 20px);
  --tap-min: 44px;
  --radius-1: 6px;
  --radius-2: 10px;
}

.content {
  max-width: var(--content-max);
  padding: 0 var(--content-pad);
  margin: 0 auto;
  font-size: var(--font-size-0);
  line-height: var(--line-height-body);
}

.card {
  padding: var(--card-pad);
  border-radius: var(--radius-2);
}
```

