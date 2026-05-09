# Architecture Guide

This document provides a deep dive into the internal architecture of OpenGame Engine. It covers the design of each subsystem, how they interact, and the patterns used throughout the codebase.

## High-Level Overview

OpenGame Engine is a modular 2D game engine built around an Entity Component System (ECS) core. The engine is compiled to WebAssembly and runs entirely in the browser, leveraging WebGL 2.0 for rendering and Web Audio API for sound.

```
┌──────────────────────────────────────────────────────────┐
│                        App                               │
│                                                          │
│  ┌───────────┐   ┌────────────┐   ┌───────────────────┐ │
│  │   World    │   │  Scheduler │   │    Renderer       │ │
│  │  (ECS)     │   │  (Systems) │   │  (WebGL2/glow)   │ │
│  └─────┬─────┘   └──────┬─────┘   └────────┬──────────┘ │
│        │                │                   │            │
│  ┌─────┴─────┐   ┌──────┴─────┐   ┌────────┴──────────┐ │
│  │  Entities  │   │  Startup   │   │  SpriteRenderer   │ │
│  │ Components │   │  Update    │   │  ShapeRenderer    │ │
│  │ Resources  │   │  Render    │   │  TextureManager   │ │
│  └───────────┘   └────────────┘   └───────────────────┘ │
│                                                          │
│  ┌───────────┐   ┌────────────┐   ┌───────────────────┐ │
│  │   Input    │   │    Audio   │   │  Scene Manager    │ │
│  │(KB/Mouse/  │   │ (WebAudio) │   │  (Stack-based)   │ │
│  │Touch/Pad)  │   │            │   │                   │ │
│  └───────────┘   └────────────┘   └───────────────────┘ │
│                                                          │
│  ┌───────────┐   ┌────────────┐   ┌───────────────────┐ │
│  │  Physics   │   │    Time    │   │    Event Bus      │ │
│  │ (Collision)│   │(Fixed DT)  │   │  (Type-erased)   │ │
│  └───────────┘   └────────────┘   └───────────────────┘ │
└──────────────────────────────────────────────────────────┘
```

## Game Loop

The engine uses a **fixed timestep with frame interpolation** pattern. This decouples physics and gameplay updates from the display refresh rate, ensuring deterministic behavior.

### Loop Phases

```
Each requestAnimationFrame callback:
│
├─ 1. Time Update
│     Calculate raw delta from Performance.now()
│     Clamp delta to max 0.25s (prevents spiral of death)
│     Accumulate time into fixed-step accumulator
│     Update FPS counter
│
├─ 2. Input Update
│     Copy event-driven state (keyboard, mouse, touch)
│     into queryable state for the current frame
│     Poll gamepad state
│
├─ 3. Fixed Update Loop
│     while (accumulator >= fixed_timestep):
│       Run all registered update systems
│       Run scene manager update
│       accumulator -= fixed_timestep
│
├─ 4. Render
│     Calculate interpolation alpha = accumulator / fixed_timestep
│     Resize canvas if needed
│     Update camera viewport
│     Clear with configured color
│     Enable blending
│     Run all registered render systems
│     Run scene manager render
│
└─ 5. Schedule Next Frame
      requestAnimationFrame(self)
```

### Implementation

The game loop is implemented in `App::run()` (crates/engine/src/app.rs). The `App` owns all subsystems and uses `Rc<RefCell<...>>` to share itself into the `requestAnimationFrame` closure.

Key constants:

- `fixed_timestep`: 1/60 second (configurable via `Time::set_fixed_timestep`)
- `max delta`: 0.25 seconds (prevents physics explosion after tab-away)

## Entity Component System (ECS)

The ECS is the heart of the engine. It separates data (components) from logic (systems) and identity (entities).

### Entity

An `Entity` is a lightweight handle wrapping a `generational_arena::Index`. Generational indices prevent the ABA problem: when an entity is despawned and a new one is created at the same slot, the generation counter ensures old handles are invalid.

```rust
pub struct Entity {
    index: generational_arena::Index,
}
```

### Component Storage

Components are stored in type-erased `HashMap<TypeId, Box<dyn ComponentStorage>>`. Each entry is a `TypedStorage<T>` containing a `generational_arena::Arena<T>`.

```
World
├── entities: Arena<EntityData>
├── storages: HashMap<TypeId, Box<dyn ComponentStorage>>
│   ├── TypeId(Transform2D) → TypedStorage<Transform2D>
│   ├── TypeId(Sprite)      → TypedStorage<Sprite>
│   ├── TypeId(RigidBody)   → TypedStorage<RigidBody>
│   └── ...
└── resources: HashMap<TypeId, Box<dyn Any>>
    ├── TypeId(GameConfig)  → Box<GameConfig>
    └── ...
```

### World

The `World` is the central container for all ECS data. It provides:

- **Entity lifecycle**: `spawn()`, `spawn_empty()`, `despawn()`, `is_alive()`
- **Component CRUD**: `insert_component()`, `get_component()`, `get_component_mut()`, `remove_component()`, `has_component()`
- **Resource management**: `insert_resource()`, `get_resource()`, `get_resource_mut()`, `remove_resource()`
- **Entity enumeration**: `entities()`, `entity_count()`

### Entity Builder

The builder pattern provides a fluent API for creating entities with components:

```rust
let entity = world.spawn()
    .with(Transform2D::new(Vec2::new(100.0, 200.0)))
    .with(Sprite::new().with_color(Color::RED))
    .with(RigidBody::dynamic())
    .build();
```

### Queries

Queries provide type-safe iteration over entities with specific component combinations:

| Query Type | Description |
|---|---|
| `QuerySingle<T>` | Read-only access to all entities with component `T` |
| `QuerySingleMut<T>` | Mutable access to all entities with component `T` |
| `QueryDouble<A, B>` | Read-only access to entities with both `A` and `B` |
| `QueryDoubleMut<A, B>` | Mutable access to both `A` and `B` (different types only) |

```rust
fn movement_system(world: &mut World, dt: f32) {
    let mut query = QueryDoubleMut::<Transform2D, RigidBody>::new(world).unwrap();
    for (_entity, transform, rb) in query.iter() {
        transform.translate(rb.velocity * dt);
    }
}
```

### Systems

Systems are the units of game logic. There are three types:

| System Type | Signature | When Run |
|---|---|---|
| Startup | `FnMut(&mut World)` | Once, at engine initialization |
| Update | `impl System` (trait with `update(&mut World, f32)`) | Every fixed timestep |
| Render | `FnMut(&mut World, f32)` | Every frame, with interpolation alpha |

The `System` trait:

```rust
pub trait System {
    fn update(&mut self, world: &mut World, dt: f32);
    fn name(&self) -> &str { /* type_name default */ }
}
```

Closures implementing `FnMut(&mut World, f32)` automatically satisfy the `System` trait.

### SystemScheduler

The `SystemScheduler` manages system execution order. Systems run in registration order:

```
Startup phase (runs once):
  startup_system_1 → startup_system_2 → ... → (cleared)

Update phase (every fixed timestep):
  update_system_1 → update_system_2 → ...

Render phase (every frame):
  render_system_1 → render_system_2 → ...
```

## Rendering

### WebGL 2.0 Backend

The renderer uses the `glow` crate for cross-platform OpenGL/WebGL access. In the browser, this maps to WebGL 2.0 (`WebGl2RenderingContext`).

The `GlBackend` manages:
- Canvas element and context
- Viewport dimensions
- Clear color and blend state
- Auto-resize on window changes

### Shader System

Shaders are compiled from GLSL ES 3.0 source strings. The engine includes built-in shaders for:

- **Sprite shader**: Vertex positions, texture coordinates, and per-vertex color
- **Shape shader**: Vertex positions with uniform color

Built-in GLSL sources are defined in `crates/engine/src/renderer/shader.rs`:

- `SPRITE_VERTEX_SHADER` / `SPRITE_FRAGMENT_SHADER`
- `SHAPE_VERTEX_SHADER` / `SHAPE_FRAGMENT_SHADER`

### SpriteRenderer

Batched sprite renderer that collects sprite quads and flushes them to the GPU in a single draw call. Each sprite vertex contains:

- Position (2D)
- Texture coordinates (UV)
- Color (RGBA, per-vertex tinting)

### ShapeRenderer

Renders geometric shapes (rectangles, circles) with solid colors. Used for prototyping, UI elements, and debug visualization.

### TextureManager

Handles texture loading, storage, and binding. Textures are loaded asynchronously from URLs and cached by handle.

### Camera2D

Provides 2D camera functionality:

- **Position**: World-space camera center
- **Zoom**: Scale factor (1.0 = default)
- **Rotation**: Camera rotation in radians
- **Viewport**: Canvas dimensions

The camera generates a view-projection matrix (`projection * view`) that transforms world coordinates to clip space. It uses an orthographic projection suitable for 2D rendering.

Coordinate system:
- Origin at center of screen
- X-axis: right
- Y-axis: up (opposite of screen space)

Coordinate conversion:
- `screen_to_world(screen_pos)`: Convert screen pixels to world coordinates
- `world_to_screen(world_pos)`: Convert world coordinates to screen pixels

## Physics

### Architecture

The physics subsystem consists of:

1. **RigidBody**: Mass, velocity, acceleration, forces, damping
2. **Collider**: Geometric shapes (AABB rectangle, circle) with offset, trigger flag, friction, restitution
3. **Solver**: Applies gravity and resolves collisions using impulse-based resolution
4. **SpatialGrid**: Broad-phase spatial hash for efficient collision pair detection
5. **PhysicsSystem**: Orchestrates the full physics step

### Physics Step

```
PhysicsSystem.step(world, dt):
│
├─ 1. Apply Gravity & Integrate
│     For each entity with (Transform2D, RigidBody):
│       Apply gravity (scaled by gravity_scale)
│       Integrate velocity → position
│
├─ 2. Broad Phase (Spatial Grid)
│     Clear grid
│     Insert all entities with (Transform2D, Collider)
│     For each entity, query nearby candidates
│
├─ 3. Narrow Phase (Collision Detection)
│     For each candidate pair:
│       Test AABB vs AABB, Circle vs Circle, or mixed
│       If collision detected, record (normal, depth, point)
│
└─ 4. Resolve Collisions
      For each collision:
        Skip if either collider is a trigger
        Apply positional correction (push apart)
        Apply velocity impulse (bounce)
```

### RigidBody Types

| Type | Behavior |
|---|---|
| `Dynamic` | Affected by gravity and forces, responds to collisions |
| `Kinematic` | Not affected by gravity, infinite mass, moves via velocity |
| `Static` | Infinite mass, does not move, acts as immovable obstacle |

### Collider Shapes

- **Rectangle**: Axis-aligned bounding box (AABB) defined by width and height
- **Circle**: Defined by radius

Both support:
- `offset`: Position offset from the entity's transform
- `is_trigger`: If true, generates collision events but no physical response
- `friction`: Surface friction coefficient
- `restitution`: Bounce coefficient (0 = no bounce, 1 = perfect bounce)
- `layer` / `mask`: Collision filtering bitmask

## Input System

### Design

The input system uses event listeners attached to the browser's `document` object. Events are captured asynchronously and stored in shared `Rc<RefCell<...>>` state. Each frame, the `InputManager::update()` method copies the event-driven state into the queryable state.

### Supported Input Devices

#### Keyboard

- `is_key_down(key)`: True while a key is held
- `is_key_pressed(key)`: True only on the frame the key was first pressed
- `is_key_released(key)`: True only on the frame the key was released

Key codes are mapped from JavaScript `KeyboardEvent.code` strings to the `KeyCode` enum.

#### Mouse

- Position tracking (screen coordinates)
- Button state (down/pressed/released) for left, right, middle
- Wheel delta for scroll events

#### Touch

- Multi-touch support with individual touch point tracking
- Each touch has: identifier, position, force (pressure)
- Events: touchstart, touchmove, touchend

#### Gamepad

- Polling-based via `navigator.getGamepads()`
- Button and axis state updated each frame

## Audio System

### Architecture

The audio system wraps the Web Audio API:

```
AudioEngine
├── AudioContext
├── Master Gain Node
│   ├── Music Gain Node → destination
│   └── SFX Gain Node → destination
├── Buffer Cache (HashMap<u32, AudioBuffer>)
└── Playing Sources (Vec<PlayingSource>)
```

### Features

- **Channel separation**: Music and SFX are routed through independent gain nodes
- **Volume control**: Master, music, and SFX volumes are independently adjustable
- **Buffer management**: Audio buffers are loaded and cached by numeric ID
- **Playback**: `play(buffer_id, volume, looping)` for SFX, `play_music(buffer_id, volume)` for music (auto-loop)

### Audio Categories

- `AudioCategory::Music`: Routed through the music gain node, defaults to looping
- `AudioCategory::Sfx`: Routed through the SFX gain node

## Scene Management

### Design

The scene manager implements a stack-based scene system. Scenes are pushed, popped, or switched:

- **Push**: Add a new scene on top of the current one (e.g., pause menu over gameplay)
- **Pop**: Remove the top scene, returning to the previous one
- **Switch**: Replace the current scene with a new one

### Scene Trait

```rust
pub trait Scene: Any {
    fn on_enter(&mut self, ctx: &mut SceneContext) {}
    fn on_exit(&mut self, ctx: &mut SceneContext) {}
    fn update(&mut self, ctx: &mut SceneContext, dt: f32);
    fn render(&mut self, ctx: &mut SceneContext, alpha: f32);
}
```

### SceneContext

A `HashMap<TypeId, Box<dyn Any>>` for sharing data between scenes (e.g., game score, settings).

### Transitions

Built-in fade transitions (`FadeIn`, `FadeOut`, `FadeInOut`) with configurable duration and color.

## Event System

### Type-Erased Event Bus

The `EventBus` provides a publish/subscribe mechanism for decoupled communication:

```rust
// Subscribe to events of type T
bus.subscribe::<CollisionEvent>(|event| {
    println!("Collision between {:?} and {:?}", event.entity_a, event.entity_b);
});

// Emit an event
bus.emit(&CollisionEvent { entity_a: e1, entity_b: e2 });
```

Internally, callbacks are stored in a `HashMap<TypeId, Vec<Callback>>`, allowing multiple subscribers per event type.

## Time System

### Fixed Timestep

The `Time` struct manages frame timing with a fixed timestep accumulator:

- `delta()`: Frame delta time (seconds)
- `elapsed()`: Total elapsed time since start
- `fps()`: Current FPS (updated every second)
- `fixed_timestep()`: Fixed physics step (default: 1/60s)
- `time_scale()`: Speed multiplier (default: 1.0)
- `alpha()`: Interpolation factor for rendering between physics steps

### Timer Utility

A simple timer for scheduling events:

```rust
let mut timer = Timer::once(2.0); // 2 second one-shot
let mut timer = Timer::repeating(0.5); // Repeating every 0.5s

if timer.update(dt) {
    // Timer fired
}
```

## Asset System

### Async Loading

Assets are loaded asynchronously using `wasm-bindgen-futures`:

- `fetch(url)`: HTTP GET returning `Vec<u8>`
- `load_image(url)`: Load an `HtmlImageElement`

### Caching

The `AssetCache` prevents redundant loading of the same asset.

## Build System

### Workspace Structure

The Cargo workspace contains four crates:

| Crate | Type | Description |
|---|---|---|
| `opengame-engine` | Library (`rlib`) | Core engine |
| `opengame-cli` | Binary (`og`) | CLI tool |
| `examples/pong` | Library | Pong example |
| `examples/platformer` | Library | Platformer example |

### WASM Build Pipeline

```
Rust Source
    │
    ├─ cargo build --target wasm32-unknown-unknown
    │
    ▼
.wasm file
    │
    ├─ wasm-bindgen (generates JS glue)
    │
    ▼
.js + .wasm output
    │
    ├─ wasm-opt (release only: size optimization)
    │
    ▼
dist/ (deployable static files)
```

### Trunk

[Trunk](https://trunkrs.dev/) orchestrates the full build:

1. Reads `index.html` for `<link data-trunk ...>` directives
2. Compiles Rust to WASM
3. Runs wasm-bindgen
4. Processes CSS
5. Copies static assets to `dist/`
6. In serve mode: watches for changes and hot-reloads

### Release Optimizations

Configured in the workspace `Cargo.toml`:

```toml
[profile.release]
opt-level = "z"       # Optimize for size
lto = true            # Link-time optimization
codegen-units = 1     # Single codegen unit (better optimization)
strip = true          # Strip debug symbols
```

Combined with `data-wasm-opt="z"` in `index.html` for wasm-opt pass.

## Configuration Files

| File | Purpose |
|---|---|
| `Cargo.toml` | Workspace definition and shared dependencies |
| `Trunk.toml` | Trunk bundler configuration |
| `rustfmt.toml` | Code formatting rules |
| `clippy.toml` | Clippy linting configuration |
| `deny.toml` | Dependency auditing rules |
| `.cargo/config.toml` | Default build target (`wasm32-unknown-unknown`) |
| `.editorconfig` | Editor settings |
| `.gitignore` | Git ignore rules |
| `.github/workflows/ci.yml` | CI pipeline definition |
