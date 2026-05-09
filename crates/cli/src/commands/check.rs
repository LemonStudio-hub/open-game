use anyhow::{bail, Result};

use crate::util::{find_project_root, print_header, print_step, print_success, run_cmd_inherit};

pub fn run(all_targets: bool) -> Result<()> {
    let root = find_project_root()?;

    print_header("Checking compilation");

    print_step("Running cargo check...");
    let mut args = vec!["check", "--all-targets"];
    if !all_targets {
        args = vec!["check"];
    }

    let exit_code = run_cmd_inherit("cargo", &args, &root)?;
    if exit_code != 0 {
        bail!("Check failed with exit code {}", exit_code);
    }

    print_success("Compilation check passed!");
    Ok(())
}
