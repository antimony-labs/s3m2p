#!/bin/bash
# Quick deploy script for Cloudflare Pages

echo "🚀 Building portfolio..."

cd portfolio/frontend

echo "📦 Installing dependencies..."
npm install

echo "🔨 Building..."
npm run build

if [ ! -d "out" ]; then
    echo "❌ Build failed - 'out' directory not found"
    exit 1
fi

echo "✅ Build complete!"
echo ""
echo "📤 To deploy:"
echo "   wrangler pages deploy out --project-name=portfolio"
echo ""
echo "Or upload the 'out' folder via Cloudflare Dashboard"

