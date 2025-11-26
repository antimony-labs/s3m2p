#!/bin/bash
# Quick deployment helper - Dashboard method

echo "🚀 Preparing for Cloudflare Dashboard Deployment"
echo ""

cd /root/repos/scratchpad/projects/github-portfolio/portfolio/frontend

echo "📦 Building project..."
npm run build

if [ ! -d ".next" ]; then
    echo "❌ Build failed!"
    exit 1
fi

echo "✅ Build complete!"
echo ""
echo "📤 Next steps:"
echo "1. Go to: https://dash.cloudflare.com"
echo "2. Navigate to: Workers & Pages → Pages"
echo "3. Click: 'Create a project' → 'Upload assets'"
echo "4. Upload the .next folder"
echo ""
echo "Or zip it first:"
echo "cd .next && zip -r ../deploy.zip . && cd .."
echo ""
echo "Deployment package ready in:"
pwd

