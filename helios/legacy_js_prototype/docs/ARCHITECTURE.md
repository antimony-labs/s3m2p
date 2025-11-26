# GitHub Portfolio - Architecture

## Overview

A unified portfolio website that aggregates all GitHub repositories with granular visibility controls.

## System Architecture

```
┌─────────────────┐
│   Frontend      │  Next.js App
│   (Vercel)      │  - Public pages
│                 │  - Admin dashboard
│                 │  - Auth pages
└────────┬────────┘
         │
         │ HTTPS API Calls
         │
┌────────▼────────┐
│   Backend API   │  Next.js API Routes / FastAPI
│   (Railway)     │  - GitHub API proxy
│                 │  - Auth endpoints
│                 │  - Visibility management
└────────┬────────┘
         │
         │ Query
         │
┌────────▼────────┐
│   Database      │  PostgreSQL
│   (Supabase)    │  - Repo visibility settings
│                 │  - Invite codes
│                 │  - User sessions
└─────────────────┘
         │
         │ GitHub API
         │
┌────────▼────────┐
│   GitHub API    │
│   - Repo data   │
│   - Auth        │
└─────────────────┘
```

## Data Flow

### Public Repos
1. User visits website
2. Frontend fetches `/api/repos/public`
3. Backend queries GitHub API
4. Returns formatted repo data
5. Frontend displays repos

### Private Repos
1. User authenticates via GitHub OAuth
2. Frontend fetches `/api/repos` with auth token
3. Backend verifies token
4. Fetches repos from GitHub API
5. Filters by visibility settings from DB
6. Returns accessible repos

### Invite-Only Repos
1. User visits with invite code: `/repo/:id?invite=CODE`
2. Backend validates invite code
3. Checks expiry and usage limits
4. Grants temporary access
5. Shows repo details

## Components

### Frontend Components
- `RepoGrid` - Grid view of repositories
- `RepoCard` - Individual repo card
- `RepoFilter` - Filter by visibility/language
- `AdminDashboard` - Manage visibility settings
- `InviteGenerator` - Create invite codes

### Backend Services
- `GitHubService` - GitHub API integration
- `VisibilityService` - Manage visibility settings
- `InviteService` - Generate/validate invites
- `AuthService` - Handle authentication

## Security

- All API calls authenticated
- Invite codes expire and have usage limits
- Database credentials encrypted
- GitHub tokens stored securely
- Rate limiting on API endpoints

