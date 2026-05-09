use anyhow::{bail, Result};

use crate::util::{find_project_root, print_header, print_step, print_success, run_cmd_inherit};

pub fn run(open: bool) -> Result<()> {
    let root = find_project_root()?;

    print_header("Generating API documentation");

    print_step("Running cargo doc...");
    let mut args = vec!["doc", "--no-deps"];
    if open {
        args.push("--open");
    }

    let exit_code = run_cmd_inherit("cargo", &args, &root)?;
    if exit_code != 0 {
        bail!("Documentation generation failed with exit code {}", exit_code);
    }

    print_success("Documentation generated!");
    if !open {
        println!(
            "  Open: {}/target/doc/opengame_engine/index.html",
            root.display()
        );
    }

    Ok(())
}
