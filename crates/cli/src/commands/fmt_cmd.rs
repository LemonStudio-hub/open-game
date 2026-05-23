use anyhow::{bail, Result};

use crate::util::{find_project_root, print_header, print_step, print_success, run_cmd_inherit};

pub fn run(check: bool) -> Result<()> {
    let root = find_project_root()?;

    if check {
        print_header("Checking code formatting");
        print_step("Running cargo fmt --check...");
        let exit_code = run_cmd_inherit("cargo", &["fmt", "--all", "--", "--check"], &root)?;
        if exit_code != 0 {
            bail!("Code is not properly formatted. Run `og fmt` to fix.");
        }
        print_success("All files properly formatted!");
    } else {
        print_header("Formatting code");
        print_step("Running cargo fmt...");
        let exit_code = run_cmd_inherit("cargo", &["fmt", "--all"], &root)?;
        if exit_code != 0 {
            bail!("Formatting failed with exit code {}", exit_code);
        }
        print_success("Code formatted successfully!");
    }

    Ok(())
}
