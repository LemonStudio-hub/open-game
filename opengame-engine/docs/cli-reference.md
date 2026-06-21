# CLI Reference

The OpenGame CLI (`og`) is a command-line tool for creating, building, serving, and managing OpenGame projects. It provides a complete development workflow from project scaffolding to production builds.

## Installation

### Build from Source

```bash
# Build the CLI
make cli

# Binary location
./target/x86_64-unknown-linux-gnu/release/og
```

### Install Globally

```bash
# Install to ~/.local/bin
make cli-install

# Verify
og --version
```

## Global Options

```
og [OPTIONS] <COMMAND>
```

| Option | Description |
|---|---|
| `--verbose` | Enable verbose output |
| `--help` | Print help information |
| `--version` | Print version information |

## Commands

### `og new`

Create a new OpenGame project with boilerplate code.

```
og new <NAME> [OPTIONS]
```

**Arguments**:
- `<NAME>`: Project name (used as directory name and crate name)

**Options**:

| Option | Short | Description |
|---|---|---|
| `--template <TEMPLATE>` | `-t` | Project template (`minimal` or `ecs`) |
| `--output <DIR>` | `-o` | Output directory (default: current directory) |
| `--git` | `-g` | Initialize a git repository |
| `--assets` | `-a` | Include starter asset files |

**Templates**:

| Template | Description |
|---|---|
| `minimal` | Basic project with empty lib.rs |
| `ecs` | ECS-based project with App runner, startup/update/render systems |

**Examples**:

```bash
# Create a minimal project
og new my-game

# Create with ECS template and git
og new my-game --template ecs --git

# Create in a specific directory
og new my-game --output ~/projects/
```

**Generated Structure**:

```
my-game/
├── Cargo.toml
├── index.html
├── Trunk.toml
├── .cargo/
│   └── config.toml
├── web/
│   └── style.css
└── src/
    └── lib.rs
```

### `og build`

Build the current project to WASM.

```
og build [OPTIONS]
```

**Options**:

| Option | Short | Description |
|---|---|---|
| `--release` | `-r` | Build in release mode with optimizations |
| `--dist <DIR>` | `-d` | Output directory (default: `dist/`) |

**Examples**:

```bash
# Debug build
og build

# Release build
og build --release

# Custom output directory
og build --release --dist public/
```

**Build Pipeline**:

1. `cargo build --target wasm32-unknown-unknown`
2. `trunk build` (generates JS glue, processes HTML)
3. (Release only) `wasm-opt -Oz` for size optimization

### `og serve`

Start a development server with hot reload.

```
og serve [OPTIONS]
```

**Options**:

| Option | Short | Description |
|---|---|---|
| `--port <PORT>` | `-p` | Server port (default: 8080) |
| `--open` | `-o` | Open browser automatically |
| `--no-watch` | | Disable file watching |

**Examples**:

```bash
# Default: serve on port 8080
og serve

# Custom port with auto-open
og serve --port 3000 --open
```

**Hot Reload Behavior**:
- Watches `src/` for Rust file changes
- Watches `web/` for CSS changes
- Watches `index.html` for HTML changes
- Rebuilds and refreshes browser automatically

### `og run`

Build and run a specific example.

```
og run <EXAMPLE> [OPTIONS]
```

**Arguments**:
- `<EXAMPLE>`: Example name (`pong` or `platformer`)

**Options**:

| Option | Short | Description |
|---|---|---|
| `--port <PORT>` | `-p` | Server port (default: 8080) |

**Examples**:

```bash
# Run the Pong example
og run pong

# Run the Platformer on a custom port
og run platformer --port 3000
```

### `og format`

Format source code using `rustfmt`.

```
og format [OPTIONS]
```

**Options**:

| Option | Short | Description |
|---|---|---|
| `--check` | `-c` | Check formatting without modifying files |

**Examples**:

```bash
# Auto-format all files
og format

# Check if files are formatted (CI usage)
og format --check
```

### `og lint`

Run clippy linter.

```
og lint [OPTIONS]
```

**Options**:

| Option | Short | Description |
|---|---|---|
| `--fix` | `-f` | Automatically fix lint issues |
| `--all-features` | | Enable all features for linting |

**Examples**:

```bash
# Run clippy
og lint

# Auto-fix issues
og lint --fix
```

### `og test`

Run all tests in the workspace.

```
og test [OPTIONS]
```

**Options**:

| Option | Short | Description |
|---|---|---|
| `--verbose` | `-v` | Show detailed test output |

**Examples**:

```bash
# Run all tests
og test

# Verbose output
og test --verbose
```

### `og assets`

Manage project assets.

```
og assets <SUBCOMMAND>
```

**Subcommands**:

#### `og assets init`

Create the assets directory structure.

```bash
og assets init
```

Creates:
```
assets/
├── images/
├── audio/
│   ├── music/
│   └── sfx/
├── fonts/
└── data/
```

#### `og assets optimize`

Optimize image assets.

```bash
og assets optimize [OPTIONS]
```

| Option | Description |
|---|---|
| `--quality <N>` | JPEG quality (1-100, default: 80) |
| `--resize <WxH>` | Resize images to fit dimensions |

### `og watch`

Watch for file changes and rebuild automatically.

```
og watch
```

This is equivalent to `og serve` but without starting the HTTP server.

### `og clean`

Remove build artifacts.

```
og clean
```

Removes:
- `target/` directory
- `dist/` directory

### `og docs`

Generate and open API documentation.

```
og docs [OPTIONS]
```

**Options**:

| Option | Description |
|---|---|
| `--open` | Open in browser (default: true) |
| `--no-deps` | Don't build documentation for dependencies |

### `og doctor`

Run environment diagnostics.

```
og doctor
```

Checks:
- Rust toolchain version and installation
- `wasm32-unknown-unknown` target availability
- Trunk installation
- Required components (clippy, rustfmt)
- Disk space
- Network connectivity (for dependency downloads)

**Example Output**:

```
OpenGame Environment Diagnostics
================================
✓ Rust toolchain: 1.78.0
✓ wasm32-unknown-unknown target: installed
✓ Trunk: 0.21.2
✓ clippy: installed
✓ rustfmt: installed
✓ Disk space: 15.2 GB available

All checks passed! Ready to develop.
```

## Makefile Equivalents

The project includes a Makefile that wraps common CLI commands:

| Make Command | CLI Equivalent |
|---|---|
| `make dev` | `trunk serve` |
| `make build` | `trunk build` |
| `make release` | `trunk build --release` |
| `make test` | `cargo test --workspace` |
| `make fmt` | `cargo fmt --all` |
| `make clippy` | `cargo clippy --workspace --all-targets` |
| `make lint` | `cargo fmt --all -- --check && cargo clippy --workspace --all-targets` |
| `make doc` | `cargo doc --workspace --open` |
| `make clean` | `cargo clean && rm -rf dist/` |
| `make cli` | `cargo build --release -p opengame-cli` |
| `make cli-install` | Build + copy to `~/.local/bin/` |

## Configuration

The CLI reads configuration from:

1. **`Cargo.toml`**: Project metadata and dependencies
2. **`Trunk.toml`**: Build and serve settings
3. **`opengame.toml`** (future): Engine-specific configuration

## Exit Codes

| Code | Meaning |
|---|---|
| 0 | Success |
| 1 | General error |
| 2 | Build failure |
| 3 | Environment error (missing tools) |
