mod components;
mod resources;
mod systems;

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

type AnimationFrameClosure = Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>>;

use opengame_engine::color::Color;
use opengame_engine::ecs::{QuerySingle, World};
use opengame_engine::ecs::system::SystemScheduler;
use opengame_engine::input::{keys::KeyCode, InputManager};
use opengame_engine::math::{Mat4, Vec3};
use opengame_engine::renderer::{GlBackend, ShapeRenderer};
use opengame_engine::time::Time;

use components::*;
use resources::*;

// ── Constants ──────────────────────────────────────────────────────────────────
pub(crate) const PLAYER_SIZE: f32 = 32.0;
pub(crate) const PLAYER_SPEED: f32 = 320.0;
pub(crate) const JUMP_FORCE: f32 = 680.0;
pub(crate) const GRAVITY: f32 = 1600.0;
pub(crate) const GROUND_Y: f32 = 568.0;

pub(crate) const BULLET_SPEED: f32 = 650.0;
pub(crate) const ENEMY_BULLET_SPEED: f32 = 380.0;
pub(crate) const SHOOT_INTERVAL: f32 = 0.22;
pub(crate) const MAX_LIVES: i32 = 3;
pub(crate) const INVINCIBLE_TIME: f32 = 2.0;

pub(crate) const MAX_PARTICLES: usize = 400;
pub(crate) const MAX_BULLETS: usize = 200;
pub(crate) const MAX_ENEMIES: usize = 25;

pub(crate) const WORLD_W: f32 = 800.0;
pub(crate) const WORLD_H: f32 = 600.0;
pub(crate) const LEVEL_W: f32 = 3200.0;

pub(crate) const CAM_DEAD_ZONE_X: f32 = 120.0;
pub(crate) const CAM_SMOOTH: f32 = 4.0;

// ── Utility ────────────────────────────────────────────────────────────────────
pub(crate) fn rand() -> f32 {
    js_sys::Math::random() as f32
}

pub(crate) fn rand_range(min: f32, max: f32) -> f32 {
    min + rand() * (max - min)
}

fn particle_color(idx: u8) -> Color {
    match idx % 7 {
        0 => Color::new(1.0, 0.9, 0.2, 1.0),
        1 => Color::new(1.0, 0.5, 0.1, 1.0),
        2 => Color::new(1.0, 0.2, 0.1, 1.0),
        3 => Color::new(1.0, 0.6, 0.8, 1.0),
        4 => Color::new(0.9, 0.4, 1.0, 1.0),
        5 => Color::new(0.4, 0.8, 1.0, 1.0),
        _ => Color::new(1.0, 1.0, 1.0, 1.0),
    }
}

// ── Static game reference for WASM exports ─────────────────────────────────────
thread_local! {
    static GAME_REF: RefCell<Option<Rc<RefCell<ScpGame>>>> = RefCell::new(None);
}

// ── ECS World Setup ────────────────────────────────────────────────────────────
fn init_world(world: &mut World) {
    world.insert_resource(GameStateRes::default());
    world.insert_resource(ScoreRes::default());
    world.insert_resource(LivesRes::default());
    world.insert_resource(CameraRes::default());
    world.insert_resource(SpawnRes::default());
    world.insert_resource(InputState::default());

    // Spawn player entity
    world.spawn()
        .with(Player {
            x: 100.0,
            y: GROUND_Y - PLAYER_SIZE,
            vy: 0.0,
            on_ground: true,
            facing_right: true,
            invincible: 0.0,
            flash: 0.0,
            shoot_timer: 0.0,
        })
        .build();
}

// ── Main Game Struct ───────────────────────────────────────────────────────────
struct ScpGame {
    gl: GlBackend,
    shapes: ShapeRenderer,
    input: InputManager,
    time: Time,
    world: World,
    scheduler: SystemScheduler,
}

impl ScpGame {
    fn new() -> Result<Self, String> {
        opengame_engine::log::init();

        let gl = GlBackend::new("game-canvas")?;
        let shapes = ShapeRenderer::new(gl.gl())?;
        let input = InputManager::new()?;

        let window = web_sys::window().ok_or("No window")?;
        let performance = window.performance().ok_or("No performance")?;
        let time = Time::new(performance);

        let mut world = World::new();
        init_world(&mut world);

        let mut scheduler = SystemScheduler::new();
        systems::register_systems(&mut scheduler);

        Ok(Self {
            gl,
            shapes,
            input,
            time,
            world,
            scheduler,
        })
    }

    fn reset_game(&mut self) {
        // Save high score
        let high = self.world.get_resource::<ScoreRes>()
            .map(|s| s.high_score.max(s.score))
            .unwrap_or(0);

        // Clear all entities and resources, re-init
        self.world.clear();
        init_world(&mut self.world);

        // Restore high score
        if let Some(score) = self.world.get_resource_mut::<ScoreRes>() {
            score.high_score = high;
        }
    }

    // ── Input ──────────────────────────────────────────────────────────────────
    fn poll_input(&mut self) {
        let state = self.world.get_resource::<GameStateRes>().unwrap();
        let gs = state.state;

        let input_state = InputState {
            left: self.input.is_key_down(KeyCode::KeyA) || self.input.is_key_down(KeyCode::ArrowLeft),
            right: self.input.is_key_down(KeyCode::KeyD) || self.input.is_key_down(KeyCode::ArrowRight),
            jump_pressed: self.input.is_key_pressed(KeyCode::Space)
                || self.input.is_key_pressed(KeyCode::KeyW)
                || self.input.is_key_pressed(KeyCode::ArrowUp),
            shoot_down: self.input.is_key_down(KeyCode::KeyJ) || self.input.is_key_down(KeyCode::KeyZ),
            start_pressed: self.input.is_key_pressed(KeyCode::Enter)
                || self.input.is_key_pressed(KeyCode::Space),
        };
        self.world.insert_resource(input_state);

        // Handle state transitions in main loop (before systems)
        match gs {
            GameState::Title => {
                if self.world.get_resource::<InputState>().unwrap().start_pressed {
                    self.world.get_resource_mut::<GameStateRes>().unwrap().state = GameState::Playing;
                    self.reset_game();
                }
            }
            GameState::GameOver => {
                let timer = self.world.get_resource::<GameStateRes>().unwrap().game_over_timer;
                if timer > 1.5 && self.world.get_resource::<InputState>().unwrap().start_pressed {
                    self.world.get_resource_mut::<GameStateRes>().unwrap().state = GameState::Playing;
                    self.reset_game();
                }
            }
            GameState::Playing => {}
        }
    }

    // ── Update ─────────────────────────────────────────────────────────────────
    fn update(&mut self, dt: f32) {
        // Update title/game-over timers
        {
            let gs = self.world.get_resource_mut::<GameStateRes>().unwrap();
            match gs.state {
                GameState::Title => { gs.title_pulse += dt * 2.0; }
                GameState::GameOver => { gs.game_over_timer += dt; }
                GameState::Playing => {}
            }
        }

        // Run all ECS systems
        let state = self.world.get_resource::<GameStateRes>().unwrap().state;
        if state == GameState::Playing {
            self.scheduler.run_update(&mut self.world, dt);
        } else if state == GameState::GameOver {
            // Still update particles and camera during game over
            self.scheduler.run_update(&mut self.world, dt);
        }
    }

    // ── Rendering ──────────────────────────────────────────────────────────────
    fn render(&mut self, _alpha: f32) {
        self.gl.resize();

        // Dark blue background
        self.gl.clear(0.04, 0.06, 0.18, 1.0);
        self.gl.enable_blend();

        // Camera projection
        let cam = self.world.get_resource::<CameraRes>().unwrap();
        let cam_x = cam.camera_x;
        let shake_amount = cam.shake_amount;
        let shake_x = if shake_amount > 0.0 { (rand() - 0.5) * shake_amount * 2.0 } else { 0.0 };
        let shake_y = if shake_amount > 0.0 { (rand() - 0.5) * shake_amount * 2.0 } else { 0.0 };
        let projection = Mat4::orthographic_rh_gl(cam_x, cam_x + WORLD_W, WORLD_H, 0.0, -1.0, 1.0);
        let view = Mat4::from_translation(Vec3::new(-shake_x, -shake_y, 0.0));
        let vp = projection * view;

        self.shapes.begin();

        // Background stars
        self.shapes.set_color(Color::new(0.6, 0.6, 0.8, 0.3));
        let cam_left = cam_x;
        let cam_right = cam_x + WORLD_W;
        for i in 0..60 {
            let sx = (i as f32 * 137.5) % LEVEL_W;
            if sx >= cam_left - 5.0 && sx <= cam_right + 5.0 {
                let sy = (i as f32 * 73.1 + 20.0) % (GROUND_Y - 40.0);
                self.shapes.draw_rect(sx, sy, 2.0, 2.0);
            }
        }

        self.render_ground(cam_left, cam_right);

        let gs = self.world.get_resource::<GameStateRes>().unwrap().state;
        match gs {
            GameState::Title => self.render_title(),
            GameState::Playing => {
                self.render_particles();
                self.render_bullets();
                self.render_enemies();
                self.render_player();
                self.render_hud();
            }
            GameState::GameOver => {
                self.render_particles();
                self.render_bullets();
                self.render_enemies();
                self.render_game_over();
            }
        }

        self.shapes.flush(self.gl.gl(), &vp);
    }

    fn render_ground(&mut self, cam_left: f32, cam_right: f32) {
        self.shapes.set_color(Color::new(0.18, 0.20, 0.24, 1.0));
        self.shapes.draw_rect(0.0, GROUND_Y, LEVEL_W, WORLD_H - GROUND_Y);

        self.shapes.set_color(Color::new(0.30, 0.34, 0.40, 1.0));
        self.shapes.draw_rect(0.0, GROUND_Y, LEVEL_W, 3.0);

        self.shapes.set_color(Color::new(0.22, 0.25, 0.30, 1.0));
        let start = ((cam_left / 42.0).floor() * 42.0 + 10.0).max(0.0);
        let mut gx = start;
        while gx <= cam_right && gx <= LEVEL_W {
            self.shapes.draw_rect(gx, GROUND_Y + 4.0, 1.0, WORLD_H - GROUND_Y - 4.0);
            gx += 42.0;
        }
    }

    fn render_player(&mut self) {
        let query = QuerySingle::<Player>::new(&self.world);
        let player = match query {
            Some(q) => {
                match q.iter().next() {
                    Some((_e, p)) => (p.x, p.y, p.invincible, p.flash),
                    None => return,
                }
            }
            None => return,
        };
        let (px, py, invincible, flash) = player;

        if invincible > 0.0 && (flash * 0.5).sin() > 0.3 {
            return;
        }

        let t = self.time.elapsed();
        let breathe = (t * 3.0).sin() * 0.5 + 0.5;
        let glow_expand = 3.0 + breathe * 4.0;
        let glow_alpha = 0.08 + breathe * 0.10;

        self.shapes.set_color(Color::new(0.0, 0.95, 1.0, glow_alpha));
        self.shapes.draw_rect(px - glow_expand, py - glow_expand, PLAYER_SIZE + glow_expand * 2.0, PLAYER_SIZE + glow_expand * 2.0);

        self.shapes.set_color(Color::new(0.0, 0.95, 1.0, 1.0));
        self.shapes.draw_rect(px, py, PLAYER_SIZE, PLAYER_SIZE);
    }

    fn render_enemies(&mut self) {
        let t = self.time.elapsed();
        let query = QuerySingle::<Enemy>::new(&self.world);
        if let Some(q) = query {
            for (_e, enemy) in q.iter() {
                if !enemy.alive { continue; }
                let s = enemy.size;
                let phase = (enemy.x * 0.05 + t * 2.8).sin() * 0.5 + 0.5;
                let glow_expand = 2.0 + phase * 3.0;
                let glow_alpha = 0.06 + phase * 0.08;

                self.shapes.set_color(Color::new(1.0, 0.45, 0.0, glow_alpha));
                self.shapes.draw_rect(enemy.x - glow_expand, enemy.y - glow_expand, s + glow_expand * 2.0, s + glow_expand * 2.0);

                let c = if enemy.flash > 0.0 {
                    Color::lerp(Color::new(1.0, 0.45, 0.0, 1.0), Color::WHITE, enemy.flash * 0.7)
                } else {
                    Color::new(1.0, 0.45, 0.0, 1.0)
                };
                self.shapes.set_color(c);
                self.shapes.draw_rect(enemy.x, enemy.y, s, s);
            }
        }
    }

    fn render_bullets(&mut self) {
        let query = QuerySingle::<Bullet>::new(&self.world);
        if let Some(q) = query {
            for (_e, bullet) in q.iter() {
                if !bullet.alive { continue; }
                if bullet.is_player {
                    self.shapes.set_color(Color::new(0.3, 0.85, 1.0, 0.25));
                    self.shapes.draw_rect(bullet.x - 6.0, bullet.y - 5.0, 16.0, 10.0);
                    self.shapes.set_color(Color::new(0.3, 0.9, 1.0, 1.0));
                    self.shapes.draw_rect(bullet.x - 3.0, bullet.y - 2.0, 10.0, 4.0);
                    self.shapes.set_color(Color::new(0.8, 1.0, 1.0, 1.0));
                    self.shapes.draw_rect(bullet.x - 1.0, bullet.y - 1.0, 6.0, 2.0);
                } else {
                    self.shapes.set_color(Color::new(1.0, 0.3, 0.2, 0.25));
                    self.shapes.draw_rect(bullet.x - 5.0, bullet.y - 4.0, 12.0, 8.0);
                    self.shapes.set_color(Color::new(1.0, 0.35, 0.2, 1.0));
                    self.shapes.draw_rect(bullet.x - 2.0, bullet.y - 2.0, 7.0, 4.0);
                    self.shapes.set_color(Color::new(1.0, 0.7, 0.5, 1.0));
                    self.shapes.draw_rect(bullet.x - 0.5, bullet.y - 1.0, 4.0, 2.0);
                }
            }
        }
    }

    fn render_particles(&mut self) {
        let query = QuerySingle::<Particle>::new(&self.world);
        if let Some(q) = query {
            for (_e, p) in q.iter() {
                if p.life <= 0.0 { continue; }
                let t = p.life / p.max_life;
                let alpha = t * t;
                let size = p.size * (0.3 + t * 0.7);
                let c = particle_color(p.color_idx).with_alpha(alpha);
                self.shapes.set_color(c);
                self.shapes.draw_rect(p.x - size * 0.5, p.y - size * 0.5, size, size);
            }
        }
    }

    fn render_hud(&mut self) {
        let score = self.world.get_resource::<ScoreRes>().unwrap().score;
        let lives = self.world.get_resource::<LivesRes>().unwrap().lives;

        let score_bars = (score / 50).min(40) as f32;
        self.shapes.set_color(Color::new(0.0, 0.0, 0.0, 0.3));
        self.shapes.draw_rect(10.0, 10.0, 204.0, 16.0);
        self.shapes.set_color(Color::new(0.3, 0.9, 0.4, 0.9));
        self.shapes.draw_rect(12.0, 12.0, score_bars * 5.0, 12.0);

        for i in 0..lives {
            let lx = 16.0 + i as f32 * 24.0;
            let ly = 36.0;
            self.shapes.set_color(Color::new(1.0, 0.85, 0.2, 0.9));
            self.shapes.draw_rect(lx, ly, 16.0, 16.0);
            self.shapes.set_color(Color::new(0.8, 0.65, 0.1, 0.9));
            self.shapes.draw_rect(lx, ly, 16.0, 2.0);
            self.shapes.draw_rect(lx, ly + 14.0, 16.0, 2.0);
        }
    }

    fn render_title(&mut self) {
        let gs = self.world.get_resource::<GameStateRes>().unwrap();
        let pulse = (gs.title_pulse).sin() * 0.15 + 0.85;
        let high_score = self.world.get_resource::<ScoreRes>().unwrap().high_score;

        let cx = WORLD_W * 0.5;
        let ty = WORLD_H * 0.28;

        // "S"
        self.shapes.set_color(Color::new(0.3, 0.7, 1.0, pulse));
        self.shapes.draw_rect(cx - 100.0, ty, 30.0, 8.0);
        self.shapes.draw_rect(cx - 108.0, ty + 8.0, 8.0, 16.0);
        self.shapes.draw_rect(cx - 100.0, ty + 24.0, 30.0, 8.0);
        self.shapes.draw_rect(cx - 78.0, ty + 32.0, 8.0, 16.0);
        self.shapes.draw_rect(cx - 100.0, ty + 48.0, 30.0, 8.0);

        // "C"
        self.shapes.draw_rect(cx - 40.0, ty, 30.0, 8.0);
        self.shapes.draw_rect(cx - 48.0, ty + 8.0, 8.0, 40.0);
        self.shapes.draw_rect(cx - 40.0, ty + 48.0, 30.0, 8.0);

        // "P"
        self.shapes.draw_rect(cx + 10.0, ty, 30.0, 8.0);
        self.shapes.draw_rect(cx + 2.0, ty + 8.0, 8.0, 48.0);
        self.shapes.draw_rect(cx + 40.0, ty + 8.0, 8.0, 20.0);
        self.shapes.draw_rect(cx + 10.0, ty + 28.0, 30.0, 8.0);

        // "SHOOTER" subtitle
        self.shapes.set_color(Color::new(1.0, 0.4, 0.2, pulse * 0.8));
        self.shapes.draw_rect(cx - 70.0, ty + 72.0, 140.0, 6.0);
        self.shapes.set_color(Color::new(1.0, 0.6, 0.3, pulse * 0.6));
        self.shapes.draw_rect(cx - 50.0, ty + 82.0, 100.0, 4.0);

        // Floating enemies decoration
        let title_pulse = self.world.get_resource::<GameStateRes>().unwrap().title_pulse;
        for i in 0..5 {
            let angle = title_pulse * 0.8 + i as f32 * std::f32::consts::TAU / 5.0;
            let radius = 80.0 + (title_pulse * 0.7 + i as f32).sin() * 15.0;
            let dx = cx + angle.cos() * radius;
            let dy = WORLD_H * 0.55 + angle.sin() * radius * 0.35;
            self.shapes.set_color(Color::new(1.0, 0.45, 0.0, 0.5 * pulse));
            self.shapes.draw_rect(dx - 8.0, dy - 8.0, 16.0, 16.0);
        }

        // Blink
        let blink = (title_pulse * 1.5).sin();
        if blink > -0.3 {
            self.shapes.set_color(Color::new(0.9, 0.9, 1.0, 0.5 + blink * 0.4));
            self.shapes.draw_rect(cx - 90.0, WORLD_H * 0.72, 180.0, 4.0);
        }

        // Controls hint
        self.shapes.set_color(Color::new(0.5, 0.5, 0.6, 0.5));
        self.shapes.draw_rect(cx - 70.0, WORLD_H * 0.80, 140.0, 2.0);
        self.shapes.draw_rect(cx - 70.0, WORLD_H * 0.80 + 18.0, 140.0, 2.0);
        self.shapes.draw_rect(cx - 70.0, WORLD_H * 0.80 + 36.0, 140.0, 2.0);

        if high_score > 0 {
            self.shapes.set_color(Color::new(1.0, 0.85, 0.3, 0.7));
            self.shapes.draw_rect(cx - 40.0, WORLD_H * 0.80 + 54.0, 80.0, 2.0);
        }
    }

    fn render_game_over(&mut self) {
        let gs = self.world.get_resource::<GameStateRes>().unwrap();
        let alpha = (gs.game_over_timer / 0.5).min(1.0);
        let score = self.world.get_resource::<ScoreRes>().unwrap().score;
        let high_score = self.world.get_resource::<ScoreRes>().unwrap().high_score;
        let cx = WORLD_W * 0.5;

        self.shapes.set_color(Color::new(0.0, 0.0, 0.0, 0.55 * alpha));
        self.shapes.draw_rect(0.0, 0.0, WORLD_W, WORLD_H);

        let center_y = WORLD_H * 0.35;

        self.shapes.set_color(Color::new(1.0, 0.2, 0.2, alpha * 0.95));
        self.shapes.draw_rect(cx - 90.0, center_y - 10.0, 180.0, 20.0);
        self.shapes.set_color(Color::new(0.8, 0.1, 0.1, alpha * 0.7));
        self.shapes.draw_rect(cx - 70.0, center_y + 14.0, 140.0, 10.0);

        let score_bars = (score / 50).min(40) as f32;
        self.shapes.set_color(Color::new(0.2, 0.2, 0.25, alpha * 0.8));
        self.shapes.draw_rect(cx - 60.0, center_y + 50.0, 120.0, 10.0);
        self.shapes.set_color(Color::new(0.3, 0.9, 0.4, alpha * 0.9));
        self.shapes.draw_rect(cx - 58.0, center_y + 52.0, score_bars * 2.9, 6.0);

        if score >= high_score && score > 0 {
            self.shapes.set_color(Color::new(1.0, 0.85, 0.3, alpha * 0.9));
            self.shapes.draw_rect(cx - 30.0, center_y + 70.0, 60.0, 3.0);
        }

        let game_over_timer = self.world.get_resource::<GameStateRes>().unwrap().game_over_timer;
        if game_over_timer > 1.5 {
            let blink = (game_over_timer * 3.0).sin();
            if blink > 0.0 {
                self.shapes.set_color(Color::new(0.9, 0.9, 1.0, alpha * 0.6 * blink));
                self.shapes.draw_rect(cx - 60.0, center_y + 95.0, 120.0, 4.0);
            }
        }
    }
}

// ── WASM Exports for Vue ───────────────────────────────────────────────────────
#[wasm_bindgen]
pub fn get_score() -> i32 {
    GAME_REF.with(|g| {
        if let Some(ref game) = *g.borrow() {
            game.borrow().world.get_resource::<ScoreRes>()
                .map(|s| s.score).unwrap_or(0)
        } else { 0 }
    })
}

#[wasm_bindgen]
pub fn get_lives() -> i32 {
    GAME_REF.with(|g| {
        if let Some(ref game) = *g.borrow() {
            game.borrow().world.get_resource::<LivesRes>()
                .map(|l| l.lives).unwrap_or(0)
        } else { 0 }
    })
}

#[wasm_bindgen]
pub fn get_game_state() -> u8 {
    GAME_REF.with(|g| {
        if let Some(ref game) = *g.borrow() {
            match game.borrow().world.get_resource::<GameStateRes>() {
                Some(gs) => match gs.state {
                    GameState::Title => 0,
                    GameState::Playing => 1,
                    GameState::GameOver => 2,
                },
                None => 0,
            }
        } else { 0 }
    })
}

#[wasm_bindgen]
pub fn get_high_score() -> i32 {
    GAME_REF.with(|g| {
        if let Some(ref game) = *g.borrow() {
            game.borrow().world.get_resource::<ScoreRes>()
                .map(|s| s.high_score).unwrap_or(0)
        } else { 0 }
    })
}

#[wasm_bindgen]
pub fn start_game() {
    GAME_REF.with(|g| {
        if let Some(ref game) = *g.borrow() {
            let mut game = game.borrow_mut();
            let current = game.world.get_resource::<GameStateRes>()
                .map(|gs| gs.state).unwrap_or(GameState::Title);
            if current == GameState::Title {
                game.world.get_resource_mut::<GameStateRes>().unwrap().state = GameState::Playing;
                game.reset_game();
            }
        }
    })
}

#[wasm_bindgen]
pub fn restart_game() {
    GAME_REF.with(|g| {
        if let Some(ref game) = *g.borrow() {
            let mut game = game.borrow_mut();
            let current = game.world.get_resource::<GameStateRes>()
                .map(|gs| gs.state).unwrap_or(GameState::GameOver);
            if current == GameState::GameOver {
                game.world.get_resource_mut::<GameStateRes>().unwrap().state = GameState::Playing;
                game.reset_game();
            }
        }
    })
}

// ── Entry Point ────────────────────────────────────────────────────────────────
#[wasm_bindgen(start)]
pub fn main() {
    let mut game = ScpGame::new().expect("Failed to create SCP Game");
    game.time.init();

    let game = Rc::new(RefCell::new(game));
    GAME_REF.with(|g| { *g.borrow_mut() = Some(game.clone()); });

    let f: AnimationFrameClosure = Rc::new(RefCell::new(None));
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
        game.poll_input();
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
