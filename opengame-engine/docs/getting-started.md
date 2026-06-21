# Getting Started

This guide walks you through setting up your development environment, installing all required tools, creating your first OpenGame project, and running it in the browser.

## System Requirements

| Requirement | Minimum | Recommended |
|---|---|---|
| OS | Linux, macOS, Windows | Any 64-bit OS |
| Rust | 1.75+ | Latest stable |
| RAM | 2 GB free | 4 GB+ |
| Disk | 500 MB | 2 GB+ (for toolchains & caches) |
| Browser | Chrome 56+, Firefox 51+, Safari 15+, Edge 79+ | Latest Chrome or Firefox |

## Step 1: Install Rust

If you do not have Rust installed, use [rustup](https://rustup.rs/):

```bash
# Linux / macOS
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# After installation, restart your shell and verify:
rustc --version
cargo --version
```

On Windows, download and run the installer from [rustup.rs](https://rustup.rs/).

## Step 2: Add the WebAssembly Target

OpenGame compiles to `wasm32-unknown-unknown`. Add this target to your Rust toolchain:

```bash
rustup target add wasm32-unknown-unknown
```

Verify the target is installed:

```bash
rustup target list --installed | grep wasm
# Expected output: wasm32-unknown-unknown
```

## Step 3: Install Trunk

[Trunk](https://trunkrs.dev/) is the WASM bundler used by OpenGame. It handles building, linking, and serving your WASM application:

```bash
cargo install trunk
```

Verify:

```bash
trunk --version
```

## Step 4: Clone the Repository

```bash
git clone https://github.com/opengame/opengame.git
cd opengame
```

## Step 5: Install All Tools (One Command)

The project provides a convenience target to install all required and recommended tools:

```bash
make install-tools
```

This runs:

```bash
rustup target add wasm32-unknown-unknown
rustup component add clippy rustfmt
cargo install trunk
cargo install wasm-bindgen-cli
```

## Step 6: Verify Your Environment

Run the built-in diagnostic tool to check that everything is in order:

```bash
# Build and run the CLI tool
cargo run -p opengame-cli -- doctor
```

This checks for:
- Rust toolchain version
- wasm32 target availability
- Trunk installation
- Required components (clippy, rustfmt)

## Step 7: Run Your First Example

Start the Pong example with hot reload:

```bash
# Using the CLI tool
cargo run -p opengame-cli -- serve

# Or using Trunk directly
trunk serve

# Or using Make
make dev
```

Open your browser and navigate to:

```
http://localhost:8080
```

You should see the Pong game rendered on a canvas. Use `W/S` to control the left paddle and `Up/Down` for the right paddle.

## Step 8: Create a New Project

Use the CLI to scaffold a new game project:

```bash
# Build the CLI tool first
make cli

# Create a new project (minimal template)
./target/x86_64-unknown-linux-gnu/release/og new my-game

# Or with the ECS + App template
./target/x86_64-unknown-linux-gnu/release/og new my-game --template ecs
```

Alternatively, install the CLI globally:

```bash
make cli-install
og new my-game
```

## Project Layout After Scaffolding

```
my-game/
├── Cargo.toml
├── index.html
├── Trunk.toml
├── web/
│   └── style.css
├── assets/
│   └── (your game assets)
└── src/
    └── lib.rs
```

## Build Targets

| Command | Description |
|---|---|
| `make dev` | Start development server with hot reload on port 8080 |
| `make build` | Build WASM in debug mode |
| `make release` | Build WASM in release mode (optimized) |
| `make test` | Run all tests |
| `make lint` | Run format check + clippy |
| `make fmt` | Auto-format all code |
| `make doc` | Generate and open API documentation |
| `make clean` | Remove all build artifacts |
| `make all` | Run the complete pipeline (fmt + clippy + test + build) |

## Development Workflow

A typical development cycle looks like this:

```
1. Edit code in src/
2. Trunk detects changes and rebuilds automatically (hot reload)
3. Browser refreshes with the new build
4. Repeat
```

For production:

```
1. Run `make release` to create an optimized build
2. The output is in the `dist/` directory
3. Deploy `dist/` to any static hosting service
```

## IDE Setup

### VS Code (Recommended)

Install the following extensions:

- **rust-analyzer**: Rust language server with inline diagnostics
- **Even Better TOML**: TOML file support
- **WebAssembly**: WASM text format support

### Other IDEs

Any IDE with Rust language support will work. The project uses standard `rustfmt.toml` and `clippy.toml` configurations that are automatically picked up by the toolchain.

## Next Steps

- Read the [Architecture Guide](architecture.md) to understand how the engine works
- Check the [API Reference](api-reference.md) for available types and functions
- Explore the [Examples Guide](examples.md) for detailed walkthroughs
- Review the [CLI Reference](cli-reference.md) for all available commands
