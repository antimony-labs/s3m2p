# DNS Configuration Guide

## 🌐 Automatic DNS Configuration

The deployment scripts automatically configure Cloudflare DNS for your domains.

### Usage

```bash
# Configure apex domain (too.foo)
python3 scripts/configure-dns.py

# Configure subdomain (me.too.foo)
python3 scripts/configure-dns.py me

# Or use bash script
bash scripts/configure-dns.sh <vercel-url>
bash scripts/configure-dns.sh <vercel-url> me
```

### What It Does

1. **Creates CNAME records** pointing to `cname.vercel-dns.com`
2. **Enables Cloudflare proxy** (orange cloud) for security and performance
3. **Supports both apex domains and subdomains**

### Requirements

Set these environment variables (or they're auto-detected from defaults):

```bash
export CLOUDFLARE_API_TOKEN="your-token"
export CLOUDFLARE_ZONE_ID="your-zone-id"
export CLOUDFLARE_ACCOUNT_ID="your-account-id"
```

### After DNS Configuration

1. **Add domains in Vercel Dashboard:**
   - Settings → Domains
   - Add: `too.foo`
   - Add: `me.too.foo`

2. **Wait 5-10 minutes** for DNS propagation

3. **Verify:**
   ```bash
   dig too.foo
   dig me.too.foo
   ```

### Manual Configuration

If scripts don't work, configure manually in Cloudflare:

1. Go to: https://dash.cloudflare.com
2. Select domain: `too.foo`
3. DNS → Records → Add record
4. **For apex domain:**
   - Type: CNAME
   - Name: `@` or `too.foo`
   - Target: `cname.vercel-dns.com`
   - Proxy: Enabled (orange cloud)
5. **For subdomain:**
   - Type: CNAME
   - Name: `me`
   - Target: `cname.vercel-dns.com`
   - Proxy: Enabled (orange cloud)

### Troubleshooting

**"API Error":**
- Check Cloudflare API token permissions
- Token needs: Zone DNS Edit, Zone Read

**"Record already exists":**
- Scripts automatically update existing records
- Check Cloudflare Dashboard to verify

**DNS not propagating:**
- Wait 10-15 minutes
- Clear DNS cache: `sudo systemd-resolve --flush-caches`
- Check with: `dig too.foo @8.8.8.8`

