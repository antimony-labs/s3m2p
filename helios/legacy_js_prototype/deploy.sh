#!/bin/bash
# Main deployment script - Always configures DNS

set -e

cd "$(dirname "$0")/.."

echo "🚀 Deploying too.foo with Auto DNS Configuration"
echo ""

# Run the full deployment script
bash scripts/deploy-vercel.sh

# Always configure DNS after deployment
echo ""
echo "🌐 Configuring DNS automatically..."

if [ -f "scripts/configure-dns.py" ]; then
    echo "→ Configuring too.foo"
    python3 scripts/configure-dns.py 2>/dev/null || echo "⚠️  DNS config skipped (may need Vercel URL)"
    
    echo "→ Configuring me.too.foo"
    python3 scripts/configure-dns.py me 2>/dev/null || echo "⚠️  DNS config skipped (may need Vercel URL)"
fi

echo ""
echo "✅ Done! DNS will be configured automatically."
echo "💡 Note: Add domains in Vercel Dashboard → Settings → Domains"

