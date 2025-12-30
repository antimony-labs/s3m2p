# LEARN Context

## Quick Facts
- **Path**: /home/curious/S3M2P/LEARN
- **Port**: 8086 (hub), 8100-8106 (tutorials)
- **Deploy**: `./SCRIPTS/deploy.sh learn --publish`
- **URL**: learn.too.foo (hub)
- **Type**: Multi-project tutorial platform

## Tutorial Projects
| Project | Port | Path |
|---------|------|------|
| AI | 8100 | LEARN/AI |
| Ubuntu | 8101 | LEARN/UBUNTU |
| OpenCV | 8102 | LEARN/OPENCV |
| Arduino | 8103 | LEARN/ARDUINO |
| ESP32 | 8104 | LEARN/ESP32 |
| Swarm | 8105 | LEARN/SWARM_ROBOTICS |
| SLAM | 8106 | LEARN/SLAM |
| Sensors | 8084 | LEARN/SENSORS |

## Validation
```bash
# Check specific tutorial
trunk build LEARN/SLAM/index.html
trunk build LEARN/AI/index.html
```

## Common Tasks
1. **Add tutorial**: Create new folder in LEARN/, add index.html and src/
2. **Add lesson**: Edit existing tutorial's index.html
3. **Add demo**: Add WASM bindings, create demo-*.html

## Features
- KaTeX for math rendering
- Mermaid diagrams
- Mobile-responsive
- Interactive canvas demos
