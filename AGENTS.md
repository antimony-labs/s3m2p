# AGENTS - Codex CLI Instructions

This repo uses LLM-only development with OpenAI Codex CLI. Codex reads this file
before starting work. Use it as the source of truth for repo-wide guidance.

## Codex workflow (best practices)
- Keep tasks small and well-scoped (aim for work that fits in about an hour). If
  the request is larger, propose a staged plan and confirm scope before coding.
- Start from clear inputs (issue/PR description, checklist, or explicit goals).
- Use Ask mode to clarify or plan; switch to Act mode after scope is agreed.
- Add narrower instructions by placing additional AGENTS.md files in specific
  subdirectories when needed.

## Quick reference

| Command | Description |
|---------|-------------|
| `cargo check --workspace` | Type check all crates |
| `cargo test --workspace` | Run all tests |
| `trunk build HELIOS/index.html` | Build helios WASM |
| `trunk build WELCOME/index.html` | Build WELCOME (too.foo) WASM |
| `./SCRIPTS/dev up <project>` | Dev server (auto-kills existing) |
| `./SCRIPTS/deploy.sh welcome --publish` | Deploy too.foo (WELCOME) |
| `./SCRIPTS/worktree.sh create <issue>` | Create worktree for issue |
| `./SCRIPTS/audit.sh` | Security audit |

## Dev server

Always use `./SCRIPTS/dev up <project>` to start dev servers. Do not run
`trunk serve` directly. The dev script binds to `0.0.0.0`, auto-kills existing
processes on the port, and sets the correct environment variables.

## Issue workflow (issue paste)

1. Extract the issue number (e.g., `#42` or from URL).
2. Run `./SCRIPTS/worktree.sh create <issue_id>`.
3. Ask the user to `cd` into the new worktree path from the script output.
4. Continue work inside the new worktree.

## Directory structure (L1)

```
S3M2P/
├── DNA/                    # Core algorithms + infrastructure
├── WELCOME/                # Landing page (too.foo)
├── HELIOS/                 # Solar system (helios.too.foo)
├── SIMULATION/             # Simulations (e.g., chladni.too.foo)
├── TOOLS/                  # User-facing tools (autocrate.too.foo, etc.)
├── MCAD/                   # Mechanical CAD (mcad.too.foo)
├── ATLAS/                  # Interactive maps (atlas.too.foo)
├── LEARN/                  # Learning tutorials (ai.too.foo, slam.too.foo, etc.)
└── BLOG/                   # Blog platform (blog.too.foo)
```

## Naming convention

| Level | Case | Example | Rationale |
|-------|------|---------|-----------|
| Category folders (L1) | UPPERCASE | `BLOG/`, `LEARN/`, `SIMULATION/` | Fixed landmarks |
| Project folders (L2) | UPPERCASE | `HELIOS/`, `AUTOCRATE/` | Fixed project names |
| Config folders | lowercase | `src/`, `dist/`, `assets/` | Standard conventions |
| Variable files | lowercase | `main.rs`, `index.html` | Content changes |
| Fixed files | As required | `Cargo.toml`, `CLAUDE.md` | Syntax requirements |

## DNA & CORE pattern

DNA = shared foundation; CORE = project-specific engines.

```
DNA (foundation)
 └── CORE engines (domain-specific)
      └── Projects (applications)
```

## Dependencies policy

Minimize external dependencies. Only use external crates for:
- GPU access (wgpu)
- Math primitives (glam)
- WASM bindings (wasm-bindgen)
- Serialization (serde)

## Project-specific context

Project deep-dives currently live in `*/CLAUDE.md`. Use those for detailed
architecture notes until they are migrated to local AGENTS.md files.
