use crate::util::{check_tool, print_header, print_kv, print_step, tool_version};
use anyhow::Result;
use colored::Colorize;

pub fn run() -> Result<()> {
    print_header("Environment Check");

    let tools = [
        ("rustc", "Rust compiler", true),
        ("cargo", "Rust package manager", true),
        ("trunk", "WASM build tool", true),
        ("wasm-bindgen", "WASM binding generator", false),
        ("wasm-opt", "WASM optimizer", false),
    ];

    let mut all_ok = true;

    for (name, desc, required) in &tools {
        let installed = check_tool(name);
        let version = tool_version(name);

        if installed {
            let ver = version.as_deref().unwrap_or("unknown");
            print_step(&format!("{} ({})", name, desc));
            print_kv("", &format!("{}", ver.green()));
        } else if *required {
            all_ok = false;
            print_step(&format!("{} ({})", name, desc));
            print_kv("", &format!("NOT INSTALLED - required").red().to_string());
        } else {
            print_step(&format!("{} ({})", name, desc));
            print_kv("", &format!("not installed (optional)").dimmed().to_string());
        }
    }

    print_step("WASM target");
    let output = std::process::Command::new("rustup")
        .args(["target", "list", "--installed"])
        .output();
    match output {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            if stdout.contains("wasm32-unknown-unknown") {
                print_kv("", &format!("{}", "installed".green()));
            } else {
                all_ok = false;
                print_kv("", &format!("{}", "NOT INSTALLED - run: rustup target add wasm32-unknown-unknown".red()));
            }
        }
        Err(_) => {
            print_kv("", &format!("{}", "rustup not found".red()));
            all_ok = false;
        }
    }

    println!();
    if all_ok {
        println!("  {}", "All checks passed! Environment is ready.".green().bold());
    } else {
        println!("  {}", "Some required tools are missing. Run `make install-tools` to install them.".yellow().bold());
    }

    Ok(())
}
