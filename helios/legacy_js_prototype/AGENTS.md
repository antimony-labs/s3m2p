# Repository Guidelines

## Project Structure & Module Organization
Next.js App Router code sits in `app/` (entry layout, client wrappers, shared lib) while static assets and global styles live in `public/` and `app/globals.css`. Portfolio metadata lives under `projects/` by discipline, each leaf containing `project.json` plus README for auto-discovery. Python prototypes live in `backend/`, automation in `scripts/`, and deployment or workflow references in `docs/`. Tests live in `tests/` with `tests/setupTests.ts` wiring shared mocks.

## Build, Test, and Development Commands
- `npm run dev` – Start the Next.js dev server with hot reload.
- `npm run api` – Launch `backend/app.py` for local API experiments.
- `npm run build` – Generate stills via `scripts/generate-stills.js`, then compile the production bundle.
- `npm start` – Serve the compiled build; run before `deploy.sh`.
- `npm test` / `npm run test:watch` – Run Vitest suites once or continuously.
- `npm run test:visual` / `npm run test:visual:update` – Build, run Playwright, and optionally refresh baselines.

## Coding Style & Naming Conventions
Write TypeScript (`.ts/.tsx`) with 2-space indentation, single quotes, and early returns to keep components shallow. Components/hooks use PascalCase and camelCase (e.g., `ClientWrapper`, `useHeliosphereData`), while utilities favor kebab-case filenames inside `app/lib/`. Prefer Tailwind utility classes over bespoke CSS and isolate browser-only logic inside client components.

## Testing Guidelines
Vitest plus Testing Library cover units and domain models; place specs next to the subject or under `tests/components/*.test.tsx` using the `{name}.test.ts` pattern. Keep integration flows within `tests/integration/`, leaning on `tests/setupTests.ts` for global mocks and clock control. Target ≥80% statement coverage for new features and add regression tests whenever closing an issue. Run Playwright visual suites before UI-heavy merges and mention notable diffs in the PR.

## Commit & Pull Request Guidelines
Follow the existing history format (`Fix: …`, `Refactor: …`) and reference issues inline (`Issue #20`) so worktree automation can track context. Keep commits focused, include updated tests, and avoid mixing frontend, backend, and content changes unless linked. PRs need a concise summary, verification notes (`npm test`, `npm run test:visual`), linked issues, and screenshots or short clips for UI adjustments.

## Security & Configuration Tips
Load secrets through `.env` as described in `config/README.md`, and run `scripts/setup.sh` once per machine to install required tooling. Never commit credentials, static tokens, or generated `out/` artifacts; extend `.gitignore` if something new appears. When editing deployment or DNS scripts, dry-run from a feature worktree and capture output to share in the PR.
