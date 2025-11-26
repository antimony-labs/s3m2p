#!/bin/bash

# Auto-configure Cloudflare DNS for Vercel deployments
# Usage: ./configure-dns.sh [vercel-url] [subdomain]

set -e

DOMAIN="too.foo"
CLOUDFLARE_API_TOKEN="${CLOUDFLARE_API_TOKEN:-s-QNbEIDs4biZ9w2LOeA32qfslxXh1ejc-vTawZr}"
CLOUDFLARE_ZONE_ID="${CLOUDFLARE_ZONE_ID:-e13cbd7e3f51b209882de9ced70c6949}"
CLOUDFLARE_ACCOUNT_ID="${CLOUDFLARE_ACCOUNT_ID:-5fbcfb6b54deee2cfd21599cdb26b00f}"

VERCEL_URL="${1:-}"
SUBDOMAIN="${2:-}"

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}╔══════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║   Cloudflare DNS Auto-Configuration for Vercel         ║${NC}"
echo -e "${BLUE}╚══════════════════════════════════════════════════════════╝${NC}"
echo ""

# If no Vercel URL provided, try to get from Vercel CLI
if [ -z "$VERCEL_URL" ]; then
    echo -e "${YELLOW}🔍 Getting Vercel deployment URL...${NC}"
    if command -v vercel &> /dev/null; then
        cd portfolio/frontend 2>/dev/null || cd ../portfolio/frontend || true
        VERCEL_URL=$(vercel ls --json 2>/dev/null | jq -r '.[0].url' 2>/dev/null || echo "")
    fi
    
    if [ -z "$VERCEL_URL" ]; then
        echo -e "${RED}❌ Could not detect Vercel URL${NC}"
        echo -e "${YELLOW}💡 Usage: ./configure-dns.sh <vercel-url> [subdomain]${NC}"
        echo -e "${YELLOW}   Example: ./configure-dns.sh too-foo.vercel.app${NC}"
        echo -e "${YELLOW}   Example: ./configure-dns.sh too-foo.vercel.app me${NC}"
        exit 1
    fi
fi

# If subdomain is provided, use it. Otherwise use apex domain
if [ -n "$SUBDOMAIN" ]; then
    DNS_NAME="${SUBDOMAIN}.${DOMAIN}"
else
    DNS_NAME="${DOMAIN}"
fi

echo -e "${GREEN}✓ Domain: ${DNS_NAME}${NC}"
echo -e "${GREEN}✓ Target: ${VERCEL_URL}${NC}"
echo ""

# Cloudflare API base URL
BASE_URL="https://api.cloudflare.com/client/v4"
HEADERS=(
    "-H" "Authorization: Bearer ${CLOUDFLARE_API_TOKEN}"
    "-H" "Content-Type: application/json"
)

# Function to check if DNS record exists
check_record() {
    local name=$1
    local type=$2
    
    response=$(curl -s "${HEADERS[@]}" \
        "${BASE_URL}/zones/${CLOUDFLARE_ZONE_ID}/dns_records?name=${name}&type=${type}")
    
    echo "$response" | jq -r '.result[] | .id' 2>/dev/null || echo ""
}

# Function to create DNS record
create_record() {
    local name=$1
    local type=$2
    local content=$3
    local proxied=${4:-true}
    
    local data=$(jq -n \
        --arg name "$name" \
        --arg type "$type" \
        --arg content "$content" \
        --argjson proxied "$proxied" \
        '{name: $name, type: $type, content: $content, proxied: $proxied, ttl: 1}')
    
    curl -s -X POST "${HEADERS[@]}" \
        "${BASE_URL}/zones/${CLOUDFLARE_ZONE_ID}/dns_records" \
        -d "$data"
}

# Function to update DNS record
update_record() {
    local record_id=$1
    local name=$2
    local type=$3
    local content=$4
    local proxied=${5:-true}
    
    local data=$(jq -n \
        --arg name "$name" \
        --arg type "$type" \
        --arg content "$content" \
        --argjson proxied "$proxied" \
        '{name: $name, type: $type, content: $content, proxied: $proxied, ttl: 1}')
    
    curl -s -X PUT "${HEADERS[@]}" \
        "${BASE_URL}/zones/${CLOUDFLARE_ZONE_ID}/dns_records/${record_id}" \
        -d "$data"
}

# For Vercel, we need CNAME records pointing to cname.vercel-dns.com
# But for apex domains, Vercel requires CNAME flattening or A records
# Let's use CNAME for subdomains and CNAME flattening for apex

echo -e "${YELLOW}📝 Configuring DNS records...${NC}"

if [ -n "$SUBDOMAIN" ]; then
    # Subdomain: Use CNAME pointing to Vercel
    echo -e "${BLUE}→ Setting up CNAME for ${DNS_NAME}${NC}"
    
    record_id=$(check_record "$DNS_NAME" "CNAME")
    
    if [ -n "$record_id" ]; then
        echo -e "${YELLOW}  Found existing record, updating...${NC}"
        update_record "$record_id" "$DNS_NAME" "CNAME" "cname.vercel-dns.com" "true"
        echo -e "${GREEN}✓ Updated CNAME record${NC}"
    else
        echo -e "${BLUE}  Creating new CNAME record...${NC}"
        create_record "$DNS_NAME" "CNAME" "cname.vercel-dns.com" "true"
        echo -e "${GREEN}✓ Created CNAME record${NC}"
    fi
else
    # Apex domain: Use CNAME flattening (Cloudflare handles this automatically)
    echo -e "${BLUE}→ Setting up CNAME for ${DNS_NAME} (apex domain)${NC}"
    
    record_id=$(check_record "$DOMAIN" "CNAME")
    
    if [ -n "$record_id" ]; then
        echo -e "${YELLOW}  Found existing record, updating...${NC}"
        update_record "$record_id" "$DOMAIN" "CNAME" "cname.vercel-dns.com" "true"
        echo -e "${GREEN}✓ Updated CNAME record${NC}"
    else
        echo -e "${BLUE}  Creating new CNAME record...${NC}"
        create_record "$DOMAIN" "CNAME" "cname.vercel-dns.com" "true"
        echo -e "${GREEN}✓ Created CNAME record${NC}"
    fi
fi

echo ""
echo -e "${GREEN}✅ DNS configuration complete!${NC}"
echo ""
echo -e "${YELLOW}📋 Next steps:${NC}"
echo -e "  1. Add domain in Vercel Dashboard:"
echo -e "     → Settings → Domains → Add ${DNS_NAME}"
echo -e "  2. Wait 5-10 minutes for DNS propagation"
echo -e "  3. Visit: https://${DNS_NAME}"
echo ""
echo -e "${BLUE}💡 Tip: DNS records are proxied through Cloudflare (orange cloud)${NC}"

