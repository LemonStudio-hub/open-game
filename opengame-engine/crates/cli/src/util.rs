use anyhow::{bail, Context, Result};
use colored::Colorize;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

pub fn find_project_root() -> Result<PathBuf> {
    let mut dir = std::env::current_dir()?;
    loop {
        if dir.join("Cargo.toml").exists() && dir.join("crates").exists() {
            return Ok(dir);
        }
        if !dir.pop() {
            bail!("Not inside an OpenGame project (no Cargo.toml + crates/ found)");
        }
    }
}

#[allow(dead_code)]
pub fn run_cmd(program: &str, args: &[&str], cwd: &Path) -> Result<Output> {
    let output = Command::new(program)
        .args(args)
        .current_dir(cwd)
        .output()
        .with_context(|| format!("Failed to execute `{}`", program))?;
    Ok(output)
}

pub fn run_cmd_inherit(program: &str, args: &[&str], cwd: &Path) -> Result<i32> {
    let status = Command::new(program)
        .args(args)
        .current_dir(cwd)
        .status()
        .with_context(|| format!("Failed to execute `{}`", program))?;
    Ok(status.code().unwrap_or(-1))
}

#[allow(dead_code)]
pub fn run_cmd_async(program: &str, args: &[&str], cwd: &Path) -> Result<std::process::Child> {
    let child = Command::new(program)
        .args(args)
        .current_dir(cwd)
        .spawn()
        .with_context(|| format!("Failed to spawn `{}`", program))?;
    Ok(child)
}

pub fn check_tool(name: &str) -> bool {
    Command::new(name)
        .arg("--version")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .is_ok()
}

pub fn tool_version(name: &str) -> Option<String> {
    let output = Command::new(name).arg("--version").output().ok()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    Some(stdout.trim().to_string())
}

pub fn print_header(msg: &str) {
    println!();
    println!("  {}", msg.bold().cyan());
    println!("  {}", "─".repeat(msg.len()).dimmed());
}

pub fn print_step(msg: &str) {
    println!("  {} {}", "→".green().bold(), msg);
}

pub fn print_warn(msg: &str) {
    println!("  {} {}", "!".yellow().bold(), msg);
}

#[allow(dead_code)]
pub fn print_error(msg: &str) {
    eprintln!("  {} {}", "✗".red().bold(), msg);
}

pub fn print_success(msg: &str) {
    println!("  {} {}", "✓".green().bold(), msg);
}

pub fn print_kv(key: &str, value: &str) {
    println!("  {:<24} {}", key.dimmed(), value);
}

pub fn file_size_display(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{:.2} MB", bytes as f64 / (1024.0 * 1024.0))
    }
}

pub fn count_files(dir: &Path, extension: &str) -> usize {
    walkdir::WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == extension))
        .count()
}

pub fn count_lines(dir: &Path, extension: &str) -> usize {
    walkdir::WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == extension))
        .filter_map(|e| std::fs::read_to_string(e.path()).ok())
        .map(|c| c.lines().count())
        .sum()
}

pub fn dir_size(dir: &Path) -> u64 {
    walkdir::WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.metadata().map_or(0, |m| m.len()))
        .sum()
}
