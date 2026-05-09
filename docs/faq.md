# Frequently Asked Questions

## Setup & Installation

### Q: `cargo build` fails with "can't find crate for `core`"

**A**: You are missing the WebAssembly target. Add it:

```bash
rustup target add wasm32-unknown-unknown
```

The project's `.cargo/config.toml` sets `wasm32-unknown-unknown` as the default target, so all `cargo build` commands target WASM automatically.

### Q: `trunk serve` fails with "trunk: command not found"

**A**: Install Trunk:

```bash
cargo install trunk
```

Make sure `~/.cargo/bin` is in your `PATH`.

### Q: `wasm-opt` not found warning during release builds

**A**: Install `wasm-opt` via `wasm-bindgen-cli`:

```bash
cargo install wasm-bindgen-cli
```

Or install `binaryen` from your system package manager. This is optional for development but recommended for release builds to minimize WASM size.

### Q: Browser shows blank canvas / "Unreachable" error

**A**: Common causes:

1. **Canvas ID mismatch**: Ensure `index.html` has `<canvas id="canvas">` and your code uses `App::new("canvas")`.
2. **Panic in startup**: Check the browser console for Rust panic messages. The engine sets up a panic hook that logs to `console.error`.
3. **Missing `start()` function**: Ensure you have a `#[wasm_bindgen(start)]` entry point in your `lib.rs`.

### Q: `cargo test` fails with WASM target errors

**A**: Tests run on the host target, not WASM. If your tests use WASM-specific APIs:

```bash
# Run tests on host target (override default)
cargo test --workspace --target x86_64-unknown-linux-gnu
```

The Makefile `make test` command handles this automatically.

### Q: `make install-tools` fails

**A**: Ensure you have:
- Rust installed via rustup (not system package manager)
- Network connectivity for downloading packages
- Sufficient disk space (~500 MB for all tools)

## Development

### Q: How do I add a new component?

**A**: Define a struct and add it to an entity:

```rust
#[derive(Debug)]
struct Health {
    current: f32,
    max: f32,
}

// Add to entity
let entity = world.spawn()
    .with(Transform2D::new(Vec2::ZERO))
    .with(Health { current: 100.0, max: 100.0 })
    .build();
```

Components must implement `'static` (no references). Use `Rc<RefCell<T>>` for shared state.

### Q: How do I query entities with specific components?

**A**: Use the typed query types:

```rust
// Single component (read-only)
let query = QuerySingle::<Health>::new(&world).unwrap();
for (entity, health) in query.iter() {
    println!("{:?}: {}/{}", entity, health.current, health.max);
}

// Two components (read-only)
let query = QueryDouble::<Transform2D, Health>::new(&world).unwrap();
for (entity, transform, health) in query.iter() {
    // ...
}

// Mutable access
let mut query = QueryDoubleMut::<Transform2D, Health>::new(world).unwrap();
for (_entity, transform, health) in query.iter_mut() {
    health.current -= 1.0;
}
```

### Q: How do I handle collisions?

**A**: Add `RigidBody` and `Collider` components to entities, then add `PhysicsSystem` to your app:

```rust
// Entity with physics
world.spawn()
    .with(Transform2D::new(Vec2::new(100.0, 200.0)))
    .with(RigidBody::dynamic())
    .with(Collider::rectangle(32.0, 32.0))
    .build();

// Add physics system
app.add_system(PhysicsSystem::new(Vec2::new(0.0, -980.0)));
```

For trigger collisions (no physical response), use `Collider::rectangle(...).with_trigger(true)`.

### Q: How do I play audio?

**A**: Use the `AudioEngine`:

```rust
// In startup, load audio buffer
let audio_data: Vec<u8> = load_audio_file().await;
let buffer = audio_engine.decode_audio_data(&audio_data).await;
let buffer_id = audio_engine.add_buffer(buffer);

// Play sound effect
audio_engine.play(buffer_id, 0.8, false);

// Play music (loops automatically)
audio_engine.play_music(buffer_id, 0.5);

// Control volumes
audio_engine.set_music_volume(0.3);
audio_engine.set_sfx_volume(0.8);
```

### Q: How do I load textures?

**A**: Use the `TextureManager`:

```rust
// Load image
let image = asset::loader::load_image("assets/player.png").await.unwrap();
let handle = texture_manager.from_image(&gl, &image);

// Use in sprite
let sprite = Sprite::new().with_texture(handle.id());
```

### Q: How do I switch scenes?

**A**: Use the `SceneManager`:

```rust
struct MenuScene;
struct GameScene;

impl Scene for MenuScene {
    fn update(&mut self, ctx: &mut SceneContext, dt: f32) {
        if start_button_clicked {
            ctx.set(GameState::new());
            // Switch happens via SceneManager
        }
    }
    fn render(&mut self, ctx: &mut SceneContext, alpha: f32) { /* ... */ }
}

// Push a scene
scene_manager.push(Box::new(GameScene));

// Pop back to previous
scene_manager.pop();

// Replace current
scene_manager.switch(Box::new(MenuScene));
```

### Q: How do I use the profiler?

**A**: Wrap code in scope guards:

```rust
fn my_system(world: &mut World, dt: f32) {
    let mut profiler = world.get_resource_mut::<Profiler>().unwrap();
    let guard = profiler.begin_scope("my_system");
    
    // ... system logic ...
    
    guard.finish(&mut profiler);
}
```

View results via the `DebugOverlay`:

```rust
debug_overlay.show_profiler = true;
```

## Performance

### Q: WASM bundle is too large

**A**: Optimization strategies:

1. **Use release mode**: `trunk build --release` or `og build --release`
2. **Verify LTO**: Check that `Cargo.toml` has `lto = true` in `[profile.release]`
3. **Strip symbols**: Ensure `strip = true` in release profile
4. **Optimize for size**: `opt-level = "z"` (already configured)
5. **Use wasm-opt**: The `data-wasm-opt="z"` attribute in `index.html` triggers wasm-opt
6. **Minimize dependencies**: Review `cargo tree` for unnecessary crates
7. **Check bundle size**: `make size` or `du -sh dist/`

### Q: Game runs slowly in the browser

**A**: Common performance issues:

1. **Too many draw calls**: Batch sprites using `SpriteRenderer::begin()`/`flush()`
2. **Unnecessary allocations**: Avoid creating `Vec` or `String` every frame
3. **Large physics step**: Reduce entity count or increase fixed timestep
4. **Console logging**: Remove `console_log!` calls in hot loops
5. **Chrome DevTools**: Use the Performance tab to identify bottlenecks

### Q: How to reduce allocations in the game loop?

**A**: Pre-allocate buffers and reuse them:

```rust
struct GameState {
    render_buffer: Vec<RenderCommand>,  // Reuse across frames
}

fn render_system(world: &mut World, alpha: f32) {
    let state = world.get_resource_mut::<GameState>().unwrap();
    state.render_buffer.clear();  // Reuse, don't reallocate
    // ... fill buffer ...
}
```

## Browser Compatibility

### Q: Which browsers are supported?

**A**: OpenGame targets modern browsers with WebGL 2.0 support:

| Browser | Minimum Version |
|---|---|
| Chrome | 56+ |
| Firefox | 51+ |
| Safari | 15+ |
| Edge | 79+ |

### Q: Safari shows WebGL errors

**A**: Safari has stricter WebGL requirements. Ensure:
- Canvas has explicit `width` and `height` attributes
- No WebGL calls before the page is fully loaded
- Audio requires user interaction before playback (Safari policy)

### Q: Mobile browser support?

**A**: Touch input is supported. Performance depends on device capability. The engine includes:
- `TouchState` for multi-touch tracking
- Responsive canvas sizing
- Touch-friendly controls

For mobile games, consider:
- Larger touch targets
- Simplified physics
- Reduced entity count

## Build & Deploy

### Q: How do I deploy to production?

**A**: Build a release bundle and deploy to any static hosting:

```bash
# Build optimized release
og build --release

# Output is in dist/
ls dist/
# index.html  my_game_bg.wasm  my_game.js  style.css
```

Deploy the `dist/` directory to:
- **GitHub Pages**: Push `dist/` contents to `gh-pages` branch
- **Netlify**: Set build command to `og build --release` and publish directory to `dist`
- **Vercel**: Similar to Netlify
- **Any static server**: Copy `dist/` to your web server's document root

### Q: CORS issues when loading assets

**A**: When loading assets via `fetch()`, the server must allow cross-origin requests. Solutions:

1. **Serve from the same origin**: Use Trunk's dev server or deploy assets alongside the WASM
2. **Configure CORS headers**: On your asset server, add `Access-Control-Allow-Origin: *`
3. **Embed assets**: For small assets, embed them directly in the WASM binary using `include_bytes!`

### Q: How do I add custom assets to the build?

**A**: In `Trunk.toml`, add copy directives:

```toml
[build]
target = "index.html"
dist = "dist"

[[copy]]
from = "assets"
to = "dist/assets"
```

Or use `<link data-trunk rel="copy-dir" href="assets">` in `index.html`.

## Contributing

### Q: How do I run the full CI pipeline locally?

**A**: Use the Makefile:

```bash
make all    # fmt + clippy + test + build
```

Or run individual steps:

```bash
make lint       # Format check + clippy
make test       # Run all tests
make build      # WASM debug build
make release    # WASM release build
```

### Q: Code formatting standards?

**A**: The project uses `rustfmt` with these settings (`rustfmt.toml`):

- Max line width: 100 characters
- Tab spaces: 4
- Imports: Merge imports grouped by crate
- Comments: Wrap at 100 characters

Run `make fmt` to auto-format all code.

### Q: How do I add a new engine module?

**A**: Follow the existing pattern:

1. Create `crates/engine/src/mymodule/mod.rs`
2. Add `pub mod mymodule;` to `crates/engine/src/lib.rs`
3. Re-export key types in the `prelude` module if they're commonly used
4. Add WASM-only guards if needed: `#[cfg(target_arch = "wasm32")]`
5. Write tests in the same file or a `tests` submodule

### Q: How do I add a new dependency?

**A**: Edit the appropriate `Cargo.toml`:

- **Engine dependency**: `crates/engine/Cargo.toml` under `[dependencies]`
- **CLI dependency**: `crates/cli/Cargo.toml` under `[dependencies]`
- **Shared dependency**: Root `Cargo.toml` under `[workspace.dependencies]`

After adding, run:

```bash
cargo check --workspace    # Verify it compiles
cargo deny check           # Audit for license issues
make lint                  # Ensure no clippy warnings
```
