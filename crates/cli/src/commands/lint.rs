use anyhow::{bail, Result};

use crate::util::{find_project_root, print_header, print_step, print_success, run_cmd_inherit};

pub fn run() -> Result<()> {
    let root = find_project_root()?;

    print_header("Running lint checks");

    print_step("Checking code formatting...");
    let fmt_code = run_cmd_inherit(
        "cargo",
        &["fmt", "--all", "--", "--check"],
        &root,
    )?;
    if fmt_code != 0 {
        bail!("Format check failed. Run `og fmt` to fix.");
    }
    print_success("Format check passed");

    print_step("Running clippy analysis...");
    let clippy_code = run_cmd_inherit(
        "cargo",
        &["clippy", "--all-targets", "--", "-D", "warnings"],
        &root,
    )?;
    if clippy_code != 0 {
        bail!("Clippy found issues. Fix them before proceeding.");
    }
    print_success("Clippy check passed");

    print_success("All lint checks passed!");
    Ok(())
}
