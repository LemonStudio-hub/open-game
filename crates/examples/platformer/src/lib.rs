use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use opengame_engine::renderer::{GlBackend, ShapeRenderer, Camera2D};
use opengame_engine::input::{InputManager, keys::KeyCode};
use opengame_engine::color::Color;
use opengame_engine::math::Vec2;
use opengame_engine::time::Time;

const PLAYER_SIZE: f32 = 24.0;
const PLAYER_SPEED: f32 = 300.0;
const JUMP_FORCE: f32 = 600.0;
const GRAVITY: f32 = 1500.0;
const GROUND_Y: f32 = 100.0;

struct Platform {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

struct Collectible {
    x: f32,
    y: f32,
    collected: bool,
}

struct PlatformerGame {
    gl: GlBackend,
    shapes: ShapeRenderer,
    camera: Camera2D,
    input: InputManager,
    time: Time,
    player_x: f32,
    player_y: f32,
    player_vx: f32,
    player_vy: f32,
    on_ground: bool,
    platforms: Vec<Platform>,
    collectibles: Vec<Collectible>,
    score: i32,
    camera_x: f32,
    camera_y: f32,
}

impl PlatformerGame {
    fn new() -> Result<Self, String> {
        opengame_engine::log::init();

        let gl = GlBackend::new("game-canvas")?;
        let camera = Camera2D::new(gl.width() as f32, gl.height() as f32);
        let shapes = ShapeRenderer::new(gl.gl())?;
        let input = InputManager::new()?;

        let window = web_sys::window().ok_or("No window")?;
        let performance = window.performance().ok_or("No performance")?;
        let time = Time::new(performance);

        let width = gl.width() as f32;

        let platforms = vec![
            Platform { x: 0.0, y: GROUND_Y, width: width * 3.0, height: 20.0 },
            Platform { x: 200.0, y: 250.0, width: 150.0, height: 20.0 },
            Platform { x: 450.0, y: 350.0, width: 150.0, height: 20.0 },
            Platform { x: 700.0, y: 250.0, width: 200.0, height: 20.0 },
            Platform { x: 1000.0, y: 400.0, width: 180.0, height: 20.0 },
            Platform { x: 1300.0, y: 300.0, width: 150.0, height: 20.0 },
            Platform { x: 1600.0, y: 200.0, width: 200.0, height: 20.0 },
            Platform { x: 1900.0, y: 350.0, width: 180.0, height: 20.0 },
            Platform { x: 2200.0, y: 250.0, width: 200.0, height: 20.0 },
            Platform { x: 2500.0, y: 400.0, width: 150.0, height: 20.0 },
        ];

        let collectibles = vec![
            Collectible { x: 250.0, y: 290.0, collected: false },
            Collectible { x: 500.0, y: 390.0, collected: false },
            Collectible { x: 750.0, y: 290.0, collected: false },
            Collectible { x: 1050.0, y: 440.0, collected: false },
            Collectible { x: 1350.0, y: 340.0, collected: false },
            Collectible { x: 1650.0, y: 240.0, collected: false },
            Collectible { x: 1950.0, y: 390.0, collected: false },
            Collectible { x: 2250.0, y: 290.0, collected: false },
            Collectible { x: 2550.0, y: 440.0, collected: false },
        ];

        Ok(Self {
            gl,
            shapes,
            camera,
            input,
            time,
            player_x: 100.0,
            player_y: GROUND_Y + PLAYER_SIZE,
            player_vx: 0.0,
            player_vy: 0.0,
            on_ground: true,
            platforms,
            collectibles,
            score: 0,
            camera_x: 0.0,
            camera_y: 0.0,
        })
    }

    fn update(&mut self, dt: f32) {
        self.player_vx = 0.0;

        if self.input.is_key_down(KeyCode::KeyA) || self.input.is_key_down(KeyCode::ArrowLeft) {
            self.player_vx = -PLAYER_SPEED;
        }
        if self.input.is_key_down(KeyCode::KeyD) || self.input.is_key_down(KeyCode::ArrowRight) {
            self.player_vx = PLAYER_SPEED;
        }

        if (self.input.is_key_pressed(KeyCode::Space) || self.input.is_key_pressed(KeyCode::KeyW) || self.input.is_key_pressed(KeyCode::ArrowUp)) && self.on_ground {
            self.player_vy = JUMP_FORCE;
            self.on_ground = false;
        }

        self.player_vy -= GRAVITY * dt;

        self.player_x += self.player_vx * dt;
        self.player_y += self.player_vy * dt;

        self.on_ground = false;

        for platform in &self.platforms {
            let player_left = self.player_x - PLAYER_SIZE / 2.0;
            let player_right = self.player_x + PLAYER_SIZE / 2.0;
            let player_bottom = self.player_y - PLAYER_SIZE;
            let player_top = self.player_y;

            let plat_left = platform.x;
            let plat_right = platform.x + platform.width;
            let plat_bottom = platform.y;
            let plat_top = platform.y + platform.height;

            if player_right > plat_left && player_left < plat_right && player_bottom < plat_top && player_top > plat_bottom {
                if self.player_vy <= 0.0 && player_bottom < plat_top && player_top > plat_top {
                    self.player_y = plat_top + PLAYER_SIZE;
                    self.player_vy = 0.0;
                    self.on_ground = true;
                }
            }
        }

        for collectible in &mut self.collectibles {
            if collectible.collected {
                continue;
            }

            let dx = self.player_x - collectible.x;
            let dy = (self.player_y - PLAYER_SIZE / 2.0) - collectible.y;
            let dist = (dx * dx + dy * dy).sqrt();

            if dist < PLAYER_SIZE {
                collectible.collected = true;
                self.score += 100;
            }
        }

        let width = self.gl.width() as f32;
        let height = self.gl.height() as f32;
        let target_camera_x = self.player_x - width / 3.0;
        let target_camera_y = self.player_y - height / 2.0;
        self.camera_x += (target_camera_x - self.camera_x) * 0.1;
        self.camera_y += (target_camera_y - self.camera_y) * 0.1;
        self.camera.position = Vec2::new(self.camera_x + width / 2.0, self.camera_y + height / 2.0);
    }

    fn render(&mut self, _alpha: f32) {
        self.gl.resize();
        let width = self.gl.width() as f32;
        let height = self.gl.height() as f32;
        self.camera.set_viewport(width, height);

        self.gl.clear(0.2, 0.6, 0.9, 1.0);
        self.gl.enable_blend();

        let vp = self.camera.view_projection();

        self.shapes.begin();

        self.shapes.set_color(Color::new(0.2, 0.8, 0.3, 1.0));
        for platform in &self.platforms {
            self.shapes.draw_rect(platform.x, platform.y, platform.width, platform.height);
        }

        self.shapes.set_color(Color::new(1.0, 0.85, 0.2, 1.0));
        for collectible in &self.collectibles {
            if !collectible.collected {
                self.shapes.draw_rect(collectible.x - 8.0, collectible.y - 8.0, 16.0, 16.0);
            }
        }

        self.shapes.set_color(Color::new(0.9, 0.2, 0.2, 1.0));
        self.shapes.draw_rect(
            self.player_x - PLAYER_SIZE / 2.0,
            self.player_y - PLAYER_SIZE,
            PLAYER_SIZE,
            PLAYER_SIZE,
        );

        self.shapes.flush(self.gl.gl(), &vp);
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    let mut game = PlatformerGame::new().expect("Failed to create Platformer game");
    game.time.init();

    let game = Rc::new(RefCell::new(game));
    let f: Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>> = Rc::new(RefCell::new(None));
    let g = f.clone();
    let game_clone = game.clone();

    let mut last_time = 0.0_f64;

    *g.borrow_mut() = Some(Closure::new(move |timestamp: f64| {
        let dt = if last_time == 0.0 {
            1.0 / 60.0
        } else {
            ((timestamp - last_time) / 1000.0).min(0.05)
        };
        last_time = timestamp;

        let mut game = game_clone.borrow_mut();
        game.time.update();
        game.input.update();
        game.update(dt as f32);
        game.render(1.0);
        drop(game);

        request_animation_frame(f.borrow().as_ref().unwrap());
    }));

    request_animation_frame(g.borrow().as_ref().unwrap());
}

fn request_animation_frame(f: &Closure<dyn FnMut(f64)>) {
    web_sys::window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .unwrap();
}
