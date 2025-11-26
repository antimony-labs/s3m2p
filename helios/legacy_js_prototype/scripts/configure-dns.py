#!/usr/bin/env python3

"""
Auto-configure Cloudflare DNS for Vercel deployments
Supports both apex domains and subdomains
"""

import os
import sys
import json
import requests
from typing import Optional, Dict

# Configuration
DOMAIN = "too.foo"
CLOUDFLARE_API_TOKEN = os.getenv("CLOUDFLARE_API_TOKEN", "s-QNbEIDs4biZ9w2LOeA32qfslxXh1ejc-vTawZr")
CLOUDFLARE_ZONE_ID = os.getenv("CLOUDFLARE_ZONE_ID", "e13cbd7e3f51b209882de9ced70c6949")
CLOUDFLARE_ACCOUNT_ID = os.getenv("CLOUDFLARE_ACCOUNT_ID", "5fbcfb6b54deee2cfd21599cdb26b00f")

BASE_URL = "https://api.cloudflare.com/client/v4"
HEADERS = {
    "Authorization": f"Bearer {CLOUDFLARE_API_TOKEN}",
    "Content-Type": "application/json"
}

VERCEL_CNAME = "cname.vercel-dns.com"


def api_call(method: str, endpoint: str, data: Optional[Dict] = None) -> Dict:
    """Make Cloudflare API call"""
    url = f"{BASE_URL}{endpoint}"
    
    if method == "GET":
        response = requests.get(url, headers=HEADERS)
    elif method == "POST":
        response = requests.post(url, headers=HEADERS, json=data)
    elif method == "PUT":
        response = requests.put(url, headers=HEADERS, json=data)
    elif method == "DELETE":
        response = requests.delete(url, headers=HEADERS)
    else:
        raise ValueError(f"Unsupported method: {method}")
    
    result = response.json()
    
    if not result.get("success"):
        errors = result.get("errors", [])
        raise Exception(f"API Error: {errors}")
    
    return result.get("result", {})


def get_dns_record(name: str, record_type: str) -> Optional[Dict]:
    """Get existing DNS record"""
    try:
        result = api_call("GET", f"/zones/{CLOUDFLARE_ZONE_ID}/dns_records?name={name}&type={record_type}")
        records = result if isinstance(result, list) else []
        return records[0] if records else None
    except Exception as e:
        print(f"⚠️  Error checking record: {e}")
        return None


def create_dns_record(name: str, record_type: str, content: str, proxied: bool = True) -> Dict:
    """Create DNS record"""
    data = {
        "name": name,
        "type": record_type,
        "content": content,
        "proxied": proxied,
        "ttl": 1  # Auto TTL
    }
    
    return api_call("POST", f"/zones/{CLOUDFLARE_ZONE_ID}/dns_records", data)


def update_dns_record(record_id: str, name: str, record_type: str, content: str, proxied: bool = True) -> Dict:
    """Update DNS record"""
    data = {
        "name": name,
        "type": record_type,
        "content": content,
        "proxied": proxied,
        "ttl": 1  # Auto TTL
    }
    
    return api_call("PUT", f"/zones/{CLOUDFLARE_ZONE_ID}/dns_records/{record_id}", data)


def configure_dns(dns_name: str, subdomain: Optional[str] = None) -> bool:
    """Configure DNS for domain or subdomain"""
    print(f"\n{'='*60}")
    print(f"🔧 Configuring DNS: {dns_name}")
    print(f"{'='*60}\n")
    
    # Check existing record
    existing = get_dns_record(dns_name, "CNAME")
    
    if existing:
        print(f"📝 Found existing CNAME record")
        print(f"   Updating to point to Vercel...")
        
        try:
            update_dns_record(
                existing["id"],
                dns_name,
                "CNAME",
                VERCEL_CNAME,
                proxied=True
            )
            print(f"✅ Updated DNS record successfully!")
            return True
        except Exception as e:
            print(f"❌ Error updating record: {e}")
            return False
    else:
        print(f"✨ Creating new CNAME record...")
        
        try:
            create_dns_record(
                dns_name,
                "CNAME",
                VERCEL_CNAME,
                proxied=True
            )
            print(f"✅ Created DNS record successfully!")
            return True
        except Exception as e:
            print(f"❌ Error creating record: {e}")
            return False


def main():
    """Main function"""
    print("\n╔══════════════════════════════════════════════════════════╗")
    print("║   Cloudflare DNS Auto-Configuration for Vercel         ║")
    print("╚══════════════════════════════════════════════════════════╝")
    
    # Parse arguments
    subdomain = sys.argv[1] if len(sys.argv) > 1 else None
    
    if subdomain:
        dns_name = f"{subdomain}.{DOMAIN}"
        print(f"\n📍 Configuring subdomain: {dns_name}")
    else:
        dns_name = DOMAIN
        print(f"\n📍 Configuring apex domain: {dns_name}")
    
    # Configure DNS
    success = configure_dns(dns_name, subdomain)
    
    if success:
        print("\n" + "="*60)
        print("✅ DNS Configuration Complete!")
        print("="*60)
        print(f"\n📋 Next steps:")
        print(f"   1. Add domain in Vercel Dashboard:")
        print(f"      → Settings → Domains → Add {dns_name}")
        print(f"   2. Wait 5-10 minutes for DNS propagation")
        print(f"   3. Visit: https://{dns_name}")
        print(f"\n💡 DNS records are proxied through Cloudflare (orange cloud)")
        print()
    else:
        print("\n❌ DNS configuration failed. Check errors above.")
        sys.exit(1)


if __name__ == "__main__":
    main()

