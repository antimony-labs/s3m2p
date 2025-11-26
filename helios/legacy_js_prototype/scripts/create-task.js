#!/usr/bin/env node
/**
 * GitHub Issue and PR Creation Script
 * 
 * Usage:
 *   node scripts/create-task.js "Fix button issue" "The Layers button is not working"
 *   node scripts/create-task.js "Add feature" "Description" --labels "enhancement,feature"
 *   node scripts/create-task.js --issue 123 --create-pr
 */

const { Octokit } = require('@octokit/rest');

const GITHUB_TOKEN = process.env.GITHUB_TOKEN;
const REPO_OWNER = 'Shivam-Bhardwaj';
const REPO_NAME = 'too.foo';

if (!GITHUB_TOKEN) {
  console.error('❌ Error: GITHUB_TOKEN environment variable is required');
  console.error('💡 Create a token at: https://github.com/settings/tokens');
  console.error('💡 Then run: export GITHUB_TOKEN=your_token_here');
  process.exit(1);
}

const octokit = new Octokit({ auth: GITHUB_TOKEN });

async function createIssue(title, description, labels = ['enhancement']) {
  try {
    const issue = await octokit.rest.issues.create({
      owner: REPO_OWNER,
      repo: REPO_NAME,
      title,
      body: description,
      labels: Array.isArray(labels) ? labels : labels.split(',').map(l => l.trim())
    });

    console.log(`✅ Issue created: #${issue.data.number}`);
    console.log(`🔗 ${issue.data.html_url}`);
    return issue.data.number;
  } catch (error) {
    console.error('❌ Error creating issue:', error.message);
    throw error;
  }
}

async function createPR(issueNumber, branchName = null) {
  try {
    if (!branchName) {
      branchName = `issue-${issueNumber}`;
    }

    // Get issue details
    const issue = await octokit.rest.issues.get({
      owner: REPO_OWNER,
      repo: REPO_NAME,
      issue_number: issueNumber
    });

    const pr = await octokit.rest.pulls.create({
      owner: REPO_OWNER,
      repo: REPO_NAME,
      title: `Fix: ${issue.data.title}`,
      head: branchName,
      base: 'main',
      body: `## Related Issue
      
Closes #${issueNumber}

## Description

${issue.data.body || 'No description provided'}

## Changes

- [ ] Implementation
- [ ] Tests
- [ ] Documentation

---
*This PR was automatically created from issue #${issueNumber}*`
    });

    console.log(`✅ PR created: #${pr.data.number}`);
    console.log(`🔗 ${pr.data.html_url}`);
    return pr.data.number;
  } catch (error) {
    console.error('❌ Error creating PR:', error.message);
    throw error;
  }
}

async function main() {
  const args = process.argv.slice(2);

  if (args.includes('--help') || args.includes('-h')) {
    console.log(`
GitHub Issue and PR Creation Script

Usage:
  node scripts/create-task.js <title> <description> [--labels "label1,label2"]
  node scripts/create-task.js --issue <number> --create-pr [--branch <branch-name>]

Examples:
  node scripts/create-task.js "Fix button" "The Layers button is not working"
  node scripts/create-task.js "Add feature" "Description" --labels "enhancement,feature"
  node scripts/create-task.js --issue 123 --create-pr
    `);
    return;
  }

  // Check if creating PR from existing issue
  const issueIndex = args.indexOf('--issue');
  if (issueIndex !== -1) {
    const issueNumber = parseInt(args[issueIndex + 1]);
    if (args.includes('--create-pr')) {
      const branchIndex = args.indexOf('--branch');
      const branchName = branchIndex !== -1 ? args[branchIndex + 1] : null;
      await createPR(issueNumber, branchName);
    } else {
      console.log('💡 Use --create-pr flag to create a PR from this issue');
    }
    return;
  }

  // Create new issue
  if (args.length < 2) {
    console.error('❌ Error: Title and description are required');
    console.error('💡 Usage: node scripts/create-task.js <title> <description>');
    process.exit(1);
  }

  const title = args[0];
  const description = args[1];
  const labelsIndex = args.indexOf('--labels');
  const labels = labelsIndex !== -1 ? args[labelsIndex + 1].split(',') : ['enhancement'];

  const issueNumber = await createIssue(title, description, labels);
  
  console.log(`\n💡 To create a PR for this issue, run:`);
  console.log(`   node scripts/create-task.js --issue ${issueNumber} --create-pr`);
}

main().catch(console.error);

