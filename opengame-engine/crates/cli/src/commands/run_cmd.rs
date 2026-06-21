use anyhow::{bail, Result};
use colored::Colorize;

use crate::util::{find_project_root, print_header, print_step, run_cmd_inherit};

pub fn run(example: &str, release: bool) -> Result<()> {
    let root = find_project_root()?;
    let example_dir = root.join(format!("crates/examples/{}", example));

    if !example_dir.exists() {
        bail!(
            "Example `{}` not found. Available examples: pong, platformer, space-blitz",
            example
        );
    }

    let manifest = example_dir.join("index.html");
    if !manifest.exists() {
        bail!("No index.html found for example `{}`", example);
    }

    let mode = if release { "release" } else { "development" };
    print_header(&format!("Running example: {} ({})", example, mode));

    print_step("Checking trunk installation...");
    if !crate::util::check_tool("trunk") {
        bail!("trunk is not installed. Run: cargo install trunk");
    }

    print_step("Starting dev server for example...");
    println!();
    println!(
        "  {}",
        format!("http://localhost:8080/{}", example).cyan().bold()
    );
    println!("  {}", "Hot reload enabled. Press Ctrl+C to stop.".dimmed());
    println!();

    let dist = format!("dist/{}", example);
    let mut args = vec![
        "serve",
        "--dist",
        &dist,
        "--port",
        "8080",
        manifest.to_str().unwrap(),
    ];
    if release {
        args.push("--release");
    }

    let exit_code = run_cmd_inherit("trunk", &args, &root)?;
    if exit_code != 0 && exit_code != 130 {
        bail!("Example server exited with code {}", exit_code);
    }

    Ok(())
}
