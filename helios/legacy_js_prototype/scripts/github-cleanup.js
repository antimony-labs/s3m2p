#!/usr/bin/env node
/**
 * GitHub cleanup utility: issues and PRs
 * - Marks/Closes stale items
 * - Closes exact-duplicate issues by title
 * - Labels items for triage
 *
 * Usage:
 *   node scripts/github-cleanup.js                # dry run
 *   node scripts/github-cleanup.js --apply        # perform actions
 *   node scripts/github-cleanup.js --owner o --repo r --days 60 --apply
 */

const { Octokit } = require('@octokit/rest');

const APPLY = process.argv.includes('--apply');
const ARG = (name, def = undefined) => {
  const i = process.argv.indexOf(name);
  return i !== -1 ? process.argv[i + 1] : def;
};

const GITHUB_TOKEN = process.env.GITHUB_TOKEN || process.env.GH_TOKEN;
if (!GITHUB_TOKEN) {
  console.error('❌ GITHUB_TOKEN (or GH_TOKEN) not set');
  process.exit(1);
}

async function main() {
  const { owner, repo } = await detectRepo();
  const octokit = new Octokit({ auth: GITHUB_TOKEN });
  const staleDays = parseInt(ARG('--days', '60'), 10);
  const now = new Date();
  const cutoff = new Date(now.getTime() - staleDays * 24 * 60 * 60 * 1000);

  console.log(`🔎 Auditing ${owner}/${repo}`);

  // Ensure useful labels exist
  await ensureLabels(octokit, owner, repo, [
    { name: 'stale', color: 'bdbdbd', description: 'No recent activity' },
    { name: 'duplicate', color: 'cccccc', description: 'Duplicate of another issue' },
    { name: 'triage', color: 'fbca04', description: 'Needs triage' },
    { name: 'in-progress', color: '1D76DB', description: 'Being worked on' },
    { name: 'blocked', color: 'b60205', description: 'Blocked by external factor' },
    { name: 'ready', color: '0E8A16', description: 'Ready for review/merge' }
  ]);

  // Issues audit
  const issues = await paginate(octokit, octokit.rest.issues.listForRepo, {
    owner, repo, state: 'open', per_page: 100
  });
  console.log(`• Open issues: ${issues.length}`);

  const normalizedTitle = (t) => t.toLowerCase().replace(/\s+/g, ' ').trim();
  const titleMap = new Map();
  for (const is of issues) {
    const k = normalizedTitle(is.title);
    if (!titleMap.has(k)) titleMap.set(k, []);
    titleMap.get(k).push(is);
  }

  const duplicates = Array.from(titleMap.values()).filter(arr => arr.length > 1);
  const staleIssues = issues.filter(i => !hasLabel(i, 'pinned') && !hasLabel(i, 'keep') && new Date(i.updated_at) < cutoff);

  // PRs audit
  const prs = await paginate(octokit, octokit.rest.pulls.list, {
    owner, repo, state: 'open', per_page: 100
  });
  console.log(`• Open PRs: ${prs.length}`);
  const stalePRs = prs.filter(p => !p.draft && new Date(p.updated_at) < cutoff);
  const oldDraftPRs = prs.filter(p => p.draft && new Date(p.updated_at) < cutoff);
  const nonIssueBranchPRs = prs.filter(p => !/^issue-\d+/.test(p.head.ref));

  console.log('\nSummary (dry-run unless --apply):');
  console.log(`  - Stale issues (>${staleDays}d): ${staleIssues.length}`);
  console.log(`  - Duplicate issue groups: ${duplicates.length}`);
  console.log(`  - Stale PRs (>${staleDays}d): ${stalePRs.length}`);
  console.log(`  - Old draft PRs (>${staleDays}d): ${oldDraftPRs.length}`);
  console.log(`  - PRs not following issue-<n> branch naming: ${nonIssueBranchPRs.length}`);

  if (!APPLY) return;

  // Close older duplicates, keep the most recently updated
  for (const group of duplicates) {
    const sorted = group.slice().sort((a, b) => new Date(b.updated_at) - new Date(a.updated_at));
    const keep = sorted[0];
    for (const dupe of sorted.slice(1)) {
      await labelAndCloseIssue(octokit, owner, repo, dupe.number, 'duplicate', `Closing as duplicate of #${keep.number}`);
    }
  }

  // Mark and close stale issues
  for (const is of staleIssues) {
    await labelAndCloseIssue(octokit, owner, repo, is.number, 'stale', `Closing due to inactivity (> ${staleDays} days). If this is still relevant, please reopen or create a fresh issue with current context.`);
  }

  // Close stale PRs
  for (const pr of stalePRs) {
    await closePrWithComment(octokit, owner, repo, pr.number, `Closing due to inactivity (> ${staleDays} days). Please reopen when ready.`);
  }

  // Close very old drafts
  for (const pr of oldDraftPRs) {
    await closePrWithComment(octokit, owner, repo, pr.number, `Closing draft PR due to inactivity (> ${staleDays} days).`);
  }

  // Label non-conforming PR branches
  for (const pr of nonIssueBranchPRs) {
    await addLabels(octokit, owner, repo, pr.number, ['triage']);
  }

  console.log('✅ Cleanup applied.');
}

async function detectRepo() {
  const owner = ARG('--owner');
  const repo = ARG('--repo');
  if (owner && repo) return { owner, repo };
  // Fallback to parsing origin
  const { execSync } = require('child_process');
  const url = execSync('git remote get-url origin', { encoding: 'utf-8' }).trim();
  let m;
  if ((m = url.match(/github\.com[:\/](.+)\/([^\/]+)\.git$/))) {
    return { owner: m[1], repo: m[2] };
  }
  throw new Error('Cannot detect repo. Provide --owner and --repo.');
}

async function paginate(octokit, method, params) {
  const all = [];
  let page = 1;
  while (true) {
    const res = await method({ ...params, page });
    const items = res.data || [];
    all.push(...items);
    if (items.length < (params.per_page || 30)) break;
    page++;
  }
  return all;
}

function hasLabel(item, name) {
  const labels = item.labels || [];
  return labels.some(l => (typeof l === 'string' ? l : l.name) === name);
}

async function ensureLabels(octokit, owner, repo, labels) {
  for (const l of labels) {
    try {
      await octokit.rest.issues.getLabel({ owner, repo, name: l.name });
    } catch (_) {
      try {
        await octokit.rest.issues.createLabel({ owner, repo, name: l.name, color: l.color, description: l.description });
      } catch (e) {
        // ignore
      }
    }
  }
}

async function addLabels(octokit, owner, repo, number, labels) {
  try {
    await octokit.rest.issues.addLabels({ owner, repo, issue_number: number, labels });
  } catch (_) {}
}

async function labelAndCloseIssue(octokit, owner, repo, number, label, comment) {
  await addLabels(octokit, owner, repo, number, [label]);
  await octokit.rest.issues.createComment({ owner, repo, issue_number: number, body: comment });
  await octokit.rest.issues.update({ owner, repo, issue_number: number, state: 'closed' });
}

async function closePrWithComment(octokit, owner, repo, number, comment) {
  await octokit.rest.issues.createComment({ owner, repo, issue_number: number, body: comment });
  await octokit.rest.pulls.update({ owner, repo, pull_number: number, state: 'closed' });
}

main().catch(err => { console.error('❌', err.message); process.exit(1); });

