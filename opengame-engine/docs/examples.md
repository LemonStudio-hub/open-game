# Examples Guide

This document provides detailed walkthroughs of the two example games included in the OpenGame repository. Each example demonstrates different aspects of the engine and serves as a reference for building your own games.

## Running Examples

```bash
# Using the CLI tool
cargo run -p opengame-cli -- run pong
cargo run -p opengame-cli -- run platformer

# Or directly with Trunk (after modifying index.html)
trunk serve
```

---

## Pong

**Location**: `crates/examples/pong/`

A classic two-player Pong game demonstrating shape rendering, real-time input handling, and simple game state management.

### What It Demonstrates

- **ShapeRenderer** usage for drawing rectangles and UI elements
- **Keyboard input** handling for two players simultaneously
- **Simple physics** (ball movement and paddle collision)
- **Game state management** (score tracking, win condition, restart)
- **Canvas-based text rendering** for HUD display
- **Debug overlay** integration

### Architecture

The game is implemented as a single startup system that enters a requestAnimationFrame loop directly (without using the full App runner). This is a simpler approach suitable for small games.

```
Game State (App struct)
├── Paddle { x, y, width, height }
│   ├── left (W/S keys)
│   └── right (Up/Down keys)
├── Ball { x, y, vx, vy }
├── Score { left, right }
├── Subsystems
│   ├── Renderer (GlBackend, Camera2D, ShapeRenderer)
│   └── Input (InputManager)
└── Game Loop (requestAnimationFrame)
```

### Controls

| Action | Key |
|---|---|
| Left paddle up | `W` |
| Left paddle down | `S` |
| Right paddle up | `Up Arrow` |
| Right paddle down | `Down Arrow` |
| Restart after score | `Space` |

### Game Rules

- Ball bounces off top and bottom walls
- Ball bounces off paddles with velocity reflection
- Missing the ball scores a point for the opponent
- First to 10 points wins
- Press Space to restart after a goal or game over

### Key Code Patterns

#### Rendering with ShapeRenderer

```rust
// Begin batch
app.shape_renderer.begin();
app.shape_renderer.set_color(Color::WHITE);

// Draw paddles
app.shape_renderer.draw_rect(
    left.x - left.w / 2.0, left.y - left.h / 2.0,
    left.w, left.h,
);
app.shape_renderer.draw_rect(
    right.x - right.w / 2.0, right.y - right.h / 2.0,
    right.w, right.h,
);

// Draw ball
app.shape_renderer.draw_rect(
    ball.x - ball_size / 2.0, ball.y - ball_size / 2.0,
    ball_size, ball_size,
);

// Flush to GPU
app.shape_renderer.flush(&app.gl, app.camera.view_projection());
```

#### Canvas Text Rendering

Text is rendered directly to the 2D canvas context (not WebGL) using `fillText`:

```rust
fn draw_centered_text(ctx: &CanvasRenderingContext2d, text: &str, y: f64, canvas_w: f64) {
    ctx.set_font("bold 32px sans-serif");
    ctx.set_text_align("center");
    ctx.fill_text(text, canvas_w / 2.0, y).unwrap();
}
```

#### Debug Overlay

```rust
app.debug_overlay.visible = true;
app.debug_overlay.show_profiler = true;
app.debug_overlay.update_stats(
    app.time.fps(),
    1000.0 / app.time.fps(),
    0,
    0,
);
```

---

## Platformer

**Location**: `crates/examples/platformer/`

A side-scrolling 2D platformer demonstrating the full ECS architecture with physics, sprite rendering, collectibles, and platform collision.

### What It Demonstrates

- **Full ECS pipeline** with components, entities, and systems
- **RigidBody and Collider** usage with gravity and collision resolution
- **SpriteRenderer** for texture-based rendering (canvas gradient placeholders)
- **Camera following** with smooth interpolation
- **Input-driven movement** with acceleration and velocity limits
- **Collectible objects** with trigger colliders
- **Platform system** with multiple platform types
- **BitmapFont** for in-game text display

### Architecture

The platformer uses the complete `App` runner with registered systems:

```
Startup Systems:
├── init_camera()           - Configure camera zoom and position
└── init_platformer_world() - Spawn all game entities

Update Systems:
├── input_system()          - Read input, apply movement/jump
├── simple_ground_system()  - Ground collision detection
├── collectible_system()    - Collectible pickup logic
├── platform_system()       - Platform collision behavior
└── (PhysicsSystem)         - Gravity and collision resolution

Render Systems:
└── render_sprites()        - Draw all sprites with interpolation
```

### Entity Types

#### Player

```rust
Entity {
    Transform2D: position, rotation, scale
    Sprite: colored quad (red, 40x56)
    RigidBody: dynamic, gravity_scale=1.0
    Collider: rectangle(40, 56)
    Player: movement state (grounded, jump_force, move_speed, velocity)
}
```

#### Ground

```rust
Entity {
    Transform2D: center of level
    Sprite: brown platform (2000x100)
    Collider: rectangle(2000, 100)
    Platform: ground type
}
```

#### Moving Platform

```rust
Entity {
    Transform2D: initial position
    Sprite: blue platform (200x30)
    Collider: rectangle(200, 30)
    Platform: Moving { start, end, speed, t }
}
```

#### Crumbling Platform

```rust
Entity {
    Transform2D: position
    Sprite: orange platform (180x30)
    Collider: rectangle(180, 30)
    Platform: Crumbling { timer, crumbling, crumble_time }
}
```

#### Collectible (Coin)

```rust
Entity {
    Transform2D: position
    Sprite: yellow circle (20x20)
    Collider: circle(10), trigger=true
    Collectible: coin type, collected=false
}
```

### Controls

| Action | Key |
|---|---|
| Move left | `A` or `Left Arrow` |
| Move right | `D` or `Right Arrow` |
| Jump | `Space`, `W`, or `Up Arrow` |

### Game Systems

#### Input System

Reads keyboard state and applies forces to the player:

```rust
fn input_system(world: &mut World, dt: f32) {
    let input = world.get_resource::<InputManager>().unwrap();
    let mut query = QueryDoubleMut::<Player, RigidBody>::new(world).unwrap();
    for (_entity, player, rb) in query.iter_mut() {
        if input.is_key_down(KeyCode::ArrowRight) || input.is_key_down(KeyCode::KeyD) {
            rb.velocity.x += player.move_speed * dt;
        }
        if input.is_key_down(KeyCode::ArrowLeft) || input.is_key_down(KeyCode::KeyA) {
            rb.velocity.x -= player.move_speed * dt;
        }
        rb.velocity.x = rb.velocity.x.clamp(-player.max_velocity, player.max_velocity);
        if (input.is_key_pressed(KeyCode::Space) || ...) && player.grounded {
            rb.velocity.y = player.jump_force;
            player.grounded = false;
        }
    }
}
```

#### Simple Ground System

Prevents the player from falling through the ground plane:

```rust
fn simple_ground_system(world: &mut World, _dt: f32) {
    let ground_y = -50.0;
    let mut query = QueryDoubleMut::<Transform2D, Player>::new(world).unwrap();
    for (_entity, transform, player) in query.iter_mut() {
        if transform.position.y <= ground_y {
            transform.position.y = ground_y;
            player.grounded = true;
        }
    }
}
```

#### Collectible System

Detects trigger collisions and removes collected items:

```rust
fn collectible_system(world: &mut World, _dt: f32) {
    // Uses distance-based collision detection
    // When player overlaps a collectible's collider:
    // 1. Mark collectible as collected
    // 2. Despawn the entity
    // 3. Update score (future: emit event)
}
```

#### Camera Follow System

Smoothly follows the player with interpolation:

```rust
fn camera_follow_system(world: &mut World, renderer: &mut Renderer) {
    if let Some(player_entity) = find_player(world) {
        if let Some(transform) = world.get_component::<Transform2D>(player_entity) {
            let current = renderer.camera.position;
            let target = transform.position;
            renderer.camera.position = current.lerp(target, 0.1); // Smooth follow
        }
    }
}
```

### Sprite Rendering

Sprites are rendered using the SpriteRenderer with interpolation for smooth movement:

```rust
fn render_sprites(world: &mut World, alpha: f32) {
    // For each entity with Transform2D + Sprite:
    //   Interpolate position using alpha
    //   Draw colored rectangle at interpolated position
    //   (Future: draw texture if sprite has texture_id)
}
```

### Level Design

The platformer includes a predefined level layout:

```
                    [C] [C] [C]
                 [M]
    [S]                    [C]
         [C]    [M]
[G] [G] [G] [G] [G] [G] [G] [G] [G] [G]

Legend:
[G] = Ground platform
[M] = Moving platform
[S] = Static platform
[C] = Collectible (coin)
```

### Extending the Platformer

To add new features:

1. **New component**: Define a struct and add it to the ECS
2. **New system**: Implement the `System` trait or use a closure
3. **Register system**: Call `app.add_system(my_system)` in the startup
4. **Spawn entities**: Use `world.spawn().with(Component).build()`

Example: Adding an enemy

```rust
struct Enemy {
    patrol_speed: f32,
    patrol_range: f32,
}

fn enemy_system(world: &mut World, dt: f32) {
    let mut query = QueryDoubleMut::<Transform2D, Enemy>::new(world).unwrap();
    for (_entity, transform, enemy) in query.iter_mut() {
        // Patrol logic
        transform.translate(Vec2::new(enemy.patrol_speed * dt, 0.0));
    }
}
```

---

## Creating Your Own Game

### Step 1: Scaffold

```bash
og new my-game --template ecs
cd my-game
```

### Step 2: Define Components

```rust
#[derive(Debug)]
struct Health(i32);

#[derive(Debug)]
struct Velocity(Vec2);
```

### Step 3: Define Systems

```rust
fn movement_system(world: &mut World, dt: f32) {
    let mut query = QueryDoubleMut::<Transform2D, Velocity>::new(world).unwrap();
    for (_entity, transform, vel) in query.iter_mut() {
        transform.translate(vel.0 * dt);
    }
}
```

### Step 4: Register in `lib.rs`

```rust
#[wasm_bindgen(start)]
pub fn start() {
    let mut app = App::new("canvas").unwrap();
    app.add_system(movement_system);
    app.run();
}
```

### Step 5: Build and Run

```bash
og serve
```
