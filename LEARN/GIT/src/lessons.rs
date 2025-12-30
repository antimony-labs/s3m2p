//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: lessons.rs | GIT/src/lessons.rs
//! PURPOSE: Git version control curriculum
//! MODIFIED: 2025-12-30
//! LAYER: LEARN → GIT
//! ═══════════════════════════════════════════════════════════════════════════════

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DemoType {
    Static,
    Diagram,
}

/// A single Git lesson
pub struct Lesson {
    pub id: usize,
    pub title: &'static str,
    pub subtitle: &'static str,
    pub icon: &'static str,
    pub phase: &'static str,
    pub demo_type: DemoType,
    pub description: &'static str,
    pub content: &'static str,
    pub key_concepts: &'static [&'static str],
    pub concept_definitions: &'static [(&'static str, &'static str)],
}

/// Git learning phases
pub static PHASES: &[&str] = &[
    "Foundations",
    "Collaboration",
    "Advanced Workflows",
    "Troubleshooting",
];

/// All Git lessons
pub static LESSONS: &[Lesson] = &[
    Lesson {
        id: 0,
        title: "What is Version Control?",
        subtitle: "Why Git Exists",
        icon: "📚",
        phase: "Foundations",
        demo_type: DemoType::Static,
        description: "Before Git, managing code versions was chaos. Learn why version control systems exist.",
        content: r###"
## The Problem

Without version control, you have chaos. Files named final_v2_ACTUALLY_FINAL.zip. Lost work. Merge conflicts resolved by copying and pasting. Email attachments.

Version control systems like Git solve this by tracking every change, enabling collaboration, and providing safety nets.

## Git is Distributed

Every developer has complete history. Commit offline, sync later. No single point of failure.

## Why Git Won

- Speed (written in C)
- Branching (cheap and fast)
- Offline workflow
- Open source
- Linux heritage

Today Git powers 90%+ of software development.
"###,
        key_concepts: &["Version Control", "Distributed", "Snapshots"],
        concept_definitions: &[
            ("Version Control", "System that tracks changes to files over time"),
            ("Distributed", "Every developer has complete repository history"),
            ("Snapshots", "Git stores complete file state at each commit"),
        ],
    },

    Lesson {
        id: 1,
        title: "Git Fundamentals",
        subtitle: "init, add, commit, status",
        icon: "🎯",
        phase: "Foundations",
        demo_type: DemoType::Static,
        description: "Install Git and learn the essential workflow: init, add, commit, status, log.",
        content: r###"
## Setup

Install Git and configure:

```
sudo apt install git
git config --global user.name "Your Name"
git config --global user.email "your@email.com"
```

## Basic Workflow

```
git init                    # Create repository
git status                  # Check status
git add file.txt            # Stage file
git commit -m "message"     # Commit snapshot
git log                     # View history
```

## The Three States

- Modified: Changed but not staged
- Staged: Marked for commit
- Committed: Saved in history

The staging area lets you group related changes into atomic commits.
"###,
        key_concepts: &["git init", "git add", "git commit", "Staging Area"],
        concept_definitions: &[
            ("git init", "Create a new Git repository in current directory"),
            ("git add", "Stage files for the next commit"),
            ("git commit", "Save a snapshot of staged changes"),
            ("Staging Area", "Prepare files before committing"),
        ],
    },

    Lesson {
        id: 2,
        title: "Branches & Merging",
        subtitle: "Parallel Development",
        icon: "🌿",
        phase: "Collaboration",
        demo_type: DemoType::Diagram,
        description: "Branches let you experiment without breaking main. Master branching, master Git.",
        content: r###"
## What is a Branch?

A lightweight pointer to a commit. Creating a branch is instant - just 40 bytes.

## Why Branch?

Experiment without affecting main code. Work on features in parallel. Fix bugs while developing features.

## Commands

```
git branch feature          # Create
git checkout feature        # Switch
git checkout -b feature     # Create and switch
git merge feature           # Merge into current branch
git branch -d feature       # Delete
```

## Merge Conflicts

When both branches change same lines, Git can't auto-merge. You must manually resolve by editing the file and choosing which version to keep.

Branches are Git's superpower.
"###,
        key_concepts: &["Branches", "git merge", "Merge Conflicts"],
        concept_definitions: &[
            ("Branches", "Lightweight pointers enabling parallel development"),
            ("git merge", "Combine changes from one branch into another"),
            ("Merge Conflicts", "When Git can't auto-merge - requires manual resolution"),
        ],
    },

    Lesson {
        id: 3,
        title: "Remote Repositories",
        subtitle: "GitHub & Collaboration",
        icon: "🌐",
        phase: "Collaboration",
        demo_type: DemoType::Static,
        description: "Connect to GitHub and collaborate with others using push and pull.",
        content: r###"
## Remotes

A remote is a Git repository on another computer (usually GitHub, GitLab, or Bitbucket).

## Commands

```
git clone <url>             # Copy remote repository
git remote add origin <url> # Add remote
git push origin main        # Upload changes
git pull origin main        # Download changes
git fetch origin            # Download without merging
```

## GitHub Workflow

1. Fork repository on GitHub
2. Clone your fork locally
3. Create feature branch
4. Make changes and commit
5. Push to your fork
6. Create Pull Request

More coming soon: SSH keys, authentication, pull requests.
"###,
        key_concepts: &["Remote", "git push", "git pull", "GitHub"],
        concept_definitions: &[
            ("Remote", "Git repository on another computer"),
            ("git push", "Upload local commits to remote"),
            ("git pull", "Download and merge remote changes"),
            ("GitHub", "Popular hosting service for Git repositories"),
        ],
    },

    Lesson {
        id: 4,
        title: "Git Workflows",
        subtitle: "Git Flow & GitHub Flow",
        icon: "🔀",
        phase: "Advanced Workflows",
        demo_type: DemoType::Static,
        description: "Different branching strategies for teams.",
        content: r###"
## Git Flow

Complex workflow for scheduled releases:
- main: production
- develop: integration
- feature/*: new features
- release/*: release preparation
- hotfix/*: emergency fixes

## GitHub Flow

Simpler workflow:
- main: always deployable
- feature branches: create, test, merge via PR

## Trunk-Based

Single main branch, short-lived feature branches. Deploy continuously.

Choose based on team size and release cycle.
"###,
        key_concepts: &["Git Flow", "GitHub Flow", "Trunk-Based"],
        concept_definitions: &[],
    },

    Lesson {
        id: 5,
        title: "Common Mistakes",
        subtitle: "Undo & Recover",
        icon: "🔧",
        phase: "Troubleshooting",
        demo_type: DemoType::Static,
        description: "Fix mistakes using reset, revert, and reflog.",
        content: r###"
## Undo Uncommitted Changes

```
git restore file.txt        # Discard changes to file
git restore .               # Discard all changes
```

## Undo Last Commit

```
git reset --soft HEAD~1     # Undo commit, keep changes staged
git reset --mixed HEAD~1    # Undo commit, keep changes unstaged
git reset --hard HEAD~1     # Undo commit, discard changes
```

## Recover Lost Commits

```
git reflog                  # Show all HEAD movements
git checkout <commit>       # Recover lost work
```

## Detached HEAD

When you checkout a specific commit instead of a branch. Create a branch to save work:

```
git checkout <commit>       # Detached HEAD
git checkout -b recovery    # Create branch to save work
```

Everyone makes mistakes. Git makes them fixable.
"###,
        key_concepts: &["git reset", "git reflog", "Detached HEAD"],
        concept_definitions: &[
            ("git reset", "Undo commits by moving branch pointer backward"),
            ("git reflog", "Log of all HEAD movements - recover lost commits"),
            ("Detached HEAD", "When HEAD points to commit instead of branch"),
        ],
    },
];
