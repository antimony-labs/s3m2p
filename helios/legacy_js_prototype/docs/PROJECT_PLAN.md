# GitHub Portfolio - Project Plan

## Overview
Create a unified website that aggregates all GitHub repositories with visibility controls (public, private, invite-only).

## Features

### Phase 1: Basic Setup
- [x] Project structure
- [ ] GitHub API integration
- [ ] Basic repo fetching
- [ ] Simple frontend

### Phase 2: Visibility Controls
- [ ] Database schema for visibility settings
- [ ] Public/Private/Invite-only categorization
- [ ] Admin dashboard for managing visibility
- [ ] Invite code generation

### Phase 3: Authentication
- [ ] GitHub OAuth integration
- [ ] User authentication
- [ ] Session management
- [ ] Access control middleware

### Phase 4: Frontend
- [ ] Repo grid/list view
- [ ] Filtering by visibility
- [ ] Search functionality
- [ ] Project detail pages
- [ ] Responsive design

### Phase 5: Advanced Features
- [ ] Invite link sharing
- [ ] Analytics/views tracking
- [ ] Custom project descriptions
- [ ] Featured projects
- [ ] Tags/categories

## Database Schema

### Repositories
- id (primary key)
- github_repo_id
- full_name
- visibility_type (public/private/invite_only)
- invite_code (nullable)
- custom_description (nullable)
- featured (boolean)
- created_at
- updated_at

### Invites
- id (primary key)
- repo_id (foreign key)
- invite_code (unique)
- expires_at (nullable)
- usage_count
- max_uses (nullable)
- created_at

### Users
- id (primary key)
- github_id
- username
- access_token (encrypted)
- created_at

## API Endpoints

### Public
- `GET /api/repos/public` - Get public repos
- `GET /api/repos/:id` - Get repo details (if accessible)

### Authenticated
- `GET /api/repos` - Get all accessible repos
- `GET /api/repos/private` - Get private repos
- `POST /api/repos/:id/visibility` - Update visibility

### Admin
- `GET /api/admin/repos` - Get all repos
- `POST /api/admin/repos/:id/visibility` - Set visibility
- `POST /api/admin/repos/:id/invite` - Generate invite code

## Tech Stack Decisions

### Frontend: Next.js
- Server-side rendering
- Built-in API routes
- Easy deployment (Vercel)
- Great TypeScript support

### Backend: Python FastAPI (optional alternative)
- Fast development
- Auto API docs
- Easy GitHub API integration

### Database: PostgreSQL
- Reliable
- Good for relational data
- Free tier available (Supabase, Railway)

### Authentication: NextAuth.js
- GitHub OAuth support
- Session management
- Easy integration

## Deployment

### Frontend: Vercel
- Free tier
- Automatic deployments
- Custom domain support

### Backend: Railway/Render
- Free tier available
- Easy PostgreSQL setup
- Environment variables

### Database: Supabase/Neon
- Free PostgreSQL
- Good API
- Easy migrations

