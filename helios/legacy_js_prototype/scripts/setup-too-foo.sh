#!/bin/bash
# Setup and deploy too.foo to Vercel

echo "🚀 Setting up too.foo deployment"

cd /root/repos/scratchpad/projects/github-portfolio

# Check if git repo exists
if [ ! -d ".git" ]; then
    echo "📦 Initializing git repository..."
    git init
    git branch -M main
fi

# Check remote
REMOTE=$(git remote get-url origin 2>/dev/null || echo "")
if [ -z "$REMOTE" ]; then
    echo ""
    echo "⚠️  No remote repository configured"
    echo ""
    echo "To connect to GitHub:"
    echo "1. Create repo on GitHub: https://github.com/new"
    echo "   Name: too.foo"
    echo ""
    echo "2. Then run:"
    echo "   git remote add origin https://github.com/YOUR_USERNAME/too.foo.git"
    echo "   git add ."
    echo "   git commit -m 'Research lab platform'"
    echo "   git push -u origin main"
    echo ""
else
    echo "✓ Remote configured: $REMOTE"
fi

echo ""
echo "📤 Deployment Steps:"
echo ""
echo "1. Push to GitHub (if not done):"
echo "   git add ."
echo "   git commit -m 'Research lab platform'"
echo "   git push origin main"
echo ""
echo "2. Deploy on Vercel:"
echo "   - Go to: https://vercel.com/new"
echo "   - Import repository: too.foo"
echo "   - Root Directory: portfolio/frontend"
echo "   - Deploy!"
echo ""
echo "3. Add custom domain:"
echo "   - Settings → Domains → Add: too.foo"
echo "   - Configure DNS in Cloudflare"
echo ""

