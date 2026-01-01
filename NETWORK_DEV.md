# Network Development Setup

Access your S3M2P dev servers from any device on your local network using `*.dev.too.foo` domains.

## Quick Start

### 1. Install Caddy (if not already installed)

```bash
# Ubuntu/Debian
sudo apt install -y debian-keyring debian-archive-keyring apt-transport-https curl
curl -1sLf 'https://dl.cloudsmith.io/public/caddy/stable/gpg.key' | sudo gpg --dearmor -o /usr/share/keyrings/caddy-stable-archive-keyring.gpg
curl -1sLf 'https://dl.cloudsmith.io/public/caddy/stable/debian.deb.txt' | sudo tee /etc/apt/sources.list.d/caddy-stable.list
sudo apt update
sudo apt install caddy

# macOS
brew install caddy

# Or download from: https://caddyserver.com/download
```

### 2. Start Caddy

```bash
# From the project root
sudo caddy run --config Caddyfile
```

**Note:** Caddy needs sudo/root to bind to port 80.

### 3. Start Dev Servers

In a new terminal:

```bash
# Start all servers
./SCRIPTS/dev up all

# Or start a single project
./SCRIPTS/dev up esp32
```

### 4. Configure DNS on Your Devices

All servers are now accessible at `http://10.0.0.114` (ports 8080-8106), but to use the nice `*.dev.too.foo` domains, you need to configure DNS.

#### Option A: Edit /etc/hosts on Each Device

**On your development laptop (Linux/Mac):**

```bash
sudo nano /etc/hosts
```

Add these lines (replace `10.0.0.114` with your actual dev machine IP):

```
10.0.0.114  welcome.dev.too.foo
10.0.0.114  helios.dev.too.foo
10.0.0.114  esp32.dev.too.foo
10.0.0.114  slam.dev.too.foo
10.0.0.114  ai.dev.too.foo
10.0.0.114  ubuntu.dev.too.foo
10.0.0.114  arduino.dev.too.foo
10.0.0.114  opencv.dev.too.foo
10.0.0.114  swarm.dev.too.foo
10.0.0.114  chladni.dev.too.foo
10.0.0.114  autocrate.dev.too.foo
10.0.0.114  blog.dev.too.foo
10.0.0.114  learn.dev.too.foo
10.0.0.114  arch.dev.too.foo
10.0.0.114  pll.dev.too.foo
10.0.0.114  power.dev.too.foo
10.0.0.114  sensors.dev.too.foo
```

**On Windows:**

1. Open `C:\Windows\System32\drivers\etc\hosts` as Administrator
2. Add the same lines as above

**On Android/iOS:**

- Requires root/jailbreak, or use Option B instead

#### Option B: Use Your Router's DNS (Recommended for Mobile)

1. Log into your router admin panel (usually `192.168.1.1` or `192.168.0.1`)
2. Find the **DNS settings** or **Local DNS** section
3. Add a wildcard DNS entry: `*.dev.too.foo` → `10.0.0.114`
4. If wildcard isn't supported, add each subdomain individually

**Router examples:**
- **DD-WRT/OpenWrt:** DNSMasq → Add local domain
- **Pi-hole:** Local DNS → Add custom DNS entry
- **pfSense:** Services → DNS Resolver → Host Overrides

#### Option C: Use dnsmasq (Advanced - Linux/Mac)

```bash
# Install dnsmasq
sudo apt install dnsmasq  # Ubuntu
brew install dnsmasq      # macOS

# Add to /etc/dnsmasq.conf
echo "address=/dev.too.foo/10.0.0.114" | sudo tee -a /etc/dnsmasq.conf

# Restart
sudo systemctl restart dnsmasq  # Linux
sudo brew services restart dnsmasq  # macOS
```

Then configure your devices to use your dev machine as DNS server.

## Usage

Once configured, access your projects at:

| Project | URL | Port |
|---------|-----|------|
| Welcome | http://welcome.dev.too.foo | 8080 |
| Helios | http://helios.dev.too.foo | 8081 |
| Chladni | http://chladni.dev.too.foo | 8082 |
| ESP32 | http://esp32.dev.too.foo | 8104 |
| SLAM | http://slam.dev.too.foo | 8106 |
| AI/ML | http://ai.dev.too.foo | 8100 |
| Ubuntu | http://ubuntu.dev.too.foo | 8101 |
| Arduino | http://arduino.dev.too.foo | 8103 |
| ... | ... | ... |

**Direct IP access** (works without DNS setup):
- http://10.0.0.114:8104 (ESP32)
- http://10.0.0.114:8106 (SLAM)
- etc.

## Testing from Phone/Tablet

1. Connect your phone to the **same WiFi network**
2. Open browser
3. Navigate to `http://esp32.dev.too.foo` (or `http://10.0.0.114:8104`)

## Troubleshooting

### Can't access from other devices

1. **Check firewall:**
   ```bash
   # Ubuntu - Allow ports 80, 8080-8106
   sudo ufw allow 80/tcp
   sudo ufw allow 8080:8106/tcp

   # Or disable firewall temporarily for testing
   sudo ufw disable
   ```

2. **Verify trunk is listening on 0.0.0.0:**
   ```bash
   netstat -tuln | grep 8104
   # Should show: 0.0.0.0:8104  (not 127.0.0.1:8104)
   ```

3. **Test direct IP access first:**
   - From phone, try `http://10.0.0.114:8104`
   - If this works but `esp32.dev.too.foo` doesn't, it's a DNS issue

### Caddy not starting

```bash
# Check if port 80 is already in use
sudo lsof -i :80

# Kill conflicting process
sudo kill <PID>

# Or use a different port (edit Caddyfile)
{
    http_port 8000
}
```

### DNS not resolving

1. Clear DNS cache:
   ```bash
   # Linux
   sudo systemd-resolve --flush-caches

   # macOS
   sudo dscacheutil -flushcache

   # Windows
   ipconfig /flushdns
   ```

2. Verify `/etc/hosts` changes were saved
3. Restart browser

## Architecture

```
[Phone] ────────┐
[Laptop] ───────┼──> [Your Dev Machine: 10.0.0.114]
[Tablet] ───────┘         │
                          ├─> Caddy (port 80)
                          │    └─> Reverse proxy to trunk servers
                          │
                          ├─> trunk serve (0.0.0.0:8104) ─> ESP32 app
                          ├─> trunk serve (0.0.0.0:8106) ─> SLAM app
                          └─> ... (other apps)
```

## Security Note

This setup makes your dev servers accessible to **anyone on your local network**. Only use on trusted networks (home/office WiFi). Don't use on public WiFi without additional security (firewall rules, authentication).

## Reverting to Localhost-Only

To make servers localhost-only again:

1. **SCRIPTS/dev:** Remove `--address 0.0.0.0` flags
2. **Caddyfile:** Change `*.dev.too.foo` back to `*.local.too.foo`
3. Restart services
