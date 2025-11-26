#!/usr/bin/env node
/**
 * Worktree Management Script
 * 
 * List, clean up, and manage git worktrees
 */

const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');
const os = require('os');

const REPO_NAME = 'too.foo';
const WORKTREE_BASE = process.env.WORKTREE_BASE || path.join(os.homedir(), '.cursor', 'worktrees', REPO_NAME);

function getMainRepoPath() {
  try {
    const gitDir = execSync('git rev-parse --git-dir', { encoding: 'utf-8' }).trim();
    if (gitDir.includes('/worktrees/')) {
      const worktreeMatch = gitDir.match(/^(.+)\/\.git\/worktrees\/[^\/]+$/);
      if (worktreeMatch) {
        return worktreeMatch[1];
      }
    }
    return execSync('git rev-parse --show-toplevel', { encoding: 'utf-8' }).trim();
  } catch (error) {
    console.error('❌ Error finding main repository:', error.message);
    process.exit(1);
  }
}

function listWorktrees() {
  const mainRepoPath = getMainRepoPath();
  console.log('🌳 Active Worktrees:\n');
  
  try {
    const output = execSync('git worktree list', { cwd: mainRepoPath, encoding: 'utf-8' });
    console.log(output);
  } catch (error) {
    console.error('❌ Error listing worktrees:', error.message);
  }
  
  // Also list worktree directories
  if (fs.existsSync(WORKTREE_BASE)) {
    console.log('\n📁 Worktree Directories:');
    const dirs = fs.readdirSync(WORKTREE_BASE, { withFileTypes: true })
      .filter(d => d.isDirectory())
      .map(d => d.name);
    
    if (dirs.length === 0) {
      console.log('   (none)');
    } else {
      dirs.forEach(dir => {
        const dirPath = path.join(WORKTREE_BASE, dir);
        const workflowFile = path.join(dirPath, '.cursor-workflow.json');
        if (fs.existsSync(workflowFile)) {
          const info = JSON.parse(fs.readFileSync(workflowFile, 'utf-8'));
          console.log(`   ${dir} - Issue #${info.issueNumber} - ${info.prUrl}`);
        } else {
          console.log(`   ${dir}`);
        }
      });
    }
  }
}

function removeWorktree(branchName) {
  const mainRepoPath = getMainRepoPath();
  const worktreePath = path.join(WORKTREE_BASE, branchName);
  
  if (!fs.existsSync(worktreePath)) {
    console.error(`❌ Worktree not found: ${worktreePath}`);
    return;
  }
  
  try {
    console.log(`🗑️  Removing worktree: ${worktreePath}`);
    execSync(`git worktree remove ${worktreePath}`, { cwd: mainRepoPath, stdio: 'inherit' });
    console.log(`✅ Worktree removed`);
  } catch (error) {
    console.error('❌ Error removing worktree:', error.message);
    console.log('💡 Try: git worktree remove --force');
  }
}

function cleanupMerged() {
  const mainRepoPath = getMainRepoPath();
  console.log('🧹 Cleaning up merged worktrees...\n');
  
  try {
    const output = execSync('git worktree list', { cwd: mainRepoPath, encoding: 'utf-8' });
    const lines = output.split('\n').filter(l => l.trim());
    
    lines.forEach(line => {
      const match = line.match(/^(.+)\s+\[(.+)\]$/);
      if (match) {
        const worktreePath = match[1].trim();
        const branchName = match[2].trim();
        
        // Check if branch is merged
        try {
          execSync(`git branch --merged main | grep -q "^\\s*${branchName}$"`, { 
            cwd: mainRepoPath, 
            stdio: 'ignore' 
          });
          console.log(`✅ ${branchName} is merged, removing worktree...`);
          execSync(`git worktree remove ${worktreePath}`, { cwd: mainRepoPath, stdio: 'inherit' });
        } catch (e) {
          // Not merged or error
        }
      }
    });
  } catch (error) {
    console.error('❌ Error cleaning up:', error.message);
  }
}

function main() {
  const args = process.argv.slice(2);
  const command = args[0];

  switch (command) {
    case 'list':
    case 'ls':
      listWorktrees();
      break;
      
    case 'remove':
    case 'rm':
      if (!args[1]) {
        console.error('❌ Usage: node scripts/worktree.js remove <branch-name>');
        process.exit(1);
      }
      removeWorktree(args[1]);
      break;
      
    case 'cleanup':
      cleanupMerged();
      break;
      
    default:
      console.log(`
Worktree Management

Usage:
  node scripts/worktree.js list          List all worktrees
  node scripts/worktree.js remove <branch>  Remove a worktree
  node scripts/worktree.js cleanup       Remove merged worktrees

Examples:
  node scripts/worktree.js list
  node scripts/worktree.js remove issue-123
  node scripts/worktree.js cleanup
      `);
  }
}

main();

