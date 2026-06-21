use anyhow::{bail, Result};

use crate::util::{find_project_root, print_header, print_step, print_success, run_cmd_inherit};

pub fn run(deny_warnings: bool) -> Result<()> {
    let root = find_project_root()?;

    print_header("Running clippy analysis");

    print_step("Analyzing code...");
    let mut args = vec!["clippy", "--all-targets"];
    if deny_warnings {
        args.push("--");
        args.push("-D");
        args.push("warnings");
    }

    let exit_code = run_cmd_inherit("cargo", &args, &root)?;
    if exit_code != 0 {
        bail!("Clippy found issues.");
    }

    print_success("Clippy analysis passed!");
    Ok(())
}
