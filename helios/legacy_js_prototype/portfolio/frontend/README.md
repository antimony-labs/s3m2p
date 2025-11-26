# Dynamic Research Lab Platform

## 🚀 Getting Started

### Prerequisites
- Node.js 18+
- PostgreSQL (for database)
- Redis (for caching)
- Cloudflare account (for deployment)

### Installation

```bash
cd portfolio/frontend
npm install
```

### Environment Setup

Create `.env.local`:

```bash
# Database
DATABASE_URL="postgresql://user:password@localhost:5432/lab_platform"

# Redis
REDIS_URL="redis://localhost:6379"

# Authentication
NEXTAUTH_URL="http://localhost:3000"
NEXTAUTH_SECRET="your-secret-key"

# Cloudflare
CLOUDFLARE_API_TOKEN="your-token"
CLOUDFLARE_ACCOUNT_ID="your-account-id"

# App
NODE_ENV="development"
```

### Development

```bash
npm run dev
```

Visit http://localhost:3000

## 🏗️ Architecture

### Dynamic Features
- **Server-Side Rendering** - Real-time data
- **API Routes** - Backend functionality
- **WebSockets** - Live updates
- **Database Integration** - Persistent storage
- **Authentication** - User management

### Deployment

Since this is NOT static, deploy to:
- **Cloudflare Workers** (with Pages Functions)
- **Vercel** (supports Next.js SSR)
- **Railway** (with Node.js runtime)
- **Custom server** (Docker/Node.js)

### Cloudflare Deployment

1. **Install Wrangler:**
   ```bash
   npm install -g wrangler
   ```

2. **Configure Cloudflare:**
   ```bash
   wrangler login
   ```

3. **Deploy with Pages:**
   ```bash
   wrangler pages deploy .next --project-name=research-lab
   ```

   Or use **Cloudflare Workers** for better performance:
   ```bash
   wrangler deploy
   ```

## 🌟 Features

- ✅ Dynamic content loading
- ✅ Real-time updates
- ✅ Database integration
- ✅ User authentication
- ✅ Admin dashboard
- ✅ Analytics tracking
- ✅ Search & filtering
- ✅ Advanced UI/UX

## 📊 Database Setup

```bash
# Install Prisma
npm install prisma @prisma/client

# Initialize
npx prisma init

# Migrate
npx prisma migrate dev
```

## 🔐 Authentication

Uses NextAuth.js for authentication:
- GitHub OAuth
- Email/Password
- Magic links
- Session management

## 📈 Analytics

- Real-time metrics
- Project analytics
- User activity
- Performance monitoring
