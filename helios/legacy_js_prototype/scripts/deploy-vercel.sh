#!/bin/bash
# Vercel Deployment Script with Auto DNS Configuration

set -e

echo "🚀 Vercel Deployment with Auto DNS Configuration"
echo ""

cd "$(dirname "$0")/.."
PROJECT_ROOT=$(pwd)
FRONTEND_DIR="$PROJECT_ROOT/portfolio/frontend"

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m'

# Check if we're in the right directory
if [ ! -d "portfolio/frontend" ]; then
    echo -e "${RED}Error: Must run from github-portfolio root directory${NC}"
    exit 1
fi

# Step 1: Build
echo -e "${BLUE}📦 Step 1: Building project...${NC}"
cd "$FRONTEND_DIR"
npm install --silent
npm run build

if [ ! -d ".next" ]; then
    echo -e "${RED}❌ Build failed!${NC}"
    exit 1
fi

echo -e "${GREEN}✅ Build complete!${NC}"
echo ""

# Step 2: Deploy to Vercel
echo -e "${BLUE}🚀 Step 2: Deploying to Vercel...${NC}"
echo -e "${YELLOW}💡 If not logged in, run: vercel login${NC}"
echo ""

if command -v vercel &> /dev/null; then
    echo -e "${YELLOW}Deploying via Vercel CLI...${NC}"
    vercel --prod --yes --confirm
    
    # Get deployment URL
    DEPLOYMENT_URL=$(vercel ls --json 2>/dev/null | jq -r '.[0].url' 2>/dev/null || echo "")
    
    if [ -n "$DEPLOYMENT_URL" ]; then
        echo -e "${GREEN}✅ Deployed to: https://${DEPLOYMENT_URL}${NC}"
        echo ""
        
        # Step 3: Configure DNS
        echo -e "${BLUE}🌐 Step 3: Configuring DNS...${NC}"
        cd "$PROJECT_ROOT"
        
        if [ -f "scripts/configure-dns.py" ]; then
            python3 scripts/configure-dns.py
            echo ""
            python3 scripts/configure-dns.py me  # Also configure me.too.foo
        elif [ -f "scripts/configure-dns.sh" ]; then
            bash scripts/configure-dns.sh "$DEPLOYMENT_URL"
            echo ""
            bash scripts/configure-dns.sh "$DEPLOYMENT_URL" me
        else
            echo -e "${YELLOW}⚠️  DNS configuration script not found${NC}"
            echo -e "${YELLOW}   Please configure DNS manually in Cloudflare Dashboard${NC}"
        fi
    else
        echo -e "${YELLOW}⚠️  Could not detect deployment URL${NC}"
        echo -e "${YELLOW}   Configure DNS manually:${NC}"
        echo -e "${YELLOW}   1. Get your Vercel deployment URL from dashboard${NC}"
        echo -e "${YELLOW}   2. Run: python3 scripts/configure-dns.py${NC}"
    fi
else
    echo -e "${YELLOW}⚠️  Vercel CLI not found${NC}"
    echo ""
    echo -e "${BLUE}📤 Manual Deployment Steps:${NC}"
    echo "   1. Go to: https://vercel.com/new"
    echo "   2. Import Git Repository: too.foo"
    echo "   3. Set Root Directory: portfolio/frontend"
    echo "   4. Deploy"
    echo ""
    echo -e "${BLUE}🌐 After deployment, configure DNS:${NC}"
    echo "   python3 scripts/configure-dns.py"
    echo "   python3 scripts/configure-dns.py me"
fi

echo ""
echo -e "${GREEN}✅ Deployment complete!${NC}"
echo ""
echo -e "${BLUE}📋 Next Steps:${NC}"
echo "   1. Add domain in Vercel Dashboard:"
echo "      → Settings → Domains → Add too.foo"
echo "      → Settings → Domains → Add me.too.foo"
echo "   2. Wait 5-10 minutes for DNS propagation"
echo "   3. Visit: https://too.foo and https://me.too.foo"
