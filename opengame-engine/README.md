# OpenGame Engine

<p align="center">
  <strong>A lightweight, high-performance 2D game engine built in Rust, targeting WebAssembly and the browser.</strong>
</p>

<p align="center">
  <a href="#quickstart">Quickstart</a> &bull;
  <a href="#features">Features</a> &bull;
  <a href="#architecture">Architecture</a> &bull;
  <a href="#documentation">Documentation</a> &bull;
  <a href="#examples">Examples</a> &bull;
  <a href="#contributing">Contributing</a> &bull;
  <a href="#license">License</a>
</p>

---

## Overview

OpenGame Engine is a modern 2D game engine designed for the web platform. Written entirely in Rust and compiled to WebAssembly, it delivers near-native performance directly in the browser with zero plugins or installations required by end users.

The engine provides a comprehensive set of tools for building 2D games: an Entity Component System (ECS) for game logic, a WebGL 2.0 renderer for hardware-accelerated graphics, a built-in physics engine with collision detection, a full input system supporting keyboard/mouse/touch/gamepad, a Web Audio API-powered sound engine, and a scene management system for organizing game states.

### Design Philosophy

- **Web-first**: Every subsystem is designed around browser APIs (WebGL2, Web Audio, WebAssembly) for seamless integration.
- **ECS-driven**: Game objects are composed from reusable components and processed by systems, enabling clean separation of concerns and high performance.
- **Zero-config development**: The CLI tool (`og`) handles project scaffolding, building, serving, and asset management out of the box.
- **Minimal bundle size**: Release builds use `wasm-opt` optimization (`opt-level = "z"`, LTO, symbol stripping) to keep WASM bundles as small as possible.

## Features

| Category | Capabilities |
|---|---|
| **Rendering** | WebGL 2.0 via `glow`, sprite rendering, shape rendering (rect/circle), bitmap text, camera2D with zoom/rotation/pan, texture management |
| **ECS** | Entity-Component architecture with `generational-arena` storage, typed component queries (`QuerySingle`, `QueryDouble`), entity builder pattern, resource management |
| **Physics** | 2D rigid body dynamics (Dynamic/Kinematic/Static), AABB and circle colliders, spatial grid broad-phase, impulse-based collision resolution, trigger volumes |
| **Input** | Keyboard (key down/pressed/released), mouse (position/buttons/wheel), multi-touch with pressure, gamepad with button mapping |
| **Audio** | Web Audio API, audio buffer management, separate Music and SFX channels, per-channel volume control, looping support |
| **Scene** | Scene stack with push/pop/switch, scene lifecycle callbacks (`on_enter`/`on_exit`/`update`/`render`), fade transitions |
| **Asset** | Async HTTP fetch, HTML image loading, asset caching |
| **Time** | High-resolution timing via `Performance` API, fixed timestep with accumulator, frame interpolation alpha, FPS counter, `Timer` utility |
| **Math** | Re-exports `glam` (Vec2, Vec3, Vec4, Mat3, Mat4), lerp, smoothstep, remap, angle helpers, orthographic projection |
| **Debug** | Debug overlay (FPS, frame time, entity count, profiler data), scope-based profiler with history |
| **CLI** | Project scaffolding, build/serve/run, code formatting, linting, clippy, testing, asset management, doctor diagnostics |

## Project Structure

```
opengame/
├── Cargo.toml                    # Workspace root
├── Makefile                      # Build & dev commands
├── Trunk.toml                    # Trunk bundler config
├── index.html                    # HTML entry point
├── LICENSE                       # MIT License
│
├── crates/
│   ├── engine/                   # Core engine library
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs            # Module root & prelude
│   │       ├── app.rs            # App runner & game loop
│   │       ├── ecs/              # Entity Component System
│   │       │   ├── entity.rs     # Entity handle
│   │       │   ├── component.rs  # Typed component storage
│   │       │   ├── world.rs      # World (entity + component + resource store)
│   │       │   ├── system.rs     # System trait & scheduler
│   │       │   ├── query.rs      # Type-safe queries
│   │       │   └── builder.rs    # Entity builder
│   │       ├── renderer/         # Rendering subsystem
│   │       │   ├── gl_backend.rs # WebGL2 backend (glow)
│   │       │   ├── shader.rs     # Shader compilation & uniforms
│   │       │   ├── sprite.rs     # Sprite batch renderer
│   │       │   ├── shape.rs      # Shape renderer (rect, circle)
│   │       │   ├── texture.rs    # Texture loading & management
│   │       │   ├── text.rs       # Bitmap font renderer
│   │       │   └── camera.rs     # Camera2D
│   │       ├── physics/          # Physics subsystem
│   │       │   ├── rigid_body.rs # RigidBody component
│   │       │   ├── collider.rs   # Collider component
│   │       │   ├── collision.rs  # Collision detection algorithms
│   │       │   ├── solver.rs     # Impulse-based solver
│   │       │   └── spatial.rs    # Spatial hash grid
│   │       ├── input/            # Input subsystem
│   │       │   ├── keyboard.rs   # Keyboard state
│   │       │   ├── mouse.rs      # Mouse state
│   │       │   ├── touch.rs      # Touch state
│   │       │   ├── gamepad.rs    # Gamepad manager
│   │       │   └── keys.rs       # Key/button code enums
│   │       ├── audio/            # Audio subsystem
│   │       │   ├── engine.rs     # AudioEngine (Web Audio API)
│   │       │   ├── source.rs     # AudioSource & AudioHandle
│   │       │   └── mixer.rs      # Audio mixer
│   │       ├── scene/            # Scene management
│   │       │   ├── manager.rs    # SceneManager & Scene trait
│   │       │   └── transition.rs # Fade transitions
│   │       ├── asset/            # Asset loading
│   │       │   ├── loader.rs     # Fetch & image loaders
│   │       │   ├── cache.rs      # Asset cache
│   │       │   └── image.rs      # Image asset
│   │       ├── math.rs           # Math utilities (re-exports glam)
│   │       ├── color.rs          # Color type & presets
│   │       ├── transform.rs      # Transform2D component
│   │       ├── sprite_component.rs # Sprite & SpriteSheet
│   │       ├── event.rs          # Type-erased event bus
│   │       ├── time.rs           # Time & Timer
│   │       ├── profiler.rs       # Performance profiler
│   │       ├── debug.rs          # Debug overlay
│   │       └── log.rs            # Console logging macros
│   │
│   ├── cli/                      # CLI tool
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── main.rs           # CLI entry point
│   │       ├── cli.rs            # Command definitions (clap)
│   │       └── commands/         # Command implementations
│   │
│   └── examples/                 # Example games
│       ├── pong/                 # Classic Pong
│       └── platformer/           # 2D Platformer
│
├── web/
│   └── style.css                 # Global styles
│
└── .github/workflows/ci.yml     # CI pipeline
```

## Quickstart

### Prerequisites

Ensure you have the following installed:

- **Rust** (1.75+): [rustup.rs](https://rustup.rs/)
- **wasm32 target**: `rustup target add wasm32-unknown-unknown`
- **Trunk**: `cargo install trunk`

### Install

```bash
# Clone the repository
git clone https://github.com/opengame/opengame.git
cd opengame

# Install development tools
make install-tools

# Verify your environment
cargo run -p opengame-cli -- doctor
```

### Run the Examples

```bash
# Start the Pong example with hot reload
cargo run -p opengame-cli -- run pong

# Or start the Platformer example
cargo run -p opengame-cli -- run platformer

# Or use Make directly
make dev
```

Open your browser at `http://localhost:8080`.

### Create a New Project

```bash
# Using the CLI tool
cargo run -p opengame-cli -- new my-game

# With ECS + App template
cargo run -p opengame-cli -- new my-game --template ecs
```

### Build for Production

```bash
# Release build with full optimization
make release

# Check bundle size
make size
```

## Architecture

OpenGame Engine follows a modular architecture centered around the ECS pattern:

```
┌─────────────────────────────────────────────────────┐
│                      App                            │
│  ┌──────────┐  ┌──────────┐  ┌──────────────────┐  │
│  │  World    │  │ Scheduler│  │    Renderer       │  │
│  │  (ECS)    │  │ (Systems)│  │  (WebGL2/glow)   │  │
│  └──────────┘  └──────────┘  └──────────────────┘  │
│  ┌──────────┐  ┌──────────┐  ┌──────────────────┐  │
│  │  Input    │  │  Audio   │  │  Scene Manager   │  │
│  │(KB/Mouse/ │  │(WebAudio)│  │  (Stack-based)   │  │
│  │Touch/Pad) │  │          │  │                  │  │
│  └──────────┘  └──────────┘  └──────────────────┘  │
│  ┌──────────┐  ┌──────────┐  ┌──────────────────┐  │
│  │  Physics  │  │  Time    │  │  Event Bus       │  │
│  │(Collision)│  │(Fixed DT)│  │  (Type-erased)   │  │
│  └──────────┘  └──────────┘  └──────────────────┘  │
└─────────────────────────────────────────────────────┘
         ▼               ▼               ▼
    ┌─────────┐    ┌─────────┐    ┌─────────┐
    │ User    │    │ User    │    │ User    │
    │ Startup │    │ Update  │    │ Render  │
    │ Systems │    │ Systems │    │ Systems │
    └─────────┘    └─────────┘    └─────────┘
```

### Game Loop

The engine uses a **fixed timestep** game loop with frame interpolation:

1. **Time Update**: Calculate delta time, accumulate fixed-step time
2. **Input Update**: Poll browser events, update state
3. **Fixed Update**: Run physics and gameplay systems at a fixed rate (default: 60 Hz)
4. **Render**: Clear the canvas, run render systems with interpolation alpha

```text
Frame N:
  ├─ Time.update()
  ├─ Input.update()
  ├─ while (accumulator >= fixed_dt):
  │    ├─ PhysicsSystem.update(fixed_dt)
  │    └─ UserSystems.update(fixed_dt)
  ├─ alpha = accumulator / fixed_dt
  ├─ gl.clear()
  └─ RenderSystems.render(alpha)
```

### ECS Pattern

Entities are lightweight handles. Components are stored in typed, contiguous arrays. Systems process entities matching specific component combinations.

```rust
// Spawn an entity with components
let player = app.world.spawn()
    .with(Transform2D::new(Vec2::new(100.0, 200.0)))
    .with(Sprite::new().with_color(Color::RED))
    .with(RigidBody::dynamic())
    .with(Collider::rectangle(32.0, 32.0))
    .build();

// Query and update in a system
fn movement_system(world: &mut World, dt: f32) {
    let mut query = QueryDoubleMut::<Transform2D, RigidBody>::new(world).unwrap();
    for (_entity, transform, rb) in query.iter() {
        transform.translate(rb.velocity * dt);
    }
}
```

## Documentation

| Document | Description |
|---|---|
| [Getting Started](docs/getting-started.md) | Environment setup, toolchain installation, first project |
| [Architecture Guide](docs/architecture.md) | Deep dive into engine subsystems, ECS, game loop, rendering pipeline |
| [API Reference](docs/api-reference.md) | Complete API documentation for all public types and functions |
| [CLI Reference](docs/cli-reference.md) | All `og` CLI commands, options, and usage examples |
| [Examples Guide](docs/examples.md) | Detailed walkthrough of the Pong and Platformer examples |
| [FAQ](docs/faq.md) | Common issues, troubleshooting, and best practices |

## Examples

### Pong

A classic two-player Pong game demonstrating shape rendering, input handling, and simple game logic.

**Controls**: `W/S` for left paddle, `Up/Down` for right paddle, `Space` to restart.

### Platformer

A side-scrolling platformer demonstrating physics, camera following, collectibles, and platform collision.

**Controls**: `A/D` or `Arrow Left/Right` to move, `Space/W/Up` to jump.

## Technology Stack

| Technology | Role | Version |
|---|---|---|
| Rust | Core language | 2021 Edition, MSRV 1.75 |
| WebAssembly | Compilation target | `wasm32-unknown-unknown` |
| WebGL 2.0 | Graphics API | via `glow` 0.16 |
| wasm-bindgen | Rust/JS interop | 0.2 |
| web-sys | Browser API bindings | 0.3 |
| glam | Math library | 0.29 |
| generational-arena | ECS entity storage | 0.2 |
| Trunk | WASM bundler | Latest |
| clap | CLI framework | 4.x |

## Development

### Common Commands

```bash
make dev              # Start dev server with hot reload
make build            # Build WASM (debug)
make release          # Build WASM (release, optimized)
make test             # Run all tests
make lint             # Format check + clippy
make fmt              # Auto-format code
make doc              # Generate & open API docs
make clean            # Remove build artifacts
make all              # Full pipeline: fmt + clippy + test + build
make cli              # Build the CLI tool
make cli-install      # Install 'og' to ~/.local/bin
```

### Testing

```bash
# Run unit tests
make test

# Run with verbose output
make test-verbose

# Run a specific test
cargo test -p opengame-engine test_spawn_and_despawn
```

### Code Quality

The project enforces consistent code quality through:

- **rustfmt**: Automated formatting ([config](rustfmt.toml): 100-char lines, 4-space indent)
- **clippy**: Static analysis with warnings as errors
- **cargo-deny**: Dependency auditing for licenses, advisories, and duplicates
- **CI**: GitHub Actions pipeline with format check, clippy, tests, WASM build, and doc generation

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Run the full pipeline (`make all`)
4. Commit your changes (`git commit -m 'Add amazing feature'`)
5. Push to the branch (`git push origin feature/amazing-feature`)
6. Open a Pull Request

Please ensure:
- All tests pass (`make test`)
- Code is formatted (`make fmt`)
- Clippy is clean (`make clippy`)
- New features include tests where applicable

## License

This project is licensed under the **MIT License**. See the [LICENSE](LICENSE) file for details.
