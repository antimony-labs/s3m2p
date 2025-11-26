# GitHub Portfolio - Monorepo Structure

A unified portfolio website where ALL your projects live in ONE GitHub repository, organized in a navigable structure.

## 🎯 Concept

Instead of having 50+ repos cluttering your GitHub profile, you have:
- **1 Main Repository** - Your portfolio website
- **All Projects Nested** - Organized in `/projects/` directory
- **Single Source of Truth** - Everything in one place

## 📁 Repository Structure

```
github-portfolio/
├── README.md                 # Portfolio homepage
├── package.json              # Main dependencies
├── .github/                  # GitHub configs
│   └── workflows/           # CI/CD
├── portfolio/                # Portfolio website code
│   ├── frontend/            # Next.js app
│   ├── backend/             # API server
│   └── public/              # Static assets
├── projects/                 # ALL YOUR PROJECTS HERE
│   ├── project-1/           # Project 1
│   │   ├── README.md
│   │   ├── project.json
│   │   └── src/
│   ├── project-2/           # Project 2
│   │   ├── README.md
│   │   └── ...
│   ├── python/              # Python projects
│   │   ├── project-a/
│   │   └── project-b/
│   ├── web/                 # Web projects
│   │   ├── project-x/
│   │   └── project-y/
│   └── ...                  # More projects
├── docs/                    # Documentation
├── scripts/                 # Utility scripts
└── config/                  # Configuration files
```

## 🎨 Features

### Project Organization
- **Nested Structure** - Projects organized by category/type
- **Individual READMEs** - Each project has its own docs
- **Custom Metadata** - Each project has a `project.json` for display
- **Visibility Control** - Public/Private/Invite-only per project

### Portfolio Website
- **Auto-Discovery** - Scans `/projects/` directory
- **Category Navigation** - Browse by type/language
- **Search** - Find projects easily
- **Project Pages** - Dedicated pages for each project

## 📋 Project Metadata Format

Each project has a `project.json`:

```json
{
  "name": "Project Name",
  "description": "Short description",
  "category": "web|python|mobile|ai|etc",
  "language": "JavaScript|Python|TypeScript|etc",
  "visibility": "public|private|invite_only",
  "invite_code": null,
  "featured": false,
  "tags": ["tag1", "tag2"],
  "live_url": "https://example.com",
  "github_url": null,
  "thumbnail": "thumbnail.png",
  "created": "2024-01-01",
  "updated": "2024-11-01"
}
```

## 🔍 How It Works

1. **Projects in `/projects/`** - All your work lives here
2. **Portfolio Scans** - Website scans directory structure
3. **Metadata Parsing** - Reads `project.json` files
4. **Dynamic Pages** - Generates pages for each project
5. **Category Views** - Groups by category/language
6. **Search Index** - Builds searchable index

## 🚀 Benefits

✅ **Clean GitHub Profile** - Only 1 repo visible
✅ **Organized Structure** - All projects in one place
✅ **Easy Navigation** - Browse all projects on website
✅ **Single Deployment** - One repo to manage
✅ **Version Control** - All projects versioned together
✅ **Easy Backups** - Clone one repo = everything

## 📖 Example Structure

```
projects/
├── ai/
│   ├── chatbot/
│   │   ├── README.md
│   │   ├── project.json
│   │   └── src/
│   └── image-classifier/
│       ├── README.md
│       ├── project.json
│       └── models/
├── web/
│   ├── e-commerce/
│   │   ├── README.md
│   │   ├── project.json
│   │   └── frontend/
│   └── portfolio-site/
│       ├── README.md
│       ├── project.json
│       └── src/
└── python/
    ├── data-analysis/
    │   ├── README.md
    │   ├── project.json
    │   └── notebooks/
    └── api-service/
        ├── README.md
        ├── project.json
        └── app/
```

