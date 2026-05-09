use anyhow::{bail, Result};

use crate::util::{find_project_root, print_header, print_step, print_success, run_cmd_inherit};

pub fn run(verbose: bool, doc: bool) -> Result<()> {
    let root = find_project_root()?;

    print_header("Running tests");

    let exit_code = if doc {
        print_step("Running doc tests...");
        let mut args = vec!["test", "-p", "engine", "--doc", "--target", "x86_64-unknown-linux-gnu"];
        if verbose {
            args.push("--");
            args.push("--nocapture");
        }
        run_cmd_inherit("cargo", &args, &root)?
    } else {
        print_step("Running unit tests...");
        let mut args = vec!["test", "-p", "engine", "--target", "x86_64-unknown-linux-gnu"];
        if verbose {
            args.push("--");
            args.push("--nocapture");
        }
        let code = run_cmd_inherit("cargo", &args, &root)?;
        if code != 0 {
            return Err(anyhow::anyhow!("Tests failed with exit code {}", code));
        }

        print_step("Running doc tests...");
        run_cmd_inherit(
            "cargo",
            &["test", "-p", "opengame-engine", "--doc", "--target", "x86_64-unknown-linux-gnu"],
            &root,
        )?
    };

    if exit_code != 0 {
        bail!("Tests failed with exit code {}", exit_code);
    }

    print_success("All tests passed!");
    Ok(())
}
