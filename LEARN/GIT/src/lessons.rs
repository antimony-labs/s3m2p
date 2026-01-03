//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: lessons.rs | GIT/src/lessons.rs
//! PURPOSE: Git version control curriculum - From history to mastery
//! MODIFIED: 2026-01-01
//! LAYER: LEARN → GIT
//! ═══════════════════════════════════════════════════════════════════════════════

/// Technical term with tooltip explanation
#[derive(Clone)]
pub struct Term {
    pub word: &'static str,
    pub short: &'static str,
    pub detail: &'static str,
}

/// Glossary of Git technical terms
pub static GLOSSARY: &[Term] = &[
    Term {
        word: "repository",
        short: "A folder tracked by Git",
        detail: "Contains your project files plus a hidden .git folder that stores \
                 the entire history. Think of it as a project folder with superpowers.",
    },
    Term {
        word: "commit",
        short: "A snapshot of your project at a point in time",
        detail: "Each commit has a unique ID (SHA-1 hash), author, timestamp, message, \
                 and pointer to parent commit(s). Commits are immutable—once created, \
                 they never change.",
    },
    Term {
        word: "staging area",
        short: "Preparation zone before commit",
        detail: "Also called 'index'. Files here are queued for the next commit. \
                 Think of it as a loading dock—items wait here before being shipped (committed).",
    },
    Term {
        word: "branch",
        short: "A movable pointer to a commit",
        detail: "Branches are incredibly cheap in Git—just 41 bytes (a SHA-1 hash + newline). \
                 Creating a branch doesn't copy any files; it just creates a new pointer.",
    },
    Term {
        word: "HEAD",
        short: "Pointer to your current position",
        detail: "Like a 'You Are Here' marker. Usually points to a branch name, \
                 which in turn points to a commit. When you commit, HEAD moves forward.",
    },
    Term {
        word: "remote",
        short: "Git repository on another computer",
        detail: "Usually GitHub, GitLab, or Bitbucket. The 'origin' remote is the default—\
                 it's where you cloned from. You can have multiple remotes.",
    },
    Term {
        word: "origin",
        short: "Default name for the main remote",
        detail: "When you clone a repository, Git automatically names that remote 'origin'. \
                 It's just a convention—you can rename it or add other remotes.",
    },
    Term {
        word: "merge",
        short: "Combine changes from two branches",
        detail: "Git finds the common ancestor, then combines changes from both branches. \
                 If the same lines were changed differently, you get a merge conflict.",
    },
    Term {
        word: "rebase",
        short: "Replay commits on top of another branch",
        detail: "Creates cleaner, linear history than merge. But it rewrites commit hashes! \
                 Never rebase commits that have been pushed to a shared branch.",
    },
    Term {
        word: "fast-forward",
        short: "Simple merge with no divergence",
        detail: "When the target branch has no new commits since you branched, \
                 Git just moves the pointer forward. No merge commit needed.",
    },
    Term {
        word: "detached HEAD",
        short: "HEAD pointing directly to a commit",
        detail: "Happens when you checkout a specific commit instead of a branch. \
                 Any commits you make aren't on a branch and can be lost! Create a branch to save work.",
    },
    Term {
        word: "cherry-pick",
        short: "Copy a single commit to another branch",
        detail: "Applies the changes from one commit to your current branch. \
                 Useful for hotfixes, but creates duplicate commits—use sparingly.",
    },
    Term {
        word: "reflog",
        short: "Log of all HEAD movements",
        detail: "Your safety net! Even after reset --hard, reflog remembers where HEAD was. \
                 Commits aren't truly lost for 90 days. Use 'git reflog' to find lost work.",
    },
    Term {
        word: "stash",
        short: "Temporarily save uncommitted changes",
        detail: "Like a clipboard for your work-in-progress. Stash changes, switch branches, \
                 do something else, then 'git stash pop' to restore your changes.",
    },
    Term {
        word: "SHA-1",
        short: "40-character unique identifier for commits",
        detail: "A cryptographic hash that identifies each commit. If any bit of the commit \
                 changes, the hash changes completely. This ensures integrity.",
    },
    Term {
        word: "upstream",
        short: "The original repository you forked from",
        detail: "When you fork a repo, 'origin' is your fork, 'upstream' is the original. \
                 You pull from upstream to stay in sync with the source project.",
    },
    Term {
        word: "pull request",
        short: "Request to merge your changes into another branch",
        detail: "A GitHub/GitLab feature (not Git itself). Opens a discussion about your changes, \
                 allows code review, runs CI checks, and eventually merges when approved.",
    },
    Term {
        word: "fork",
        short: "Your personal copy of someone else's repository",
        detail: "A GitHub feature. Creates a full copy under your account where you can \
                 make changes freely. Used in open-source contribution workflows.",
    },
    Term {
        word: "clone",
        short: "Download a complete copy of a repository",
        detail: "Unlike downloading a ZIP, cloning gets the full history and sets up \
                 the remote connection. You can push and pull after cloning.",
    },
    Term {
        word: "fetch",
        short: "Download changes without merging",
        detail: "Gets new commits from remote but doesn't change your working files. \
                 Lets you review changes before merging. 'git pull' = fetch + merge.",
    },
    Term {
        word: "worktree",
        short: "Additional working directory for same repo",
        detail: "Lets you have multiple branches checked out simultaneously in different directories. \
                 All worktrees share the same .git database. Great for context switching.",
    },
    Term {
        word: "signed commit",
        short: "Commit with cryptographic proof of authorship",
        detail: "Uses GPG or SSH to prove you actually made the commit. \
                 GitHub shows 'Verified' badge. Prevents impersonation.",
    },
];

/// A single Git lesson with rich pedagogical structure
pub struct Lesson {
    pub id: usize,
    pub title: &'static str,
    pub subtitle: &'static str,
    pub icon: &'static str,
    pub phase: &'static str,
    /// Hook - why should I care? (1-2 sentences)
    pub why_it_matters: &'static str,
    /// Brief description for the card
    pub description: &'static str,
    /// Plain-language explanation with analogies
    pub intuition: &'static str,
    /// Main markdown content with code examples
    pub content: &'static str,
    /// Key concepts for this lesson
    pub key_concepts: &'static [&'static str],
    /// Concept definitions for tooltips
    pub concept_definitions: &'static [(&'static str, &'static str)],
    /// Bullet points to remember
    pub key_takeaways: &'static [&'static str],
    /// Best practices section
    pub dos_and_donts: &'static str,
    /// Advanced content (collapsible)
    pub going_deeper: &'static str,
    /// Common mistakes to avoid
    pub common_mistakes: &'static str,
}

/// Git learning phases
pub static PHASES: &[&str] = &[
    "Origins & Philosophy",
    "Foundations",
    "Branching & Merging",
    "Collaboration",
    "Advanced Workflows",
    "Best Practices",
    "Software Engineering",
];

/// All Git lessons - 18 lessons across 7 phases
pub static LESSONS: &[Lesson] = &[
    // ═══════════════════════════════════════════════════════════════════════════
    // PHASE 1: ORIGINS & PHILOSOPHY
    // ═══════════════════════════════════════════════════════════════════════════
    Lesson {
        id: 0,
        title: "The Birth of Git",
        subtitle: "A Crisis Creates a Revolution",
        icon: "📜",
        phase: "Origins & Philosophy",
        why_it_matters: "Every tool has a story. Git was born from a crisis—and that crisis shaped \
                         everything about how Git works today. Understanding the 'why' helps you \
                         understand the 'how'.",
        description: "The dramatic story of how Linus Torvalds created Git in 10 days after a licensing dispute.",
        intuition: r#"<h3>The BitKeeper Crisis of 2005</h3>

In 2002, the Linux kernel—the largest open-source project in the world—was managed using BitKeeper,
a proprietary version control system. It was fast, distributed, and worked beautifully for Linux's
thousands of contributors.

<strong>Until April 2005.</strong>

BitMover, the company behind BitKeeper, revoked the free license after a developer allegedly
reverse-engineered their protocol. Suddenly, thousands of Linux developers were stranded.
The kernel's development ground to a halt.

<h3>Linus's Three Choices</h3>

Linus Torvalds faced three options:

<strong>1. Pay for BitKeeper</strong>
Rejected. Open source philosophy demanded open tools.

<strong>2. Switch to CVS or Subversion</strong>
Rejected. "CVS is not even worth talking about. If you like it, you are demented."
Subversion was "designed to be a better CVS" which meant it inherited CVS's fundamental flaws.

<strong>3. Build something better</strong>
✓ Chosen. In just <strong>10 days</strong>, Linus wrote the first version of Git.

<h3>The Design Philosophy</h3>

Git wasn't designed to be user-friendly. It was designed to be:

• <strong>Correct:</strong> Data integrity via SHA-1 hashes. Corruption is impossible to hide.
• <strong>Fast:</strong> "If it takes more than a second, it's broken"
• <strong>Distributed:</strong> Every developer has the full history. No single point of failure.
• <strong>Simple branching:</strong> Linux needed thousands of parallel experiments.

<h3>The Name</h3>

Linus named it "Git"—British slang for an unpleasant person. As he explained:
"I'm an egotistical bastard, and I name all my projects after myself. First Linux, now Git."

(He later suggested it could stand for "Global Information Tracker" if you're feeling generous.)"#,
        content: r###"
## Timeline

| Date | Event |
|------|-------|
| 2002 | Linux adopts BitKeeper |
| April 3, 2005 | BitKeeper license revoked |
| April 7, 2005 | Linus starts writing Git |
| April 16, 2005 | Git manages Linux kernel (10 days!) |
| June 16, 2005 | Linux 2.6.12 released via Git |
| December 2005 | Git 1.0 released |

## Design Goals (from Linus)

1. **Speed** - Subsecond operations for everything
2. **Simple design** - Data model is elegant
3. **Strong support for parallel development** - Thousands of branches
4. **Fully distributed** - No central server required
5. **Handle large projects efficiently** - Linux has millions of lines

## Git's Core Insight

Previous systems tracked *changes* (deltas). Git tracks *snapshots*.

```
CVS/SVN: Δ1 → Δ2 → Δ3 → Δ4
Git:     S1   S2   S3   S4  (full snapshots, compressed)
```

This makes branching and merging trivial—you're comparing complete states, not reconstructing history.
"###,
        key_concepts: &["BitKeeper", "Distributed VCS", "SHA-1", "Snapshots"],
        concept_definitions: &[
            ("BitKeeper", "Proprietary distributed VCS that inspired Git's design"),
            ("Distributed VCS", "Every clone has complete history—no central server needed"),
            ("SHA-1", "Cryptographic hash ensuring data integrity"),
            ("Snapshots", "Git stores complete file states, not differences"),
        ],
        key_takeaways: &[
            "Git was built in 10 days to solve a real crisis",
            "Designed for correctness and speed over user-friendliness",
            "Distributed = no single point of failure",
            "Snapshots (not deltas) enable fast branching",
        ],
        dos_and_donts: "",
        going_deeper: r#"<strong>The Linux Development Model:</strong>
Linux has a 'lieutenant' system. Linus doesn't review every patch—subsystem maintainers do.
Git's distributed model perfectly supports this hierarchy. Each maintainer has their own tree,
pulls from contributors, and Linus pulls from maintainers.

<strong>Impact:</strong>
Today, Git powers 90%+ of software development. GitHub has 100M+ developers.
Git's success influenced distributed systems thinking across the industry."#,
        common_mistakes: "",
    },

    Lesson {
        id: 1,
        title: "Distributed vs Centralized",
        subtitle: "Why Git Won",
        icon: "🌐",
        phase: "Origins & Philosophy",
        why_it_matters: "Understanding the difference between centralized (SVN/CVS) and distributed (Git) \
                         version control explains why Git commands work the way they do.",
        description: "Compare centralized VCS (SVN) with distributed VCS (Git) and understand why distributed won.",
        intuition: r#"<h3>The Library Analogy</h3>

<strong>Centralized VCS (SVN/CVS):</strong>
Imagine a single library with one copy of every book. To read a book, you must go to the library.
To write notes in a book, you must check it out (lock), write, and return it (unlock).
If the library burns down, everything is lost.

<strong>Distributed VCS (Git):</strong>
Everyone has a complete copy of the library. You can read, annotate, and reorganize your copy
anytime, even offline. When you want to share, you sync with others. If any library burns down,
dozens of complete copies exist elsewhere.

<h3>The Key Differences</h3>

| Centralized (SVN) | Distributed (Git) |
|-------------------|-------------------|
| Need network to commit | Commit offline, sync later |
| Single point of failure | Everyone has full history |
| Lock-based workflow | Branch-based workflow |
| Slow branching (copies files) | Instant branching (pointer) |
| Revision numbers (1, 2, 3...) | SHA-1 hashes |

<h3>Why Distributed Won</h3>

1. <strong>Offline work:</strong> Commit on planes, trains, or anywhere without WiFi
2. <strong>Speed:</strong> All operations are local except push/pull
3. <strong>Backup:</strong> Every clone is a full backup
4. <strong>Experimentation:</strong> Cheap branches encourage trying things
5. <strong>Collaboration:</strong> Fork, experiment, merge—without asking permission"#,
        content: r###"
## Centralized (SVN) Workflow

```
         ┌─────────────┐
         │   Server    │  ← Single point of failure
         │  (trunk)    │
         └──────┬──────┘
                │
    ┌───────────┼───────────┐
    │           │           │
    ▼           ▼           ▼
 [Dev A]     [Dev B]     [Dev C]
 checkout    checkout    checkout
```

Every commit goes directly to the server. No local history.

## Distributed (Git) Workflow

```
         ┌─────────────┐
         │   GitHub    │  ← Optional central hub
         │  (remote)   │
         └──────┬──────┘
                │ push/pull
    ┌───────────┼───────────┐
    │           │           │
    ▼           ▼           ▼
 ┌─────┐     ┌─────┐     ┌─────┐
 │Clone│     │Clone│     │Clone│
 │  A  │ ←─→ │  B  │ ←─→ │  C  │
 └─────┘     └─────┘     └─────┘
   Full       Full        Full
  History    History     History
```

Every clone has complete history. Can sync peer-to-peer.

## Speed Comparison

| Operation | SVN | Git |
|-----------|-----|-----|
| Commit | Network round-trip | Instant (local) |
| View history | Network round-trip | Instant (local) |
| Create branch | Minutes (copies files) | Instant (41 bytes) |
| Switch branch | Network round-trip | Instant (local) |
"###,
        key_concepts: &["Centralized VCS", "Distributed VCS", "Clone", "Local Operations"],
        concept_definitions: &[
            ("Centralized VCS", "Single server holds all history (SVN, CVS, Perforce)"),
            ("Distributed VCS", "Every clone has full history (Git, Mercurial)"),
            ("Clone", "Complete copy of repository including all history"),
            ("Local Operations", "Commands that don't need network (commit, branch, log)"),
        ],
        key_takeaways: &[
            "In Git, every clone is a full backup",
            "Commit, branch, and log work offline",
            "GitHub is a hub, not a requirement",
            "Distributed = resilient + fast + flexible",
        ],
        dos_and_donts: "",
        going_deeper: r#"<strong>SVN is still used:</strong>
Some organizations still use SVN for large binary assets or strict access control.
Git-LFS and Perforce fill this niche for games/media.

<strong>Mercurial:</strong>
Mercurial (hg) was created the same week as Git, also in response to BitKeeper.
It's similarly distributed but more user-friendly. Facebook used Mercurial internally.
Git won market share due to GitHub's network effects."#,
        common_mistakes: r#"<strong>Thinking GitHub IS Git:</strong>
GitHub is a hosting service built on Git. You can use Git without GitHub (GitLab, Bitbucket,
self-hosted, or no remote at all).

<strong>Treating Git like SVN:</strong>
Don't commit directly to main. Don't avoid branching. Embrace the distributed workflow."#,
    },

    // ═══════════════════════════════════════════════════════════════════════════
    // PHASE 2: FOUNDATIONS
    // ═══════════════════════════════════════════════════════════════════════════
    Lesson {
        id: 2,
        title: "Git Fundamentals",
        subtitle: "init, add, commit, status, log",
        icon: "🎯",
        phase: "Foundations",
        why_it_matters: "These five commands cover 80% of daily Git usage. Master them and you'll \
                         handle most version control tasks with confidence.",
        description: "Install Git and learn the essential daily workflow: init, add, commit, status, log.",
        intuition: r#"<h3>The Checkpoint Analogy</h3>

Think of Git like a video game's save system:

• <strong>git init</strong> = Start a new game (enable saving)
• <strong>git add</strong> = Select which items to save (stage)
• <strong>git commit</strong> = Create a save point (checkpoint)
• <strong>git status</strong> = Check what's changed since last save
• <strong>git log</strong> = View all your save points

Unlike video games, Git keeps <em>every</em> save point forever. You can always go back.

<h3>The Workflow</h3>

```
1. Edit files (work in your editor)
       ↓
2. git status (see what changed)
       ↓
3. git add <files> (stage changes)
       ↓
4. git commit -m "message" (create checkpoint)
       ↓
5. Repeat
```"#,
        content: r###"
## Setup (One-Time)

```bash
# Install Git
sudo apt install git          # Ubuntu/Debian
brew install git              # macOS

# Configure identity (required for commits)
git config --global user.name "Your Name"
git config --global user.email "you@example.com"

# Optional: set default branch name
git config --global init.defaultBranch main
```

## Starting a Repository

```bash
# Create new repository
git init

# Or clone an existing one
git clone https://github.com/user/repo.git
```

## The Daily Workflow

```bash
# Check current status
git status

# Stage specific files
git add file.txt
git add src/

# Stage all changes
git add .

# Commit with message
git commit -m "Add login feature"

# View commit history
git log
git log --oneline        # Compact view
git log --graph          # Show branch structure
```

## Understanding Status Output

```
On branch main
Changes to be committed:
  (use "git restore --staged <file>..." to unstage)
        modified:   staged_file.txt     ← Ready to commit

Changes not staged for commit:
        modified:   unstaged_file.txt   ← Changed but not staged

Untracked files:
        new_file.txt                    ← Git doesn't know about this
```
"###,
        key_concepts: &["git init", "git add", "git commit", "git status", "git log"],
        concept_definitions: &[
            ("git init", "Create a new Git repository in current directory"),
            ("git add", "Stage files for the next commit"),
            ("git commit", "Save a snapshot of staged changes"),
            ("git status", "Show working tree status"),
            ("git log", "View commit history"),
        ],
        key_takeaways: &[
            "git init creates the .git folder (repository)",
            "git add stages changes (prepares for commit)",
            "git commit creates a permanent snapshot",
            "git status shows what's staged/unstaged/untracked",
            "git log shows commit history",
        ],
        dos_and_donts: r###"## ✅ DO

- **Commit often**: Small, focused commits are easier to understand and revert
- **Write meaningful messages**: "Fix null pointer in auth" not "fix bug"
- **Check status before committing**: Make sure you're committing what you expect

## ❌ DON'T

- **Don't commit everything at once**: Stage related changes together
- **Don't use `git add .` blindly**: Review what you're staging
- **Don't leave work uncommitted overnight**: Commit your progress
"###,
        going_deeper: r#"<strong>The .git folder:</strong>
Everything Git knows lives in .git/. The objects/ folder stores all file content (as compressed blobs).
The refs/ folder stores branch pointers. HEAD is a file pointing to your current branch.

<strong>Staging area internals:</strong>
The staging area (index) is a binary file at .git/index. It's a snapshot of what your next
commit will look like. This two-step process (stage then commit) gives you control over
what goes into each commit."#,
        common_mistakes: r#"<strong>Committing without staging:</strong>
`git commit -m "message"` only commits staged changes. If you didn't `git add`, nothing is committed.
Use `git commit -am "message"` to add+commit tracked files (but not new files).

<strong>Forgetting to init:</strong>
"fatal: not a git repository" means you're not in a Git repo. Run `git init` or navigate to the right folder."#,
    },

    Lesson {
        id: 3,
        title: "The Three States",
        subtitle: "Working Directory, Staging, Repository",
        icon: "📦",
        phase: "Foundations",
        why_it_matters: "Git has three states for your files. Understanding this model is the key to \
                         knowing what each command does and why.",
        description: "Master the three-state model: working directory, staging area, and repository.",
        intuition: r#"<h3>The Shipping Analogy</h3>

Think of Git as a shipping warehouse:

<strong>1. Working Directory (Your Desk)</strong>
Where you do actual work. Files here can be modified freely.
Git doesn't care about changes until you stage them.

<strong>2. Staging Area (The Loading Dock)</strong>
Items you've decided to ship. You're selecting which boxes go on the truck.
You can add/remove items before the truck leaves.

<strong>3. Repository (The Warehouse Archive)</strong>
Once committed, items are archived permanently. Can't be changed, only added to.

```
┌─────────────────┐     git add     ┌─────────────────┐    git commit    ┌─────────────────┐
│    Working      │ ──────────────► │    Staging      │ ───────────────► │   Repository    │
│    Directory    │                 │    Area         │                  │   (.git/)       │
│                 │ ◄────────────── │    (Index)      │                  │                 │
└─────────────────┘  git restore    └─────────────────┘                  └─────────────────┘
```

<h3>Why Two Steps?</h3>

The staging area lets you craft precise commits. Changed 10 files but only want to commit 3?
Stage just those 3. This creates cleaner, more meaningful history."#,
        content: r###"
## File States

```
                  ┌─────────────────────────────────────────┐
                  │           Working Directory              │
                  │  ┌──────────┐  ┌──────────┐  ┌────────┐ │
                  │  │Untracked │  │ Modified │  │Unchanged│ │
                  │  └────┬─────┘  └────┬─────┘  └────────┘ │
                  └───────┼─────────────┼───────────────────┘
                          │ git add     │ git add
                          ▼             ▼
                  ┌─────────────────────────────────────────┐
                  │           Staging Area (Index)           │
                  │  ┌──────────────────────────────────┐   │
                  │  │         Staged Changes           │   │
                  │  └────────────────┬─────────────────┘   │
                  └───────────────────┼─────────────────────┘
                                      │ git commit
                                      ▼
                  ┌─────────────────────────────────────────┐
                  │           Repository (.git/)             │
                  │  ┌──────────────────────────────────┐   │
                  │  │     Committed (Permanent)        │   │
                  │  └──────────────────────────────────┘   │
                  └─────────────────────────────────────────┘
```

## Commands for Each Transition

```bash
# Working → Staging
git add <file>              # Stage specific file
git add .                   # Stage all changes
git add -p                  # Stage parts of files (interactive)

# Staging → Working (unstage)
git restore --staged <file> # Remove from staging
git reset HEAD <file>       # Alternative (older style)

# Working → Discard
git restore <file>          # Discard working changes
git checkout -- <file>      # Alternative (older style)

# Staging → Repository
git commit -m "message"     # Commit staged changes
```

## Git Objects (What's in .git/)

Git stores four types of objects:

| Object | Purpose |
|--------|---------|
| **blob** | File contents (compressed) |
| **tree** | Directory listing (pointers to blobs) |
| **commit** | Snapshot + metadata + parent pointer |
| **tag** | Named pointer to a commit |
"###,
        key_concepts: &["Working Directory", "Staging Area", "Repository", "Git Objects"],
        concept_definitions: &[
            ("Working Directory", "Your actual files on disk"),
            ("Staging Area", "Files queued for the next commit (index)"),
            ("Repository", "Permanent history stored in .git/"),
            ("Git Objects", "Blobs (files), trees (directories), commits, tags"),
        ],
        key_takeaways: &[
            "Three states: Working → Staging → Repository",
            "Staging lets you craft precise commits",
            "Commits are permanent snapshots",
            "All Git data lives in the .git folder",
        ],
        dos_and_donts: r###"## ✅ DO

- **Use staging strategically**: Commit related changes together
- **Use `git add -p`**: Stage specific parts of files
- **Review before committing**: `git diff --staged` shows what's staged

## ❌ DON'T

- **Don't delete .git/**: That's your entire history!
- **Don't mix unrelated changes in one commit**: Keep commits focused
"###,
        going_deeper: r#"<strong>Content-Addressable Storage:</strong>
Git is a content-addressable filesystem. Files are stored by their SHA-1 hash.
If two files have identical content, they're stored once. This makes Git space-efficient.

<strong>Plumbing vs Porcelain:</strong>
Git has low-level "plumbing" commands (hash-object, cat-file, update-index) and
high-level "porcelain" commands (add, commit, status). You use porcelain; Git uses plumbing internally."#,
        common_mistakes: r#"<strong>Confusing staged and committed:</strong>
Staged ≠ saved. If you `git add` but don't `git commit`, your changes aren't in history yet.
`git status` shows the difference.

<strong>Editing staged files:</strong>
If you `git add file.txt` then edit file.txt again, the new edits are NOT staged.
You need to `git add file.txt` again to stage the latest version."#,
    },

    // ═══════════════════════════════════════════════════════════════════════════
    // PHASE 3: BRANCHING & MERGING
    // ═══════════════════════════════════════════════════════════════════════════
    Lesson {
        id: 4,
        title: "Branches",
        subtitle: "Parallel Development",
        icon: "🌿",
        phase: "Branching & Merging",
        why_it_matters: "Branches are Git's superpower. They let you experiment, work on features, \
                         and fix bugs without affecting the main codebase. Cheap branches = fearless development.",
        description: "Learn why Git branches are so fast and how to use them for parallel development.",
        intuition: r#"<h3>The Parallel Universe Analogy</h3>

Imagine you could create parallel universes to try different approaches:

• Universe A: You try a risky refactor
• Universe B: You fix a bug
• Universe C: The stable production version

In Git, these universes are called <strong>branches</strong>. Creating a branch is instant.
You can switch between them freely. When an experiment works, merge it back.
If it fails, just delete the branch—no harm done.

<h3>Why Git Branches Are Different</h3>

In SVN, creating a branch copies all files. Slow and expensive.

In Git, a branch is just a 41-byte file containing a commit hash. Creating a branch is literally:
```
echo "a1b2c3d4..." > .git/refs/heads/feature
```

This is why Git encourages branching for everything. It costs nothing."#,
        content: r###"
## How Branches Work

```
                        HEAD
                         ↓
                       main
                         ↓
    [A] ← [B] ← [C] ← [D]
                   ↖
                    feature (branch pointer)
```

A branch is just a pointer to a commit. HEAD points to the current branch.

## Essential Branch Commands

```bash
# List branches
git branch              # Local branches
git branch -a           # All branches (including remote)

# Create branch
git branch feature      # Create but don't switch
git checkout -b feature # Create and switch (old way)
git switch -c feature   # Create and switch (new way)

# Switch branches
git checkout feature    # Old way
git switch feature      # New way (Git 2.23+)

# Delete branch
git branch -d feature   # Delete (safe - must be merged)
git branch -D feature   # Force delete (even if unmerged)

# Rename branch
git branch -m old-name new-name
```

## Common Workflow

```bash
# Start a new feature
git switch -c feature/login

# Make changes, commit
git add .
git commit -m "Add login form"

# Switch back to main
git switch main

# Merge the feature
git merge feature/login

# Clean up
git branch -d feature/login
```

## Branch Naming Conventions

```
feature/user-auth      # New feature
bugfix/null-pointer    # Bug fix
hotfix/security-patch  # Urgent production fix
release/v2.0           # Release preparation
experiment/new-ui      # Experimental work
```
"###,
        key_concepts: &["Branch", "HEAD", "git switch", "git branch"],
        concept_definitions: &[
            ("Branch", "Lightweight pointer to a commit (41 bytes)"),
            ("HEAD", "Special pointer indicating current branch/commit"),
            ("git switch", "Modern command to switch branches (Git 2.23+)"),
            ("git branch", "List, create, or delete branches"),
        ],
        key_takeaways: &[
            "Branches are just pointers (instant to create)",
            "Use branches for every feature/bugfix",
            "HEAD points to your current branch",
            "Delete merged branches to keep things clean",
        ],
        dos_and_donts: r###"## ✅ DO

- **Branch early, branch often**: Branches are free
- **Use descriptive names**: `feature/add-auth` not `my-branch`
- **Keep main stable**: Never commit broken code to main
- **Delete merged branches**: Keep your branch list clean

## ❌ DON'T

- **Don't work directly on main**: Always use feature branches
- **Don't let branches live too long**: Merge often to avoid conflicts
- **Don't create branches from uncommitted work**: Commit or stash first
"###,
        going_deeper: r#"<strong>Branch storage:</strong>
Local branches are files in `.git/refs/heads/`. Remote-tracking branches are in `.git/refs/remotes/`.
The content is just the SHA-1 hash of the commit they point to.

<strong>The reflog for branches:</strong>
Each branch has its own reflog. `git reflog show feature` shows the history of that branch pointer.
Useful for recovering from mistakes."#,
        common_mistakes: r#"<strong>Working on wrong branch:</strong>
Always check `git branch` or `git status` before making changes.
If you commit to wrong branch, use `git cherry-pick` or `git reset`.

<strong>Unmerged commits warning:</strong>
`git branch -d` refuses to delete branches with unmerged commits. This protects you.
Use `-D` only if you're sure you don't need those commits."#,
    },

    Lesson {
        id: 5,
        title: "Merging & Rebasing",
        subtitle: "Combining Work",
        icon: "🔀",
        phase: "Branching & Merging",
        why_it_matters: "Eventually, branches need to come together. Understanding merge vs rebase \
                         helps you choose the right strategy and handle conflicts confidently.",
        description: "Master the two ways to integrate changes: merge and rebase. Handle conflicts like a pro.",
        intuition: r#"<h3>Two Ways to Combine Branches</h3>

<strong>Merge (Preserve History)</strong>
Like a family tree—you can see exactly when branches diverged and rejoined.
Creates a "merge commit" with two parents.

```
main:    A ─ B ─ C ─ ─ ─ M  (merge commit)
               ↘       ↗
feature:        D ─ E
```

<strong>Rebase (Linear History)</strong>
Like rewriting history—your commits are "replayed" on top of the target branch.
No merge commit. Cleaner, linear history.

```
Before rebase:     main: A ─ B ─ C
                              ↘
                   feature:    D ─ E

After rebase:      main: A ─ B ─ C ─ D' ─ E'
```

<h3>When to Use Which</h3>

| Use Merge | Use Rebase |
|-----------|------------|
| Shared/public branches | Local/private branches |
| Want to preserve history | Want clean linear history |
| Merging feature into main | Updating feature from main |"#,
        content: r###"
## Merge Types

### Fast-Forward Merge

When main hasn't changed since you branched:

```bash
git switch main
git merge feature
# Just moves the pointer forward—no merge commit
```

```
Before:  main: A ─ B
                   ↘
         feature:   C ─ D

After:   main: A ─ B ─ C ─ D
```

### Three-Way Merge

When both branches have new commits:

```bash
git switch main
git merge feature
# Creates a merge commit with two parents
```

## Rebase

```bash
# On feature branch, replay commits on top of main
git switch feature
git rebase main

# Then fast-forward main
git switch main
git merge feature
```

## Handling Merge Conflicts

```bash
# Git pauses at conflict
Auto-merging file.txt
CONFLICT (content): Merge conflict in file.txt

# Open file—conflict markers show both versions
<<<<<<< HEAD
Your changes on main
=======
Their changes on feature
>>>>>>> feature

# Edit the file to resolve
# Then stage and complete
git add file.txt
git commit  # Or 'git rebase --continue' for rebase
```

## Aborting a Merge/Rebase

```bash
git merge --abort    # Cancel merge, return to before
git rebase --abort   # Cancel rebase, return to before
```
"###,
        key_concepts: &["Merge", "Rebase", "Fast-forward", "Merge Conflict"],
        concept_definitions: &[
            ("Merge", "Combine branches, creating merge commit if needed"),
            ("Rebase", "Replay commits on top of another branch"),
            ("Fast-forward", "Simple pointer move when no divergence"),
            ("Merge Conflict", "When same lines changed differently in both branches"),
        ],
        key_takeaways: &[
            "Merge preserves history; rebase linearizes it",
            "Fast-forward happens when target has no new commits",
            "Conflicts require manual resolution",
            "Never rebase public/shared branches",
        ],
        dos_and_donts: r###"## ✅ DO

- **Merge for shared branches**: main, develop, release branches
- **Rebase for local cleanup**: Before pushing, to get linear history
- **Pull with rebase**: `git pull --rebase` keeps history clean
- **Resolve conflicts carefully**: Test after resolving

## ❌ DON'T

- **Don't rebase public branches**: Others have copies—you'll cause chaos
- **Don't force-push after rebase to shared branches**: Same reason
- **Don't ignore conflicts**: Resolve them properly, don't just pick one side
"###,
        going_deeper: r#"<strong>Interactive Rebase:</strong>
`git rebase -i HEAD~5` lets you edit, squash, reorder, or drop the last 5 commits.
Powerful for cleaning up messy history before sharing.

<strong>Merge Strategies:</strong>
Git has multiple merge strategies (recursive, octopus, ours, theirs).
The default 'recursive' handles most cases. `git merge -X theirs` auto-resolves conflicts in their favor."#,
        common_mistakes: r#"<strong>Rebasing shared branches:</strong>
If you've pushed commits and then rebase, you've rewritten public history.
Others who pulled those commits now have divergent history. Use `git merge` instead.

<strong>Not understanding conflict markers:</strong>
The code between `<<<<<<<` and `=======` is yours.
The code between `=======` and `>>>>>>>` is theirs.
Delete the markers and keep the code you want."#,
    },

    // ═══════════════════════════════════════════════════════════════════════════
    // PHASE 4: COLLABORATION
    // ═══════════════════════════════════════════════════════════════════════════
    Lesson {
        id: 6,
        title: "Remote Repositories",
        subtitle: "Sync with the World",
        icon: "☁️",
        phase: "Collaboration",
        why_it_matters: "Git is distributed, but teams need a shared hub. Understanding remotes, \
                         push, pull, and fetch is essential for collaboration.",
        description: "Connect to GitHub/GitLab and collaborate using remotes, push, pull, and fetch.",
        intuition: r#"<h3>The Sync Analogy</h3>

Think of remotes like cloud storage for your Git repository:

• <strong>clone</strong>: Download the entire cloud copy to your computer
• <strong>fetch</strong>: Check what's new in the cloud (but don't download)
• <strong>pull</strong>: Download changes and merge into your branch
• <strong>push</strong>: Upload your commits to the cloud

<h3>Origin and Upstream</h3>

When you clone a repository, Git names that remote "origin".

```
YOUR COMPUTER                    GITHUB
┌─────────────┐                 ┌─────────────┐
│  Your Clone │ ◄────────────── │   origin    │
│   (local)   │ ──────────────► │  (remote)   │
└─────────────┘   push/pull     └─────────────┘
```

For forks, there's also "upstream" (the original repo you forked from)."#,
        content: r###"
## Remote Commands

```bash
# Clone repository (creates 'origin' remote)
git clone https://github.com/user/repo.git
git clone git@github.com:user/repo.git    # SSH

# View remotes
git remote -v

# Add remote
git remote add origin https://github.com/user/repo.git
git remote add upstream https://github.com/original/repo.git

# Fetch (download without merging)
git fetch origin              # Get all branches
git fetch origin main         # Get specific branch

# Pull (fetch + merge)
git pull origin main
git pull --rebase origin main # Fetch + rebase instead

# Push
git push origin main                    # Push to remote
git push -u origin feature              # Push and set upstream
git push                                # Push to tracked upstream
```

## Tracking Branches

```bash
# See tracking relationship
git branch -vv

# Set upstream for existing branch
git branch --set-upstream-to=origin/feature feature

# Push and set upstream in one command
git push -u origin feature
```

## Typical Workflow

```bash
# 1. Start fresh from remote
git fetch origin
git switch -c feature origin/main

# 2. Do work, commit
git add .
git commit -m "Add feature"

# 3. Stay in sync
git fetch origin
git rebase origin/main

# 4. Push your work
git push -u origin feature
```
"###,
        key_concepts: &["Remote", "Origin", "Push", "Pull", "Fetch"],
        concept_definitions: &[
            ("Remote", "A Git repository on another computer"),
            ("Origin", "Default name for the remote you cloned from"),
            ("Push", "Upload local commits to remote"),
            ("Pull", "Download and merge remote changes"),
            ("Fetch", "Download remote changes without merging"),
        ],
        key_takeaways: &[
            "Clone creates 'origin' remote automatically",
            "Fetch downloads but doesn't merge",
            "Pull = fetch + merge",
            "Push -u sets upstream tracking",
        ],
        dos_and_donts: r###"## ✅ DO

- **Fetch before starting work**: See if there are updates
- **Pull before pushing**: Avoid rejected pushes
- **Use SSH keys**: More secure than passwords
- **Set upstream with -u**: Makes future pushes easier

## ❌ DON'T

- **Don't force-push to shared branches**: Unless you know what you're doing
- **Don't push incomplete work to main**: Use feature branches
- **Don't ignore push rejections**: Pull first, resolve conflicts
"###,
        going_deeper: r#"<strong>SSH vs HTTPS:</strong>
HTTPS works everywhere but requires tokens/passwords.
SSH uses key-based auth—set it up once and never enter credentials again.

<strong>Fetch vs Pull:</strong>
Experienced developers often `fetch` then manually inspect changes before merging.
This gives more control: `git log origin/main..main` shows what's different."#,
        common_mistakes: r#"<strong>"Updates were rejected":</strong>
Someone pushed before you. Solution: `git pull --rebase` then push again.

<strong>Pushing to wrong remote:</strong>
Always verify with `git remote -v`. You might have multiple remotes."#,
    },

    Lesson {
        id: 7,
        title: "Pull Requests",
        subtitle: "Code Review Workflow",
        icon: "🔍",
        phase: "Collaboration",
        why_it_matters: "Pull requests are how teams review and integrate code. They're the gateway \
                         to contributing to open source and the standard for professional development.",
        description: "Master the GitHub Pull Request workflow: fork, branch, PR, review, merge.",
        intuition: r#"<h3>The Proposal Analogy</h3>

A Pull Request is like a formal proposal:

1. <strong>Fork</strong>: Make your own copy of the project
2. <strong>Branch</strong>: Create a workspace for your changes
3. <strong>Develop</strong>: Make changes, commit
4. <strong>Push</strong>: Upload to your fork
5. <strong>PR</strong>: "Hey, I made something useful—want to pull it into the main project?"

Maintainers review your code, suggest changes, and eventually merge (or reject).

<h3>The Workflow</h3>

```
┌──────────────┐     fork      ┌──────────────┐
│   upstream   │ ────────────► │  your fork   │
│  (original)  │               │   (origin)   │
└──────────────┘               └──────────────┘
       ▲                              │
       │                              │ clone
       │                              ▼
       │                       ┌──────────────┐
       │                       │    local     │
       │                       │    clone     │
       │                       └──────────────┘
       │                              │
       │                              │ push
       │        Pull Request          ▼
       └────────────────────── your fork/feature
```"#,
        content: r###"
## Fork Workflow

```bash
# 1. Fork on GitHub (click button)

# 2. Clone your fork
git clone git@github.com:YOUR-USERNAME/repo.git
cd repo

# 3. Add upstream remote
git remote add upstream git@github.com:ORIGINAL/repo.git

# 4. Create feature branch
git switch -c feature/awesome

# 5. Make changes, commit
git add .
git commit -m "Add awesome feature"

# 6. Push to your fork
git push -u origin feature/awesome

# 7. Open PR on GitHub
```

## Staying in Sync with Upstream

```bash
# Fetch latest from upstream
git fetch upstream

# Update your main branch
git switch main
git merge upstream/main
git push origin main

# Rebase your feature branch
git switch feature/awesome
git rebase upstream/main
git push --force-with-lease origin feature/awesome
```

## PR Best Practices

### Title
```
feat: add user authentication
fix: resolve null pointer in checkout
docs: update installation guide
```

### Description
```markdown
## Summary
Brief description of changes.

## Changes
- Added login form
- Integrated OAuth2
- Added tests

## Testing
Describe how to test these changes.

Closes #123
```

## Responding to Reviews

```bash
# Make requested changes
git add .
git commit -m "Address review feedback"
git push

# Or amend if it's minor
git add .
git commit --amend --no-edit
git push --force-with-lease
```
"###,
        key_concepts: &["Fork", "Pull Request", "Code Review", "Upstream"],
        concept_definitions: &[
            ("Fork", "Personal copy of a repository on GitHub"),
            ("Pull Request", "Request to merge your changes into another branch/repo"),
            ("Code Review", "Process of examining code before merging"),
            ("Upstream", "The original repository you forked from"),
        ],
        key_takeaways: &[
            "Fork → Branch → Commit → Push → PR",
            "Keep PRs focused and well-documented",
            "Respond to review feedback promptly",
            "Sync with upstream regularly",
        ],
        dos_and_donts: r###"## ✅ DO

- **Write clear PR descriptions**: Help reviewers understand your changes
- **Keep PRs small**: Easier to review, faster to merge
- **Respond to feedback**: Engage with reviewers constructively
- **Test before opening PR**: Don't waste reviewers' time on broken code

## ❌ DON'T

- **Don't open PRs without context**: Include why, not just what
- **Don't force-push without warning**: Reviewers lose their comments
- **Don't take feedback personally**: Reviews improve code quality
- **Don't merge your own PRs**: Get at least one approval
"###,
        going_deeper: r#"<strong>Conventional Commits:</strong>
Many projects use prefixes: feat:, fix:, docs:, chore:, test:, refactor:
These help auto-generate changelogs and semantic versions.

<strong>Draft PRs:</strong>
Open a "Draft PR" for work-in-progress. Signals you want early feedback but aren't ready for merge."#,
        common_mistakes: r#"<strong>"Close" vs "Refs":</strong>
`Closes #123` auto-closes the issue on merge.
`Refs #123` links without closing.
Be careful in commit messages—use Refs in commits, Closes in PR body only.

<strong>Merge conflicts in PRs:</strong>
If main updated, your PR may have conflicts. Rebase locally, force-push, conflicts resolved."#,
    },

    Lesson {
        id: 8,
        title: "GitHub Issues & Projects",
        subtitle: "Track Work Like a Pro",
        icon: "📋",
        phase: "Collaboration",
        why_it_matters: "Code without context is just text. Issues document what needs to be done, \
                         why, and by whom. Projects organize issues into actionable workflows.",
        description: "Master GitHub Issues for tracking bugs and features, and Projects for team organization.",
        intuition: r#"<h3>The Issue as a Contract</h3>

An issue is a contract between the reporter and the developer:

<strong>Reporter says:</strong> "Here's what's broken/needed, and how to reproduce it."
<strong>Developer says:</strong> "I understand and will fix/build it."

Good issues prevent miscommunication. Bad issues waste everyone's time.

<h3>The Lifecycle</h3>

```
┌────────┐   triage   ┌────────┐   assign   ┌────────────┐
│  Open  │ ─────────► │Labeled │ ─────────► │ In Progress│
└────────┘            └────────┘            └────────────┘
                                                   │
                      ┌────────┐   PR merged  ─────┘
                      │ Closed │ ◄────────────────
                      └────────┘
```"#,
        content: r###"
## Creating Effective Issues

### Bug Report

```markdown
## Description
Brief summary of the bug.

## Steps to Reproduce
1. Go to settings page
2. Click "Save"
3. Observe error message

## Expected Behavior
Settings should save successfully.

## Actual Behavior
Error: "Failed to save preferences"

## Environment
- OS: Ubuntu 22.04
- Browser: Firefox 120
- Version: v2.3.1
```

### Feature Request

```markdown
## Summary
What feature do you want?

## Problem
What problem does this solve?

## Proposed Solution
How should it work?

## Alternatives Considered
What other approaches did you consider?
```

## Labels

| Label | Purpose |
|-------|---------|
| `bug` | Something is broken |
| `enhancement` | New feature request |
| `documentation` | Docs need work |
| `good first issue` | Great for newcomers |
| `help wanted` | Extra attention needed |
| `priority: high` | Needs immediate attention |
| `wontfix` | Won't be addressed |

## Milestones

Group issues by release:
- v1.0 - MVP
- v1.1 - Bug fixes
- v2.0 - Major features

## GitHub Projects (Kanban)

```
┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐
│   Backlog   │  │   To Do     │  │ In Progress │  │    Done     │
├─────────────┤  ├─────────────┤  ├─────────────┤  ├─────────────┤
│ Issue #45   │  │ Issue #42   │  │ Issue #40   │  │ Issue #38   │
│ Issue #46   │  │ Issue #43   │  │ Issue #41   │  │ Issue #39   │
│ Issue #47   │  │             │  │             │  │             │
└─────────────┘  └─────────────┘  └─────────────┘  └─────────────┘
```

## Linking PRs to Issues

```bash
# In commit message (links but doesn't close)
Refs #42

# In PR description (closes on merge)
Closes #42
Fixes #42
Resolves #42
```
"###,
        key_concepts: &["Issues", "Labels", "Milestones", "Projects"],
        concept_definitions: &[
            ("Issues", "Trackable units of work (bugs, features, tasks)"),
            ("Labels", "Categories for organizing issues"),
            ("Milestones", "Groups of issues targeted for a release"),
            ("Projects", "Kanban boards for workflow visualization"),
        ],
        key_takeaways: &[
            "Good issues save time for everyone",
            "Use labels to categorize and prioritize",
            "Link PRs to issues with Closes/Refs",
            "Projects visualize workflow status",
        ],
        dos_and_donts: r###"## ✅ DO

- **Search before creating**: Maybe it's already reported
- **Use templates**: Consistent format helps triage
- **Add reproduction steps**: Essential for bugs
- **Update status**: Move cards on project boards

## ❌ DON'T

- **Don't create vague issues**: "It's broken" is useless
- **Don't create duplicate issues**: Search first
- **Don't mix concerns**: One issue per problem
- **Don't close without explanation**: Say why it's resolved
"###,
        going_deeper: r#"<strong>Issue Templates:</strong>
Create `.github/ISSUE_TEMPLATE/bug_report.md` and `feature_request.md` to standardize submissions.
Templates guide reporters to include necessary information.

<strong>Automation:</strong>
GitHub Actions can auto-label, auto-assign, and auto-close stale issues.
Projects can auto-move cards when PRs are opened/merged."#,
        common_mistakes: r#"<strong>Closing keywords in commits:</strong>
"Closes #42" in a commit message will close the issue when ANY PR containing that commit merges.
Use "Refs #42" in commits, save "Closes #42" for the PR description.

<strong>Orphaned issues:</strong>
Issues without labels or milestones get lost. Triage regularly."#,
    },

    // ═══════════════════════════════════════════════════════════════════════════
    // PHASE 5: ADVANCED WORKFLOWS
    // ═══════════════════════════════════════════════════════════════════════════
    Lesson {
        id: 9,
        title: "Branching Strategies",
        subtitle: "Team Workflows",
        icon: "🔀",
        phase: "Advanced Workflows",
        why_it_matters: "Teams need consistent rules for branching. The right strategy depends on \
                         team size, release frequency, and deployment model.",
        description: "Compare Git Flow, GitHub Flow, and Trunk-Based Development. Choose the right strategy.",
        intuition: r#"<h3>Three Philosophies</h3>

<strong>Git Flow</strong>: For scheduled releases (e.g., mobile apps, enterprise software)
- Many long-lived branches
- Formal release process
- Complex but controlled

<strong>GitHub Flow</strong>: For continuous deployment (e.g., web apps)
- Main is always deployable
- Feature branches, PRs, merge, deploy
- Simple and fast

<strong>Trunk-Based</strong>: For high-velocity teams (e.g., Google, Facebook)
- Everyone commits to main
- Short-lived branches (hours, not days)
- Requires excellent CI/CD"#,
        content: r###"
## Git Flow

```
main (production)     ────●────────────────────●────────►
                          ↑                    ↑
hotfix                    │    ●───●           │
                          │        ↓           │
release                   │    ●───●───●───────│
                          ↑        ↑           ↑
develop              ─────●────●───●───●───────●────────►
                              ↑       ↑
feature/a                ●────●       │
feature/b                    ●────────●
```

**Branches:**
- `main`: Production releases only
- `develop`: Integration branch
- `feature/*`: New features (from develop)
- `release/*`: Release preparation
- `hotfix/*`: Emergency production fixes

## GitHub Flow

```
main (always deployable) ────●────●────●────●────●────►
                              ↑    ↑    ↑    ↑
feature branches         ●────●    │    │    │
                              ●────●    │    │
                                   ●────●    │
                                        ●────●
```

**Rules:**
1. Main is always deployable
2. Branch from main for all changes
3. Open PR for discussion
4. Deploy from branch (optional)
5. Merge after review

## Trunk-Based Development

```
main ────●────●────●────●────●────●────●────●────►
          ↑    ↑    ↑    ↑
         (short-lived feature branches, <1 day)
```

**Requirements:**
- Strong CI/CD pipeline
- Feature flags for incomplete features
- High test coverage
- Small, frequent commits

## Comparison

| Aspect | Git Flow | GitHub Flow | Trunk-Based |
|--------|----------|-------------|-------------|
| Complexity | High | Low | Medium |
| Release cadence | Scheduled | Continuous | Continuous |
| Branch lifespan | Long | Days | Hours |
| Best for | Versioned releases | Web apps | High-velocity teams |
"###,
        key_concepts: &["Git Flow", "GitHub Flow", "Trunk-Based", "Feature Flags"],
        concept_definitions: &[
            ("Git Flow", "Complex branching with main, develop, feature, release, hotfix"),
            ("GitHub Flow", "Simple model: main + feature branches + PRs"),
            ("Trunk-Based", "Everyone commits to main with short-lived branches"),
            ("Feature Flags", "Toggle features on/off without branching"),
        ],
        key_takeaways: &[
            "Git Flow for scheduled releases",
            "GitHub Flow for continuous deployment",
            "Trunk-Based for high-velocity teams",
            "Match strategy to release model",
        ],
        dos_and_donts: r###"## ✅ DO

- **Document your strategy**: Team should agree on conventions
- **Automate what you can**: CI/CD reduces human error
- **Keep branches short-lived**: Long branches = merge pain
- **Review before merge**: Quality gate is essential

## ❌ DON'T

- **Don't mix strategies**: Pick one and stick with it
- **Don't skip CI**: Breaking main is expensive
- **Don't create branch chaos**: Follow naming conventions
"###,
        going_deeper: r#"<strong>Release Branches:</strong>
Git Flow's release branches allow stabilization while develop continues.
Useful for products with support windows (v1.x vs v2.x).

<strong>Feature Flags:</strong>
Alternative to branching—deploy incomplete code with flags off.
Enables trunk-based development without breaking production."#,
        common_mistakes: r#"<strong>Long-lived feature branches:</strong>
The longer a branch lives, the harder merging becomes.
If it's taking weeks, split the feature into smaller pieces.

<strong>Develop branch rot:</strong>
In Git Flow, if develop gets too far ahead of main, release merges become painful.
Do regular releases to keep them synchronized."#,
    },

    Lesson {
        id: 10,
        title: "Advanced Operations",
        subtitle: "Power User Tools",
        icon: "⚡",
        phase: "Advanced Workflows",
        why_it_matters: "These commands separate beginners from power users. Master them to handle \
                         complex situations with confidence.",
        description: "Learn cherry-pick, bisect, stash, and interactive rebase for advanced Git workflows.",
        intuition: r#"<h3>The Power Tools</h3>

<strong>cherry-pick</strong>: Copy a single commit to another branch.
Like copying one chapter from one book to another.

<strong>bisect</strong>: Find which commit introduced a bug.
Binary search through history—incredibly efficient.

<strong>stash</strong>: Temporarily hide changes.
Like putting papers in a drawer while you clean your desk.

<strong>rebase -i</strong>: Rewrite history.
Edit, squash, reorder, or drop commits before sharing."#,
        content: r###"
## Cherry-Pick

```bash
# Copy commit abc123 to current branch
git cherry-pick abc123

# Cherry-pick without committing (just apply changes)
git cherry-pick --no-commit abc123

# Cherry-pick a range
git cherry-pick abc123..def456
```

**Use case:** Backport a bugfix from main to a release branch.

## Bisect (Binary Search for Bugs)

```bash
# Start bisect session
git bisect start

# Mark current commit as bad
git bisect bad

# Mark known good commit
git bisect good v1.0

# Git checks out middle commit
# Test, then mark it:
git bisect good  # or
git bisect bad

# Repeat until Git finds the culprit
# When done:
git bisect reset
```

**Automated bisect:**
```bash
git bisect run ./test-script.sh
# Returns 0 = good, 1+ = bad
```

## Stash

```bash
# Save current changes
git stash

# Save with message
git stash push -m "Work in progress on login"

# List stashes
git stash list

# Apply most recent stash (keep in list)
git stash apply

# Apply and remove from list
git stash pop

# Apply specific stash
git stash apply stash@{2}

# Drop a stash
git stash drop stash@{0}
```

## Interactive Rebase

```bash
# Rebase last 5 commits interactively
git rebase -i HEAD~5
```

Opens editor with commands:
```
pick abc123 First commit
pick def456 Second commit
squash ghi789 Third commit (squash into second)
reword jkl012 Fourth commit (edit message)
drop mno345 Fifth commit (remove entirely)
```

**Commands:**
- `pick`: Keep commit as-is
- `reword`: Keep commit, edit message
- `squash`: Combine with previous commit
- `fixup`: Like squash but discard message
- `drop`: Remove commit entirely
- `edit`: Pause to amend commit
"###,
        key_concepts: &["cherry-pick", "bisect", "stash", "rebase -i"],
        concept_definitions: &[
            ("cherry-pick", "Copy specific commits to current branch"),
            ("bisect", "Binary search to find bug-introducing commit"),
            ("stash", "Temporarily save uncommitted changes"),
            ("rebase -i", "Interactive rebase to edit commit history"),
        ],
        key_takeaways: &[
            "cherry-pick for selective backporting",
            "bisect finds bugs in O(log n) commits",
            "stash saves WIP without committing",
            "rebase -i cleans history before sharing",
        ],
        dos_and_donts: r###"## ✅ DO

- **Use stash for quick context switches**: Don't commit WIP
- **Use bisect for mysterious regressions**: Let Git find the bug
- **Squash before pushing**: Clean history is easier to review
- **Use --fixup with rebase**: `git commit --fixup abc123` + `git rebase -i --autosquash`

## ❌ DON'T

- **Don't cherry-pick carelessly**: Creates duplicate commits
- **Don't rebase -i pushed commits**: Rewrites public history
- **Don't forget stashed changes**: They expire after 90 days
"###,
        going_deeper: r#"<strong>The reflog protects you:</strong>
Even after dangerous operations, reflog keeps references for 90 days.
`git reflog` shows all HEAD movements. You can recover almost anything.

<strong>Autosquash workflow:</strong>
1. Make fix: `git commit --fixup abc123`
2. Later: `git rebase -i --autosquash main`
3. Git automatically reorders and squashes"#,
        common_mistakes: r#"<strong>Cherry-pick conflicts:</strong>
The commit may depend on earlier changes not present in target branch.
You'll need to resolve conflicts manually.

<strong>Interactive rebase on wrong branch:</strong>
Always double-check which branch you're on before `rebase -i`.
`git branch` shows current branch with asterisk."#,
    },

    // ═══════════════════════════════════════════════════════════════════════════
    // PHASE 6: BEST PRACTICES
    // ═══════════════════════════════════════════════════════════════════════════
    Lesson {
        id: 11,
        title: "Commit Hygiene",
        subtitle: "Write History Well",
        icon: "✨",
        phase: "Best Practices",
        why_it_matters: "Commits are the documentation of your project's evolution. Good commit hygiene \
                         makes debugging, reviewing, and understanding code dramatically easier.",
        description: "Master atomic commits, meaningful messages, and what should never be committed.",
        intuition: r#"<h3>The Book Chapter Analogy</h3>

Each commit should be like a book chapter:
- <strong>Focused</strong>: One topic per chapter
- <strong>Complete</strong>: Doesn't leave the reader hanging
- <strong>Titled well</strong>: You can understand the content from the title

<h3>The Atomic Commit</h3>

An atomic commit is the smallest change that makes sense on its own.

<strong>Bad:</strong>
```
"Update files"
- Fix login bug
- Add new feature
- Update README
- Refactor database
```

<strong>Good:</strong>
```
"Fix null pointer in login handler"
"Add password reset feature"
"Update README with new endpoints"
"Refactor database connection pooling"
```

Each can be reverted, cherry-picked, or understood independently."#,
        content: r###"
## The Anatomy of a Good Commit Message

```
feat: add password reset functionality

Implement password reset flow with email verification.
Users can now request a password reset link that expires
after 24 hours.

- Add PasswordResetController
- Create email template
- Add expiration check middleware

Closes #234
```

**Structure:**
1. **Subject line**: 50 chars max, imperative mood
2. **Blank line**
3. **Body**: Explain WHY, not what (the diff shows what)
4. **Footer**: References, breaking changes

## Conventional Commits

```
feat: add user authentication
fix: resolve race condition in queue
docs: update API documentation
style: format code with prettier
refactor: extract validation logic
test: add unit tests for auth
chore: update dependencies
perf: optimize database queries
```

## What to Commit

| ✅ Commit | ❌ Don't Commit |
|----------|----------------|
| Source code | Build artifacts (dist/, target/) |
| Config files | Dependencies (node_modules/) |
| Documentation | Secrets (.env, credentials) |
| Tests | IDE settings (.idea/, .vscode/) |
| .gitignore | Large binaries |

## The .gitignore File

```gitignore
# Dependencies
node_modules/
vendor/

# Build artifacts
dist/
target/
*.pyc
__pycache__/

# Environment
.env
.env.local
*.pem

# IDE
.idea/
.vscode/
*.swp

# OS
.DS_Store
Thumbs.db

# Logs
*.log
logs/
```
"###,
        key_concepts: &["Atomic Commits", "Commit Messages", ".gitignore", "Conventional Commits"],
        concept_definitions: &[
            ("Atomic Commits", "Smallest complete change that makes sense alone"),
            ("Commit Messages", "Documentation of what and why you changed"),
            (".gitignore", "File specifying intentionally untracked files"),
            ("Conventional Commits", "Standardized commit message format"),
        ],
        key_takeaways: &[
            "One logical change per commit",
            "Imperative mood in subject line",
            "Explain WHY in the body",
            "Never commit secrets or build artifacts",
        ],
        dos_and_donts: r###"## ✅ DO

- **Write meaningful subjects**: "Fix auth timeout" not "fix bug"
- **Use imperative mood**: "Add" not "Added" or "Adding"
- **Keep subject under 50 chars**: It's a summary
- **Explain the WHY**: The diff shows the what
- **Reference issues**: `Refs #42` or `Closes #42`

## ❌ DON'T

- **Don't commit secrets**: API keys, passwords, tokens
- **Don't commit generated files**: node_modules, dist, *.pyc
- **Don't use vague messages**: "WIP", "fix", "update"
- **Don't mix concerns**: "Fix bug and add feature" = 2 commits
- **Don't commit broken code to main**: Use branches
"###,
        going_deeper: r#"<strong>Signed commits:</strong>
`git commit -S` signs your commit with your GPG key.
Proves you actually made the commit. GitHub shows "Verified" badge.

<strong>Commit hooks:</strong>
Pre-commit hooks can enforce formatting, run tests, check for secrets.
Tools: Husky (JS), pre-commit (Python), lefthook (Go)."#,
        common_mistakes: r#"<strong>Committing secrets:</strong>
Even if you remove them in the next commit, they're in history forever.
Use tools like `git-secrets` or `gitleaks` to prevent this.

<strong>Mega-commits:</strong>
"Implement entire feature" commits are hard to review and revert.
Break work into logical steps, commit each step."#,
    },

    Lesson {
        id: 12,
        title: "Undo & Recovery",
        subtitle: "Fix Any Mistake",
        icon: "🔧",
        phase: "Best Practices",
        why_it_matters: "Everyone makes mistakes. Git's power is not just tracking changes—it's \
                         the ability to undo almost anything. Know these commands and nothing is permanent.",
        description: "Master reset, revert, reflog, and other recovery techniques for any situation.",
        intuition: r#"<h3>The Safety Net</h3>

Git is like a time machine with multiple layers of protection:

1. <strong>Working changes</strong>: Not saved yet—restore from HEAD
2. <strong>Staged changes</strong>: In index—unstage with reset
3. <strong>Committed locally</strong>: In history—reset or amend
4. <strong>Pushed to remote</strong>: Public—use revert (don't rewrite)

<h3>Reset vs Revert</h3>

<strong>Reset</strong>: Move branch pointer backward, erasing commits from history.
Only use for local, unpushed commits.

<strong>Revert</strong>: Create a NEW commit that undoes a previous commit.
Safe for public history—doesn't rewrite anything."#,
        content: r###"
## Undo Working Changes

```bash
# Discard changes in one file
git restore file.txt

# Discard all changes
git restore .

# Older syntax
git checkout -- file.txt
```

## Undo Staged Changes (Unstage)

```bash
# Unstage file (keep changes in working directory)
git restore --staged file.txt

# Older syntax
git reset HEAD file.txt
```

## Undo Last Commit (Local Only!)

```bash
# Undo commit, keep changes staged
git reset --soft HEAD~1

# Undo commit, keep changes unstaged
git reset --mixed HEAD~1    # This is the default

# Undo commit AND discard changes (DANGEROUS)
git reset --hard HEAD~1
```

## Amend Last Commit

```bash
# Fix commit message
git commit --amend -m "New message"

# Add forgotten files to last commit
git add forgotten_file.txt
git commit --amend --no-edit
```

## Undo Pushed Commits (Safe Way)

```bash
# Create new commit that undoes the changes
git revert abc123

# Revert merge commit (specify parent)
git revert -m 1 abc123
```

## Recover Lost Commits (Reflog)

```bash
# Show all HEAD movements
git reflog

# Output:
# abc123 HEAD@{0}: commit: Latest commit
# def456 HEAD@{1}: commit: Previous commit
# ghi789 HEAD@{2}: reset: moving to HEAD~1  ← Found it!

# Recover
git checkout ghi789           # Detached HEAD
git switch -c recovery        # Create branch to save
# Or
git reset --hard ghi789       # Move branch pointer there
```

## Summary: Undo Scenarios

| Scenario | Command |
|----------|---------|
| Discard working changes | `git restore <file>` |
| Unstage file | `git restore --staged <file>` |
| Undo last commit (keep changes) | `git reset --soft HEAD~1` |
| Undo last commit (discard changes) | `git reset --hard HEAD~1` |
| Undo pushed commit | `git revert <commit>` |
| Find lost commits | `git reflog` |
| Recover lost commit | `git reset --hard <commit>` |
"###,
        key_concepts: &["reset", "revert", "reflog", "restore"],
        concept_definitions: &[
            ("reset", "Move branch pointer backward (rewrites history)"),
            ("revert", "Create commit that undoes another commit"),
            ("reflog", "Log of all HEAD movements—your safety net"),
            ("restore", "Discard working or staged changes"),
        ],
        key_takeaways: &[
            "reset for local commits, revert for pushed commits",
            "reflog keeps commits for 90 days after 'deletion'",
            "--soft keeps changes staged, --hard discards everything",
            "When in doubt, create a backup branch first",
        ],
        dos_and_donts: r###"## ✅ DO

- **Use revert for public history**: Safe—doesn't rewrite
- **Check reflog when panicking**: Your work is probably there
- **Create backup branch before risky operations**: `git branch backup`
- **Understand --soft vs --hard**: Know what you're discarding

## ❌ DON'T

- **Don't reset --hard pushed commits**: Teammates will hate you
- **Don't panic**: Reflog has your back
- **Don't force-push to shared branches**: Unless you really know what you're doing
"###,
        going_deeper: r#"<strong>ORIG_HEAD:</strong>
After operations that move HEAD (merge, rebase, reset), ORIG_HEAD points to where HEAD was before.
`git reset ORIG_HEAD` is a quick undo.

<strong>Recovering deleted branches:</strong>
`git reflog` shows when branches were deleted. Find the last commit and recreate:
`git switch -c recovered abc123`"#,
        common_mistakes: r#"<strong>Force-push after reset:</strong>
If you `git reset` then `git push --force`, everyone who pulled is in trouble.
Their copies now have commits that don't exist on remote.

<strong>Confusing reset modes:</strong>
--soft: Moves HEAD only. Staging and working directory unchanged.
--mixed: Moves HEAD, clears staging. Working directory unchanged.
--hard: Moves HEAD, clears staging AND working directory. Changes gone."#,
    },

    // ═══════════════════════════════════════════════════════════════════════════
    // PHASE 7: SOFTWARE ENGINEERING EXCELLENCE
    // ═══════════════════════════════════════════════════════════════════════════
    Lesson {
        id: 13,
        title: "Monorepos & Scale",
        subtitle: "Enterprise Git Patterns",
        icon: "🏗️",
        phase: "Software Engineering",
        why_it_matters: "As projects grow, you'll face decisions about repository structure. \
                         Understanding monorepos, submodules, and LFS helps you scale effectively.",
        description: "Compare monorepo vs polyrepo. Learn submodules, subtrees, and Git LFS for large files.",
        intuition: r#"<h3>The Repository Philosophy</h3>

<strong>Polyrepo (Many Repositories)</strong>
One repo per project/service. Frontend, backend, mobile each in their own repo.
Simple to understand, clear boundaries, independent versioning.

<strong>Monorepo (Single Repository)</strong>
Everything in one repo. Google, Facebook, Microsoft use monorepos.
Atomic changes across projects, shared tooling, single source of truth.

<h3>Trade-offs</h3>

| Polyrepo | Monorepo |
|----------|----------|
| Clear boundaries | Atomic cross-project changes |
| Independent releases | Shared code/tooling |
| Simpler permissions | Single source of truth |
| Smaller clone size | Better discoverability |
| Harder code sharing | Needs custom tooling at scale |"#,
        content: r###"
## Monorepo Structure

```
monorepo/
├── apps/
│   ├── web/
│   ├── mobile/
│   └── api/
├── packages/
│   ├── shared-utils/
│   ├── design-system/
│   └── types/
├── tools/
│   └── scripts/
└── package.json
```

## Git Submodules

Include another repo inside your repo at a specific commit:

```bash
# Add submodule
git submodule add https://github.com/lib/library.git libs/library

# Clone repo with submodules
git clone --recurse-submodules https://github.com/user/repo.git

# Update submodules after pulling
git submodule update --init --recursive

# Update to latest commit in submodule
cd libs/library
git pull origin main
cd ../..
git add libs/library
git commit -m "Update library submodule"
```

## Git Subtrees

Alternative to submodules—history is merged:

```bash
# Add subtree
git subtree add --prefix=libs/library https://github.com/lib/library.git main --squash

# Pull updates
git subtree pull --prefix=libs/library https://github.com/lib/library.git main --squash

# Push changes back
git subtree push --prefix=libs/library https://github.com/lib/library.git main
```

## Git LFS (Large File Storage)

Store large files (images, videos, binaries) outside the repo:

```bash
# Install LFS
git lfs install

# Track file types
git lfs track "*.psd"
git lfs track "*.mp4"
git lfs track "assets/**"

# Check tracked patterns
cat .gitattributes

# See LFS files
git lfs ls-files
```

**How it works:**
- LFS stores a pointer in Git (small)
- Actual file lives on LFS server
- Downloaded on demand
"###,
        key_concepts: &["Monorepo", "Submodules", "Subtrees", "Git LFS"],
        concept_definitions: &[
            ("Monorepo", "Single repository containing multiple projects"),
            ("Submodules", "Include other Git repos at specific commits"),
            ("Subtrees", "Merge another repo's history into your repo"),
            ("Git LFS", "Store large files outside main Git history"),
        ],
        key_takeaways: &[
            "Monorepo for shared code, polyrepo for independence",
            "Submodules link repos at specific commits",
            "LFS keeps large files from bloating repo",
            "All approaches have trade-offs—choose based on needs",
        ],
        dos_and_donts: r###"## ✅ DO

- **Use LFS for large binaries**: Images, videos, datasets
- **Keep submodules updated**: Stale submodules cause confusion
- **Document your repo structure**: README explaining layout
- **Use monorepo tooling**: Nx, Turborepo, Bazel for large monorepos

## ❌ DON'T

- **Don't commit large binaries directly**: Use LFS
- **Don't ignore submodule changes**: They're easy to miss
- **Don't assume one approach fits all**: Evaluate trade-offs
"###,
        going_deeper: r#"<strong>Sparse Checkout:</strong>
For huge monorepos, clone only the directories you need:
`git sparse-checkout set apps/web packages/shared`

<strong>Monorepo Tooling:</strong>
- Nx: Smart builds, caching, dependency graph
- Turborepo: Incremental builds for JS/TS
- Bazel: Google's build system, extreme scale
- Lerna: JS monorepo management (older)"#,
        common_mistakes: r#"<strong>Submodule commit not pushed:</strong>
If you update a submodule but forget to push it, others can't clone.
Always push submodule changes first, then the parent repo.

<strong>Binary files in history:</strong>
Once committed, binary files live forever in history, even if deleted.
Use LFS from the start, or use `git filter-branch` to remove (complex)."#,
    },

    Lesson {
        id: 14,
        title: "Git Hooks & Automation",
        subtitle: "Automate Quality",
        icon: "🤖",
        phase: "Software Engineering",
        why_it_matters: "Automation catches mistakes before they become problems. Git hooks enforce \
                         quality standards at commit time. CI/CD extends this to the entire pipeline.",
        description: "Set up pre-commit hooks, integrate with CI/CD, and automate code quality checks.",
        intuition: r#"<h3>The Quality Gate</h3>

Hooks are scripts that run automatically at key points:

<strong>Pre-commit</strong>: Before commit is created
- Lint code
- Run formatter
- Check for secrets
- Validate commit message

<strong>Pre-push</strong>: Before push to remote
- Run tests
- Check branch naming

<strong>CI/CD</strong>: After push to remote
- Build project
- Run full test suite
- Deploy to staging/production"#,
        content: r###"
## Git Hook Locations

```
.git/hooks/
├── pre-commit         # Before commit
├── prepare-commit-msg # Edit default message
├── commit-msg         # Validate commit message
├── pre-push           # Before push
├── pre-rebase         # Before rebase
└── post-merge         # After merge
```

## Simple Pre-Commit Hook

```bash
#!/bin/bash
# .git/hooks/pre-commit

# Run linter
npm run lint
if [ $? -ne 0 ]; then
    echo "Linting failed. Fix errors before committing."
    exit 1
fi

# Check for console.log
if git diff --cached | grep -q "console.log"; then
    echo "Warning: console.log found in staged changes"
    read -p "Continue anyway? (y/n) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

exit 0
```

## Using Husky (Node.js)

```bash
# Install
npm install husky --save-dev
npx husky init

# Add pre-commit hook
echo "npm run lint" > .husky/pre-commit

# Add commit-msg hook (with commitlint)
echo "npx --no -- commitlint --edit \$1" > .husky/commit-msg
```

## Using pre-commit (Python)

```yaml
# .pre-commit-config.yaml
repos:
  - repo: https://github.com/pre-commit/pre-commit-hooks
    rev: v4.4.0
    hooks:
      - id: trailing-whitespace
      - id: end-of-file-fixer
      - id: check-yaml
      - id: check-added-large-files

  - repo: https://github.com/psf/black
    rev: 23.3.0
    hooks:
      - id: black

  - repo: https://github.com/gitleaks/gitleaks
    rev: v8.16.1
    hooks:
      - id: gitleaks
```

```bash
# Install
pip install pre-commit
pre-commit install
pre-commit run --all-files
```

## GitHub Actions CI

```yaml
# .github/workflows/ci.yml
name: CI

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Setup Node
        uses: actions/setup-node@v4
        with:
          node-version: '20'

      - name: Install dependencies
        run: npm ci

      - name: Lint
        run: npm run lint

      - name: Test
        run: npm test

      - name: Build
        run: npm run build
```
"###,
        key_concepts: &["Git Hooks", "Pre-commit", "CI/CD", "GitHub Actions"],
        concept_definitions: &[
            ("Git Hooks", "Scripts triggered by Git events"),
            ("Pre-commit", "Hook that runs before commit is created"),
            ("CI/CD", "Continuous Integration/Deployment automation"),
            ("GitHub Actions", "GitHub's built-in CI/CD platform"),
        ],
        key_takeaways: &[
            "Hooks catch issues before they're committed",
            "Use tools like Husky or pre-commit for easy setup",
            "CI/CD runs on every push—comprehensive testing",
            "Automate everything that can be automated",
        ],
        dos_and_donts: r###"## ✅ DO

- **Keep hooks fast**: Slow hooks frustrate developers
- **Share hooks via config**: Husky, pre-commit, lefthook
- **Scan for secrets**: Use gitleaks or git-secrets
- **Run tests in CI**: Full suite, every push

## ❌ DON'T

- **Don't skip hooks habitually**: --no-verify should be rare
- **Don't put slow tests in pre-commit**: Save those for CI
- **Don't ignore CI failures**: Fix them immediately
"###,
        going_deeper: r#"<strong>Branch Protection Rules:</strong>
GitHub can require:
- PRs before merging
- Passing CI checks
- Code review approvals
- Signed commits
Configure in Settings > Branches.

<strong>Secret Scanning:</strong>
GitHub automatically scans for leaked secrets.
Add `gitleaks` to pre-commit for local protection."#,
        common_mistakes: r#"<strong>Hooks not shared:</strong>
.git/hooks/ isn't tracked. Use Husky, pre-commit, or commit scripts to share.

<strong>Skipping hooks too often:</strong>
If everyone uses `--no-verify`, hooks are useless.
If hooks are too slow/annoying, fix them rather than bypassing."#,
    },

    Lesson {
        id: 15,
        title: "Worktrees",
        subtitle: "Multiple Working Directories",
        icon: "📂",
        phase: "Advanced Workflows",
        why_it_matters: "Ever needed to check something on another branch without losing your current work? \
                         Worktrees let you have multiple branches checked out simultaneously in different directories.",
        description: "Use git worktree to work on multiple branches in parallel without stashing or committing.",
        intuition: r#"<h3>The Multiple Desks Analogy</h3>

Imagine you're working on a complex project at your desk. Papers are spread everywhere—you're in the middle of something important.

Suddenly, your boss asks you to quickly review yesterday's report. With a single desk, you'd have to:
1. Stack all your papers neatly (git stash)
2. Pull out yesterday's papers
3. Review them
4. Put them away
5. Unstack your original papers (git stash pop)

<strong>With multiple desks</strong> (worktrees):
1. Walk to another desk that already has yesterday's papers
2. Review them
3. Walk back to your original desk—nothing moved

<h3>How Worktrees Work</h3>

```
~/projects/myapp/           # Main worktree (main branch)
~/projects/myapp-feature/   # Feature worktree (feature/login branch)
~/projects/myapp-hotfix/    # Hotfix worktree (hotfix/security branch)
```

All three share the same .git database but have separate working directories.
No stashing. No lost context. No commit-before-switching."#,
        content: r###"
## Creating Worktrees

```bash
# Create worktree for existing branch
git worktree add ../myapp-feature feature/login

# Create worktree with new branch
git worktree add -b hotfix/urgent ../myapp-hotfix main

# Create worktree at specific commit
git worktree add ../myapp-v1 v1.0.0
```

## Managing Worktrees

```bash
# List all worktrees
git worktree list

# Output:
# /home/user/myapp         abc123 [main]
# /home/user/myapp-feature def456 [feature/login]
# /home/user/myapp-hotfix  ghi789 [hotfix/urgent]

# Remove worktree (after deleting directory)
rm -rf ../myapp-hotfix
git worktree prune

# Or remove directly
git worktree remove ../myapp-hotfix

# Lock worktree (prevent accidental deletion)
git worktree lock ../myapp-feature
git worktree unlock ../myapp-feature
```

## Use Cases

### 1. Quick Bug Fix While Developing

```bash
# You're on feature branch, need to fix production bug
git worktree add -b hotfix/crash ../hotfix main
cd ../hotfix
# Fix bug, commit, push, PR
cd ../myapp
git worktree remove ../hotfix
# Continue feature work—nothing was disturbed
```

### 2. Reviewing Pull Requests

```bash
# Check out PR branch for testing
git fetch origin pull/123/head:pr-123
git worktree add ../pr-review pr-123
cd ../pr-review
# Test, review, comment
git worktree remove ../pr-review
```

### 3. Comparing Versions

```bash
# Check out old version for comparison
git worktree add ../v1-reference v1.0.0
# Now you can diff, test, or run both versions simultaneously
```

### 4. Long-Running Tasks

```bash
# Build takes 30 minutes? Keep working on another branch
git worktree add ../myapp-build main
cd ../myapp-build
make release &
cd ../myapp
# Continue development while build runs
```

## Worktrees vs Alternatives

| Scenario | Worktree | Stash | Clone |
|----------|----------|-------|-------|
| Quick branch switch | ✅ Instant | ⚠️ Slower | ❌ Slow |
| Parallel branches | ✅ Easy | ❌ Not possible | ✅ Possible |
| Shared history | ✅ Yes | ✅ Yes | ⚠️ Separate |
| Disk space | ✅ Minimal | ✅ None | ❌ Full copy |
| Complexity | ⚠️ Medium | ✅ Low | ⚠️ Medium |
"###,
        key_concepts: &["Worktree", "git worktree add", "git worktree list", "Parallel Development"],
        concept_definitions: &[
            ("Worktree", "Additional working directory linked to same repository"),
            ("git worktree add", "Create new worktree for a branch"),
            ("git worktree list", "Show all worktrees for this repository"),
            ("Parallel Development", "Working on multiple branches simultaneously"),
        ],
        key_takeaways: &[
            "Worktrees = multiple branches checked out simultaneously",
            "Share same .git database, minimal disk space",
            "No stashing or committing required to switch context",
            "Perfect for hotfixes, PR reviews, comparisons",
        ],
        dos_and_donts: r###"## ✅ DO

- **Use for context switching**: Hotfixes while developing features
- **Use for PR reviews**: Test branches without disrupting work
- **Clean up after yourself**: Remove worktrees when done
- **Lock important worktrees**: Prevent accidental removal

## ❌ DON'T

- **Don't checkout same branch in multiple worktrees**: Git prevents this
- **Don't forget to prune**: `git worktree prune` cleans stale entries
- **Don't leave worktrees forever**: They can get stale and confusing
"###,
        going_deeper: r#"<strong>Bare repositories with worktrees:</strong>
Advanced setup: Clone as bare repo, add worktrees for each branch.
`git clone --bare repo.git .bare`
`git worktree add main main`
All branches are worktrees, no "main" working directory.

<strong>IDE support:</strong>
Most IDEs handle worktrees well—each worktree opens as a separate project.
VSCode, IntelliJ, and others recognize worktree structure."#,
        common_mistakes: r#"<strong>Same branch in multiple worktrees:</strong>
Git prevents this because it would cause confusion—which worktree's changes win?
If you need this, one worktree should be detached HEAD.

<strong>Forgetting worktrees exist:</strong>
`git worktree list` shows all worktrees. Check periodically and clean up stale ones."#,
    },

    Lesson {
        id: 16,
        title: "Security & Signing",
        subtitle: "Verified Commits & Secrets",
        icon: "🔐",
        phase: "Software Engineering",
        why_it_matters: "Anyone can set git config user.name to 'Linus Torvalds'. Signed commits prove \
                         identity cryptographically. And leaked secrets in Git history are a nightmare.",
        description: "Sign commits with GPG/SSH, protect secrets, and understand Git security best practices.",
        intuition: r#"<h3>The Identity Problem</h3>

Git's user.name and user.email are just text—anyone can fake them:

```bash
git config user.name "Elon Musk"
git config user.email "elon@tesla.com"
git commit -m "Fire everyone"
```

This commit would <em>look like</em> it came from Elon. Scary, right?

<strong>Signed commits</strong> solve this. They use cryptographic signatures that can only be
created with your private key. GitHub shows a "Verified" badge on signed commits.

<h3>The Secrets Problem</h3>

Git remembers <em>everything</em>. If you commit an API key and then delete it,
the key is still in history. Forever. Anyone who clones can find it.

```bash
# This shows ALL content ever committed
git log -p --all | grep "API_KEY"
```

Prevention is the only real solution—once secrets are in history,
you need to rewrite history (painful) or rotate the credentials (essential)."#,
        content: r###"
## GPG Signed Commits

### Setup GPG Key

```bash
# Generate key pair
gpg --full-generate-key
# Choose: RSA and RSA, 4096 bits, no expiration

# List keys (get the KEY_ID)
gpg --list-secret-keys --keyid-format=long
# Output: sec rsa4096/KEY_ID 2024-01-01 [SC]

# Export public key (add to GitHub)
gpg --armor --export KEY_ID

# Configure Git
git config --global user.signingkey KEY_ID
git config --global commit.gpgsign true   # Sign all commits
```

### Signing Commits

```bash
# Sign a single commit
git commit -S -m "Signed commit"

# Verify signatures
git log --show-signature

# Verify specific commit
git verify-commit abc123
```

## SSH Signed Commits (Git 2.34+)

```bash
# Configure SSH signing
git config --global gpg.format ssh
git config --global user.signingkey ~/.ssh/id_ed25519.pub

# Sign commits (same as GPG)
git commit -S -m "SSH signed commit"
```

## Protecting Secrets

### Prevention: .gitignore

```gitignore
# Environment files
.env
.env.local
.env.*.local

# Credentials
*.pem
*.key
credentials.json
secrets.yaml

# IDE with potential secrets
.idea/
.vscode/settings.json
```

### Prevention: git-secrets

```bash
# Install
brew install git-secrets  # macOS
# or build from source

# Initialize in repo
git secrets --install

# Add AWS patterns
git secrets --register-aws

# Add custom patterns
git secrets --add 'password\s*=\s*.+'
git secrets --add --allowed 'password = "example"'

# Scan history
git secrets --scan-history
```

### Prevention: Pre-commit Hooks

```yaml
# .pre-commit-config.yaml
repos:
  - repo: https://github.com/gitleaks/gitleaks
    rev: v8.18.0
    hooks:
      - id: gitleaks
```

### Response: If Secrets Are Leaked

```bash
# 1. Immediately rotate the credential!
# This is the most important step.

# 2. Remove from history (if necessary)
# Use BFG Repo-Cleaner (faster than filter-branch)
bfg --replace-text secrets.txt repo.git

# 3. Force push (coordinate with team)
git push --force --all

# 4. Notify affected parties
```

## GitHub Security Features

| Feature | Purpose |
|---------|---------|
| Secret scanning | Alerts on committed secrets |
| Dependabot | Security updates for dependencies |
| Code scanning | SAST analysis |
| Branch protection | Require signed commits |
| Verified badge | Shows commit is signed |
"###,
        key_concepts: &["Signed Commits", "GPG", "git-secrets", "Verified Badge"],
        concept_definitions: &[
            ("Signed Commits", "Cryptographically prove commit authorship"),
            ("GPG", "GNU Privacy Guard—encryption and signing tool"),
            ("git-secrets", "Tool to prevent committing secrets"),
            ("Verified Badge", "GitHub indicator that commit signature is valid"),
        ],
        key_takeaways: &[
            "Signed commits prove identity cryptographically",
            "GPG or SSH keys can sign commits",
            "Once secrets are in history, they're there forever",
            "Prevention > cure: use .gitignore and pre-commit hooks",
        ],
        dos_and_donts: r###"## ✅ DO

- **Sign commits on important projects**: Open source, corporate
- **Use git-secrets or gitleaks**: Prevent accidents
- **Rotate leaked credentials immediately**: Don't just delete from repo
- **Enable GitHub secret scanning**: Free protection
- **Add .env to .gitignore before first commit**: Prevention is easier

## ❌ DON'T

- **Don't commit secrets, ever**: Not even "temporarily"
- **Don't rely on .gitignore after commit**: It's too late
- **Don't share GPG private keys**: One key per person
- **Don't ignore secret scanning alerts**: They're real problems
"###,
        going_deeper: r#"<strong>Requiring signed commits:</strong>
GitHub branch protection can require all commits to be signed.
Settings > Branches > Branch protection > Require signed commits.

<strong>Vigilant mode:</strong>
GitHub can mark unsigned commits as "Unverified" rather than just not showing a badge.
Helps identify potentially spoofed commits.

<strong>Rewriting history:</strong>
BFG Repo-Cleaner is 10-100x faster than git filter-branch for removing secrets.
But it requires force-push and coordination—everyone must re-clone."#,
        common_mistakes: r#"<strong>Deleting secret doesn't remove it:</strong>
`git rm .env && git commit` leaves .env in history.
You need to rewrite history with BFG or filter-branch.

<strong>GPG key expiration:</strong>
If your GPG key expires, old commits still show as verified.
But you can't make new signed commits until you extend or replace the key."#,
    },

    Lesson {
        id: 17,
        title: "The Dos and Don'ts",
        subtitle: "Git Best Practices Reference",
        icon: "📋",
        phase: "Best Practices",
        why_it_matters: "After 17 lessons, here's your cheat sheet. A comprehensive reference of \
                         everything you should and shouldn't do with Git, organized by category.",
        description: "The ultimate Git best practices reference card. Print it, bookmark it, live by it.",
        intuition: r#"<h3>The Master Reference</h3>

This lesson consolidates all the dos and don'ts from the entire curriculum.
Use it as a quick reference when you're unsure about best practices.

<h3>The Golden Rules</h3>

1. <strong>Commit early, commit often</strong>—small commits are easier to understand and revert
2. <strong>Never rebase public history</strong>—you'll break everyone's clones
3. <strong>Never commit secrets</strong>—they're in history forever
4. <strong>Write meaningful commit messages</strong>—your future self will thank you
5. <strong>Use branches for everything</strong>—main should always be deployable"#,
        content: r###"
## Commits

| ✅ DO | ❌ DON'T |
|-------|----------|
| Commit early and often | Batch many changes into one commit |
| Write meaningful messages | Use "fix", "update", "WIP" |
| Use imperative mood: "Add feature" | Use past tense: "Added feature" |
| Keep commits atomic (one purpose) | Mix unrelated changes |
| Reference issues: Refs #42 | Leave commits orphaned |

### Commit Message Template
```
type: short summary (50 chars)

Longer explanation of WHY this change was needed.
The diff shows WHAT changed; the message explains WHY.

Refs #42
```

## Branches

| ✅ DO | ❌ DON'T |
|-------|----------|
| Use feature branches | Work directly on main |
| Use descriptive names: `feature/user-auth` | Use vague names: `my-branch` |
| Delete merged branches | Let stale branches accumulate |
| Keep branches short-lived | Let branches diverge for weeks |
| Pull/rebase from main regularly | Wait until merge to discover conflicts |

## History

| ✅ DO | ❌ DON'T |
|-------|----------|
| Rebase local branches before pushing | Rebase pushed/shared branches |
| Use `revert` for public history | Use `reset` on public history |
| Squash before merging (if team prefers) | Leave "fix typo" commits in PR |
| Use interactive rebase for cleanup | Force-push to shared branches |

## Collaboration

| ✅ DO | ❌ DON'T |
|-------|----------|
| Pull before pushing | Force-push without team agreement |
| Write clear PR descriptions | Open PRs without context |
| Respond to review feedback | Ignore or argue with reviewers |
| Keep PRs small and focused | Create 2000-line PRs |
| Test before opening PR | Push broken code for review |

## Remotes

| ✅ DO | ❌ DON'T |
|-------|----------|
| Use SSH keys for auth | Commit with HTTPS and passwords |
| Fetch before making assumptions | Assume your local is current |
| Name remotes clearly (origin, upstream) | Use confusing remote names |
| Push with `-u` to set upstream | Manually specify remote every time |

## Security

| ✅ DO | ❌ DON'T |
|-------|----------|
| Use .gitignore for secrets | Commit .env files, even "just for testing" |
| Use git-secrets or gitleaks | Rely on manual review |
| Sign commits on important repos | Assume user.name is trustworthy |
| Rotate leaked credentials immediately | Just delete the file and push |
| Enable GitHub secret scanning | Ignore security alerts |

## Recovery

| ✅ DO | ❌ DON'T |
|-------|----------|
| Check reflog when panicking | Panic and think data is lost |
| Use `revert` for pushed commits | Use `reset --hard` on pushed commits |
| Create backup branch before risky ops | YOLO dangerous operations |
| Understand reset modes (soft/mixed/hard) | Use `--hard` without thinking |

## Workflow

| ✅ DO | ❌ DON'T |
|-------|----------|
| Agree on team workflow | Mix Git Flow and GitHub Flow |
| Document your conventions | Assume everyone knows |
| Use CI/CD for testing | Rely only on local tests |
| Require PR reviews | Merge your own PRs |
| Use branch protection rules | Allow direct pushes to main |

## Organization

| ✅ DO | ❌ DON'T |
|-------|----------|
| Use clear issue descriptions | Create vague issues |
| Link PRs to issues | Leave issues orphaned |
| Use labels and milestones | Let issues pile up without organization |
| Close issues with context | Close without explanation |

## Large Files & Scale

| ✅ DO | ❌ DON'T |
|-------|----------|
| Use Git LFS for binaries | Commit large files directly |
| Use sparse checkout for monorepos | Clone entire monorepo unnecessarily |
| Clean up submodules | Let submodules go stale |
| Keep repo size reasonable | Let history bloat with binaries |
"###,
        key_concepts: &["Best Practices", "Git Hygiene", "Team Conventions", "Reference Card"],
        concept_definitions: &[
            ("Best Practices", "Proven patterns that prevent problems"),
            ("Git Hygiene", "Habits that keep repository clean and useful"),
            ("Team Conventions", "Agreed-upon standards for consistency"),
            ("Reference Card", "Quick-lookup guide for common decisions"),
        ],
        key_takeaways: &[
            "Commit early, often, and meaningfully",
            "Never rebase public history",
            "Never commit secrets",
            "Use branches for everything",
            "When in doubt, check the reflog",
        ],
        dos_and_donts: "",  // This IS the dos and don'ts lesson!
        going_deeper: r#"<strong>Team onboarding:</strong>
Use this reference as part of developer onboarding.
New team members should read this before their first commit.

<strong>Automation:</strong>
Many of these rules can be enforced with:
- Pre-commit hooks (formatting, secrets)
- Branch protection rules (require reviews)
- CI/CD (require passing tests)

<strong>Continuous improvement:</strong>
Update this list as your team learns from mistakes.
Every incident is an opportunity to add a new guideline."#,
        common_mistakes: r#"<strong>Not having documented conventions:</strong>
"Everyone knows how we do things" is a myth.
Write it down. This lesson is a template.

<strong>Enforcing too late:</strong>
Catch problems at commit time with hooks, not at PR time with reviews.
The earlier you catch issues, the cheaper they are to fix."#,
    },
];
