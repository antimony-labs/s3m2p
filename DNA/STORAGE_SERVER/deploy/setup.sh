#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
# FILE: setup.sh | DNA/STORAGE_SERVER/deploy/setup.sh
# PURPOSE: Set up storage server on VPS (run as root)
# USAGE: scp this to server, then: chmod +x setup.sh && ./setup.sh
# ═══════════════════════════════════════════════════════════════════════════════

set -e

echo "=== ATLAS Storage Server Setup ==="

# 1. Update system
apt update && apt upgrade -y

# 2. Install dependencies
apt install -y curl build-essential pkg-config libssl-dev nginx certbot python3-certbot-nginx ufw

# 3. Install Rust
if ! command -v rustc &> /dev/null; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi

# 4. Create app user
if ! id "atlas" &>/dev/null; then
    useradd -r -s /bin/false -d /opt/atlas atlas
fi

# 5. Create directories
mkdir -p /opt/atlas/data
mkdir -p /opt/atlas/bin
chown -R atlas:atlas /opt/atlas

# 6. Configure firewall
ufw allow ssh
ufw allow 80
ufw allow 443
ufw --force enable

# 7. Create systemd service
cat > /etc/systemd/system/atlas-storage.service << 'EOF'
[Unit]
Description=ATLAS Storage Server
After=network.target

[Service]
Type=simple
User=atlas
Group=atlas
WorkingDirectory=/opt/atlas
Environment=DATA_DIR=/opt/atlas/data
Environment=RUST_LOG=info
ExecStart=/opt/atlas/bin/storage-server
Restart=always
RestartSec=5

[Install]
WantedBy=multi-user.target
EOF

# 8. Create nginx config (will configure domain later)
cat > /etc/nginx/sites-available/atlas-storage << 'EOF'
server {
    listen 80;
    server_name _;  # Replace with actual domain

    location / {
        proxy_pass http://127.0.0.1:3000;
        proxy_http_version 1.1;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;

        # CORS headers (backup, server also handles this)
        add_header Access-Control-Allow-Origin * always;
        add_header Access-Control-Allow-Methods "GET, POST, OPTIONS" always;
        add_header Access-Control-Allow-Headers "Content-Type" always;

        if ($request_method = OPTIONS) {
            return 204;
        }
    }
}
EOF

ln -sf /etc/nginx/sites-available/atlas-storage /etc/nginx/sites-enabled/
rm -f /etc/nginx/sites-enabled/default

nginx -t && systemctl reload nginx

echo ""
echo "=== Setup Complete ==="
echo ""
echo "Next steps:"
echo "1. Build the server binary on your dev machine:"
echo "   cargo build --release -p storage-server"
echo ""
echo "2. Copy binary to server:"
echo "   scp target/release/storage-server root@SERVER:/opt/atlas/bin/"
echo ""
echo "3. Start the service:"
echo "   systemctl daemon-reload"
echo "   systemctl enable atlas-storage"
echo "   systemctl start atlas-storage"
echo ""
echo "4. (Optional) Set up HTTPS with a domain:"
echo "   Edit /etc/nginx/sites-available/atlas-storage"
echo "   Change server_name to your domain"
echo "   Run: certbot --nginx -d your-domain.com"
echo ""
