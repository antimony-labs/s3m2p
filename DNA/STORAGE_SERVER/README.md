# ATLAS Storage Server

Serves preprocessed map data for the ATLAS project.

## Endpoints

| Endpoint | Description |
|----------|-------------|
| `GET /v1/health` | Health check |
| `GET /v1/atlas/health` | ATLAS-specific health |
| `GET /v1/atlas/layers` | List available layers |
| `GET /v1/atlas/:layer/:resolution` | Get layer data |

### Layer Naming

```
GET /v1/atlas/countries/110m  → countries_110m.geojson
GET /v1/atlas/states/10m      → states_10m.geojson
GET /v1/atlas/places/10m      → places_10m.geojson
```

## Local Development

```bash
cd DNA/STORAGE_SERVER/deploy
./run-local.sh
```

Then test:
```bash
curl http://127.0.0.1:3000/v1/atlas/layers
curl http://127.0.0.1:3000/v1/atlas/countries/110m | head -c 500
```

## VPS Deployment

### 1. Initial Server Setup

Copy setup script to server and run:

```bash
scp deploy/setup.sh root@YOUR_SERVER:/root/
ssh root@YOUR_SERVER
chmod +x setup.sh && ./setup.sh
```

### 2. Build and Deploy Binary

```bash
# On dev machine
cargo build --release -p storage-server
scp target/release/storage-server root@YOUR_SERVER:/opt/atlas/bin/
ssh root@YOUR_SERVER "systemctl daemon-reload && systemctl enable atlas-storage && systemctl start atlas-storage"
```

### 3. Sync Data

```bash
./deploy/sync-atlas-data.sh YOUR_SERVER_IP
```

### 4. Verify

```bash
curl http://YOUR_SERVER:3000/v1/health
curl http://YOUR_SERVER:3000/v1/atlas/layers
```

## GitHub Actions Deployment

Add these secrets to your repository:

| Secret | Description |
|--------|-------------|
| `VPS_HOST` | Server IP address |
| `VPS_SSH_KEY` | Private SSH key for deployment |

The workflow triggers on:
- Push to `DNA/STORAGE_SERVER/**`
- Push to `ATLAS/assets/**`
- Manual dispatch

## Data Format

The server serves both:
- `.geojson` - Standard GeoJSON (larger, human-readable)
- `.geo` - Custom binary format (smaller, faster parsing) [TODO]

## Architecture

```
Client (ATLAS WASM)
        ↓
    nginx (port 80/443)
        ↓
    storage-server (port 3000)
        ↓
    /opt/atlas/data/atlas/
```
