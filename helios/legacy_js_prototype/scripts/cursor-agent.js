#!/usr/bin/env node
/**
 * Cursor Agent - Automated Issue → PR Workflow with Worktrees
 * 
 * This script automates the entire workflow using git worktrees:
 * 1. Creates GitHub issue from description
 * 2. Creates branch
 * 3. Creates worktree for the branch
 * 4. Creates PR
 * 5. Returns worktree path for you to work on
 * 
 * Usage in Cursor:
 *   node scripts/cursor-agent.js "Fix the Layers button - it's not working"
 * 
 * This allows multiple agents to work on different changes simultaneously.
 */

const { Octokit } = require('@octokit/rest');
const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');
const os = require('os');

const GITHUB_TOKEN = process.env.GITHUB_TOKEN;
const REPO_OWNER = 'Shivam-Bhardwaj';
const REPO_NAME = 'too.foo';

// Worktree base directory
const WORKTREE_BASE = process.env.WORKTREE_BASE || path.join(os.homedir(), '.cursor', 'worktrees', REPO_NAME);

if (!GITHUB_TOKEN) {
  console.error('❌ Error: GITHUB_TOKEN environment variable is required');
  console.error('💡 Create a token at: https://github.com/settings/tokens');
  console.error('💡 Then run: export GITHUB_TOKEN=your_token_here');
  process.exit(1);
}

const octokit = new Octokit({ auth: GITHUB_TOKEN });

// Get the main repo path from current worktree
function getMainRepoPath() {
  try {
    const gitDir = execSync('git rev-parse --git-dir', { encoding: 'utf-8' }).trim();
    // If we're in a worktree, git-dir points to .git/worktrees/{name}
    if (gitDir.includes('/worktrees/')) {
      const worktreeMatch = gitDir.match(/^(.+)\/\.git\/worktrees\/[^\/]+$/);
      if (worktreeMatch) {
        return worktreeMatch[1];
      }
    }
    // Otherwise, we're in the main repo
    return execSync('git rev-parse --show-toplevel', { encoding: 'utf-8' }).trim();
  } catch (error) {
    console.error('❌ Error finding main repository:', error.message);
    process.exit(1);
  }
}

async function findExistingIssue(title) {
  const sanitizedTitle = title.replace(/"/g, '\\"');
  const query = `repo:${REPO_OWNER}/${REPO_NAME} is:issue is:open in:title "${sanitizedTitle}"`;

  try {
    const { data } = await octokit.rest.search.issuesAndPullRequests({ q: query });
    const match = data.items.find((item) => item.title === title && item.state === 'open');

    if (match) {
      const issue = await octokit.rest.issues.get({
        owner: REPO_OWNER,
        repo: REPO_NAME,
        issue_number: match.number,
      });
      console.log(`🔄 Reusing existing issue: #${issue.data.number}`);
      console.log(`   ${issue.data.html_url}`);
      return issue.data;
    }
  } catch (error) {
    console.warn('⚠️  Failed to search for existing issues:', error.message);
  }

  return null;
}

async function createIssue(title, description) {
  try {
    const existing = await findExistingIssue(title);
    if (existing) {
      return existing;
    }

    const issue = await octokit.rest.issues.create({
      owner: REPO_OWNER,
      repo: REPO_NAME,
      title: title,
      body: description,
      labels: ['enhancement', 'automated']
    });

    console.log(`✅ Issue created: #${issue.data.number}`);
    console.log(`🔗 ${issue.data.html_url}`);
    return issue.data;
  } catch (error) {
    console.error('❌ Error creating issue:', error.message);
    throw error;
  }
}

async function createBranchAndWorktree(issueNumber, mainRepoPath) {
  const branchName = `issue-${issueNumber}`;
  const worktreePath = path.join(WORKTREE_BASE, branchName);
  
  try {
    // Fetch latest
    console.log('📥 Fetching latest from origin...');
    execSync('git fetch origin', { cwd: mainRepoPath, stdio: 'inherit' });
    
    // Check if branch already exists remotely
    let branchExists = false;
    try {
      execSync(`git show-ref --verify --quiet refs/remotes/origin/${branchName}`, { 
        cwd: mainRepoPath, 
        stdio: 'ignore' 
      });
      branchExists = true;
      console.log(`📦 Branch ${branchName} exists remotely, checking it out...`);
    } catch (e) {
      // Branch doesn't exist, create it
    }
    
    // Ensure we're on main/master in main repo
    try {
      execSync('git checkout main', { cwd: mainRepoPath, stdio: 'inherit' });
    } catch (e) {
      try {
        execSync('git checkout master', { cwd: mainRepoPath, stdio: 'inherit' });
      } catch (e2) {
        console.log('⚠️  Could not checkout main/master');
      }
    }
    
    // Pull latest changes
    try {
      execSync('git pull origin main 2>/dev/null || git pull origin master 2>/dev/null', { 
        cwd: mainRepoPath, 
        stdio: 'inherit' 
      });
    } catch (e) {
      // Ignore pull errors
    }
    
    // Create branch if it doesn't exist
    if (!branchExists) {
      execSync(`git checkout -b ${branchName}`, { cwd: mainRepoPath, stdio: 'inherit' });
      console.log(`✅ Branch created: ${branchName}`);
    } else {
      execSync(`git checkout ${branchName}`, { cwd: mainRepoPath, stdio: 'inherit' });
      execSync(`git pull origin ${branchName}`, { cwd: mainRepoPath, stdio: 'inherit' });
    }
    
    // Check if worktree already exists
    if (fs.existsSync(worktreePath)) {
      console.log(`⚠️  Worktree already exists at: ${worktreePath}`);
      console.log(`💡 Using existing worktree`);
      return { branchName, worktreePath };
    }
    
    // Create worktree directory
    fs.mkdirSync(worktreePath, { recursive: true });
    
    // Create worktree
    console.log(`🌳 Creating worktree at: ${worktreePath}`);
    execSync(`git worktree add ${worktreePath} ${branchName}`, { 
      cwd: mainRepoPath, 
      stdio: 'inherit' 
    });
    
    console.log(`✅ Worktree created: ${worktreePath}`);
    return { branchName, worktreePath };
  } catch (error) {
    console.error('❌ Error creating branch/worktree:', error.message);
    throw error;
  }
}

async function createPR(issueNumber, branchName, issueTitle, issueBody) {
  try {
    const pr = await octokit.rest.pulls.create({
      owner: REPO_OWNER,
      repo: REPO_NAME,
      title: `Fix: ${issueTitle}`,
      head: branchName,
      base: 'main',
      body: `## Related Issue
      
Closes #${issueNumber}

## Description

${issueBody}

## Status

- [ ] Implementation
- [ ] Ready for review

---
*This PR was automatically created from issue #${issueNumber}*

**Next steps:**
1. Work on the changes in the worktree
2. Commit and push your changes
3. Vercel will automatically create a preview deployment
4. Test in the preview URL
5. Merge when ready for production`
    });

    console.log(`✅ PR created: #${pr.data.number}`);
    console.log(`🔗 ${pr.data.html_url}`);
    
    // Link PR to issue
    await octokit.rest.issues.createComment({
      owner: REPO_OWNER,
      repo: REPO_NAME,
      issue_number: issueNumber,
      body: `🔗 Related PR: #${pr.data.number} - ${pr.data.html_url}`
    });
    
    return pr.data;
  } catch (error) {
    console.error('❌ Error creating PR:', error.message);
    throw error;
  }
}

async function main() {
  const args = process.argv.slice(2);
  
  if (args.length === 0 || args.includes('--help') || args.includes('-h')) {
    console.log(`
Cursor Agent - Automated Issue → PR Workflow with Worktrees

Usage:
  node scripts/cursor-agent.js "<task description>"

Example:
  node scripts/cursor-agent.js "Fix the Layers button - it's not working"

What it does:
  1. Creates GitHub issue
  2. Creates branch (issue-{number})
  3. Creates worktree for the branch
  4. Creates PR
  5. Returns worktree path for you to work on

After running:
  - Work in the worktree directory
  - Commit: git commit -m "Fix: description"
  - Push: git push origin issue-{number}
  - Vercel will create preview automatically

Environment variables:
  WORKTREE_BASE - Base directory for worktrees (default: ~/.cursor/worktrees/too.foo)
    `);
    return;
  }

  const description = args.join(' ');
  const title = description.length > 50 ? description.substring(0, 47) + '...' : description;

  console.log('🚀 Starting automated workflow with worktrees...\n');
  console.log(`📝 Task: ${description}\n`);

  try {
    const mainRepoPath = getMainRepoPath();
    console.log(`📁 Main repo: ${mainRepoPath}`);
    console.log(`🌳 Worktree base: ${WORKTREE_BASE}\n`);

    // Step 1: Create issue
    console.log('📋 Step 1: Creating issue...');
    const issue = await createIssue(title, description);
    console.log('');

    // Step 2: Create branch and worktree
    console.log('🌿 Step 2: Creating branch and worktree...');
    const { branchName, worktreePath } = await createBranchAndWorktree(issue.number, mainRepoPath);
    console.log('');

    // Step 3: Create PR
    console.log('🔀 Step 3: Creating PR...');
    const pr = await createPR(issue.number, branchName, issue.title, issue.body);
    console.log('');

    // Summary
    console.log('\n' + '='.repeat(60));
    console.log('✅ Workflow Complete!');
    console.log('='.repeat(60));
    console.log(`\n📋 Issue: #${issue.number} - ${issue.html_url}`);
    console.log(`🌿 Branch: ${branchName}`);
    console.log(`🌳 Worktree: ${worktreePath}`);
    console.log(`🔀 PR: #${pr.number} - ${pr.html_url}`);
    console.log(`\n💡 Work in the worktree directory:`);
    console.log(`   cd ${worktreePath}`);
    console.log(`\n💡 Make your changes, then:`);
    console.log(`   git add .`);
    console.log(`   git commit -m "Fix: ${title}"`);
    console.log(`   git push origin ${branchName}`);
    console.log(`\n🚀 Vercel will automatically create a preview deployment!`);
    console.log(`🔗 Preview URL will be available in the PR comments or Vercel dashboard`);
    
    // Save info to file for Cursor to use
    const info = {
      issueNumber: issue.number,
      issueUrl: issue.html_url,
      branchName: branchName,
      worktreePath: worktreePath,
      prNumber: pr.number,
      prUrl: pr.html_url,
      description: description,
      mainRepoPath: mainRepoPath
    };
    
    const workflowFile = path.join(worktreePath, '.cursor-workflow.json');
    fs.writeFileSync(workflowFile, JSON.stringify(info, null, 2));
    
    // Also save in main repo for reference
    const mainWorkflowFile = path.join(mainRepoPath, `.cursor-workflow-${issue.number}.json`);
    fs.writeFileSync(mainWorkflowFile, JSON.stringify(info, null, 2));
    
    console.log(`\n📄 Workflow info saved to:`);
    console.log(`   ${workflowFile}`);
    console.log(`   ${mainWorkflowFile}`);
    
  } catch (error) {
    console.error('\n❌ Workflow failed:', error.message);
    process.exit(1);
  }
}

main().catch(console.error);
