use anyhow::Result;
use std::path::Path;

use crate::util::{
    count_files, count_lines, dir_size, file_size_display, find_project_root, print_header,
    print_kv, print_step,
};

pub fn run() -> Result<()> {
    let root = find_project_root()?;

    print_header("Project Information");

    let engine_src = root.join("crates/engine/src");
    let examples_dir = root.join("crates/examples");
    let assets_dir = root.join("assets");

    print_step("Engine");
    if engine_src.exists() {
        let rs_files = count_files(&engine_src, "rs");
        let total_lines = count_lines(&engine_src, "rs");
        print_kv("Source files", &rs_files.to_string());
        print_kv("Lines of code", &total_lines.to_string());
    }

    print_kv("Engine path", "crates/engine/");

    print_step("Examples");
    if examples_dir.exists() {
        for entry in std::fs::read_dir(&examples_dir)? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                let name = entry.file_name().to_string_lossy().to_string();
                let src = entry.path().join("src");
                if src.exists() {
                    let lines = count_lines(&src, "rs");
                    print_kv(&name, &format!("{} lines", lines));
                } else {
                    print_kv(&name, "no src/");
                }
            }
        }
    }

    print_step("Assets");
    if assets_dir.exists() {
        let total_size = dir_size(&assets_dir);
        print_kv("Total size", &file_size_display(total_size));

        for entry in std::fs::read_dir(&assets_dir)? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                let name = entry.file_name().to_string_lossy().to_string();
                let size = dir_size(&entry.path());
                let file_count = count_all_files(&entry.path());
                print_kv(
                    &name,
                    &format!("{} files, {}", file_count, file_size_display(size)),
                );
            }
        }
    }

    print_step("Build");
    let dist = root.join("dist");
    if dist.exists() {
        let size = dir_size(&dist);
        print_kv("dist/", &file_size_display(size));
    } else {
        print_kv("dist/", "not built");
    }

    let target = root.join("target");
    if target.exists() {
        let size = dir_size(&target);
        print_kv("target/", &file_size_display(size));
    }

    println!();
    print_step("Engine Modules");
    let modules = [
        ("ecs", "Entity Component System"),
        (
            "renderer",
            "WebGL2 rendering (sprites, shapes, text, camera)",
        ),
        (
            "physics",
            "2D physics (rigid body, collision, spatial hash)",
        ),
        ("input", "Input handling (keyboard, mouse, touch, gamepad)"),
        ("audio", "Audio engine (Web Audio API, music/SFX mixer)"),
        ("scene", "Scene management (stack, transitions)"),
        ("asset", "Asset loading and caching"),
        ("profiler", "Performance profiling"),
        ("debug", "Debug overlay"),
        ("event", "Type-safe event bus"),
        ("time", "Time management (fixed timestep, timers)"),
        ("math", "Math utilities (glam)"),
        ("color", "Color types and presets"),
        ("transform", "2D transform component"),
        ("sprite_component", "Sprite and sprite sheet"),
    ];

    for (name, desc) in &modules {
        print_kv(name, desc);
    }

    println!();
    Ok(())
}

fn count_all_files(dir: &Path) -> usize {
    walkdir::WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .count()
}
