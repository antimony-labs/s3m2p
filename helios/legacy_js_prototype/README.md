# GitHub Portfolio

A unified website that combines all your GitHub repositories with public, private, and invite-only visibility controls.

## 🎯 Goal

Create a single portfolio website where you can:
- Display all your GitHub repos
- Control visibility (public, private, invite-only)
- Share invite links for private projects
- Manage everything from one dashboard

## 🚀 Quick Start

```bash
cd /root/repos/scratchpad/projects/github-portfolio
./scripts/setup.sh
cd portfolio/frontend
npm run dev
```

## 🌐 Deployment

**Deploy with automatic DNS configuration:**

```bash
./deploy.sh
```

This will:
1. Build the Next.js app
2. Deploy to Vercel
3. **Automatically configure Cloudflare DNS** for `too.foo` and `me.too.foo`

**Manual DNS configuration:**

```bash
# Configure apex domain
python3 scripts/configure-dns.py

# Configure subdomain
python3 scripts/configure-dns.py me
```

See [DNS Configuration Guide](docs/DNS_CONFIGURATION.md) for details.

## 📁 Repository Structure

```
github-portfolio/           # THE ONLY REPO ON YOUR GITHUB
├── README.md              # Portfolio homepage
├── project.json           # Portfolio metadata
├── portfolio/             # Portfolio website code
│   ├── frontend/          # Next.js app
│   ├── backend/           # API server
│   └── public/            # Static assets
├── projects/              # ALL YOUR PROJECTS HERE ⭐
│   ├── web/
│   │   ├── project-1/
│   │   │   ├── project.json
│   │   │   ├── README.md
│   │   │   └── src/
│   │   └── project-2/
│   ├── python/
│   │   └── data-analysis/
│   ├── ai/
│   │   └── chatbot/
│   └── ...                # More projects nested here
├── docs/                  # Documentation
├── scripts/               # Utility scripts
└── config/                # Configuration files
```

## 🔧 Setup

1. **Install dependencies:**
   ```bash
   ./scripts/setup.sh
   ```

2. **Configure environment:**
   - Copy `.env.example` to `.env`
   - Add your GitHub token and OAuth credentials

3. **Run development server:**
   ```bash
   npm run dev
   ```

## 🌳 Parallel Feature Development

**Work on multiple features simultaneously without conflicts!**

```bash
# Start a new feature
npm run task "Your feature description"

# List active worktrees
npm run worktrees
```

📖 **Documentation:**
- [Quick Reference](WORKTREE_CHEATSHEET.md) - One-page cheat sheet
- [Complete Guide](WORKTREE_GUIDE.md) - Full workflow documentation

## 📚 Documentation

- [Worktree Guide](WORKTREE_GUIDE.md) - **Parallel feature development** ⭐
- [Project Plan](docs/PROJECT_PLAN.md) - Detailed feature breakdown
- [Architecture](docs/ARCHITECTURE.md) - System design and data flow
- [Configuration](config/README.md) - Environment setup
- [Testing Guide](docs/TESTING.md) - Unit, integration, and visual test workflows

## ✅ Testing

```bash
npm test                     # unit + integration (Vitest)
npm run test:visual          # Playwright regression (requires baselines)
npm run test:visual:update   # rebuild site + refresh screenshots
```

## 🛠 Tech Stack

- **Frontend:** Next.js + React + Tailwind CSS
- **Backend:** Node.js API Routes (or Python FastAPI)
- **Database:** PostgreSQL (Supabase/Neon)
- **Auth:** NextAuth.js with GitHub OAuth
- **Deployment:** Vercel (frontend) + Railway (backend)

## ✨ Features

### Project Organization
- ✅ **Nested Structure** - Projects organized by category/type
- 📁 **Auto-Discovery** - Portfolio scans `/projects/` automatically
- 📋 **Metadata Files** - Each project has `project.json` for display
- 🔍 **Search & Filter** - Find projects by category, language, tags

### Visibility Controls
- 🌐 **Public** - Visible to everyone
- 🔒 **Private** - Visible only to authenticated users
- 🎫 **Invite-Only** - Shareable invite links

### Navigation
- 📂 **Category Pages** - Browse by type (`/category/web`, `/category/python`)
- 🏷️ **Tag Filtering** - Filter by tags
- 🔎 **Search** - Search across all projects
- 📄 **Project Pages** - Individual pages for each project

### Benefits
- **Clean GitHub Profile** - Only 1 repo visible
- **Organized** - All projects in one place
- **Easy Backup** - Clone one repo = everything
- **Version Control** - All projects versioned together

## 🎨 Next Steps

1. Set up Next.js frontend
2. Create database schema
3. Implement GitHub API integration
4. Build admin dashboard
5. Add authentication
6. Deploy!
