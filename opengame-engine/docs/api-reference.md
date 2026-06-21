# API Reference

Complete reference documentation for all public types, traits, and functions in the OpenGame Engine.

## Table of Contents

- [Prelude](#prelude)
- [App](#app)
- [ECS](#ecs)
  - [Entity](#entity)
  - [World](#world)
  - [EntityBuilder](#entitybuilder)
  - [System](#system)
  - [SystemScheduler](#systemscheduler)
  - [Queries](#queries)
- [Rendering](#rendering)
  - [GlBackend](#glbackend)
  - [Camera2D](#camera2d)
  - [SpriteRenderer](#spriterenderer)
  - [ShapeRenderer](#shaperenderer)
  - [TextureManager](#texturemanager)
  - [Shader](#shader)
  - [BitmapFont](#bitmapfont)
- [Components](#components)
  - [Transform2D](#transform2d)
  - [Sprite](#sprite)
  - [SpriteSheet](#spritesheet)
  - [RigidBody](#rigidbody)
  - [Collider](#collider)
- [Physics](#physics)
  - [PhysicsSystem](#physicssystem)
  - [Solver](#solver)
  - [CollisionInfo](#collisioninfo)
- [Input](#input)
  - [InputManager](#inputmanager)
  - [KeyCode](#keycode)
  - [MouseButton](#mousebutton)
- [Audio](#audio)
  - [AudioEngine](#audioengine)
  - [AudioSource](#audiosource)
  - [AudioHandle](#audiohandle)
- [Scene](#scene)
  - [Scene (trait)](#scene-trait)
  - [SceneManager](#scenemanager)
  - [SceneContext](#scenecontext)
  - [Transition](#transition)
- [Time](#time)
  - [Time](#time-1)
  - [Timer](#timer)
- [Math](#math)
- [Color](#color)
- [Events](#events)
  - [EventBus](#eventbus)
- [Profiler](#profiler)
- [Debug](#debug)
  - [DebugOverlay](#debugoverlay)
- [Logging](#logging)
- [Asset](#asset)
  - [AssetCache](#assetcache)

---

## Prelude

The `opengame_engine::prelude` module re-exports the most commonly used types:

```rust
use opengame_engine::prelude::*;
```

Includes: `Vec2`, `Vec3`, `Vec4`, `Mat3`, `Mat4`, `Time`, `Timer`, `Color`, `Transform2D`, `Sprite`, `Entity`, `World`, `System`, `SystemScheduler`, `EntityBuilder`, `QuerySingle`, `QuerySingleMut`, `QueryDouble`, `QueryDoubleMut`, `Camera2D`, `RigidBody`, `Collider`, `PhysicsSystem`, `Scene`, `SceneContext`, `SceneManager`, `Profiler`, `ScopeGuard`, `ProfileReport`, `DebugOverlay`, `DebugPosition`, and WASM-only: `TextureHandle`, `TextureManager`, `SpriteRenderer`, `ShapeRenderer`, `BitmapFont`, `InputManager`, `KeyCode`, `MouseButton`, `AudioEngine`, `AudioSource`, `AudioHandle`, `App`, `Renderer`, `Commands`.

---

## App

The main application runner. Owns all subsystems and drives the game loop.

**Module**: `opengame_engine::app` (WASM only)

### `App::new(canvas_id: &str) -> Result<Self, String>`

Create a new application instance. Initializes all subsystems.

**Parameters**:
- `canvas_id`: The HTML `id` attribute of the `<canvas>` element

**Returns**: `Result<App, String>`

### `App::set_clear_color(&mut self, color: Color)`

Set the background clear color for each frame.

### `App::add_startup_system(&mut self, system: impl FnMut(&mut World) + 'static)`

Register a startup system that runs once at initialization.

### `App::add_system(&mut self, system: impl System + 'static)`

Register an update system that runs every fixed timestep.

### `App::add_render_system(&mut self, system: impl FnMut(&mut World, f32) + 'static)`

Register a render system that runs every frame.

### `App::run(mut self)`

Start the game loop. This consumes the `App` and never returns.

### Fields

| Field | Type | Description |
|---|---|---|
| `world` | `World` | The ECS world |
| `renderer` | `Renderer` | Rendering subsystem |
| `input` | `InputManager` | Input subsystem |
| `audio` | `AudioEngine` | Audio subsystem |
| `time` | `Time` | Time management |
| `events` | `EventBus` | Event bus |
| `scene_manager` | `SceneManager` | Scene management |

### Renderer

| Field | Type | Description |
|---|---|---|
| `gl` | `GlBackend` | WebGL backend |
| `camera` | `Camera2D` | Main camera |
| `sprite_renderer` | `SpriteRenderer` | Sprite batch renderer |
| `shape_renderer` | `ShapeRenderer` | Shape renderer |
| `texture_manager` | `TextureManager` | Texture management |

### Commands

Helper for world mutations within systems.

```rust
pub struct Commands<'a> { /* ... */ }

impl<'a> Commands<'a> {
    pub fn spawn(&mut self) -> EntityBuilder<'_>;
    pub fn despawn(&mut self, entity: Entity) -> bool;
    pub fn insert_resource<T: 'static>(&mut self, resource: T);
}
```

---

## ECS

### Entity

A lightweight handle to an entity in the world.

**Module**: `opengame_engine::ecs::entity`

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity { /* generational_arena::Index */ }

impl Entity {
    pub fn id(&self) -> u64;
}
```

### World

The central container for entities, components, and resources.

**Module**: `opengame_engine::ecs::world`

#### Entity Management

| Method | Signature | Description |
|---|---|---|
| `new()` | `-> Self` | Create an empty world |
| `spawn_empty()` | `-> Entity` | Spawn an entity without components |
| `spawn()` | `-> EntityBuilder` | Spawn with builder pattern |
| `despawn()` | `(entity: Entity) -> bool` | Remove entity and all its components |
| `is_alive()` | `(entity: Entity) -> bool` | Check if entity exists |
| `entity_count()` | `-> usize` | Number of alive entities |
| `entities()` | `-> Vec<Entity>` | List all entities |

#### Component Management

| Method | Signature | Description |
|---|---|---|
| `insert_component<T>()` | `(entity: Entity, component: T)` | Add component to entity |
| `get_component<T>()` | `(entity: Entity) -> Option<&T>` | Get immutable component reference |
| `get_component_mut<T>()` | `(entity: Entity) -> Option<&mut T>` | Get mutable component reference |
| `remove_component<T>()` | `(entity: Entity) -> Option<T>` | Remove and return component |
| `has_component<T>()` | `(entity: Entity) -> bool` | Check if entity has component |

#### Resource Management

| Method | Signature | Description |
|---|---|---|
| `insert_resource<T>()` | `(resource: T)` | Store a unique resource |
| `get_resource<T>()` | `-> Option<&T>` | Get immutable resource reference |
| `get_resource_mut<T>()` | `-> Option<&mut T>` | Get mutable resource reference |
| `remove_resource<T>()` | `-> Option<T>` | Remove and return resource |
| `has_resource<T>()` | `-> bool` | Check if resource exists |

#### Utility

| Method | Signature | Description |
|---|---|---|
| `clear()` | `()` | Remove all entities, components, and resources |

### EntityBuilder

Fluent builder for creating entities with components.

**Module**: `opengame_engine::ecs::builder`

```rust
let entity = world.spawn()
    .with(Transform2D::new(Vec2::ZERO))
    .with(Sprite::new())
    .with(RigidBody::dynamic())
    .build();
```

| Method | Signature | Description |
|---|---|---|
| `with<T>()` | `(component: T) -> Self` | Add a component |
| `entity()` | `-> Entity` | Get the entity handle |
| `build()` | `-> Entity` | Finalize and return the entity |

### System

Trait for update systems.

**Module**: `opengame_engine::ecs::system`

```rust
pub trait System {
    fn update(&mut self, world: &mut World, dt: f32);
    fn name(&self) -> &str { /* default: type_name */ }
}
```

Closures `FnMut(&mut World, f32)` automatically implement `System`.

### SystemScheduler

Manages system registration and execution.

**Module**: `opengame_engine::ecs::system`

| Method | Signature | Description |
|---|---|---|
| `new()` | `-> Self` | Create empty scheduler |
| `add_startup_system()` | `(system: impl FnMut(&mut World) + 'static)` | Register startup system |
| `add_system()` | `(system: impl System + 'static)` | Register update system |
| `add_render_system()` | `(system: impl FnMut(&mut World, f32) + 'static)` | Register render system |
| `run_startup()` | `(&mut self, world: &mut World)` | Execute startup systems (once) |
| `run_update()` | `(&mut self, world: &mut World, dt: f32)` | Execute update systems |
| `run_render()` | `(&mut self, world: &mut World, alpha: f32)` | Execute render systems |

### Queries

#### `QuerySingle<T>`

Read-only query for all entities with component `T`.

```rust
let query = QuerySingle::<Transform2D>::new(&world).unwrap();
for (entity, transform) in query.iter() {
    println!("{:?} at {:?}", entity, transform.position);
}
```

| Method | Description |
|---|---|
| `new(world) -> Option<Self>` | Create query from world |
| `get(entity) -> Option<&T>` | Get component for entity |
| `iter() -> impl Iterator<Item = (Entity, &T)>` | Iterate all matches |
| `len() -> usize` | Count of matching entities |
| `is_empty() -> bool` | True if no matches |

#### `QuerySingleMut<T>`

Mutable version of `QuerySingle`. Also provides `iter_mut()`.

#### `QueryDouble<A, B>`

Read-only query for entities with both `A` and `B`.

```rust
let query = QueryDouble::<Transform2D, Sprite>::new(&world).unwrap();
for (entity, transform, sprite) in query.iter() {
    // ...
}
```

#### `QueryDoubleMut<A, B>`

Mutable access to both components. **Note**: `A` and `B` must be different types.

---

## Rendering

### GlBackend

Low-level WebGL 2.0 context wrapper. (WASM only)

**Module**: `opengame_engine::renderer::gl_backend`

| Method | Description |
|---|---|
| `new(canvas_id) -> Result<Self, String>` | Initialize WebGL context |
| `gl() -> &glow::Context` | Get the GL context |
| `width() -> u32` | Canvas width in pixels |
| `height() -> u32` | Canvas height in pixels |
| `resize()` | Resize canvas to match CSS layout |
| `clear(r, g, b, a)` | Clear the framebuffer |
| `enable_blend()` | Enable alpha blending |

### Camera2D

2D camera with position, zoom, rotation, and viewport management.

**Module**: `opengame_engine::renderer::camera`

| Field | Type | Default | Description |
|---|---|---|---|
| `position` | `Vec2` | `ZERO` | Camera center in world space |
| `zoom` | `f32` | `1.0` | Zoom factor |
| `rotation` | `f32` | `0.0` | Rotation in radians |

| Method | Description |
|---|---|
| `new(width, height) -> Self` | Create camera with viewport size |
| `set_viewport(width, height)` | Update viewport dimensions |
| `projection() -> Mat4` | Get projection matrix |
| `view() -> Mat4` | Get view matrix |
| `view_projection() -> Mat4` | Get combined VP matrix |
| `screen_to_world(screen_pos) -> Vec2` | Convert screen to world coords |
| `world_to_screen(world_pos) -> Vec2` | Convert world to screen coords |

### SpriteRenderer

Batched sprite renderer. (WASM only)

**Module**: `opengame_engine::renderer::sprite`

| Method | Description |
|---|---|
| `new(gl) -> Result<Self, String>` | Create renderer |
| `begin()` | Start a new batch |
| `draw(texture, position, size, color, ...)` | Add a sprite to the batch |
| `flush(gl, view_projection)` | Submit batch to GPU |

### ShapeRenderer

Renders filled shapes. (WASM only)

**Module**: `opengame_engine::renderer::shape`

| Method | Description |
|---|---|
| `new(gl) -> Result<Self, String>` | Create renderer |
| `begin()` | Start a new batch |
| `set_color(color)` | Set current draw color |
| `draw_rect(x, y, width, height)` | Add a filled rectangle |
| `flush(gl, view_projection)` | Submit batch to GPU |

### TextureManager

Manages texture loading and storage. (WASM only)

**Module**: `opengame_engine::renderer::texture`

| Method | Description |
|---|---|
| `new() -> Self` | Create manager |
| `init(gl)` | Initialize default textures |
| `from_image(gl, image) -> TextureHandle` | Create texture from image element |
| `get(handle) -> Option<&Texture>` | Get texture by handle |
| `bind(gl, handle)` | Bind texture for rendering |

### Shader

Shader program wrapper.

**Module**: `opengame_engine::renderer::shader`

| Method | Description |
|---|---|
| `new(gl, vertex_src, fragment_src) -> Result<Self, String>` | Compile and link |
| `bind(gl)` | Activate shader |
| `unbind(gl)` | Deactivate shader |
| `set_uniform_1f(gl, name, value)` | Set float uniform |
| `set_uniform_2f(gl, name, x, y)` | Set vec2 uniform |
| `set_uniform_4f(gl, name, x, y, z, w)` | Set vec4 uniform |
| `set_uniform_mat4(gl, name, matrix)` | Set mat4 uniform |
| `set_uniform_1i(gl, name, value)` | Set int uniform |

---

## Components

### Transform2D

2D transformation component with position, rotation, scale, and matrix caching.

**Module**: `opengame_engine::transform`

| Field | Type | Default |
|---|---|---|
| `position` | `Vec2` | `ZERO` |
| `rotation` | `f32` | `0.0` |
| `scale` | `Vec2` | `ONE` |

| Method | Description |
|---|---|
| `new(position) -> Self` | Create at position |
| `with_rotation(angle) -> Self` | Builder: set rotation |
| `with_scale(scale) -> Self` | Builder: set scale |
| `with_uniform_scale(s) -> Self` | Builder: uniform scale |
| `set_position(pos)` | Set position (marks dirty) |
| `translate(offset)` | Move by offset |
| `set_rotation(angle)` | Set rotation |
| `rotate(angle)` | Rotate by angle |
| `set_scale(scale)` | Set scale |
| `is_dirty() -> bool` | Matrix needs update |
| `update_matrix()` | Recompute matrices if dirty |
| `local_matrix() -> Mat3` | Get local transform matrix |
| `world_matrix() -> Mat3` | Get world transform matrix |
| `forward() -> Vec2` | Forward direction vector |
| `right() -> Vec2` | Right direction vector |

### Sprite

Rendering component for textured or colored quads.

**Module**: `opengame_engine::sprite_component`

| Field | Type | Default |
|---|---|---|
| `texture_id` | `Option<u32>` | `None` |
| `color` | `Color` | `WHITE` |
| `flip_x` | `bool` | `false` |
| `flip_y` | `bool` | `false` |
| `visible` | `bool` | `true` |
| `layer` | `i32` | `0` |
| `size` | `Option<Vec2>` | `None` |
| `anchor` | `Vec2` | `(0.5, 0.5)` |

Builder methods: `with_texture()`, `with_color()`, `with_size()`, `with_layer()`, `with_anchor()`.

### SpriteSheet

Sprite sheet definition for frame-based animations.

**Module**: `opengame_engine::sprite_component`

| Method | Description |
|---|---|
| `new(texture_id, frame_w, frame_h, cols, rows) -> Self` | Create sprite sheet |
| `frame_uv(index) -> (Vec2, Vec2)` | Get UV coordinates for frame |
| `total_frames() -> u32` | Total number of frames |

### RigidBody

Physics rigid body component.

**Module**: `opengame_engine::physics::rigid_body`

| Field | Type | Default (Dynamic) |
|---|---|---|
| `body_type` | `BodyType` | `Dynamic` |
| `velocity` | `Vec2` | `ZERO` |
| `acceleration` | `Vec2` | `ZERO` |
| `mass` | `f32` | `1.0` |
| `gravity_scale` | `f32` | `1.0` |
| `angular_velocity` | `f32` | `0.0` |
| `angular_damping` | `f32` | `0.0` |
| `linear_damping` | `f32` | `0.0` |

#### Constructors

| Method | Body Type | Mass |
|---|---|---|
| `RigidBody::dynamic()` | Dynamic | 1.0 |
| `RigidBody::kinematic()` | Kinematic | Infinity |
| `RigidBody::static_body()` | Static | Infinity |

#### Builder Methods

`with_mass(f32)`, `with_gravity_scale(f32)`, `with_velocity(Vec2)`

#### Instance Methods

| Method | Description |
|---|---|
| `apply_force(force: Vec2)` | Accumulate force (cleared each step) |
| `apply_impulse(impulse: Vec2)` | Instant velocity change |
| `is_dynamic() -> bool` | Check body type |
| `is_static() -> bool` | Check body type |
| `is_kinematic() -> bool` | Check body type |
| `inv_mass() -> f32` | Inverse mass (0 for static/infinite) |

### Collider

Physics collider component.

**Module**: `opengame_engine::physics::collider`

| Field | Type | Default |
|---|---|---|
| `shape` | `ColliderShape` | - |
| `offset` | `Vec2` | `ZERO` |
| `is_trigger` | `bool` | `false` |
| `friction` | `f32` | `0.0` |
| `restitution` | `f32` | `1.0` |
| `layer` | `u32` | `1` |
| `mask` | `u32` | `u32::MAX` |

#### Constructors

| Method | Description |
|---|---|
| `Collider::rectangle(width, height)` | AABB collider |
| `Collider::circle(radius)` | Circle collider |

#### Builder Methods

`with_offset(Vec2)`, `with_trigger(bool)`, `with_restitution(f32)`, `with_layer(u32)`, `with_mask(u32)`

#### ColliderShape

```rust
pub enum ColliderShape {
    Rectangle { width: f32, height: f32 },
    Circle { radius: f32 },
}
```

---

## Physics

### PhysicsSystem

Orchestrates the full physics simulation. Implements `System`.

**Module**: `opengame_engine::physics`

| Method | Description |
|---|---|
| `new(gravity: Vec2) -> Self` | Create with gravity vector |
| `with_gravity(gravity) -> Self` | Builder: set gravity |
| `gravity() -> Vec2` | Get current gravity |
| `set_gravity(gravity)` | Set gravity |
| `collisions: Vec<CollisionInfo>` | Collision results from last step |

```rust
// Usage
app.add_system(PhysicsSystem::new(Vec2::new(0.0, -980.0)));
```

### Solver

Impulse-based collision resolver.

**Module**: `opengame_engine::physics::solver`

| Method | Description |
|---|---|
| `new(gravity) -> Self` | Create solver |
| `apply_gravity(rb)` | Apply gravity to rigid body |
| `integrate(rb, pos, dt)` | Euler integration |
| `resolve_collision(info, rb_a, pos_a, rb_b, pos_b)` | Resolve collision |

---

## Input

### InputManager

Unified input manager. (WASM only)

**Module**: `opengame_engine::input`

| Method | Description |
|---|---|
| `new() -> Result<Self, String>` | Create and attach event listeners |
| `update()` | Sync event state to queryable state |
| `mouse_position() -> Vec2` | Current mouse position |
| `is_key_down(key) -> bool` | Key is held |
| `is_key_pressed(key) -> bool` | Key was just pressed |
| `is_key_released(key) -> bool` | Key was just released |
| `is_mouse_down(button) -> bool` | Mouse button held |
| `is_mouse_pressed(button) -> bool` | Mouse button just pressed |
| `mouse_wheel() -> f32` | Scroll wheel delta |

#### Sub-modules

- `input.keyboard`: `KeyboardState`
- `input.mouse`: `MouseState`
- `input.touch`: `TouchState`
- `input.gamepad`: `GamepadManager`

### KeyCode

Enum mapping keyboard keys. Created from JavaScript `KeyboardEvent.code`.

Common variants: `KeyA`-`KeyZ`, `Digit0`-`Digit9`, `ArrowUp/Down/Left/Right`, `Space`, `Enter`, `Escape`, `ShiftLeft/Right`, `Tab`, `Backspace`, `F1`-`F12`.

### MouseButton

```rust
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Other(u16),
}
```

---

## Audio

### AudioEngine

Web Audio API manager. (WASM only)

**Module**: `opengame_engine::audio::engine`

| Method | Description |
|---|---|
| `new() -> Result<Self, String>` | Create engine with AudioContext |
| `add_buffer(buffer) -> u32` | Register audio buffer, returns ID |
| `play(buffer_id, volume, looping) -> Option<AudioHandle>` | Play as SFX |
| `play_music(buffer_id, volume) -> Option<AudioHandle>` | Play as music (auto-loop) |
| `set_master_volume(volume)` | Set master volume |
| `set_music_volume(volume)` | Set music channel volume |
| `set_sfx_volume(volume)` | Set SFX channel volume |
| `resume()` | Resume audio context |
| `suspend()` | Suspend audio context |
| `is_resumed() -> bool` | Check if context is running |

### AudioSource

Audio source configuration.

**Module**: `opengame_engine::audio::source`

```rust
pub struct AudioSource {
    pub buffer_id: u32,
    pub volume: f32,
    pub looping: bool,
    pub category: AudioCategory,
}
```

Constructors: `AudioSource::new(buffer_id)`, `AudioSource::music(buffer_id)`

Builder: `with_volume(f32)`, `with_looping(bool)`

### AudioHandle

Opaque handle to a playing audio instance.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AudioHandle(pub u32);
```

### AudioCategory

```rust
pub enum AudioCategory {
    Music,
    Sfx,
}
```

---

## Scene

### Scene (trait)

Interface for game scenes.

**Module**: `opengame_engine::scene::manager`

```rust
pub trait Scene: Any {
    fn on_enter(&mut self, _ctx: &mut SceneContext) {}
    fn on_exit(&mut self, _ctx: &mut SceneContext) {}
    fn update(&mut self, ctx: &mut SceneContext, dt: f32);
    fn render(&mut self, ctx: &mut SceneContext, alpha: f32);
}
```

### SceneManager

Stack-based scene manager.

**Module**: `opengame_engine::scene::manager`

| Method | Description |
|---|---|
| `new() -> Self` | Create empty manager |
| `push(scene)` | Push scene onto stack |
| `pop()` | Remove top scene |
| `switch(scene)` | Replace top scene |
| `current() -> Option<&dyn Scene>` | Get current scene (immutable) |
| `current_mut() -> Option<&mut dyn Scene>` | Get current scene (mutable) |
| `depth() -> usize` | Scene stack depth |
| `is_empty() -> bool` | True if no scenes |
| `update(ctx, dt)` | Update current scene |
| `render(ctx, alpha)` | Render current scene |

### SceneContext

Type-erased shared state for scenes.

| Method | Description |
|---|---|
| `new() -> Self` | Create empty context |
| `set<T>(value)` | Store a value |
| `get<T>() -> Option<&T>` | Retrieve a value |
| `get_mut<T>() -> Option<&mut T>` | Retrieve mutable |

### Transition

Scene transition effects.

**Module**: `opengame_engine::scene::transition`

| Constructor | Description |
|---|---|
| `Transition::fade_in(duration, color)` | Fade from opaque to transparent |
| `Transition::fade_out(duration, color)` | Fade from transparent to opaque |
| `Transition::fade_in_out(duration, color)` | Fade in then out |

| Method | Description |
|---|---|
| `update(dt)` | Advance transition |
| `is_finished() -> bool` | Check completion |
| `progress() -> f32` | Progress (0.0 to 1.0) |
| `alpha() -> f32` | Current overlay alpha |
| `color() -> Color` | Current overlay color |

---

## Time

### Time

Frame timing and fixed timestep management.

**Module**: `opengame_engine::time`

| Method | Description |
|---|---|
| `new(performance) -> Self` | Create with Performance API |
| `init()` | Initialize timing baseline |
| `update()` | Update frame timing |
| `delta() -> f32` | Frame delta time (seconds) |
| `elapsed() -> f32` | Total elapsed time |
| `frame_count() -> u64` | Total frames rendered |
| `fps() -> f32` | Current FPS |
| `fixed_timestep() -> f32` | Fixed step duration |
| `set_fixed_timestep(dt)` | Set fixed step duration |
| `time_scale() -> f32` | Speed multiplier |
| `set_time_scale(scale)` | Set speed multiplier |
| `accumulator() -> f32` | Current step accumulator |
| `consume_fixed_step() -> bool` | Consume one fixed step if available |
| `alpha() -> f32` | Interpolation alpha for rendering |

### Timer

Simple timer utility for scheduling events.

**Module**: `opengame_engine::time`

| Constructor | Description |
|---|---|
| `Timer::once(duration)` | One-shot timer |
| `Timer::repeating(duration)` | Repeating timer |

| Method | Description |
|---|---|
| `update(dt) -> bool` | Advance; returns `true` when fired |
| `reset()` | Reset timer |
| `progress() -> f32` | Progress (0.0 to 1.0) |
| `is_finished() -> bool` | True if one-shot has fired |
| `remaining() -> f32` | Time remaining |

---

## Math

**Module**: `opengame_engine::math`

Re-exports from `glam`: `Vec2`, `Vec3`, `Vec4`, `Mat3`, `Mat4`.

### Constants

| Name | Value |
|---|---|
| `PI` | `std::f32::consts::PI` |
| `TAU` | `std::f32::consts::TAU` |
| `DEG_TO_RAD` | `PI / 180.0` |
| `RAD_TO_DEG` | `180.0 / PI` |

### Functions

| Function | Signature | Description |
|---|---|---|
| `deg_to_rad` | `(f32) -> f32` | Degrees to radians |
| `rad_to_deg` | `(f32) -> f32` | Radians to degrees |
| `lerp` | `(a, b, t) -> f32` | Linear interpolation |
| `lerp_vec2` | `(a, b, t) -> Vec2` | Vec2 linear interpolation |
| `clamp` | `(value, min, max) -> f32` | Clamp to range |
| `smoothstep` | `(edge0, edge1, x) -> f32` | Smooth Hermite interpolation |
| `inverse_lerp` | `(a, b, value) -> f32` | Inverse linear interpolation |
| `remap` | `(value, from_min, from_max, to_min, to_max) -> f32` | Remap between ranges |
| `vec2_angle` | `(a, b) -> f32` | Angle between two points |
| `vec2_from_angle` | `(angle) -> Vec2` | Unit vector from angle |
| `ortho_matrix` | `(left, right, bottom, top) -> Mat4` | Orthographic projection |

---

## Color

**Module**: `opengame_engine::color`

### Predefined Colors

`WHITE`, `BLACK`, `RED`, `GREEN`, `BLUE`, `YELLOW`, `CYAN`, `MAGENTA`, `ORANGE`, `PURPLE`, `GRAY`, `DARK_GRAY`, `LIGHT_GRAY`, `TRANSPARENT`

### Constructor Methods

| Method | Description |
|---|---|
| `Color::new(r, g, b, a)` | From float components (0.0-1.0) |
| `Color::rgb(r, g, b)` | RGB with alpha 1.0 |
| `Color::rgba_u8(r, g, b, a)` | From u8 components (0-255) |
| `Color::hex(hex)` | From hex u32 (e.g., `0xFF0000`) |

### Instance Methods

| Method | Description |
|---|---|
| `with_alpha(a) -> Self` | Return copy with new alpha |
| `to_array() -> [f32; 4]` | Convert to array |
| `to_vec4() -> Vec4` | Convert to glam Vec4 |
| `lerp(other, t) -> Color` | Linear interpolate between colors |

---

## Events

### EventBus

Type-erased publish/subscribe event system.

**Module**: `opengame_engine::event`

| Method | Description |
|---|---|
| `new() -> Self` | Create empty bus |
| `subscribe<T>(callback)` | Register callback for event type `T` |
| `emit<T>(event)` | Broadcast event to all subscribers |

### Built-in Event Types

```rust
pub struct WindowResizeEvent {
    pub width: u32,
    pub height: u32,
}

pub struct CollisionEvent {
    pub entity_a: generational_arena::Index,
    pub entity_b: generational_arena::Index,
}
```

---

## Profiler

**Module**: `opengame_engine::profiler`

### Profiler

| Method | Description |
|---|---|
| `new(max_frame_history) -> Self` | Create profiler |
| `begin_frame(timestamp_us)` | Start frame profiling |
| `end_frame(timestamp_us)` | End frame profiling |
| `record_scope(name, duration_us)` | Record a scope measurement |
| `begin_scope(name) -> ScopeGuard` | Start scoped measurement |
| `report() -> ProfileReport` | Generate report |
| `frame_time_ms() -> f32` | Last frame time |
| `avg_frame_time_ms() -> f32` | Average frame time |
| `fps() -> f32` | Calculated FPS |
| `frame_history() -> &[f32]` | Frame time history |
| `reset()` | Clear all data |

### ScopeGuard

RAII scope timer. Call `finish(profiler)` to record.

### ProfileReport

| Field | Type |
|---|---|
| `scopes` | `Vec<ScopeEntry>` |
| `frame_time_ms` | `f32` |
| `avg_frame_time_ms` | `f32` |
| `fps` | `f32` |

Method: `format_text() -> String`

---

## Debug

### DebugOverlay

Runtime debug information overlay.

**Module**: `opengame_engine::debug`

| Field | Type | Default |
|---|---|---|
| `visible` | `bool` | `true` |
| `show_fps` | `bool` | `true` |
| `show_entity_count` | `bool` | `true` |
| `show_frame_time` | `bool` | `true` |
| `show_profiler` | `bool` | `false` |
| `text_color` | `Color` | `GREEN` |
| `bg_color` | `Color` | `(0,0,0,0.7)` |
| `padding` | `f32` | `8.0` |
| `font_size` | `f32` | `16.0` |
| `position` | `DebugPosition` | `TopLeft` |

| Method | Description |
|---|---|
| `update_stats(fps, frame_time_ms, entity_count, component_type_count)` | Update displayed stats |
| `set_profiler_lines(lines: Vec<String>)` | Set profiler output lines |
| `text_lines() -> Vec<String>` | Get current display lines |
| `toggle()` | Toggle visibility |
| `toggle_fps()` | Toggle FPS display |
| `toggle_profiler()` | Toggle profiler display |
| `bg_size(char_width, char_height) -> Vec2` | Calculate background size |
| `bg_position(screen_w, screen_h, bg_size) -> Vec2` | Calculate position |

### DebugPosition

```rust
pub enum DebugPosition {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}
```

---

## Logging

**Module**: `opengame_engine::log`

### Initialization

```rust
opengame_engine::log::init(); // Sets panic hook (call once at startup)
```

### Macros

| Macro | Description |
|---|---|
| `console_log!(...)` | Log to `console.log` |
| `console_warn!(...)` | Log to `console.warn` |
| `console_error!(...)` | Log to `console.error` |

Usage:

```rust
console_log!("Player at ({}, {})", x, y);
console_warn!("High entity count: {}", count);
console_error!("Failed to load texture: {}", path);
```

---

## Asset

### AssetCache

Caches loaded assets to prevent redundant fetches.

**Module**: `opengame_engine::asset::cache`

### Loader Functions

**Module**: `opengame_engine::asset::loader` (WASM only)

| Function | Signature | Description |
|---|---|---|
| `fetch(url)` | `async -> Result<Vec<u8>, String>` | HTTP GET, returns bytes |
| `load_image(url)` | `async -> Result<HtmlImageElement, String>` | Load image element |
