# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

OpenGame Engine is a lightweight 2D game engine written in Rust that compiles to WebAssembly for browser-based games. It uses an Entity Component System (ECS) architecture with a fixed timestep game loop. The project is in early prototype stage (VTP phase 1).

## Build & Development Commands

Prerequisites: Rust 1.75+, `wasm32-unknown-unknown` target, Trunk bundler. Run `make install-tools` to set up.

| Command | Purpose |
|---------|---------|
| `make dev` | Dev server with hot reload (port 8080) via Trunk |
| `make build` | Debug WASM build |
| `make release` | Optimized release WASM build (wasm-opt level "z", LTO) |
| `make test` | Run engine unit + doc tests on native target |
| `make lint` | Format check + clippy (CI gate) |
| `make fmt` | Auto-format all code |
| `make clippy` | Clippy with `-D warnings` |
| `make all` | Full CI pipeline: fmt + clippy + test + build |
| `make doc` | Generate and open API docs |
| `make size` | Release build + show WASM bundle size |
| `make clean` | Remove build artifacts and `dist/` |

**CLI tool** (`og`, or `cargo run -p opengame-cli --`):
- `og new <name> [--template ecs]` — scaffold project
- `og build [--release] [--example <name>]` — WASM build
- `og serve [--port 8080]` / `og dev` — dev server
- `og run <example> [--release]` — run an example
- `og doctor` — check environment/tool dependencies

**Running a single test:**
```bash
cargo test -p opengame-engine --target x86_64-unknown-linux-gnu <test_name>
```

Tests must run on native target (`x86_64-unknown-linux-gnu`), not WASM.

## Workspace Structure

```
Cargo.toml              # Workspace root (resolver v2)
crates/
  engine/               # Core engine library (opengame-engine)
  cli/                  # CLI tool (opengame-cli, binary: og)
  examples/
    pong/               # Classic Pong example
    platformer/         # 2D platformer example
    space-blitz/        # Space shooter example
assets/                 # Game assets (audio/, fonts/, textures/)
web/                    # Static web files (style.css)
docs/                   # Extensive documentation
```

## Architecture

### ECS Core (`crates/engine/src/ecs/`)
- `Entity` — generational-arena backed entity IDs
- `Component` — trait for data attached to entities, stored in typed arenas
- `World` — owns all entity/component storage and resources
- `System` — trait with `update(world, time)` or `render(world, renderer)`
- `SystemScheduler` — runs startup systems once, then update/render each frame
- Queries: `QuerySingle/Mut`, `QueryDouble/Mut` for typed component access

### App & Game Loop (`crates/engine/src/app.rs`, WASM-only)
`App` is the top-level entry point owning: `World`, `SystemScheduler`, `Renderer`, `InputManager`, `AudioEngine`, `Time`, `EventBus`, `SceneManager`. The loop: startup systems → `requestAnimationFrame` → time update → input poll → fixed-step update loop → alpha interpolation → clear + render.

### Renderer (`crates/engine/src/renderer/`)
WebGL2 via `glow`. Sub-modules: `GlBackend`, `Camera2D`, `SpriteRenderer` (batched), `ShapeRenderer` (rect/circle), `TextRenderer` (bitmap fonts), `TextureManager`.

### Physics (`crates/engine/src/physics/`)
RigidBody (Dynamic/Kinematic/Static), Collider (AABB/circle, triggers with friction/restitution), iterative impulse-based collision solver with per-collider material properties, spatial hash grid broad-phase. Physics step uses max dt clamp (0.1s) for stability.

### Other Systems
- **Input** (`input/`) — keyboard, mouse, multi-touch, gamepad via browser event listeners
- **Audio** (`audio/`) — Web Audio API, music/SFX channels with mixer
- **Scene** (`scene/`) — stack-based scene manager with lifecycle callbacks and fade transitions
- **Asset** (`asset/`) — async HTTP fetch loader with cache
- **Event** (`event.rs`) — type-erased event bus with subscribe/unsubscribe (returns `SubscriptionId`)
- **Math** (`math.rs`) — re-exports `glam`, plus lerp/smoothstep/remap/angle helpers

### Interior Mutability
`Camera2D` and `Transform2D` use `Cell`-based dirty flags so matrix getters (`projection()`, `view()`, `local_matrix()`, `world_matrix()`) take `&self` instead of `&mut self`.

### Conditional Compilation
Many modules are gated behind `#[cfg(target_arch = "wasm32")]`: `app`, `asset`, `audio`, `input`, and renderer submodules (gl_backend, shape, sprite, text, texture). The ECS, physics, math, color, transform, time, profiler, debug, and scene modules compile for all targets, enabling native testing.

## Code Style

- Edition 2021, 100-char max line width, 4-space indent (per `rustfmt.toml`)
- MSRV: 1.75 (per `clippy.toml`)
- Clippy: max 8 function args, type complexity threshold 350
- Licenses allowed: MIT, Apache-2.0, BSD, ISC, Unicode, Zlib, 0BSD (per `deny.toml`)

## CI Pipeline

Runs on push/PR to `main`: fmt → clippy (wasm32 target) → test (native target, engine + CLI) → trunk build (release) → doc generation. All jobs must pass.

## Key Configuration Files

| File | Purpose |
|------|---------|
| `.cargo/config.toml` | Default build target: `wasm32-unknown-unknown` |
| `Trunk.toml` | Trunk bundler config (entry: `index.html`, output: `dist/`, port 8080) |
| `clippy.toml` | MSRV, lint thresholds |
| `rustfmt.toml` | Formatting rules |
| `deny.toml` | Dependency license/advisory auditing |

## Adding a New Example

Create `crates/examples/<name>/` with `Cargo.toml` (crate-type `["cdylib", "rlib"]`, dep on `opengame-engine`), `src/lib.rs` with `#[wasm_bindgen(start)]` entry point, and `index.html` with `<link data-trunk rel="rust" data-wasm-opt="z">`. Add the crate to workspace members in root `Cargo.toml`.
