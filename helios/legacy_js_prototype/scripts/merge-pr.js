#!/usr/bin/env node
/**
 * Merge PR with local gate checks and snapshot guard
 *
 * Usage:
 *   node scripts/merge-pr.js <pr-number> [--visual]
 *
 * Behavior:
 *   - Creates a temporary worktree for the PR head
 *   - Runs local checks (tests + build). If --visual or ENABLE_VISUAL_TESTS=1, runs visual tests
 *   - Blocks merge if visual snapshots changed vs main (keeps them on branch)
 *   - Merges via squash on success
 */

const { Octokit } = require('@octokit/rest');
const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');
const os = require('os');

const GITHUB_TOKEN = process.env.GITHUB_TOKEN || process.env.GH_TOKEN;

if (!GITHUB_TOKEN) {
  console.error('❌ GITHUB_TOKEN or GH_TOKEN not set');
  process.exit(1);
}

const prNumber = parseInt(process.argv[2], 10);
if (!prNumber) {
  console.error('Usage: node scripts/merge-pr.js <pr-number> [--visual]');
  process.exit(1);
}

const RUN_VISUAL = process.argv.includes('--visual') || process.env.ENABLE_VISUAL_TESTS === '1';

function getRemoteRepo() {
  const url = execSync('git remote get-url origin', { encoding: 'utf-8' }).trim();
  let m;
  if ((m = url.match(/github\.com[:\/](.+)\/([^\/]+)\.git$/))) {
    return { owner: m[1], repo: m[2] };
  }
  throw new Error('Cannot parse origin remote');
}

function getMainRepoPath() {
  const gitDir = execSync('git rev-parse --git-dir', { encoding: 'utf-8' }).trim();
  if (gitDir.includes('/worktrees/')) {
    const m = gitDir.match(/^(.+)\/\.git\/worktrees\/[^\/]+$/);
    if (m) return m[1];
  }
  return execSync('git rev-parse --show-toplevel', { encoding: 'utf-8' }).trim();
}

async function main() {
  const { owner, repo } = getRemoteRepo();
  const octokit = new Octokit({ auth: GITHUB_TOKEN });

  // Get PR details
  const { data: pr } = await octokit.rest.pulls.get({ owner, repo, pull_number: prNumber });
  const headRef = pr.head.ref; // branch name
  const worktreeBase = path.join(os.homedir(), '.cursor', 'worktrees', repo);
  const worktreePath = path.join(worktreeBase, `merge-pr-${prNumber}`);
  const mainRepoPath = getMainRepoPath();

  // Prepare branch in a worktree
  fs.mkdirSync(worktreePath, { recursive: true });
  execSync('git fetch origin', { cwd: mainRepoPath, stdio: 'inherit' });
  try { execSync(`git worktree remove ${worktreePath} --force`, { cwd: mainRepoPath, stdio: 'ignore' }); } catch (_) {}
  execSync(`git worktree add ${worktreePath} ${headRef}`, { cwd: mainRepoPath, stdio: 'inherit' });

  // Run local checks in the worktree
  const env = { ...process.env };
  if (RUN_VISUAL) env.ENABLE_VISUAL_TESTS = '1';
  execSync('bash scripts/run-checks.sh', { cwd: worktreePath, stdio: 'inherit', env });

  // Snapshot guard: disallow merging visual snapshots to main by default
  execSync('git fetch origin main', { cwd: worktreePath, stdio: 'inherit' });
  const changed = execSync('bash -lc "git diff --name-only origin/main...HEAD | rg ^tests/visual/__screenshots__/ || true"', { cwd: worktreePath, encoding: 'utf-8' }).trim();
  if (changed) {
    console.error('❌ Visual snapshot updates detected in this PR. Keep them in the branch.');
    console.error('   Files:\n' + changed);
    console.error('   If this is intentional, set ALLOW_SNAPSHOT_MERGE=1 and rerun.');
    if (process.env.ALLOW_SNAPSHOT_MERGE !== '1') {
      process.exit(1);
    }
  }

  // Merge PR via squash
  console.log('✅ Local checks passed. Merging PR...');
  await octokit.rest.pulls.merge({
    owner,
    repo,
    pull_number: prNumber,
    merge_method: 'squash'
  });

  console.log('🎉 PR merged successfully.');

  // Cleanup worktree
  try { execSync(`git worktree remove ${worktreePath} --force`, { cwd: mainRepoPath, stdio: 'ignore' }); } catch (_) {}
}

main().catch(err => { console.error('❌', err.message); process.exit(1); });

