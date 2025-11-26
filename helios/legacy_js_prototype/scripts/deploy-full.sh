#!/bin/bash

# Research Lab Deployment Script
# Run this script to deploy your portfolio

set -e  # Exit on error

echo "🚀 Starting deployment..."

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Check if we're in the right directory
if [ ! -d "portfolio/frontend" ]; then
    echo -e "${RED}Error: Must run from github-portfolio root directory${NC}"
    exit 1
fi

# Step 1: Check Node.js
echo -e "${YELLOW}Step 1: Checking Node.js...${NC}"
if ! command -v node &> /dev/null; then
    echo -e "${RED}Error: Node.js not found. Please install Node.js 18+${NC}"
    exit 1
fi
NODE_VERSION=$(node --version)
echo -e "${GREEN}✓ Node.js version: $NODE_VERSION${NC}"

# Step 2: Check Wrangler
echo -e "${YELLOW}Step 2: Checking Wrangler...${NC}"
if ! command -v wrangler &> /dev/null; then
    echo -e "${YELLOW}Wrangler not found. Installing...${NC}"
    npm install -g wrangler
fi
WRANGLER_VERSION=$(wrangler --version)
echo -e "${GREEN}✓ Wrangler version: $WRANGLER_VERSION${NC}"

# Step 3: Install dependencies
echo -e "${YELLOW}Step 3: Installing dependencies...${NC}"
cd portfolio/frontend
if [ ! -d "node_modules" ]; then
    npm install
else
    echo -e "${GREEN}✓ Dependencies already installed${NC}"
fi

# Step 4: Build
echo -e "${YELLOW}Step 4: Building project...${NC}"
npm run build

if [ ! -d ".next" ]; then
    echo -e "${RED}Error: Build failed - .next directory not found${NC}"
    exit 1
fi
echo -e "${GREEN}✓ Build successful${NC}"

# Step 5: Deploy
echo -e "${YELLOW}Step 5: Deploying to Cloudflare Pages...${NC}"
read -p "Project name (default: research-lab): " PROJECT_NAME
PROJECT_NAME=${PROJECT_NAME:-research-lab}

wrangler pages deploy .next --project-name=$PROJECT_NAME

echo -e "${GREEN}✅ Deployment complete!${NC}"
echo ""
echo "Next steps:"
echo "1. Go to Cloudflare Dashboard → Pages"
echo "2. Click on your project: $PROJECT_NAME"
echo "3. Add your custom domain"
echo "4. Wait for DNS propagation"
echo ""
echo "🌍 Your site should be live at: https://$PROJECT_NAME-xxxxx.pages.dev"

