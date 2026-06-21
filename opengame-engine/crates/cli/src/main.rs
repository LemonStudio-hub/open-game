mod cli;
mod commands;
mod util;

use clap::Parser;
use cli::{AssetsAction, Cli, Commands};
use colored::Colorize;

fn main() {
    let cli = Cli::parse();

    if let Some(ref path) = cli.manifest_path {
        std::env::set_current_dir(path).unwrap_or_else(|e| {
            eprintln!("{}: {}", "Error".red().bold(), e);
            std::process::exit(1);
        });
    }

    let result = match cli.command {
        Commands::New { name, template } => commands::new_cmd::run(&name, template.as_deref()),
        Commands::Build { release, example } => commands::build::run(release, example.as_deref()),
        Commands::Serve {
            port,
            address,
            open,
        } => commands::serve::run(port, address.as_deref(), open),
        Commands::Run { example, release } => commands::run_cmd::run(&example, release),
        Commands::Check { all_targets } => commands::check::run(all_targets),
        Commands::Test { verbose, doc } => commands::test_cmd::run(verbose, doc),
        Commands::Lint => commands::lint::run(),
        Commands::Fmt { check } => commands::fmt_cmd::run(check),
        Commands::Clippy { deny_warnings } => commands::clippy::run(deny_warnings),
        Commands::Clean { all } => commands::clean::run(all),
        Commands::Doctor => commands::doctor::run(),
        Commands::Info => commands::info::run(),
        Commands::Assets { action } => match action {
            AssetsAction::List { r#type } => commands::assets::run_list(r#type.as_deref()),
            AssetsAction::Validate => commands::assets::run_validate(),
            AssetsAction::Size => commands::assets::run_size(),
        },
        Commands::Doc { open } => commands::doc::run(open),
    };

    if let Err(e) = result {
        eprintln!();
        eprintln!("  {} {}", "Error:".red().bold(), e);
        std::process::exit(1);
    }
}
