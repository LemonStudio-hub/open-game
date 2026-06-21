use anyhow::{bail, Result};
use colored::Colorize;

use crate::util::{find_project_root, print_header, print_step, run_cmd_inherit};

pub fn run(port: u16, address: Option<&str>, open: bool) -> Result<()> {
    let root = find_project_root()?;

    print_header("Starting development server");

    print_step("Checking trunk installation...");
    if !crate::util::check_tool("trunk") {
        bail!("trunk is not installed. Run: cargo install trunk");
    }

    let addr = address.unwrap_or("0.0.0.0");
    let port_str = port.to_string();

    print_step(&format!("Server: {}:{}", addr, port));
    println!();
    println!("  {}", "Hot reload enabled. Press Ctrl+C to stop.".dimmed());
    println!();

    let mut args = vec!["serve", "--address", addr, "--port", &port_str];
    if open {
        args.push("--open");
    }

    let exit_code = run_cmd_inherit("trunk", &args, &root)?;
    if exit_code != 0 && exit_code != 130 {
        bail!("Dev server exited with code {}", exit_code);
    }

    Ok(())
}
