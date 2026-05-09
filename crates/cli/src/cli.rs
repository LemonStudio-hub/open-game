use clap::{Parser, Subcommand};

const BANNER: &str = r#"
  ___                    ____                  
 / _ \ _ __   ___ _ __  / ___|  __ ___   _____ 
| | | | '_ \ / _ \ '_ \| |  _ / _` \ \ / / _ \
| |_| | |_) |  __/ | | | |_| | (_| |\ V /  __/
 \___/| .__/ \___|_| |_|\____|\__,_| \_/ \___|
      |_|   Engine CLI Tool
"#;

#[derive(Parser)]
#[command(
    name = "og",
    version,
    about = "CLI tool for OpenGame Engine",
    long_about = BANNER,
    arg_required_else_help = true,
    propagate_version = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(long, global = true, help = "Working directory")]
    pub manifest_path: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Create a new game project")]
    New {
        #[arg(help = "Project name")]
        name: String,
        #[arg(long, help = "Use ECS + App template instead of minimal")]
        template: Option<String>,
    },

    #[command(about = "Build WASM package")]
    Build {
        #[arg(long, help = "Build in release mode with optimizations")]
        release: bool,
        #[arg(long, help = "Example name to build")]
        example: Option<String>,
    },

    #[command(alias = "dev", about = "Start development server with hot reload")]
    Serve {
        #[arg(long, default_value = "8080", help = "Port number")]
        port: u16,
        #[arg(long, help = "Bind address")]
        address: Option<String>,
        #[arg(long, help = "Open browser automatically")]
        open: bool,
    },

    #[command(about = "Run an example")]
    Run {
        #[arg(help = "Example name (pong, platformer)")]
        example: String,
        #[arg(long, help = "Run in release mode")]
        release: bool,
    },

    #[command(about = "Check compilation without producing artifacts")]
    Check {
        #[arg(long, help = "Check all targets")]
        all_targets: bool,
    },

    #[command(about = "Run tests")]
    Test {
        #[arg(long, help = "Show test output")]
        verbose: bool,
        #[arg(long, help = "Run doc tests only")]
        doc: bool,
    },

    #[command(about = "Run all lint checks (format + clippy)")]
    Lint,

    #[command(about = "Format source code")]
    Fmt {
        #[arg(long, help = "Check formatting without modifying files")]
        check: bool,
    },

    #[command(about = "Run clippy static analysis")]
    Clippy {
        #[arg(long, help = "Treat warnings as errors")]
        deny_warnings: bool,
    },

    #[command(about = "Clean build artifacts")]
    Clean {
        #[arg(long, help = "Also remove dist/ directory")]
        all: bool,
    },

    #[command(about = "Check development environment and tool dependencies")]
    Doctor,

    #[command(about = "Show project information and engine stats")]
    Info,

    #[command(about = "Manage game assets")]
    Assets {
        #[command(subcommand)]
        action: AssetsAction,
    },

    #[command(about = "Generate API documentation")]
    Doc {
        #[arg(long, help = "Open in browser")]
        open: bool,
    },
}

#[derive(Subcommand)]
pub enum AssetsAction {
    #[command(about = "List all assets")]
    List {
        #[arg(long, help = "Filter by type (textures, audio, fonts)")]
        r#type: Option<String>,
    },
    #[command(about = "Validate asset files")]
    Validate,
    #[command(about = "Show asset size summary")]
    Size,
}
