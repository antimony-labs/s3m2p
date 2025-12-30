# WELCOME Context

## Quick Facts
- **Path**: /home/curious/S3M2P/WELCOME
- **Port**: 8080
- **Deploy**: `./SCRIPTS/deploy.sh welcome --publish`
- **URL**: too.foo
- **Type**: WASM (Rust/Canvas)

## Key Files
| File | Purpose |
|------|---------|
| src/main.rs | WASM entry, particle sim, event loop |
| index.html | Entry point with constellation layout |
| assets/ | Project icons and static assets |

## Validation
```bash
cargo check -p welcome
trunk build WELCOME/index.html
```

## Common Tasks
1. **Update project links**: Edit `.monolith` anchors in `index.html`
2. **Add project to constellation**: Add icon to `assets/islands/`, add `.monolith` element
3. **Change particle sim**: Modify `src/main.rs`, uses DNA/sim

## Common Issues
- Keep boid count low (~200-400) for landing page performance
- Target 60 FPS on mobile
- First paint should be fast (it's a landing page)
