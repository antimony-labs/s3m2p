# Vercel Deployment Guide

## 🚀 Deploy to Vercel (Easiest for Next.js!)

### Quick Deploy

```bash
cd /root/repos/scratchpad/projects/github-portfolio/portfolio/frontend

# Login (opens browser)
vercel login

# Deploy to production
vercel --prod --yes
```

### What Happens

1. **Vercel detects Next.js** automatically
2. **Builds your project**
3. **Deploys** to production
4. **Gives you URL:** `https://your-project.vercel.app`
5. **Sets up HTTPS** automatically

### Add Custom Domain

After deployment:
1. Go to Vercel Dashboard: https://vercel.com/dashboard
2. Click your project
3. Settings → Domains
4. Add your domain
5. Vercel configures DNS automatically

### GitHub Integration (Auto-Deploy)

1. **Push to GitHub:**
   ```bash
   cd /root/repos/scratchpad/projects/github-portfolio
   git add .
   git commit -m "Research lab platform"
   git push origin main
   ```

2. **In Vercel Dashboard:**
   - Import Project → GitHub
   - Select repository
   - Root Directory: `portfolio/frontend`
   - Deploy!

Now every push = auto-deploy!

---

## ✨ Why Vercel?

- ✅ **Perfect Next.js support** - Built by Next.js creators
- ✅ **Zero config** - Just works
- ✅ **Fast deployments** - ~30 seconds
- ✅ **Free SSL** - Automatic HTTPS
- ✅ **Preview deployments** - Every branch gets a URL
- ✅ **Edge Network** - Fast worldwide
- ✅ **Built-in analytics**

---

## 🎯 Ready to Deploy?

Run:
```bash
cd /root/repos/scratchpad/projects/github-portfolio/portfolio/frontend
vercel login
vercel --prod --yes
```

Done! 🎉

