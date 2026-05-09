use anyhow::Result;
use std::fs;

use crate::util::{dir_size, file_size_display, find_project_root, print_header, print_step, print_success, run_cmd_inherit};

pub fn run(all: bool) -> Result<()> {
    let root = find_project_root()?;

    print_header("Cleaning build artifacts");

    print_step("Running cargo clean...");
    run_cmd_inherit("cargo", &["clean"], &root)?;
    print_success("Removed target/");

    if all {
        let dist = root.join("dist");
        if dist.exists() {
            let size = dir_size(&dist);
            fs::remove_dir_all(&dist)?;
            print_success(&format!(
                "Removed dist/ ({})",
                file_size_display(size)
            ));
        } else {
            print_step("dist/ not found, skipping");
        }
    }

    print_success("Clean complete!");
    Ok(())
}
