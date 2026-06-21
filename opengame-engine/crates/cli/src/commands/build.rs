use anyhow::{bail, Result};
use colored::Colorize;

use crate::util::{
    dir_size, file_size_display, find_project_root, print_header, print_step, print_success,
    run_cmd_inherit,
};

pub fn run(release: bool, example: Option<&str>) -> Result<()> {
    let root = find_project_root()?;
    let mode = if release { "release" } else { "development" };

    print_header(&format!("Building WASM package ({})", mode));

    print_step("Checking trunk installation...");
    if !crate::util::check_tool("trunk") {
        bail!("trunk is not installed. Run: cargo install trunk");
    }

    print_step("Compiling Rust to WASM...");

    let exit_code = if let Some(ex) = example {
        let example_dir = root.join(format!("crates/examples/{}", ex));
        if !example_dir.exists() {
            bail!("Example `{}` not found at {}", ex, example_dir.display());
        }
        let manifest = example_dir.join("index.html");
        if !manifest.exists() {
            bail!("No index.html found for example `{}`", ex);
        }
        let dist = format!("dist/{}", ex);
        let mut args = vec!["build", "--dist", &dist, manifest.to_str().unwrap()];
        if release {
            args.push("--release");
        }
        run_cmd_inherit("trunk", &args, &root)?
    } else {
        let mut args = vec!["build"];
        if release {
            args.push("--release");
        }
        run_cmd_inherit("trunk", &args, &root)?
    };

    if exit_code != 0 {
        bail!("Build failed with exit code {}", exit_code);
    }

    let dist = root.join("dist");
    if dist.exists() {
        let total_size = dir_size(&dist);
        print_success(&format!(
            "Build complete! Output size: {}",
            file_size_display(total_size).bold()
        ));
        println!("  Output: {}", dist.display().to_string().dimmed());
    } else {
        print_success("Build complete!");
    }

    Ok(())
}
