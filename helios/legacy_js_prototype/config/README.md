# Configuration

## Environment Variables

```bash
# GitHub
GITHUB_TOKEN=your_github_personal_access_token
GITHUB_USERNAME=your_username

# Authentication
NEXTAUTH_URL=http://localhost:3000
NEXTAUTH_SECRET=your_secret_key_here
GITHUB_CLIENT_ID=your_github_oauth_app_client_id
GITHUB_CLIENT_SECRET=your_github_oauth_app_client_secret

# Database
DATABASE_URL=postgresql://user:password@localhost:5432/github_portfolio

# Optional
NODE_ENV=development
```

## GitHub OAuth Setup

1. Go to GitHub Settings → Developer settings → OAuth Apps
2. Create new OAuth App
3. Set Authorization callback URL: `http://localhost:3000/api/auth/callback/github`
4. Copy Client ID and Client Secret

## GitHub Personal Access Token

1. Go to GitHub Settings → Developer settings → Personal access tokens
2. Generate new token with scopes:
   - `repo` (for private repos)
   - `read:user` (for user info)
3. Save token securely

