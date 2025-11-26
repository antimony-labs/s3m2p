# Cloudflare Deployment Guide

## Deploy to Cloudflare Pages

### Option 1: Using Wrangler CLI (Recommended)

1. **Install Wrangler:**
   ```bash
   npm install -g wrangler
   ```

2. **Login to Cloudflare:**
   ```bash
   wrangler login
   ```

3. **Build the project:**
   ```bash
   cd portfolio/frontend
   npm run build
   ```

4. **Deploy:**
   ```bash
   wrangler pages deploy out --project-name=portfolio
   ```

### Option 2: Using Cloudflare Dashboard

1. **Prepare the build:**
   ```bash
   cd portfolio/frontend
   npm run build
   ```

2. **Upload to Cloudflare Pages:**
   - Go to Cloudflare Dashboard → Pages
   - Create new project
   - Upload the `out` folder
   - Set build command: `npm run build`
   - Set output directory: `out`

### Option 3: Connect GitHub Repository

1. **Push to GitHub:**
   ```bash
   git add .
   git commit -m "Portfolio website"
   git push origin main
   ```

2. **In Cloudflare Dashboard:**
   - Go to Pages → Create a project
   - Connect your GitHub repository
   - Set build settings:
     - Build command: `cd portfolio/frontend && npm install && npm run build`
     - Build output directory: `portfolio/frontend/out`
     - Root directory: `portfolio/frontend`

### Custom Domain Setup

1. **Add Custom Domain:**
   - In Cloudflare Pages, go to your project
   - Click "Custom domains"
   - Add your domain (e.g., `yourdomain.com`)

2. **Update DNS:**
   - Cloudflare will automatically configure DNS
   - Or manually add CNAME record pointing to your Pages URL

### Environment Variables

If you need environment variables:
- In Cloudflare Dashboard: Pages → Your project → Settings → Environment Variables

### Build Configuration

The `next.config.js` is already configured for static export, which works perfectly with Cloudflare Pages.

### Quick Deploy Script

Create `deploy.sh`:
```bash
#!/bin/bash
cd portfolio/frontend
npm install
npm run build
wrangler pages deploy out --project-name=portfolio
```

Make it executable:
```bash
chmod +x deploy.sh
```

Then deploy:
```bash
./deploy.sh
```

