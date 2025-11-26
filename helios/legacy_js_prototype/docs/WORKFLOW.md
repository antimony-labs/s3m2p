# Local-First Workflow (GitHub as VCS Only)

This repository treats GitHub as a version control service only. All tests and slow checks run locally on the server, and pushes are blocked if checks fail.

## Day-to-day Flow

- Create an issue (or paste an existing issue link/number)
- Start working from a dedicated worktree
- Commit with metadata, push, and open/continue the PR

### Start from an existing issue

```
# Using an issue number
npm run issue -- 123

# Or using an issue URL
npm run issue -- https://github.com/<owner>/<repo>/issues/123
```

This will:
- Create/reset a branch `issue-123` from `main`
- Create a git worktree under `~/.cursor/worktrees/too.foo/issue-123`
- Create a PR if one doesn’t already exist
- Save `.cursor-workflow.json` in the worktree with quick links

### Work in the worktree

```
cd ~/.cursor/worktrees/too.foo/issue-123
npm install   # first time in this worktree
# make changes…
git add . && git commit -m "Fix: concise description" && git push
```

## Local Checks (pre-push)

Pre-push hooks run consolidated checks:
- Unit/integration tests (`npm test`)
- Production build (`npm run build`)
- Optional visual tests if `ENABLE_VISUAL_TESTS=1`

To skip (not recommended): `SKIP_PREPUSH_CHECKS=1 git push`

## Visual Snapshot Policy

- Do not merge visual snapshot updates into `main` directly.
- Keep Playwright baselines under `tests/visual/__screenshots__/` on feature branches until reviewed.
- The pre-push hook blocks pushing snapshot changes on `main`.

## Merge PRs When Satisfied

Run local gates on the PR and merge via squash:

```
npm run merge-pr -- <pr-number>

# Include visual tests (optional)
ENABLE_VISUAL_TESTS=1 npm run merge-pr -- <pr-number> --visual
```

If the PR modifies visual baselines, the merge is blocked by default (set `ALLOW_SNAPSHOT_MERGE=1` to override).

## One-time Setup

```
export GITHUB_TOKEN=your_token
npm install
bash scripts/setup-git-hooks.sh
```

## Utilities

- List/cleanup worktrees: `npm run worktrees` and `node scripts/worktree.js cleanup`
- Create issue+PR+worktree from description: `npm run task "Your task description"`
