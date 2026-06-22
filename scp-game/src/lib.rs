use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

type AnimationFrameClosure = Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>>;

use opengame_engine::color::Color;
use opengame_engine::input::{keys::KeyCode, InputManager};
use opengame_engine::math::{Mat4, Vec3};
use opengame_engine::renderer::{GlBackend, ShapeRenderer};
use opengame_engine::time::Time;

// ── Constants ──────────────────────────────────────────────────────────────────
const PLAYER_SIZE: f32 = 32.0;
const PLAYER_SPEED: f32 = 320.0;
const JUMP_FORCE: f32 = 680.0;
const GRAVITY: f32 = 1600.0;
const GROUND_Y: f32 = 568.0;

const BULLET_SPEED: f32 = 650.0;
const ENEMY_BULLET_SPEED: f32 = 380.0;
const SHOOT_INTERVAL: f32 = 0.22;
const MAX_LIVES: i32 = 3;
const INVINCIBLE_TIME: f32 = 2.0;

const MAX_PARTICLES: usize = 400;
const MAX_BULLETS: usize = 200;
const MAX_ENEMIES: usize = 25;

const WORLD_W: f32 = 800.0;
const WORLD_H: f32 = 600.0;
const LEVEL_W: f32 = 3200.0; // total level width (camera scrolls horizontally)

// Camera
const CAM_DEAD_ZONE_X: f32 = 120.0; // half-width of horizontal dead zone
const CAM_SMOOTH: f32 = 4.0;        // lerp speed (higher = snappier)

// ── Game State ─────────────────────────────────────────────────────────────────
#[derive(Clone, Copy, PartialEq)]
enum GameState {
    Title,
    Playing,
    GameOver,
}

// ── Components ─────────────────────────────────────────────────────────────────
struct Bullet {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    alive: bool,
    is_player: bool,
}

struct Enemy {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    hp: i32,
    alive: bool,
    on_ground: bool,
    shoot_timer: f32,
    ai_timer: f32,
    flash: f32,
    size: f32,
}

struct Particle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    life: f32,
    max_life: f32,
    size: f32,
    color_idx: u8,
}

// ── Utility ────────────────────────────────────────────────────────────────────
fn rand() -> f32 {
    js_sys::Math::random() as f32
}

fn rand_range(min: f32, max: f32) -> f32 {
    min + rand() * (max - min)
}

fn particle_color(idx: u8) -> Color {
    match idx % 7 {
        0 => Color::new(1.0, 0.9, 0.2, 1.0),  // yellow
        1 => Color::new(1.0, 0.5, 0.1, 1.0),  // orange
        2 => Color::new(1.0, 0.2, 0.1, 1.0),  // red
        3 => Color::new(1.0, 0.6, 0.8, 1.0),  // pink
        4 => Color::new(0.9, 0.4, 1.0, 1.0),  // purple
        5 => Color::new(0.4, 0.8, 1.0, 1.0),  // cyan
        _ => Color::new(1.0, 1.0, 1.0, 1.0),  // white
    }
}

// ── Static game reference for WASM exports ─────────────────────────────────────
thread_local! {
    static GAME_REF: RefCell<Option<Rc<RefCell<ScpGame>>>> = RefCell::new(None);
}

// ── Main Game Struct ───────────────────────────────────────────────────────────
struct ScpGame {
    gl: GlBackend,
    shapes: ShapeRenderer,
    input: InputManager,
    time: Time,

    state: GameState,

    // Player
    player_x: f32,
    player_y: f32,
    player_vy: f32,
    on_ground: bool,
    lives: i32,
    invincible: f32,
    flash: f32,
    shoot_timer: f32,
    facing_right: bool,

    // Entities
    bullets: Vec<Bullet>,
    enemies: Vec<Enemy>,
    particles: Vec<Particle>,

    // Scoring
    score: i32,
    high_score: i32,

    // Spawning
    spawn_timer: f32,
    spawn_interval: f32,
    difficulty_timer: f32,

    // Camera
    camera_x: f32,
    camera_y: f32,

    // Effects
    shake_amount: f32,

    // Title
    title_pulse: f32,
    game_over_timer: f32,
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

        Ok(Self {
            gl,
            shapes,
            input,
            time,
            state: GameState::Title,
            player_x: 100.0,
            player_y: GROUND_Y - PLAYER_SIZE,
            player_vy: 0.0,
            on_ground: true,
            lives: MAX_LIVES,
            invincible: 0.0,
            flash: 0.0,
            shoot_timer: 0.0,
            facing_right: true,
            bullets: Vec::with_capacity(MAX_BULLETS),
            enemies: Vec::with_capacity(MAX_ENEMIES),
            particles: Vec::with_capacity(MAX_PARTICLES),
            score: 0,
            high_score: 0,
            spawn_timer: 1.5,
            spawn_interval: 2.0,
            difficulty_timer: 0.0,
            camera_x: 0.0,
            camera_y: 0.0,
            shake_amount: 0.0,
            title_pulse: 0.0,
            game_over_timer: 0.0,
        })
    }

    // ── Spawning ───────────────────────────────────────────────────────────────
    fn spawn_particle(x: f32, y: f32, vx: f32, vy: f32, life: f32, size: f32, ci: u8) -> Particle {
        Particle { x, y, vx, vy, life, max_life: life, size, color_idx: ci }
    }

    fn spawn_explosion(x: f32, y: f32, count: usize, power: f32) -> Vec<Particle> {
        let mut v = Vec::with_capacity(count);
        for i in 0..count {
            let angle = rand() * std::f32::consts::TAU;
            let speed = rand_range(60.0, power);
            v.push(Self::spawn_particle(
                x, y,
                angle.cos() * speed,
                angle.sin() * speed,
                rand_range(0.3, 0.9),
                rand_range(3.0, 7.0),
                (i % 7) as u8,
            ));
        }
        v
    }

    fn spawn_bullet_shatter(x: f32, y: f32) -> Vec<Particle> {
        let mut v = Vec::with_capacity(5);
        for i in 0..5 {
            let angle = rand() * std::f32::consts::TAU;
            let speed = rand_range(40.0, 120.0);
            v.push(Self::spawn_particle(
                x, y,
                angle.cos() * speed,
                angle.sin() * speed,
                rand_range(0.15, 0.35),
                rand_range(1.5, 3.0),
                (i % 7) as u8,
            ));
        }
        v
    }

    fn spawn_enemy(&mut self) {
        if self.enemies.len() >= MAX_ENEMIES {
            return;
        }
        let size = rand_range(24.0, 34.0);
        // Spawn just off the left edge of the camera view
        let spawn_x = self.camera_x - size;
        self.enemies.push(Enemy {
            x: spawn_x,
            y: GROUND_Y - size,
            vx: rand_range(60.0, 130.0),
            vy: 0.0,
            hp: 2,
            alive: true,
            on_ground: true,
            shoot_timer: rand_range(0.8, 2.0),
            ai_timer: rand_range(0.5, 1.5),
            flash: 0.0,
            size,
        });
    }

    // ── Reset ──────────────────────────────────────────────────────────────────
    fn reset_game(&mut self) {
        if self.score > self.high_score {
            self.high_score = self.score;
        }
        self.player_x = 100.0;
        self.player_y = GROUND_Y - PLAYER_SIZE;
        self.player_vy = 0.0;
        self.on_ground = true;
        self.lives = MAX_LIVES;
        self.invincible = 0.0;
        self.flash = 0.0;
        self.shoot_timer = 0.0;
        self.facing_right = true;
        self.bullets.clear();
        self.enemies.clear();
        self.particles.clear();
        self.score = 0;
        self.spawn_timer = 1.5;
        self.spawn_interval = 2.0;
        self.difficulty_timer = 0.0;
        self.camera_x = 0.0;
        self.camera_y = 0.0;
        self.shake_amount = 0.0;
    }

    // ── Player Shoot ───────────────────────────────────────────────────────────
    fn player_shoot(&mut self) {
        if self.shoot_timer > 0.0 {
            return;
        }
        self.shoot_timer = SHOOT_INTERVAL;

        let bx = if self.facing_right {
            self.player_x + PLAYER_SIZE
        } else {
            self.player_x - 8.0
        };
        let bvx = if self.facing_right { BULLET_SPEED } else { -BULLET_SPEED };

        if self.bullets.len() < MAX_BULLETS {
            self.bullets.push(Bullet {
                x: bx,
                y: self.player_y + PLAYER_SIZE * 0.4,
                vx: bvx,
                vy: 0.0,
                alive: true,
                is_player: true,
            });
        }
    }

    // ── Player Hit ─────────────────────────────────────────────────────────────
    fn do_player_hit(&mut self) {
        self.lives -= 1;
        self.shake_amount = 14.0;

        let explosion = Self::spawn_explosion(self.player_x + PLAYER_SIZE * 0.5, self.player_y + PLAYER_SIZE * 0.5, 25, 220.0);
        for p in explosion {
            if self.particles.len() < MAX_PARTICLES {
                self.particles.push(p);
            }
        }

        if self.lives <= 0 {
            self.state = GameState::GameOver;
            self.game_over_timer = 0.0;
            let death = Self::spawn_explosion(self.player_x + PLAYER_SIZE * 0.5, self.player_y + PLAYER_SIZE * 0.5, 45, 350.0);
            for p in death {
                if self.particles.len() < MAX_PARTICLES {
                    self.particles.push(p);
                }
            }
        } else {
            self.invincible = INVINCIBLE_TIME;
            self.flash = 0.0;
        }
    }

    // ── Update: Title ──────────────────────────────────────────────────────────
    fn update_title(&mut self, dt: f32) {
        self.title_pulse += dt * 2.0;
        if self.input.is_key_pressed(KeyCode::Enter) || self.input.is_key_pressed(KeyCode::Space) {
            self.state = GameState::Playing;
            self.reset_game();
        }
    }

    // ── Update: Playing ────────────────────────────────────────────────────────
    fn update_playing(&mut self, dt: f32) {
        // Shake decay
        if self.shake_amount > 0.0 {
            self.shake_amount = (self.shake_amount - 6.0 * dt).max(0.0);
        }

        // Difficulty ramp
        self.difficulty_timer += dt;
        if self.difficulty_timer > 10.0 {
            self.difficulty_timer = 0.0;
            self.spawn_interval = (self.spawn_interval * 0.88).max(0.4);
        }

        // ── Player movement ────────────────────────────────────────────────────
        let move_speed = PLAYER_SPEED * dt;
        if self.input.is_key_down(KeyCode::KeyA) || self.input.is_key_down(KeyCode::ArrowLeft) {
            self.player_x -= move_speed;
            self.facing_right = false;
        }
        if self.input.is_key_down(KeyCode::KeyD) || self.input.is_key_down(KeyCode::ArrowRight) {
            self.player_x += move_speed;
            self.facing_right = true;
        }
        self.player_x = self.player_x.clamp(0.0, LEVEL_W - PLAYER_SIZE);

        // Jump
        if (self.input.is_key_pressed(KeyCode::Space)
            || self.input.is_key_pressed(KeyCode::KeyW)
            || self.input.is_key_pressed(KeyCode::ArrowUp))
            && self.on_ground
        {
            self.player_vy = -JUMP_FORCE;
            self.on_ground = false;
        }

        // Gravity
        self.player_vy += GRAVITY * dt;
        self.player_y += self.player_vy * dt;

        // Ground collision
        if self.player_y >= GROUND_Y - PLAYER_SIZE {
            self.player_y = GROUND_Y - PLAYER_SIZE;
            self.player_vy = 0.0;
            self.on_ground = true;
        }

        // ── Shoot ──────────────────────────────────────────────────────────────
        self.shoot_timer = (self.shoot_timer - dt).max(0.0);
        if self.input.is_key_down(KeyCode::KeyJ) || self.input.is_key_down(KeyCode::KeyZ) {
            self.player_shoot();
        }

        // Invincibility
        if self.invincible > 0.0 {
            self.invincible -= dt;
            self.flash += dt * 15.0;
        }

        // ── Enemy spawning ─────────────────────────────────────────────────────
        self.spawn_timer -= dt;
        if self.spawn_timer <= 0.0 {
            self.spawn_enemy();
            self.spawn_timer = self.spawn_interval;
        }

        // ── Update enemies (gravity + AI) ───────────────────────────────────
        let player_x = self.player_x;
        let mut shoot_events: Vec<Bullet> = Vec::new();
        for enemy in &mut self.enemies {
            if !enemy.alive { continue; }
            enemy.flash = (enemy.flash - dt * 5.0).max(0.0);

            // AI timer
            enemy.ai_timer -= dt;
            if enemy.ai_timer <= 0.0 {
                enemy.ai_timer = rand_range(0.4, 1.0);
                let dx = player_x - enemy.x;
                let dist = dx.abs();

                // Chase player: move toward player
                if dist > 60.0 {
                    enemy.vx = if dx > 0.0 { rand_range(70.0, 140.0) } else { -rand_range(70.0, 140.0) };
                } else {
                    // Close enough — stop and shoot
                    enemy.vx = 0.0;
                }

                // Jump if player is above and enemy is on ground
                if self.player_y < enemy.y - 40.0 && enemy.on_ground && rand() < 0.35 {
                    enemy.vy = -480.0;
                    enemy.on_ground = false;
                }
            }

            // Gravity
            enemy.vy += GRAVITY * dt;
            enemy.x += enemy.vx * dt;
            enemy.y += enemy.vy * dt;

            // Ground collision
            if enemy.y >= GROUND_Y - enemy.size {
                enemy.y = GROUND_Y - enemy.size;
                enemy.vy = 0.0;
                enemy.on_ground = true;
            }

            // Keep in bounds horizontally
            enemy.x = enemy.x.clamp(0.0, LEVEL_W - enemy.size);

            // Enemy shooting — aim at player
            enemy.shoot_timer -= dt;
            if enemy.shoot_timer <= 0.0 {
                enemy.shoot_timer = rand_range(1.2, 2.8);
                let bvx: f32;
                let bvy: f32;
                let dx = player_x + PLAYER_SIZE * 0.5 - enemy.x;
                let dy = self.player_y + PLAYER_SIZE * 0.5 - enemy.y;
                let dist = (dx * dx + dy * dy).sqrt();
                if dist > 1.0 {
                    let speed = ENEMY_BULLET_SPEED;
                    bvx = dx / dist * speed;
                    bvy = dy / dist * speed;
                } else {
                    bvx = -ENEMY_BULLET_SPEED;
                    bvy = 0.0;
                }
                if self.bullets.len() < MAX_BULLETS {
                    shoot_events.push(Bullet {
                        x: enemy.x + enemy.size * 0.5,
                        y: enemy.y + enemy.size * 0.5,
                        vx: bvx,
                        vy: bvy,
                        alive: true,
                        is_player: false,
                    });
                }
            }

            // Despawn if far behind camera
            if enemy.x < self.camera_x - 200.0 {
                enemy.alive = false;
            }
        }
        for b in shoot_events {
            if self.bullets.len() < MAX_BULLETS {
                self.bullets.push(b);
            }
        }

        // ── Update bullets ─────────────────────────────────────────────────────
        for bullet in &mut self.bullets {
            if !bullet.alive { continue; }
            bullet.x += bullet.vx * dt;
            bullet.y += bullet.vy * dt;

            // Boundary shatter
            let mut shattered = false;
            if bullet.x < -10.0 || bullet.x > LEVEL_W + 10.0 || bullet.y < -10.0 || bullet.y > WORLD_H + 10.0 {
                bullet.alive = false;
                shattered = true;
            }
            if shattered {
                let shatter = Self::spawn_bullet_shatter(
                    bullet.x.clamp(0.0, LEVEL_W),
                    bullet.y.clamp(0.0, WORLD_H),
                );
                for p in shatter {
                    if self.particles.len() < MAX_PARTICLES {
                        self.particles.push(p);
                    }
                }
            }
        }

        // ── Update particles ───────────────────────────────────────────────────
        for p in &mut self.particles {
            if p.life <= 0.0 { continue; }
            p.x += p.vx * dt;
            p.y += p.vy * dt;
            p.vx *= 0.98;
            p.vy *= 0.98;
            p.life -= dt;
        }

        // ── Collisions ─────────────────────────────────────────────────────────
        self.process_collisions();

        // ── Cleanup ────────────────────────────────────────────────────────────
        self.bullets.retain(|b| b.alive);
        self.enemies.retain(|e| e.alive);
        self.particles.retain(|p| p.life > 0.0);
    }

    fn process_collisions(&mut self) {
        let px = self.player_x;
        let py = self.player_y;
        let pw = PLAYER_SIZE;
        let ph = PLAYER_SIZE;

        // Player bullets vs enemies
        for bullet in &mut self.bullets {
            if !bullet.alive || !bullet.is_player { continue; }
            let bx = bullet.x;
            let by = bullet.y;

            for enemy in &mut self.enemies {
                if !enemy.alive { continue; }
                let es = enemy.size * 0.5;

                // AABB check
                if bx + 4.0 > enemy.x - es
                    && bx - 4.0 < enemy.x + es
                    && by + 3.0 > enemy.y - es
                    && by - 3.0 < enemy.y + es
                {
                    bullet.alive = false;
                    enemy.hp -= 1;
                    enemy.flash = 1.0;

                    if enemy.hp <= 0 {
                        enemy.alive = false;
                        let points = 100;
                        self.score += points;

                        // Large geometric explosion
                        let explosion = Self::spawn_explosion(enemy.x, enemy.y, 22, 250.0);
                        for p in explosion {
                            if self.particles.len() < MAX_PARTICLES {
                                self.particles.push(p);
                            }
                        }
                        self.shake_amount = (self.shake_amount + 6.0).min(18.0);
                    } else {
                        // Hit spark
                        let spark = Self::spawn_explosion(bx, by, 4, 80.0);
                        for p in spark {
                            if self.particles.len() < MAX_PARTICLES {
                                self.particles.push(p);
                            }
                        }
                    }
                    break;
                }
            }
        }

        // Enemy bullets vs player
        if self.invincible <= 0.0 {
            for bullet in &mut self.bullets {
                if !bullet.alive || bullet.is_player { continue; }
                let bx = bullet.x;
                let by = bullet.y;

                if bx + 3.0 > px && bx - 3.0 < px + pw && by + 3.0 > py && by - 3.0 < py + ph {
                    bullet.alive = false;
                    self.do_player_hit();
                    break;
                }
            }
        }

        // Enemies vs player (contact damage)
        if self.invincible <= 0.0 {
            for enemy in &mut self.enemies {
                if !enemy.alive { continue; }
                let es = enemy.size * 0.5;

                if px + pw > enemy.x - es
                    && px < enemy.x + es
                    && py + ph > enemy.y - es
                    && py < enemy.y + es
                {
                    enemy.alive = false;
                    let explosion = Self::spawn_explosion(enemy.x, enemy.y, 18, 200.0);
                    for p in explosion {
                        if self.particles.len() < MAX_PARTICLES {
                            self.particles.push(p);
                        }
                    }
                    self.do_player_hit();
                    break;
                }
            }
        }
    }

    // ── Update: Game Over ──────────────────────────────────────────────────────
    fn update_game_over(&mut self, dt: f32) {
        self.game_over_timer += dt;
        for p in &mut self.particles {
            if p.life <= 0.0 { continue; }
            p.x += p.vx * dt;
            p.y += p.vy * dt;
            p.vx *= 0.98;
            p.vy *= 0.98;
            p.life -= dt;
        }
        self.particles.retain(|p| p.life > 0.0);

        if self.game_over_timer > 1.5
            && (self.input.is_key_pressed(KeyCode::Space) || self.input.is_key_pressed(KeyCode::Enter))
        {
            self.state = GameState::Playing;
            self.reset_game();
        }
    }

    fn update(&mut self, dt: f32) {
        match self.state {
            GameState::Title => self.update_title(dt),
            GameState::Playing => self.update_playing(dt),
            GameState::GameOver => self.update_game_over(dt),
        }
    }

    // ── Rendering ──────────────────────────────────────────────────────────────
    fn render_ground(&mut self) {
        // Ground surface — dark cool gray (full level width)
        self.shapes.set_color(Color::new(0.18, 0.20, 0.24, 1.0));
        self.shapes.draw_rect(0.0, GROUND_Y, LEVEL_W, WORLD_H - GROUND_Y);

        // Ground line highlight
        self.shapes.set_color(Color::new(0.30, 0.34, 0.40, 1.0));
        self.shapes.draw_rect(0.0, GROUND_Y, LEVEL_W, 3.0);

        // Subtle grid lines on ground (visible portion only)
        self.shapes.set_color(Color::new(0.22, 0.25, 0.30, 1.0));
        let cam_left = self.camera_x;
        let cam_right = self.camera_x + WORLD_W;
        let start = ((cam_left / 42.0).floor() * 42.0 + 10.0).max(0.0);
        let mut gx = start;
        while gx <= cam_right && gx <= LEVEL_W {
            self.shapes.draw_rect(gx, GROUND_Y + 4.0, 1.0, WORLD_H - GROUND_Y - 4.0);
            gx += 42.0;
        }
    }

    fn render_player(&mut self) {
        if self.invincible > 0.0 && (self.flash * 0.5).sin() > 0.3 {
            return;
        }
        let t = self.time.elapsed();
        let breathe = (t * 3.0).sin() * 0.5 + 0.5; // 0.0..1.0
        let glow_expand = 3.0 + breathe * 4.0;
        let glow_alpha = 0.08 + breathe * 0.10;

        // Glow
        self.shapes.set_color(Color::new(0.0, 0.95, 1.0, glow_alpha));
        self.shapes.draw_rect(
            self.player_x - glow_expand,
            self.player_y - glow_expand,
            PLAYER_SIZE + glow_expand * 2.0,
            PLAYER_SIZE + glow_expand * 2.0,
        );

        // Body
        self.shapes.set_color(Color::new(0.0, 0.95, 1.0, 1.0));
        self.shapes.draw_rect(self.player_x, self.player_y, PLAYER_SIZE, PLAYER_SIZE);
    }

    fn render_enemies(&mut self) {
        let t = self.time.elapsed();
        for enemy in &self.enemies {
            if !enemy.alive { continue; }
            let s = enemy.size;
            // Each enemy breathes at a slightly offset phase
            let phase = (enemy.x * 0.05 + t * 2.8).sin() * 0.5 + 0.5;
            let glow_expand = 2.0 + phase * 3.0;
            let glow_alpha = 0.06 + phase * 0.08;

            // Glow
            self.shapes.set_color(Color::new(1.0, 0.45, 0.0, glow_alpha));
            self.shapes.draw_rect(
                enemy.x - glow_expand,
                enemy.y - glow_expand,
                s + glow_expand * 2.0,
                s + glow_expand * 2.0,
            );

            // Body
            let c = if enemy.flash > 0.0 {
                Color::lerp(Color::new(1.0, 0.45, 0.0, 1.0), Color::WHITE, enemy.flash * 0.7)
            } else {
                Color::new(1.0, 0.45, 0.0, 1.0)
            };
            self.shapes.set_color(c);
            self.shapes.draw_rect(enemy.x, enemy.y, s, s);
        }
    }

    fn render_bullets(&mut self) {
        for bullet in &self.bullets {
            if !bullet.alive { continue; }
            if bullet.is_player {
                // Player bullet: cyan with glow
                self.shapes.set_color(Color::new(0.3, 0.85, 1.0, 0.25));
                self.shapes.draw_rect(bullet.x - 6.0, bullet.y - 5.0, 16.0, 10.0);
                self.shapes.set_color(Color::new(0.3, 0.9, 1.0, 1.0));
                self.shapes.draw_rect(bullet.x - 3.0, bullet.y - 2.0, 10.0, 4.0);
                self.shapes.set_color(Color::new(0.8, 1.0, 1.0, 1.0));
                self.shapes.draw_rect(bullet.x - 1.0, bullet.y - 1.0, 6.0, 2.0);
            } else {
                // Enemy bullet: red-orange
                self.shapes.set_color(Color::new(1.0, 0.3, 0.2, 0.25));
                self.shapes.draw_rect(bullet.x - 5.0, bullet.y - 4.0, 12.0, 8.0);
                self.shapes.set_color(Color::new(1.0, 0.35, 0.2, 1.0));
                self.shapes.draw_rect(bullet.x - 2.0, bullet.y - 2.0, 7.0, 4.0);
                self.shapes.set_color(Color::new(1.0, 0.7, 0.5, 1.0));
                self.shapes.draw_rect(bullet.x - 0.5, bullet.y - 1.0, 4.0, 2.0);
            }
        }
    }

    fn render_particles(&mut self) {
        for p in &self.particles {
            if p.life <= 0.0 { continue; }
            let t = p.life / p.max_life;
            let alpha = t * t;
            let size = p.size * (0.3 + t * 0.7);
            let c = particle_color(p.color_idx).with_alpha(alpha);
            self.shapes.set_color(c);
            self.shapes.draw_rect(p.x - size * 0.5, p.y - size * 0.5, size, size);
        }
    }

    fn render_hud(&mut self) {
        // Score display via shapes (drawn as geometric indicators)
        // Score bar
        let score_bars = (self.score / 50).min(40) as f32;
        self.shapes.set_color(Color::new(0.0, 0.0, 0.0, 0.3));
        self.shapes.draw_rect(10.0, 10.0, 204.0, 16.0);
        self.shapes.set_color(Color::new(0.3, 0.9, 0.4, 0.9));
        self.shapes.draw_rect(12.0, 12.0, score_bars * 5.0, 12.0);

        // Lives
        for i in 0..self.lives {
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
        let w = WORLD_W;
        let h = WORLD_H;
        let pulse = (self.title_pulse).sin() * 0.15 + 0.85;

        // Title block — "SCP" as large geometric letters
        let cx = w * 0.5;
        let ty = h * 0.28;

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

        // Floating enemies decoration (orange squares)
        for i in 0..5 {
            let angle = self.title_pulse * 0.8 + i as f32 * std::f32::consts::TAU / 5.0;
            let radius = 80.0 + (self.title_pulse * 0.7 + i as f32).sin() * 15.0;
            let dx = cx + angle.cos() * radius;
            let dy = h * 0.55 + angle.sin() * radius * 0.35;
            self.shapes.set_color(Color::new(1.0, 0.45, 0.0, 0.5 * pulse));
            self.shapes.draw_rect(dx - 8.0, dy - 8.0, 16.0, 16.0);
        }

        // "PRESS SPACE TO START" blink
        let blink = (self.title_pulse * 1.5).sin();
        if blink > -0.3 {
            self.shapes.set_color(Color::new(0.9, 0.9, 1.0, 0.5 + blink * 0.4));
            self.shapes.draw_rect(cx - 90.0, h * 0.72, 180.0, 4.0);
        }

        // Controls hint
        self.shapes.set_color(Color::new(0.5, 0.5, 0.6, 0.5));
        self.shapes.draw_rect(cx - 70.0, h * 0.80, 140.0, 2.0);
        self.shapes.draw_rect(cx - 70.0, h * 0.80 + 18.0, 140.0, 2.0);
        self.shapes.draw_rect(cx - 70.0, h * 0.80 + 36.0, 140.0, 2.0);

        if self.high_score > 0 {
            self.shapes.set_color(Color::new(1.0, 0.85, 0.3, 0.7));
            self.shapes.draw_rect(cx - 40.0, h * 0.80 + 54.0, 80.0, 2.0);
        }
    }

    fn render_game_over(&mut self) {
        let w = WORLD_W;
        let h = WORLD_H;
        let alpha = (self.game_over_timer / 0.5).min(1.0);
        let cx = w * 0.5;

        // Dim overlay
        self.shapes.set_color(Color::new(0.0, 0.0, 0.0, 0.55 * alpha));
        self.shapes.draw_rect(0.0, 0.0, w, h);

        let center_y = h * 0.35;

        // "GAME OVER" block
        self.shapes.set_color(Color::new(1.0, 0.2, 0.2, alpha * 0.95));
        self.shapes.draw_rect(cx - 90.0, center_y - 10.0, 180.0, 20.0);
        self.shapes.set_color(Color::new(0.8, 0.1, 0.1, alpha * 0.7));
        self.shapes.draw_rect(cx - 70.0, center_y + 14.0, 140.0, 10.0);

        // Score indicator bar
        let score_bars = (self.score / 50).min(40) as f32;
        self.shapes.set_color(Color::new(0.2, 0.2, 0.25, alpha * 0.8));
        self.shapes.draw_rect(cx - 60.0, center_y + 50.0, 120.0, 10.0);
        self.shapes.set_color(Color::new(0.3, 0.9, 0.4, alpha * 0.9));
        self.shapes.draw_rect(cx - 58.0, center_y + 52.0, score_bars * 2.9, 6.0);

        // High score indicator
        if self.score >= self.high_score && self.score > 0 {
            self.shapes.set_color(Color::new(1.0, 0.85, 0.3, alpha * 0.9));
            self.shapes.draw_rect(cx - 30.0, center_y + 70.0, 60.0, 3.0);
        }

        // "REPLAY" prompt
        if self.game_over_timer > 1.5 {
            let blink = (self.game_over_timer * 3.0).sin();
            if blink > 0.0 {
                self.shapes.set_color(Color::new(0.9, 0.9, 1.0, alpha * 0.6 * blink));
                self.shapes.draw_rect(cx - 60.0, center_y + 95.0, 120.0, 4.0);
            }
        }
    }

    fn update_camera(&mut self, dt: f32) {
        // Target: player center
        let target_x = self.player_x + PLAYER_SIZE * 0.5;

        // Dead zone: only move camera if player is outside the dead zone band
        let cam_center = self.camera_x + WORLD_W * 0.5;
        let diff = target_x - cam_center;

        let desired_x = if diff > CAM_DEAD_ZONE_X {
            // Player is right of dead zone right edge
            target_x - CAM_DEAD_ZONE_X - WORLD_W * 0.5
        } else if diff < -CAM_DEAD_ZONE_X {
            // Player is left of dead zone left edge
            target_x + CAM_DEAD_ZONE_X - WORLD_W * 0.5
        } else {
            // Inside dead zone — keep current camera
            self.camera_x
        };

        // Smooth follow (lerp)
        self.camera_x += (desired_x - self.camera_x) * CAM_SMOOTH * dt;

        // Boundary constraints: clamp so camera doesn't go past level edges
        self.camera_x = self.camera_x.clamp(0.0, LEVEL_W - WORLD_W);

        // Vertical: fixed (no vertical scrolling needed)
        self.camera_y = 0.0;
    }

    fn render(&mut self, _alpha: f32) {
        self.gl.resize();

        // Update camera (dead zone + smooth follow + bounds)
        if self.state == GameState::Playing || self.state == GameState::GameOver {
            let dt = self.time.delta();
            self.update_camera(dt);
        }

        // Dark blue background
        self.gl.clear(0.04, 0.06, 0.18, 1.0);
        self.gl.enable_blend();

        // Viewport projection: shows [camera_x .. camera_x+WORLD_W] x [0 .. WORLD_H]
        // y-down: swap top/bottom
        let left = self.camera_x;
        let right = self.camera_x + WORLD_W;
        let projection = Mat4::orthographic_rh_gl(left, right, WORLD_H, 0.0, -1.0, 1.0);

        // Apply shake via view translation
        let shake_x = if self.shake_amount > 0.0 {
            (rand() - 0.5) * self.shake_amount * 2.0
        } else {
            0.0
        };
        let shake_y = if self.shake_amount > 0.0 {
            (rand() - 0.5) * self.shake_amount * 2.0
        } else {
            0.0
        };
        let view = Mat4::from_translation(Vec3::new(-shake_x, -shake_y, 0.0));
        let vp = projection * view;
        self.shapes.begin();

        // Background stars (tile across visible area)
        self.shapes.set_color(Color::new(0.6, 0.6, 0.8, 0.3));
        let cam_left = self.camera_x;
        let cam_right = self.camera_x + WORLD_W;
        for i in 0..60 {
            let sx = (i as f32 * 137.5) % LEVEL_W;
            if sx >= cam_left - 5.0 && sx <= cam_right + 5.0 {
                let sy = (i as f32 * 73.1 + 20.0) % (GROUND_Y - 40.0);
                self.shapes.draw_rect(sx, sy, 2.0, 2.0);
            }
        }

        self.render_ground();

        match self.state {
            GameState::Title => {
                self.render_title();
            }
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
}

// ── WASM Exports for Vue ───────────────────────────────────────────────────────
#[wasm_bindgen]
pub fn get_score() -> i32 {
    GAME_REF.with(|g| {
        if let Some(ref game) = *g.borrow() {
            game.borrow().score
        } else {
            0
        }
    })
}

#[wasm_bindgen]
pub fn get_lives() -> i32 {
    GAME_REF.with(|g| {
        if let Some(ref game) = *g.borrow() {
            game.borrow().lives
        } else {
            0
        }
    })
}

#[wasm_bindgen]
pub fn get_game_state() -> u8 {
    GAME_REF.with(|g| {
        if let Some(ref game) = *g.borrow() {
            match game.borrow().state {
                GameState::Title => 0,
                GameState::Playing => 1,
                GameState::GameOver => 2,
            }
        } else {
            0
        }
    })
}

#[wasm_bindgen]
pub fn get_high_score() -> i32 {
    GAME_REF.with(|g| {
        if let Some(ref game) = *g.borrow() {
            game.borrow().high_score
        } else {
            0
        }
    })
}

#[wasm_bindgen]
pub fn start_game() {
    GAME_REF.with(|g| {
        if let Some(ref game) = *g.borrow() {
            let mut game = game.borrow_mut();
            if game.state == GameState::Title {
                game.state = GameState::Playing;
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
            if game.state == GameState::GameOver {
                game.state = GameState::Playing;
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

    // Store reference for WASM exports
    GAME_REF.with(|g| {
        *g.borrow_mut() = Some(game.clone());
    });

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
