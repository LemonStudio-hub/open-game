use anyhow::{bail, Result};
use colored::Colorize;
use std::fs;
use std::path::Path;

use crate::util::{print_header, print_step, print_success};

pub fn run(name: &str, template: Option<&str>) -> Result<()> {
    let project_dir = Path::new(name);
    if project_dir.exists() {
        bail!("Directory `{}` already exists", name);
    }

    let tpl = template.unwrap_or("minimal");
    print_header(&format!("Creating new project: {}", name));

    print_step("Creating directory structure...");
    fs::create_dir_all(project_dir.join("src"))?;
    fs::create_dir_all(project_dir.join("assets/textures"))?;
    fs::create_dir_all(project_dir.join("assets/audio"))?;
    fs::create_dir_all(project_dir.join("assets/fonts"))?;

    print_step("Generating Cargo.toml...");
    let cargo_toml = format!(
        r#"[package]
name = "{name}"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
opengame-engine = {{ path = "../engine" }}
wasm-bindgen = {{ workspace = true }}
js-sys = {{ workspace = true }}
web-sys = {{ workspace = true, features = [
    "Window",
    "Performance",
] }}
"#
    );
    fs::write(project_dir.join("Cargo.toml"), cargo_toml)?;

    print_step("Generating index.html...");
    let index_html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0, user-scalable=no" />
    <title>{name}</title>
    <link data-trunk rel="rust" data-wasm-opt="z" />
    <style>
        * {{ margin: 0; padding: 0; box-sizing: border-box; }}
        body {{ background: #000; overflow: hidden; }}
        canvas {{ display: block; width: 100vw; height: 100vh; }}
    </style>
</head>
<body>
    <canvas id="game-canvas"></canvas>
</body>
</html>
"#
    );
    fs::write(project_dir.join("index.html"), index_html)?;

    match tpl {
        "ecs" | "full" => {
            print_step("Generating ECS + App template...");
            let lib_rs = format!(
                r#"use wasm_bindgen::prelude::*;

use opengame_engine::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {{
    opengame_engine::log::init();

    let mut app = App::new("game-canvas").expect("Failed to create app");
    app.set_clear_color(Color::new(0.1, 0.1, 0.15, 1.0));

    app.add_startup_system(|world: &mut World| {{
        console_log!("Game started!");
    }});

    app.add_system(|world: &mut World, dt: f32| {{
    }});

    app.add_render_system(|world: &mut World, alpha: f32| {{
    }});

    app.run();
}}
"#
            );
            fs::write(project_dir.join("src/lib.rs"), lib_rs)?;
        }
        _ => {
            print_step("Generating minimal template...");
            let lib_rs = format!(
                r#"use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use opengame_engine::renderer::{{GlBackend, ShapeRenderer, Camera2D}};
use opengame_engine::color::Color;
use opengame_engine::time::Time;

struct Game {{
    gl: GlBackend,
    shapes: ShapeRenderer,
    camera: Camera2D,
    time: Time,
}}

impl Game {{
    fn new() -> Result<Self, String> {{
        opengame_engine::log::init();

        let gl = GlBackend::new("game-canvas")?;
        let camera = Camera2D::new(gl.width() as f32, gl.height() as f32);
        let shapes = ShapeRenderer::new(gl.gl())?;

        let window = web_sys::window().ok_or("No window")?;
        let performance = window.performance().ok_or("No performance")?;
        let time = Time::new(performance);

        Ok(Self {{ gl, shapes, camera, time }})
    }}

    fn update(&mut self, dt: f32) {{
    }}

    fn render(&mut self, alpha: f32) {{
        self.gl.resize();
        let width = self.gl.width() as f32;
        let height = self.gl.height() as f32;
        self.camera.set_viewport(width, height);

        self.gl.clear(0.1, 0.1, 0.15, 1.0);
        self.gl.enable_blend();

        let vp = self.camera.view_projection();
        self.shapes.begin();
        self.shapes.set_color(Color::WHITE);
        self.shapes.flush(self.gl.gl(), &vp);
    }}
}}

#[wasm_bindgen(start)]
pub fn main() {{
    let mut game = Game::new().expect("Failed to create game");
    game.time.init();

    let game = Rc::new(RefCell::new(game));
    let f: Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>> = Rc::new(RefCell::new(None));
    let g = f.clone();
    let game_clone = game.clone();

    let mut last_time = 0.0_f64;

    *g.borrow_mut() = Some(Closure::new(move |timestamp: f64| {{
        let dt = if last_time == 0.0 {{
            1.0 / 60.0
        }} else {{
            ((timestamp - last_time) / 1000.0).min(0.05)
        }};
        last_time = timestamp;

        let mut game = game_clone.borrow_mut();
        game.time.update();
        game.update(dt as f32);
        game.render(1.0);
        drop(game);

        request_animation_frame(f.borrow().as_ref().unwrap());
    }}));

    request_animation_frame(g.borrow().as_ref().unwrap());
}}

fn request_animation_frame(f: &Closure<dyn FnMut(f64)>) {{
    web_sys::window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .unwrap();
}}
"#
            );
            fs::write(project_dir.join("src/lib.rs"), lib_rs)?;
        }
    }

    print_step("Generating .gitignore...");
    fs::write(
        project_dir.join(".gitignore"),
        "/target\n/dist\n",
    )?;

    print_success(&format!("Project `{}` created successfully!", name));
    println!();
    println!("  {}", "Next steps:".bold());
    println!("    cd {}", name);
    println!("    og serve");
    println!();

    Ok(())
}
