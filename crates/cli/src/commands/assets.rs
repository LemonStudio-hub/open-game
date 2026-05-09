use anyhow::{bail, Result};
use colored::Colorize;
use std::path::Path;

use crate::util::{file_size_display, find_project_root, print_header, print_kv, print_step, print_success, print_warn};

pub fn run_list(filter_type: Option<&str>) -> Result<()> {
    let root = find_project_root()?;
    let assets_dir = root.join("assets");

    if !assets_dir.exists() {
        bail!("No assets/ directory found");
    }

    print_header("Game Assets");

    let types = if let Some(t) = filter_type {
        vec![t.to_string()]
    } else {
        vec![
            "textures".to_string(),
            "audio".to_string(),
            "fonts".to_string(),
        ]
    };

    let mut total_count = 0;
    let mut total_size: u64 = 0;

    for asset_type in &types {
        let dir = assets_dir.join(asset_type);
        if !dir.exists() {
            continue;
        }

        println!();
        print_step(&format!("{}/", asset_type));

        for entry in walkdir::WalkDir::new(&dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path();
            let relative = path.strip_prefix(&assets_dir).unwrap_or(path);
            let size = entry.metadata().map_or(0, |m| m.len());
            total_count += 1;
            total_size += size;
            print_kv(
                &relative.display().to_string(),
                &file_size_display(size),
            );
        }
    }

    println!();
    print_kv("Total files", &total_count.to_string());
    print_kv("Total size", &file_size_display(total_size));

    Ok(())
}

pub fn run_validate() -> Result<()> {
    let root = find_project_root()?;
    let assets_dir = root.join("assets");

    if !assets_dir.exists() {
        bail!("No assets/ directory found");
    }

    print_header("Validating Assets");

    let mut issues = 0;
    let mut checked = 0;

    let valid_texture_exts = ["png", "jpg", "jpeg", "gif", "webp", "bmp", "svg"];
    let valid_audio_exts = ["mp3", "ogg", "wav", "flac", "aac", "webm"];
    let valid_font_exts = ["ttf", "otf", "woff", "woff2", "fnt", "json"];

    for entry in walkdir::WalkDir::new(&assets_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        let ext = path
            .extension()
            .map(|e| e.to_string_lossy().to_lowercase())
            .unwrap_or_default();
        let relative = path.strip_prefix(&assets_dir).unwrap_or(path);
        let relative_str = relative.display().to_string();

        checked += 1;

        let is_valid = if relative_str.starts_with("textures/") {
            valid_texture_exts.contains(&ext.as_str())
        } else if relative_str.starts_with("audio/") {
            valid_audio_exts.contains(&ext.as_str())
        } else if relative_str.starts_with("fonts/") {
            valid_font_exts.contains(&ext.as_str())
        } else {
            true
        };

        if !is_valid {
            issues += 1;
            print_warn(&format!("Unknown file type: {}", relative_str));
        }

        let size = entry.metadata().map_or(0, |m| m.len());
        if size == 0 {
            issues += 1;
            print_warn(&format!("Empty file: {}", relative_str));
        }
    }

    println!();
    if issues == 0 {
        print_success(&format!("All {} assets are valid!", checked));
    } else {
        println!(
            "  {} {} issues found in {} assets",
            "⚠".yellow().bold(),
            issues,
            checked
        );
    }

    Ok(())
}

pub fn run_size() -> Result<()> {
    let root = find_project_root()?;
    let assets_dir = root.join("assets");

    if !assets_dir.exists() {
        bail!("No assets/ directory found");
    }

    print_header("Asset Size Summary");

    let mut total: u64 = 0;

    for entry in std::fs::read_dir(&assets_dir)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            let name = entry.file_name().to_string_lossy().to_string();
            let size = dir_size_recursive(&entry.path());
            total += size;
            let bar_len = ((size as f64 / 1024.0) as usize).min(40);
            let bar = "█".repeat(bar_len);
            println!(
                "  {:<14} {:>10}  {}",
                name.dimmed(),
                file_size_display(size).bold(),
                bar.cyan()
            );
        }
    }

    println!();
    print_kv("Total", &file_size_display(total).bold().to_string());

    Ok(())
}

fn dir_size_recursive(dir: &Path) -> u64 {
    walkdir::WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.metadata().map_or(0, |m| m.len()))
        .sum()
}
