# Step-by-Step Deployment Guide

## 🎯 Goal
Deploy your research lab platform to Cloudflare with your custom domain.

---

## STEP 1: Prerequisites Check ✅

```bash
# Check Node.js version (need 18+)
node --version

# Check npm
npm --version

# Navigate to project
cd /root/repos/scratchpad/projects/github-portfolio
```

**Expected:** Node.js v18+ and npm 9+

---

## STEP 2: Install Dependencies 📦

```bash
cd portfolio/frontend
npm install
```

**What happens:** Installs Next.js, React, TypeScript, and all dependencies
**Time:** ~2-3 minutes

---

## STEP 3: Build the Project 🔨

```bash
npm run build
```

**What happens:** Compiles your code into production-ready files
**Expected output:** `✓ Compiled successfully`
**Time:** ~1-2 minutes

---

## STEP 4: Install Cloudflare Wrangler 🚀

```bash
npm install -g wrangler
wrangler --version
```

**Expected:** `wrangler 3.x.x`

---

## STEP 5: Login to Cloudflare 🔐

```bash
wrangler login
```

**What happens:**
1. Opens browser
2. Click "Allow" to authorize
3. Returns to terminal

---

## STEP 6: Deploy to Cloudflare 📤

```bash
# Make sure you're in portfolio/frontend
cd portfolio/frontend

# Deploy
wrangler pages deploy .next --project-name=research-lab
```

**Expected output:**
```
✨ Success! Uploaded .next directory
📦 Created project 'research-lab'
🌍 Deployment URL: https://research-lab-xxxxx.pages.dev
```

**Copy this URL!** Your site is live here.

---

## STEP 7: Add Custom Domain 🌐

### In Cloudflare Dashboard:

1. **Go to:** https://dash.cloudflare.com
2. **Navigate to:** Workers & Pages → Pages
3. **Click:** `research-lab` project
4. **Click:** "Custom domains" tab
5. **Click:** "Set up a custom domain"
6. **Enter:** your domain (e.g., `yourdomain.com`)
7. **Click:** "Continue"

**Cloudflare will:**
- ✅ Automatically add DNS records
- ✅ Provision SSL certificate
- ✅ Set up HTTPS

**Wait:** 5-30 minutes for DNS propagation

---

## STEP 8: Verify ✅

1. **Check Cloudflare URL:**
   ```
   https://research-lab-xxxxx.pages.dev
   ```

2. **Check Custom Domain:**
   ```
   https://yourdomain.com
   ```

**Testing:**
- [ ] Homepage loads
- [ ] Projects display
- [ ] Search works
- [ ] HTTPS enabled

---

## Quick Deploy Script 🚀

Want to automate everything? Run:

```bash
cd /root/repos/scratchpad/projects/github-portfolio
./scripts/deploy-full.sh
```

This does steps 1-6 automatically!

---

## Troubleshooting 🔧

**Build fails?**
```bash
cd portfolio/frontend
npm install
npm run build
# Check error messages
```

**Deploy fails?**
```bash
wrangler login  # Make sure you're logged in
wrangler pages project list  # Check projects
```

**Domain not working?**
- Check DNS in Cloudflare Dashboard
- Wait for DNS propagation (check: https://dnschecker.org)
- Verify SSL certificate is active

---

## After Deployment 🎉

1. **Add your projects** to `/projects/` directory
2. **Rebuild and redeploy:**
   ```bash
   cd portfolio/frontend
   npm run build
   wrangler pages deploy .next --project-name=research-lab
   ```
3. **Monitor** in Cloudflare Dashboard

---

## Need Help? 🆘

- Cloudflare Dashboard: https://dash.cloudflare.com
- Check deployment logs: `wrangler pages deployment tail`
- DNS checker: https://dnschecker.org

