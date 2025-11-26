#!/usr/bin/env node
/**
 * Work on Existing Issue → Branch + Worktree + PR (if missing)
 *
 * Usage:
 *   node scripts/work-on-issue.js 123
 *   node scripts/work-on-issue.js https://github.com/<owner>/<repo>/issues/123
 *   npm run issue -- 123
 */

const { Octokit } = require('@octokit/rest');
const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');
const os = require('os');

const GITHUB_TOKEN = process.env.GITHUB_TOKEN || process.env.GH_TOKEN;

function getRemoteRepo() {
  const url = execSync('git remote get-url origin', { encoding: 'utf-8' }).trim();
  // Handle SSH and HTTPS remotes
  // Examples:
  //   git@github.com:owner/repo.git
  //   https://github.com/owner/repo.git
  let owner, repo;
  if (url.startsWith('git@')) {
    const m = url.match(/github\.com:(.+)\/(.+)\.git$/);
    if (m) { owner = m[1]; repo = m[2]; }
  } else if (url.startsWith('https://')) {
    const m = url.match(/github\.com\/(.+)\/(.+)\.git$/);
    if (m) { owner = m[1]; repo = m[2]; }
  }
  if (!owner || !repo) {
    throw new Error(`Unable to parse origin remote: ${url}`);
  }
  return { owner, repo };
}

function parseIssueArg(arg) {
  // Accept: number, #number, owner/repo#number, URL
  if (!arg) throw new Error('Issue number or URL is required');
  if (/^#?\d+$/.test(arg)) return parseInt(arg.replace('#', ''), 10);
  // owner/repo#123
  const shard = arg.match(/^[^\s#]+\/[^\s#]+#(\d+)$/);
  if (shard) return parseInt(shard[1], 10);
  // URL
  try {
    const u = new URL(arg);
    const parts = u.pathname.split('/').filter(Boolean); // [owner, repo, 'issues', '123']
    if (parts[2] === 'issues' && /^\d+$/.test(parts[3])) {
      return parseInt(parts[3], 10);
    }
  } catch (_) {}
  throw new Error(`Unrecognized issue reference: ${arg}`);
}

function getMainRepoPath() {
  try {
    const gitDir = execSync('git rev-parse --git-dir', { encoding: 'utf-8' }).trim();
    if (gitDir.includes('/worktrees/')) {
      const m = gitDir.match(/^(.+)\/\.git\/worktrees\/[^\/]+$/);
      if (m) return m[1];
    }
    return execSync('git rev-parse --show-toplevel', { encoding: 'utf-8' }).trim();
  } catch (e) {
    throw new Error(`Cannot determine main repository path: ${e.message}`);
  }
}

async function ensureBranchAndWorktree({ owner, repo }, issueNumber, mainRepoPath) {
  const branchName = `issue-${issueNumber}`;
  const WORKTREE_BASE = process.env.WORKTREE_BASE || path.join(os.homedir(), '.cursor', 'worktrees', repo);
  const worktreePath = path.join(WORKTREE_BASE, branchName);

  // Fetch and base branch
  try {
    execSync('git fetch origin', { cwd: mainRepoPath, stdio: 'inherit' });
  } catch (_) {}

  // Determine default branch (main/master)
  let defaultBranch = 'main';
  try {
    execSync('git show-ref --verify --quiet refs/remotes/origin/main', { cwd: mainRepoPath });
    defaultBranch = 'main';
  } catch (_) {
    defaultBranch = 'master';
  }

  // Create/reset local branch from default
  execSync(`git checkout -B ${branchName} origin/${defaultBranch}`, { cwd: mainRepoPath, stdio: 'inherit' });

  // Ensure worktree
  if (!fs.existsSync(worktreePath)) {
    fs.mkdirSync(worktreePath, { recursive: true });
    execSync(`git worktree add ${worktreePath} ${branchName}`, { cwd: mainRepoPath, stdio: 'inherit' });
  }

  // Install commit hooks into main repo (shared by worktrees)
  try {
    execSync('bash scripts/setup-git-hooks.sh', { cwd: mainRepoPath, stdio: 'inherit' });
  } catch (_) {}

  return { branchName, worktreePath };
}

async function ensurePrForBranch(octokit, { owner, repo }, issue, branchName) {
  // Check existing PRs from branch
  const headRef = `${owner}:${branchName}`;
  const existing = await octokit.rest.pulls.list({ owner, repo, state: 'open', head: headRef });
  if (existing.data && existing.data.length > 0) {
    return existing.data[0];
  }
  // Create PR
  const pr = await octokit.rest.pulls.create({
    owner,
    repo,
    title: `Fix: ${issue.title}`,
    head: branchName,
    base: issue.pull_request && issue.pull_request.base ? issue.pull_request.base.ref : 'main',
    body: `Closes #${issue.number}\n\nCreated automatically for local worktree-based development.`
  });
  return pr.data;
}

async function main() {
  const arg = process.argv[2];
  if (!GITHUB_TOKEN) {
    console.error('❌ Error: GITHUB_TOKEN or GH_TOKEN must be set in env');
    process.exit(1);
  }
  if (!arg) {
    console.error('Usage: node scripts/work-on-issue.js <issue-number|url>');
    process.exit(1);
  }

  const issueNumber = parseIssueArg(arg);
  const { owner, repo } = getRemoteRepo();
  const octokit = new Octokit({ auth: GITHUB_TOKEN });

  console.log(`📋 Issue: #${issueNumber} (${owner}/${repo})`);

  // Fetch issue details
  const issue = (await octokit.rest.issues.get({ owner, repo, issue_number: issueNumber })).data;

  const mainRepoPath = getMainRepoPath();
  const { branchName, worktreePath } = await ensureBranchAndWorktree({ owner, repo }, issueNumber, mainRepoPath);

  // Push branch upstream if missing to allow PR creation
  try {
    execSync(`git push -u origin ${branchName}`, { cwd: mainRepoPath, stdio: 'inherit' });
  } catch (e) {
    // Ignore push failures (already exists, etc.)
  }

  // PR
  const pr = await ensurePrForBranch(octokit, { owner, repo }, issue, branchName);

  // Save workflow metadata
  const info = {
    issueNumber: issue.number,
    issueUrl: issue.html_url,
    branchName,
    worktreePath,
    prNumber: pr.number,
    prUrl: pr.html_url
  };

  const workflowFile = path.join(worktreePath, '.cursor-workflow.json');
  fs.writeFileSync(workflowFile, JSON.stringify(info, null, 2));

  console.log('\n✅ Ready to work!');
  console.log(`🌳 Worktree: ${worktreePath}`);
  console.log(`🔀 PR: #${pr.number} - ${pr.html_url}`);
  console.log('\nNext:');
  console.log(`  cd ${worktreePath}`);
  console.log('  npm install  # if first time in this worktree');
  console.log(`  git add . && git commit -m "Fix: ${issue.title}" && git push`);
}

main().catch(err => { console.error('❌', err.message); process.exit(1); });

