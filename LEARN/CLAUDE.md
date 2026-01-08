# Learn - Interactive Tutorial Platform

Multi-topic learning platform with interactive demos built in Rust/WASM. Each tutorial has its own subdomain on too.foo.

## Build & Run

```bash
# Main learn hub
trunk serve LEARN/index.html --open           # Port 8086

# Individual tutorials (use dev-serve.sh)
./SCRIPTS/dev-serve.sh slam                   # Port 8106
./SCRIPTS/dev-serve.sh ai                     # Port 8100
./SCRIPTS/dev-serve.sh esp32                  # Port 8104
```

## Architecture

```
LEARN/
├── index.html              # Hub landing page
├── learn_core/             # Shared learning components
├── learn_web/              # Web rendering utilities
│
├── AI/                     # ai.too.foo - ML/AI tutorials
├── SLAM/                   # slam.too.foo - SLAM with 5 interactive demos
├── ESP32/                  # esp32.too.foo - ESP32 programming
├── ARDUINO/                # arduino.too.foo - Arduino tutorials
├── UBUNTU/                 # ubuntu.too.foo - Ubuntu/Linux tutorials
├── OPENCV/                 # opencv.too.foo - Computer vision
├── SWARM_ROBOTICS/         # swarm.too.foo - Swarm robotics
├── SENSORS/                # sensors.too.foo - Sensor demos
└── ML/                     # ML fundamentals (core library)
```

## Tutorial Projects

### SLAM (slam.too.foo)
Interactive SLAM tutorials with 5 demos:
- Odometry simulation
- Lidar scanning
- EKF localization
- Particle filter
- Dark hallway navigation
- Full math theory with Mermaid diagrams

### AI (ai.too.foo)
ML fundamentals curriculum (12 lessons):
- Phase 1: Foundations (Linear/Logistic Regression, Neural Nets)
- Phase 2: Deep Learning (CNNs, Policy Networks)
- Phase 3: Reinforcement Learning (Q-Learning, MCTS)
- Phase 4: Advanced (AlphaZero, LLMs)

### ESP32 (esp32.too.foo)
Embedded programming tutorials:
- PWM control
- ADC reading
- I2C communication
- WiFi connectivity

### Arduino (arduino.too.foo)
Arduino programming basics and projects

### Ubuntu (ubuntu.too.foo)
Linux terminal curriculum (beginner → advanced)

### OpenCV (opencv.too.foo)
Computer vision with OpenCV

### Swarm Robotics (swarm.too.foo)
Multi-agent coordination and swarm algorithms

### Sensors (sensors.too.foo)
Sensor testing and calibration demos

## Features

- **Static site** - No server, deploys to Cloudflare Pages
- **KaTeX** - Math rendering for equations
- **Interactive demos** - Canvas/WebGL visualizations
- **Responsive** - Mobile-optimized with pop-out demo windows
- **Mermaid diagrams** - Algorithm flow visualization

## Dev Server Ports

| Tutorial | Port | URL |
|----------|------|-----|
| learn (hub) | 8086 | http://127.0.0.1:8086 |
| ai | 8100 | http://127.0.0.1:8100 |
| ubuntu | 8101 | http://127.0.0.1:8101 |
| opencv | 8102 | http://127.0.0.1:8102 |
| arduino | 8103 | http://127.0.0.1:8103 |
| esp32 | 8104 | http://127.0.0.1:8104 |
| swarm | 8105 | http://127.0.0.1:8105 |
| slam | 8106 | http://127.0.0.1:8106 |
| git | 8107 | http://127.0.0.1:8107 |
| ds | 8108 | http://127.0.0.1:8108 |
| sensors | 8084 | http://127.0.0.1:8084 |
