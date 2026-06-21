use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

type AnimationFrameClosure = Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>>;

use opengame_engine::color::Color;
use opengame_engine::input::{keys::KeyCode, InputManager};
use opengame_engine::math::Vec2;
use opengame_engine::renderer::{Camera2D, GlBackend, ShapeRenderer};
use opengame_engine::time::Time;

const PLAYER_SPEED: f32 = 420.0;
const PLAYER_WIDTH: f32 = 30.0;
const PLAYER_HEIGHT: f32 = 36.0;
const BULLET_SPEED: f32 = 650.0;
const ENEMY_BULLET_SPEED: f32 = 300.0;
const SHOOT_INTERVAL: f32 = 0.12;
const MAX_LIVES: i32 = 3;
const INVINCIBLE_TIME: f32 = 2.0;
const STAR_LAYERS: usize = 3;
const STARS_PER_LAYER: usize = 60;
const MAX_PARTICLES: usize = 300;
const MAX_BULLETS: usize = 200;
const MAX_ENEMIES: usize = 30;
const MAX_POWERUPS: usize = 5;

#[derive(Clone, Copy, PartialEq)]
enum GameState {
    Title,
    Playing,
    GameOver,
}

#[derive(Clone, Copy, PartialEq)]
enum EnemyType {
    Scout,
    Fighter,
    Dreadnought,
}

#[derive(Clone, Copy, PartialEq)]
enum PowerUpType {
    Shield,
    TripleShot,
    RapidFire,
}

struct Star {
    x: f32,
    y: f32,
    brightness: f32,
    size: f32,
    speed: f32,
}

struct Bullet {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    alive: bool,
    is_player: bool,
    size: f32,
}

struct Enemy {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    hp: i32,
    max_hp: i32,
    alive: bool,
    kind: EnemyType,
    shoot_timer: f32,
    angle: f32,
    flash: f32,
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

struct PowerUp {
    x: f32,
    y: f32,
    vy: f32,
    alive: bool,
    kind: PowerUpType,
    angle: f32,
}

struct CollisionEvent {
    explosion_x: f32,
    explosion_y: f32,
    explosion_count: usize,
    explosion_power: f32,
    shake: f32,
    score: i32,
    enemy_x: f32,
    enemy_y: f32,
    spawn_powerup: bool,
    hit_player: bool,
    shield_absorb: bool,
}

struct ShootEvent {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    size: f32,
}

struct SpaceBlitz {
    gl: GlBackend,
    shapes: ShapeRenderer,
    camera: Camera2D,
    input: InputManager,
    time: Time,
    state: GameState,

    player_x: f32,
    player_y: f32,
    player_lives: i32,
    player_invincible: f32,
    player_flash: f32,
    has_shield: bool,
    shield_timer: f32,
    triple_shot: bool,
    triple_shot_timer: f32,
    rapid_fire: bool,
    rapid_fire_timer: f32,
    shoot_timer: f32,
    player_engine_glow: f32,

    bullets: Vec<Bullet>,
    enemies: Vec<Enemy>,
    particles: Vec<Particle>,
    powerups: Vec<PowerUp>,
    stars: Vec<Star>,

    score: i32,
    high_score: i32,
    wave: i32,
    wave_timer: f32,
    enemies_spawned: i32,
    enemies_per_wave: i32,
    spawn_timer: f32,
    difficulty_mult: f32,
    combo: i32,
    combo_timer: f32,

    shake_amount: f32,
    shake_decay: f32,

    title_pulse: f32,
    game_over_timer: f32,

    bg_hue: f32,
}

fn rand() -> f32 {
    js_sys::Math::random() as f32
}

fn rand_range(min: f32, max: f32) -> f32 {
    min + rand() * (max - min)
}

fn rand_int(min: i32, max: i32) -> i32 {
    min + (rand() * (max - min + 1) as f32) as i32
}

fn enemy_color(idx: u8) -> Color {
    match idx % 3 {
        0 => Color::new(1.0, 0.3, 0.3, 1.0),
        1 => Color::new(1.0, 0.6, 0.2, 1.0),
        _ => Color::new(0.9, 0.2, 0.8, 1.0),
    }
}

fn particle_color(idx: u8) -> Color {
    match idx % 6 {
        0 => Color::new(1.0, 0.9, 0.3, 1.0),
        1 => Color::new(1.0, 0.5, 0.1, 1.0),
        2 => Color::new(1.0, 0.2, 0.1, 1.0),
        3 => Color::new(0.9, 0.6, 1.0, 1.0),
        4 => Color::new(0.4, 0.7, 1.0, 1.0),
        _ => Color::new(1.0, 1.0, 1.0, 1.0),
    }
}

fn powerup_color(kind: PowerUpType) -> Color {
    match kind {
        PowerUpType::Shield => Color::new(0.2, 0.8, 1.0, 1.0),
        PowerUpType::TripleShot => Color::new(1.0, 0.8, 0.2, 1.0),
        PowerUpType::RapidFire => Color::new(1.0, 0.3, 0.5, 1.0),
    }
}

fn enemy_width(kind: EnemyType) -> f32 {
    match kind {
        EnemyType::Scout => 20.0,
        EnemyType::Fighter => 28.0,
        EnemyType::Dreadnought => 44.0,
    }
}

fn enemy_height(kind: EnemyType) -> f32 {
    match kind {
        EnemyType::Scout => 16.0,
        EnemyType::Fighter => 20.0,
        EnemyType::Dreadnought => 28.0,
    }
}

impl SpaceBlitz {
    fn new() -> Result<Self, String> {
        opengame_engine::log::init();

        let gl = GlBackend::new("game-canvas")?;
        let camera = Camera2D::new(gl.width() as f32, gl.height() as f32);
        let shapes = ShapeRenderer::new(gl.gl())?;
        let input = InputManager::new()?;

        let window = web_sys::window().ok_or("No window")?;
        let performance = window.performance().ok_or("No performance")?;
        let time = Time::new(performance);

        let w = gl.width() as f32;
        let h = gl.height() as f32;

        let mut stars = Vec::with_capacity(STAR_LAYERS * STARS_PER_LAYER);
        for layer in 0..STAR_LAYERS {
            let speed = 20.0 + layer as f32 * 40.0;
            let brightness = 0.2 + layer as f32 * 0.25;
            let size = 1.0 + layer as f32 * 0.5;
            for _ in 0..STARS_PER_LAYER {
                stars.push(Star {
                    x: rand() * w,
                    y: rand() * h,
                    brightness: brightness * rand_range(0.6, 1.0),
                    size: size * rand_range(0.5, 1.5),
                    speed,
                });
            }
        }

        Ok(Self {
            gl,
            shapes,
            camera,
            input,
            time,
            state: GameState::Title,
            player_x: w / 2.0,
            player_y: 80.0,
            player_lives: MAX_LIVES,
            player_invincible: 0.0,
            player_flash: 0.0,
            has_shield: false,
            shield_timer: 0.0,
            triple_shot: false,
            triple_shot_timer: 0.0,
            rapid_fire: false,
            rapid_fire_timer: 0.0,
            shoot_timer: 0.0,
            player_engine_glow: 0.0,
            bullets: Vec::with_capacity(MAX_BULLETS),
            enemies: Vec::with_capacity(MAX_ENEMIES),
            particles: Vec::with_capacity(MAX_PARTICLES),
            powerups: Vec::with_capacity(MAX_POWERUPS),
            stars,
            score: 0,
            high_score: 0,
            wave: 0,
            wave_timer: 0.0,
            enemies_spawned: 0,
            enemies_per_wave: 5,
            spawn_timer: 0.0,
            difficulty_mult: 1.0,
            combo: 0,
            combo_timer: 0.0,
            shake_amount: 0.0,
            shake_decay: 5.0,
            title_pulse: 0.0,
            game_over_timer: 0.0,
            bg_hue: 0.0,
        })
    }

    fn spawn_particle(
        x: f32,
        y: f32,
        vx: f32,
        vy: f32,
        life: f32,
        size: f32,
        color_idx: u8,
    ) -> Particle {
        Particle {
            x,
            y,
            vx,
            vy,
            life,
            max_life: life,
            size,
            color_idx,
        }
    }

    fn spawn_explosion_particles(x: f32, y: f32, count: usize, power: f32) -> Vec<Particle> {
        let mut result = Vec::with_capacity(count);
        for i in 0..count {
            let angle = rand() * std::f32::consts::TAU;
            let speed = rand_range(50.0, power);
            let vx = angle.cos() * speed;
            let vy = angle.sin() * speed;
            let life = rand_range(0.3, 0.8);
            let size = rand_range(2.0, 5.0);
            let color_idx = (i % 6) as u8;
            result.push(Self::spawn_particle(x, y, vx, vy, life, size, color_idx));
        }
        result
    }

    fn add_bullet(
        bullets: &mut Vec<Bullet>,
        x: f32,
        y: f32,
        vx: f32,
        vy: f32,
        is_player: bool,
        size: f32,
    ) {
        if bullets.len() < MAX_BULLETS {
            bullets.push(Bullet {
                x,
                y,
                vx,
                vy,
                alive: true,
                is_player,
                size,
            });
        }
    }

    fn make_enemy_shoot_events(enemy: &Enemy, difficulty_mult: f32) -> Vec<ShootEvent> {
        let (bx, by) = (enemy.x, enemy.y - 10.0);
        let speed = ENEMY_BULLET_SPEED * difficulty_mult.sqrt();
        match enemy.kind {
            EnemyType::Scout => {
                vec![ShootEvent {
                    x: bx,
                    y: by,
                    vx: 0.0,
                    vy: -speed,
                    size: 2.5,
                }]
            }
            EnemyType::Fighter => {
                vec![
                    ShootEvent {
                        x: bx,
                        y: by,
                        vx: -speed * 0.15,
                        vy: -speed,
                        size: 3.0,
                    },
                    ShootEvent {
                        x: bx,
                        y: by,
                        vx: speed * 0.15,
                        vy: -speed,
                        size: 3.0,
                    },
                ]
            }
            EnemyType::Dreadnought => (0..3)
                .map(|i| {
                    let angle = std::f32::consts::PI + (i as f32 - 1.0) * 0.25;
                    ShootEvent {
                        x: bx,
                        y: by,
                        vx: angle.cos() * speed,
                        vy: angle.sin() * speed,
                        size: 3.5,
                    }
                })
                .collect(),
        }
    }

    fn spawn_enemy(&mut self) {
        if self.enemies.len() >= MAX_ENEMIES {
            return;
        }
        let w = self.gl.width() as f32;
        let kind_roll = rand();
        let kind = if self.wave < 3 {
            EnemyType::Scout
        } else if self.wave < 6 {
            if kind_roll < 0.6 {
                EnemyType::Scout
            } else {
                EnemyType::Fighter
            }
        } else if kind_roll < 0.35 {
            EnemyType::Scout
        } else if kind_roll < 0.75 {
            EnemyType::Fighter
        } else {
            EnemyType::Dreadnought
        };

        let (hp, vy, shoot_interval) = match kind {
            EnemyType::Scout => (1, rand_range(-80.0, -50.0), rand_range(1.5, 3.0)),
            EnemyType::Fighter => (3, rand_range(-60.0, -35.0), rand_range(1.0, 2.0)),
            EnemyType::Dreadnought => (8, rand_range(-40.0, -20.0), rand_range(0.8, 1.5)),
        };

        let vx = rand_range(-30.0, 30.0) * self.difficulty_mult;

        self.enemies.push(Enemy {
            x: rand_range(40.0, w - 40.0),
            y: self.gl.height() as f32 + 30.0,
            vx,
            vy: vy * self.difficulty_mult.sqrt(),
            hp,
            max_hp: hp,
            alive: true,
            kind,
            shoot_timer: shoot_interval,
            angle: 0.0,
            flash: 0.0,
        });
    }

    fn try_spawn_powerup(x: f32, y: f32, current_count: usize) -> Option<PowerUp> {
        if current_count >= MAX_POWERUPS || rand() > 0.25 {
            return None;
        }
        let kind = match rand_int(0, 2) {
            0 => PowerUpType::Shield,
            1 => PowerUpType::TripleShot,
            _ => PowerUpType::RapidFire,
        };
        Some(PowerUp {
            x,
            y,
            vy: -60.0,
            alive: true,
            kind,
            angle: 0.0,
        })
    }

    fn start_new_wave(&mut self) {
        self.wave += 1;
        self.wave_timer = 2.0;
        self.enemies_spawned = 0;
        self.enemies_per_wave = 4 + self.wave * 2;
        self.spawn_timer = 0.0;
        self.difficulty_mult = 1.0 + (self.wave - 1) as f32 * 0.12;
    }

    fn reset_game(&mut self) {
        let w = self.gl.width() as f32;
        let h = self.gl.height() as f32;

        if self.score > self.high_score {
            self.high_score = self.score;
        }

        self.player_x = w / 2.0;
        self.player_y = 80.0;
        self.player_lives = MAX_LIVES;
        self.player_invincible = 0.0;
        self.player_flash = 0.0;
        self.has_shield = false;
        self.shield_timer = 0.0;
        self.triple_shot = false;
        self.triple_shot_timer = 0.0;
        self.rapid_fire = false;
        self.rapid_fire_timer = 0.0;
        self.shoot_timer = 0.0;
        self.bullets.clear();
        self.enemies.clear();
        self.particles.clear();
        self.powerups.clear();
        self.score = 0;
        self.wave = 0;
        self.wave_timer = 0.0;
        self.enemies_spawned = 0;
        self.enemies_per_wave = 5;
        self.spawn_timer = 0.0;
        self.difficulty_mult = 1.0;
        self.combo = 0;
        self.combo_timer = 0.0;
        self.shake_amount = 0.0;
        self.bg_hue = 0.0;

        for star in &mut self.stars {
            star.y = rand() * h;
        }
    }

    fn update_stars(&mut self, dt: f32) {
        let w = self.gl.width() as f32;
        let h = self.gl.height() as f32;
        for star in &mut self.stars {
            star.y -= star.speed * dt;
            if star.y < -2.0 {
                star.y = h + 2.0;
                star.x = rand() * w;
            }
        }
    }

    fn update_title(&mut self, dt: f32) {
        self.title_pulse += dt * 2.0;
        self.update_stars(dt);
        self.bg_hue += dt * 0.02;

        if self.input.is_key_pressed(KeyCode::Enter) || self.input.is_key_pressed(KeyCode::Space) {
            self.state = GameState::Playing;
            self.reset_game();
            self.start_new_wave();
        }
    }

    fn update_playing(&mut self, dt: f32) {
        let w = self.gl.width() as f32;
        let h = self.gl.height() as f32;

        self.update_stars(dt);
        self.bg_hue += dt * 0.01;

        if self.shake_amount > 0.0 {
            self.shake_amount = (self.shake_amount - self.shake_decay * dt).max(0.0);
        }

        let move_speed = PLAYER_SPEED * dt;
        if self.input.is_key_down(KeyCode::KeyA) || self.input.is_key_down(KeyCode::ArrowLeft) {
            self.player_x -= move_speed;
        }
        if self.input.is_key_down(KeyCode::KeyD) || self.input.is_key_down(KeyCode::ArrowRight) {
            self.player_x += move_speed;
        }
        if self.input.is_key_down(KeyCode::KeyW) || self.input.is_key_down(KeyCode::ArrowUp) {
            self.player_y += move_speed;
        }
        if self.input.is_key_down(KeyCode::KeyS) || self.input.is_key_down(KeyCode::ArrowDown) {
            self.player_y -= move_speed;
        }
        self.player_x = self
            .player_x
            .clamp(PLAYER_WIDTH / 2.0, w - PLAYER_WIDTH / 2.0);
        self.player_y = self
            .player_y
            .clamp(PLAYER_HEIGHT / 2.0, h - PLAYER_HEIGHT / 2.0);

        self.shoot_timer = (self.shoot_timer - dt).max(0.0);
        if self.input.is_key_down(KeyCode::Space) {
            self.player_shoot();
        }

        self.player_engine_glow += dt * 8.0;

        if self.player_invincible > 0.0 {
            self.player_invincible -= dt;
            self.player_flash += dt * 15.0;
        }

        if self.has_shield {
            self.shield_timer -= dt;
            if self.shield_timer <= 0.0 {
                self.has_shield = false;
            }
        }
        if self.triple_shot {
            self.triple_shot_timer -= dt;
            if self.triple_shot_timer <= 0.0 {
                self.triple_shot = false;
            }
        }
        if self.rapid_fire {
            self.rapid_fire_timer -= dt;
            if self.rapid_fire_timer <= 0.0 {
                self.rapid_fire = false;
            }
        }

        if self.combo_timer > 0.0 {
            self.combo_timer -= dt;
            if self.combo_timer <= 0.0 {
                self.combo = 0;
            }
        }

        if self.wave_timer > 0.0 {
            self.wave_timer -= dt;
        } else {
            self.spawn_timer -= dt;
            if self.spawn_timer <= 0.0 && self.enemies_spawned < self.enemies_per_wave {
                self.spawn_enemy();
                self.enemies_spawned += 1;
                self.spawn_timer = rand_range(0.3, 1.2) / self.difficulty_mult.sqrt();
            }

            if self.enemies_spawned >= self.enemies_per_wave && self.enemies.is_empty() {
                self.start_new_wave();
            }
        }

        for bullet in &mut self.bullets {
            if !bullet.alive {
                continue;
            }
            bullet.x += bullet.vx * dt;
            bullet.y += bullet.vy * dt;
            if bullet.y > h + 20.0 || bullet.y < -20.0 || bullet.x < -20.0 || bullet.x > w + 20.0 {
                bullet.alive = false;
            }
        }

        let mut shoot_events: Vec<ShootEvent> = Vec::new();
        for enemy in &mut self.enemies {
            if !enemy.alive {
                continue;
            }
            enemy.x += enemy.vx * dt;
            enemy.y += enemy.vy * dt;
            enemy.angle += dt * 2.0;
            enemy.flash = (enemy.flash - dt * 5.0).max(0.0);

            if enemy.x < 20.0 || enemy.x > w - 20.0 {
                enemy.vx = -enemy.vx;
            }

            enemy.shoot_timer -= dt;
            if enemy.shoot_timer <= 0.0 && enemy.y < h - 50.0 {
                enemy.shoot_timer = match enemy.kind {
                    EnemyType::Scout => rand_range(1.5, 3.0),
                    EnemyType::Fighter => rand_range(1.0, 2.0),
                    EnemyType::Dreadnought => rand_range(0.8, 1.5),
                } / self.difficulty_mult.sqrt();

                shoot_events.extend(Self::make_enemy_shoot_events(enemy, self.difficulty_mult));
            }

            if enemy.y < -40.0 {
                enemy.alive = false;
            }
        }
        for ev in shoot_events {
            Self::add_bullet(&mut self.bullets, ev.x, ev.y, ev.vx, ev.vy, false, ev.size);
        }

        for pu in &mut self.powerups {
            if !pu.alive {
                continue;
            }
            pu.y += pu.vy * dt;
            pu.angle += dt * 3.0;
            if pu.y < -20.0 {
                pu.alive = false;
            }
        }

        for p in &mut self.particles {
            if p.life <= 0.0 {
                continue;
            }
            p.x += p.vx * dt;
            p.y += p.vy * dt;
            p.vx *= 0.98;
            p.vy *= 0.98;
            p.life -= dt;
        }

        self.process_collisions();

        self.bullets.retain(|b| b.alive);
        self.enemies.retain(|e| e.alive);
        self.particles.retain(|p| p.life > 0.0);
        self.powerups.retain(|p| p.alive);
    }

    fn process_collisions(&mut self) {
        let px = self.player_x;
        let py = self.player_y;
        let pw = PLAYER_WIDTH / 2.0;
        let ph = PLAYER_HEIGHT / 2.0;

        let mut events: Vec<CollisionEvent> = Vec::new();

        for bullet in &mut self.bullets {
            if !bullet.alive || !bullet.is_player {
                continue;
            }
            let bx = bullet.x;
            let by = bullet.y;
            let bs = bullet.size;

            for enemy in &mut self.enemies {
                if !enemy.alive {
                    continue;
                }
                let ew = enemy_width(enemy.kind) / 2.0;
                let eh = enemy_height(enemy.kind) / 2.0;

                if bx + bs > enemy.x - ew
                    && bx - bs < enemy.x + ew
                    && by + bs > enemy.y - eh
                    && by - bs < enemy.y + eh
                {
                    bullet.alive = false;
                    enemy.hp -= 1;
                    enemy.flash = 1.0;

                    if enemy.hp <= 0 {
                        enemy.alive = false;
                        let points = match enemy.kind {
                            EnemyType::Scout => 100,
                            EnemyType::Fighter => 250,
                            EnemyType::Dreadnought => 500,
                        };
                        let explosion_count = match enemy.kind {
                            EnemyType::Scout => 12,
                            EnemyType::Fighter => 20,
                            EnemyType::Dreadnought => 35,
                        };
                        let shake = match enemy.kind {
                            EnemyType::Scout => 3.0,
                            EnemyType::Fighter => 6.0,
                            EnemyType::Dreadnought => 12.0,
                        };
                        events.push(CollisionEvent {
                            explosion_x: bx,
                            explosion_y: by,
                            explosion_count: 4,
                            explosion_power: 80.0,
                            shake: 0.0,
                            score: 0,
                            enemy_x: enemy.x,
                            enemy_y: enemy.y,
                            spawn_powerup: true,
                            hit_player: false,
                            shield_absorb: false,
                        });
                        events.push(CollisionEvent {
                            explosion_x: enemy.x,
                            explosion_y: enemy.y,
                            explosion_count,
                            explosion_power: 200.0,
                            shake,
                            score: points,
                            enemy_x: 0.0,
                            enemy_y: 0.0,
                            spawn_powerup: false,
                            hit_player: false,
                            shield_absorb: false,
                        });
                    } else {
                        events.push(CollisionEvent {
                            explosion_x: bx,
                            explosion_y: by,
                            explosion_count: 4,
                            explosion_power: 80.0,
                            shake: 0.0,
                            score: 0,
                            enemy_x: 0.0,
                            enemy_y: 0.0,
                            spawn_powerup: false,
                            hit_player: false,
                            shield_absorb: false,
                        });
                    }
                    break;
                }
            }
        }

        for bullet in &mut self.bullets {
            if !bullet.alive || bullet.is_player {
                continue;
            }
            let bx = bullet.x;
            let by = bullet.y;
            let bs = bullet.size;

            if bx + bs > px - pw && bx - bs < px + pw && by + bs > py - ph && by - bs < py + ph {
                if self.player_invincible > 0.0 {
                    continue;
                }
                bullet.alive = false;

                if self.has_shield {
                    events.push(CollisionEvent {
                        explosion_x: bx,
                        explosion_y: by,
                        explosion_count: 10,
                        explosion_power: 120.0,
                        shake: 4.0,
                        score: 0,
                        enemy_x: 0.0,
                        enemy_y: 0.0,
                        spawn_powerup: false,
                        hit_player: false,
                        shield_absorb: true,
                    });
                } else {
                    events.push(CollisionEvent {
                        explosion_x: bx,
                        explosion_y: by,
                        explosion_count: 0,
                        explosion_power: 0.0,
                        shake: 0.0,
                        score: 0,
                        enemy_x: 0.0,
                        enemy_y: 0.0,
                        spawn_powerup: false,
                        hit_player: true,
                        shield_absorb: false,
                    });
                }
            }
        }

        for enemy in &mut self.enemies {
            if !enemy.alive {
                continue;
            }
            let ew = enemy_width(enemy.kind) / 2.0;
            let eh = enemy_height(enemy.kind) / 2.0;

            if px + pw > enemy.x - ew
                && px - pw < enemy.x + ew
                && py + ph > enemy.y - eh
                && py - ph < enemy.y + eh
            {
                if self.player_invincible > 0.0 {
                    continue;
                }
                enemy.alive = false;

                if self.has_shield {
                    events.push(CollisionEvent {
                        explosion_x: enemy.x,
                        explosion_y: enemy.y,
                        explosion_count: 15,
                        explosion_power: 150.0,
                        shake: 6.0,
                        score: 0,
                        enemy_x: 0.0,
                        enemy_y: 0.0,
                        spawn_powerup: false,
                        hit_player: false,
                        shield_absorb: true,
                    });
                } else {
                    events.push(CollisionEvent {
                        explosion_x: enemy.x,
                        explosion_y: enemy.y,
                        explosion_count: 20,
                        explosion_power: 180.0,
                        shake: 0.0,
                        score: 0,
                        enemy_x: 0.0,
                        enemy_y: 0.0,
                        spawn_powerup: false,
                        hit_player: true,
                        shield_absorb: false,
                    });
                }
            }
        }

        for pu in &mut self.powerups {
            if !pu.alive {
                continue;
            }
            let dist = ((px - pu.x).powi(2) + (py - pu.y).powi(2)).sqrt();
            if dist < 25.0 {
                pu.alive = false;
                events.push(CollisionEvent {
                    explosion_x: pu.x,
                    explosion_y: pu.y,
                    explosion_count: 8,
                    explosion_power: 60.0,
                    shake: 0.0,
                    score: 0,
                    enemy_x: 0.0,
                    enemy_y: 0.0,
                    spawn_powerup: false,
                    hit_player: false,
                    shield_absorb: false,
                });
                match pu.kind {
                    PowerUpType::Shield => {
                        self.has_shield = true;
                        self.shield_timer = 8.0;
                    }
                    PowerUpType::TripleShot => {
                        self.triple_shot = true;
                        self.triple_shot_timer = 10.0;
                    }
                    PowerUpType::RapidFire => {
                        self.rapid_fire = true;
                        self.rapid_fire_timer = 8.0;
                    }
                }
            }
        }

        let mut max_shake = 0.0f32;
        let mut player_hit = false;
        let mut shield_absorbed = false;

        for ev in events {
            if ev.explosion_count > 0 {
                let new_particles = Self::spawn_explosion_particles(
                    ev.explosion_x,
                    ev.explosion_y,
                    ev.explosion_count,
                    ev.explosion_power,
                );
                for p in new_particles {
                    if self.particles.len() < MAX_PARTICLES {
                        self.particles.push(p);
                    }
                }
            }

            if ev.shake > max_shake {
                max_shake = ev.shake;
            }

            if ev.score > 0 {
                self.combo += 1;
                self.combo_timer = 2.0;
                let combo_mult = (self.combo).min(10);
                self.score += ev.score * combo_mult;
            }

            if ev.spawn_powerup {
                if let Some(pu) =
                    Self::try_spawn_powerup(ev.enemy_x, ev.enemy_y, self.powerups.len())
                {
                    self.powerups.push(pu);
                }
            }

            if ev.shield_absorb {
                shield_absorbed = true;
            }

            if ev.hit_player {
                player_hit = true;
            }
        }

        if max_shake > self.shake_amount {
            self.shake_amount = max_shake;
        }

        if shield_absorbed {
            self.has_shield = false;
            self.shield_timer = 0.0;
        }

        if player_hit {
            self.do_player_hit();
        }
    }

    fn player_shoot(&mut self) {
        let shoot_interval = if self.rapid_fire {
            SHOOT_INTERVAL * 0.4
        } else {
            SHOOT_INTERVAL
        };
        if self.shoot_timer > 0.0 {
            return;
        }
        self.shoot_timer = shoot_interval;

        Self::add_bullet(
            &mut self.bullets,
            self.player_x,
            self.player_y + PLAYER_HEIGHT / 2.0,
            0.0,
            BULLET_SPEED,
            true,
            3.0,
        );

        if self.triple_shot {
            let spread: f32 = 0.15;
            let dx1 = spread.sin() * BULLET_SPEED;
            let dy1 = spread.cos() * BULLET_SPEED;
            Self::add_bullet(
                &mut self.bullets,
                self.player_x - 8.0,
                self.player_y + PLAYER_HEIGHT / 2.0 - 4.0,
                -dx1,
                dy1,
                true,
                2.5,
            );
            Self::add_bullet(
                &mut self.bullets,
                self.player_x + 8.0,
                self.player_y + PLAYER_HEIGHT / 2.0 - 4.0,
                dx1,
                dy1,
                true,
                2.5,
            );
        }
    }

    fn do_player_hit(&mut self) {
        self.player_lives -= 1;
        self.shake_amount = 15.0;
        let new_particles =
            Self::spawn_explosion_particles(self.player_x, self.player_y, 25, 200.0);
        for p in new_particles {
            if self.particles.len() < MAX_PARTICLES {
                self.particles.push(p);
            }
        }
        self.combo = 0;
        self.combo_timer = 0.0;

        if self.player_lives <= 0 {
            self.state = GameState::GameOver;
            self.game_over_timer = 0.0;
            let death_particles =
                Self::spawn_explosion_particles(self.player_x, self.player_y, 40, 300.0);
            for p in death_particles {
                if self.particles.len() < MAX_PARTICLES {
                    self.particles.push(p);
                }
            }
        } else {
            self.player_invincible = INVINCIBLE_TIME;
            self.player_flash = 0.0;
        }
    }

    fn update_game_over(&mut self, dt: f32) {
        self.game_over_timer += dt;
        self.update_stars(dt);

        for p in &mut self.particles {
            if p.life <= 0.0 {
                continue;
            }
            p.x += p.vx * dt;
            p.y += p.vy * dt;
            p.vx *= 0.98;
            p.vy *= 0.98;
            p.life -= dt;
        }
        self.particles.retain(|p| p.life > 0.0);

        if self.game_over_timer > 1.5
            && (self.input.is_key_pressed(KeyCode::Space)
                || self.input.is_key_pressed(KeyCode::Enter))
        {
            self.state = GameState::Playing;
            self.reset_game();
            self.start_new_wave();
        }
    }

    fn update(&mut self, dt: f32) {
        match self.state {
            GameState::Title => self.update_title(dt),
            GameState::Playing => self.update_playing(dt),
            GameState::GameOver => self.update_game_over(dt),
        }
    }

    fn render_stars(&mut self) {
        for star in &self.stars {
            let alpha = star.brightness;
            let g = 0.8 + star.brightness * 0.2;
            self.shapes.set_color(Color::new(g, g, g + 0.1, alpha));
            self.shapes.draw_rect(
                star.x - star.size * 0.5,
                star.y - star.size * 0.5,
                star.size,
                star.size,
            );
        }
    }

    fn render_player(&mut self) {
        if self.player_invincible > 0.0 && (self.player_flash * 0.5).sin() > 0.3 {
            return;
        }

        let px = self.player_x;
        let py = self.player_y;

        let glow = (self.player_engine_glow).sin().abs();
        let engine_color = Color::new(0.3 + glow * 0.5, 0.5 + glow * 0.3, 1.0, 0.6 + glow * 0.3);
        self.shapes.set_color(engine_color);
        let ew = 6.0;
        let eh = 8.0 + glow * 6.0;
        self.shapes
            .draw_rect(px - ew / 2.0, py - PLAYER_HEIGHT / 2.0 - eh, ew, eh);

        self.shapes.set_color(Color::new(0.8, 0.85, 0.95, 1.0));
        self.shapes.draw_rect(
            px - PLAYER_WIDTH / 2.0,
            py - PLAYER_HEIGHT / 4.0,
            PLAYER_WIDTH,
            PLAYER_HEIGHT / 2.0,
        );

        self.shapes.set_color(Color::new(0.6, 0.7, 0.9, 1.0));
        let nose_w = PLAYER_WIDTH * 0.5;
        let nose_h = PLAYER_HEIGHT * 0.5;
        self.shapes.draw_rect(
            px - nose_w / 2.0,
            py + PLAYER_HEIGHT / 4.0 - nose_h / 2.0,
            nose_w,
            nose_h,
        );

        self.shapes.set_color(Color::new(0.5, 0.55, 0.7, 1.0));
        self.shapes.draw_rect(
            px - PLAYER_WIDTH / 2.0 - 4.0,
            py - PLAYER_HEIGHT / 4.0 - 2.0,
            6.0,
            10.0,
        );
        self.shapes.draw_rect(
            px + PLAYER_WIDTH / 2.0 - 2.0,
            py - PLAYER_HEIGHT / 4.0 - 2.0,
            6.0,
            10.0,
        );

        self.shapes.set_color(Color::new(0.4, 0.7, 1.0, 0.9));
        self.shapes.draw_rect(px - 3.0, py + 2.0, 6.0, 6.0);

        if self.has_shield {
            let pulse = (self.time.elapsed() * 4.0).sin() * 0.15 + 0.35;
            self.shapes.set_color(Color::new(0.3, 0.7, 1.0, pulse));
            self.shapes.draw_circle_outline(px, py, 28.0, 2.0, 24);
        }
    }

    fn render_enemies(&mut self) {
        for enemy in &self.enemies {
            if !enemy.alive {
                continue;
            }
            let x = enemy.x;
            let y = enemy.y;

            let base_color = enemy_color(enemy.kind as u8);
            let c = if enemy.flash > 0.0 {
                Color::lerp(base_color, Color::WHITE, enemy.flash * 0.7)
            } else {
                base_color
            };

            match enemy.kind {
                EnemyType::Scout => {
                    self.shapes.set_color(c);
                    self.shapes.draw_rect(x - 10.0, y - 8.0, 20.0, 16.0);
                    self.shapes.set_color(Color::lerp(c, Color::BLACK, 0.3));
                    self.shapes.draw_rect(x - 4.0, y - 12.0, 8.0, 6.0);
                }
                EnemyType::Fighter => {
                    self.shapes.set_color(c);
                    self.shapes.draw_rect(x - 14.0, y - 10.0, 28.0, 20.0);
                    self.shapes.set_color(Color::lerp(c, Color::BLACK, 0.3));
                    self.shapes.draw_rect(x - 18.0, y - 6.0, 6.0, 12.0);
                    self.shapes.draw_rect(x + 12.0, y - 6.0, 6.0, 12.0);
                    self.shapes.set_color(Color::new(1.0, 0.3, 0.2, 0.8));
                    self.shapes.draw_rect(x - 3.0, y - 14.0, 6.0, 5.0);
                }
                EnemyType::Dreadnought => {
                    self.shapes.set_color(c);
                    self.shapes.draw_rect(x - 22.0, y - 14.0, 44.0, 28.0);
                    self.shapes
                        .set_color(Color::lerp(c, Color::new(0.2, 0.1, 0.1, 1.0), 0.3));
                    self.shapes.draw_rect(x - 26.0, y - 10.0, 6.0, 20.0);
                    self.shapes.draw_rect(x + 20.0, y - 10.0, 6.0, 20.0);
                    self.shapes.set_color(Color::new(1.0, 0.1, 0.1, 0.9));
                    self.shapes.draw_rect(x - 6.0, y - 18.0, 12.0, 5.0);
                    self.shapes.draw_rect(x - 3.0, y + 14.0, 6.0, 4.0);

                    let hp_frac = enemy.hp as f32 / enemy.max_hp as f32;
                    self.shapes.set_color(Color::new(0.2, 0.2, 0.25, 0.8));
                    self.shapes.draw_rect(x - 16.0, y + 18.0, 32.0, 3.0);
                    self.shapes.set_color(Color::new(0.2, 1.0, 0.3, 0.9));
                    self.shapes
                        .draw_rect(x - 16.0, y + 18.0, 32.0 * hp_frac, 3.0);
                }
            }
        }
    }

    fn render_bullets(&mut self) {
        for bullet in &self.bullets {
            if !bullet.alive {
                continue;
            }
            let c = if bullet.is_player {
                Color::new(0.3, 0.8, 1.0, 1.0)
            } else {
                Color::new(1.0, 0.4, 0.3, 1.0)
            };
            self.shapes.set_color(c);
            let s = bullet.size;
            self.shapes
                .draw_rect(bullet.x - s, bullet.y - s * 1.5, s * 2.0, s * 3.0);

            let glow_c = c.with_alpha(0.3);
            self.shapes.set_color(glow_c);
            self.shapes
                .draw_rect(bullet.x - s * 2.0, bullet.y - s * 2.0, s * 4.0, s * 4.0);
        }
    }

    fn render_powerups(&mut self) {
        for pu in &self.powerups {
            if !pu.alive {
                continue;
            }
            let c = powerup_color(pu.kind);
            let pulse = (pu.angle * 2.0).sin() * 0.2 + 0.8;
            self.shapes.set_color(c.with_alpha(pulse));
            self.shapes.draw_circle(pu.x, pu.y, 10.0, 8);

            self.shapes.set_color(Color::WHITE.with_alpha(0.7));
            match pu.kind {
                PowerUpType::Shield => {
                    self.shapes.draw_circle_outline(pu.x, pu.y, 6.0, 2.0, 12);
                }
                PowerUpType::TripleShot => {
                    self.shapes.draw_rect(pu.x - 5.0, pu.y - 1.0, 3.0, 3.0);
                    self.shapes.draw_rect(pu.x - 1.0, pu.y - 1.0, 3.0, 3.0);
                    self.shapes.draw_rect(pu.x + 3.0, pu.y - 1.0, 3.0, 3.0);
                }
                PowerUpType::RapidFire => {
                    self.shapes.draw_rect(pu.x - 1.5, pu.y - 5.0, 3.0, 10.0);
                    self.shapes.draw_rect(pu.x - 5.0, pu.y + 1.0, 10.0, 3.0);
                }
            }
        }
    }

    fn render_particles(&mut self) {
        for p in &self.particles {
            if p.life <= 0.0 {
                continue;
            }
            let t = p.life / p.max_life;
            let alpha = t * t;
            let size = p.size * t;
            let c = particle_color(p.color_idx).with_alpha(alpha);
            self.shapes.set_color(c);
            self.shapes
                .draw_rect(p.x - size * 0.5, p.y - size * 0.5, size, size);
        }
    }

    fn render_hud(&mut self) {
        let w = self.gl.width() as f32;
        let h = self.gl.height() as f32;

        self.shapes.set_color(Color::new(0.0, 0.0, 0.0, 0.4));
        self.shapes.draw_rect(0.0, h - 36.0, w, 36.0);

        self.shapes.set_color(Color::new(0.0, 0.0, 0.0, 0.3));
        self.shapes.draw_rect(0.0, 0.0, w, 30.0);

        for i in 0..self.player_lives {
            let lx = 20.0 + i as f32 * 22.0;
            let ly = h - 26.0;
            self.shapes.set_color(Color::new(0.3, 0.7, 1.0, 0.9));
            self.shapes.draw_rect(lx - 4.0, ly - 4.0, 8.0, 8.0);
            self.shapes.draw_rect(lx - 2.0, ly + 4.0, 4.0, 4.0);
        }

        if self.has_shield {
            let remaining = self.shield_timer / 8.0;
            self.shapes.set_color(Color::new(0.3, 0.7, 1.0, 0.6));
            self.shapes
                .draw_rect(100.0, h - 30.0, 60.0 * remaining, 6.0);
        }
        if self.triple_shot {
            let remaining = self.triple_shot_timer / 10.0;
            self.shapes.set_color(Color::new(1.0, 0.8, 0.2, 0.6));
            self.shapes
                .draw_rect(100.0, h - 20.0, 60.0 * remaining, 6.0);
        }
        if self.rapid_fire {
            let remaining = self.rapid_fire_timer / 8.0;
            self.shapes.set_color(Color::new(1.0, 0.3, 0.5, 0.6));
            self.shapes
                .draw_rect(100.0, h - 20.0, 60.0 * remaining, 6.0);
        }

        if self.combo > 1 {
            let combo_alpha = (self.combo_timer / 2.0).min(1.0);
            let pulse = (self.combo_timer * 10.0).sin() * 0.1 + 0.9;
            self.shapes
                .set_color(Color::new(1.0, 0.9, 0.2, combo_alpha * pulse));
            self.shapes.draw_rect(w / 2.0 - 40.0, 50.0, 80.0, 4.0);
        }
    }

    fn render_title(&mut self) {
        let w = self.gl.width() as f32;
        let h = self.gl.height() as f32;

        let pulse = (self.title_pulse).sin() * 0.15 + 0.85;

        let title_y = h * 0.35;
        self.shapes.set_color(Color::new(0.3, 0.6, 1.0, pulse));

        let tw = 180.0;
        let th = 20.0;
        self.shapes
            .draw_rect(w / 2.0 - tw / 2.0, title_y - th / 2.0, tw, th);
        self.shapes
            .set_color(Color::new(0.4, 0.7, 1.0, pulse * 0.7));
        self.shapes
            .draw_rect(w / 2.0 - tw * 0.35, title_y + th / 2.0, tw * 0.7, th * 0.6);
        self.shapes.set_color(Color::new(0.2, 0.5, 0.9, 0.5));
        self.shapes
            .draw_rect(w / 2.0 - 5.0, title_y - th, 10.0, th * 0.5);

        for i in 0..3 {
            let angle = self.title_pulse + i as f32 * std::f32::consts::TAU / 3.0;
            let radius = 50.0;
            let dx = angle.cos() * radius;
            let dy = angle.sin() * radius * 0.3;
            self.shapes
                .set_color(Color::new(1.0, 0.6, 0.2, 0.7 * pulse));
            self.shapes
                .draw_rect(w / 2.0 + dx - 6.0, title_y + 40.0 + dy - 6.0, 12.0, 12.0);
        }

        let blink = (self.title_pulse * 1.5).sin();
        if blink > -0.3 {
            self.shapes
                .set_color(Color::new(0.9, 0.9, 1.0, 0.6 + blink * 0.3));
            self.shapes.draw_rect(w / 2.0 - 80.0, h * 0.6, 160.0, 3.0);
        }

        let controls_y = h * 0.75;
        self.shapes.set_color(Color::new(0.5, 0.5, 0.6, 0.6));
        self.shapes
            .draw_rect(w / 2.0 - 60.0, controls_y, 120.0, 2.0);
        self.shapes
            .draw_rect(w / 2.0 - 60.0, controls_y + 22.0, 120.0, 2.0);
        self.shapes
            .draw_rect(w / 2.0 - 60.0, controls_y + 44.0, 120.0, 2.0);

        self.shapes.set_color(Color::new(0.7, 0.8, 0.9, 0.5));
        self.shapes
            .draw_rect(w / 2.0 - 50.0, controls_y + 60.0, 100.0, 2.0);

        if self.high_score > 0 {
            self.shapes.set_color(Color::new(1.0, 0.85, 0.3, 0.7));
            self.shapes
                .draw_rect(w / 2.0 - 30.0, controls_y + 75.0, 60.0, 2.0);
        }
    }

    fn render_game_over(&mut self) {
        let w = self.gl.width() as f32;
        let h = self.gl.height() as f32;

        let alpha = (self.game_over_timer / 0.5).min(1.0);

        self.shapes
            .set_color(Color::new(0.0, 0.0, 0.0, 0.5 * alpha));
        self.shapes.draw_rect(0.0, 0.0, w, h);

        let center_y = h * 0.4;

        self.shapes
            .set_color(Color::new(1.0, 0.2, 0.2, alpha * 0.9));
        self.shapes
            .draw_rect(w / 2.0 - 80.0, center_y - 8.0, 160.0, 16.0);
        self.shapes
            .set_color(Color::new(0.8, 0.1, 0.1, alpha * 0.6));
        self.shapes
            .draw_rect(w / 2.0 - 60.0, center_y + 10.0, 120.0, 8.0);

        self.shapes
            .set_color(Color::new(1.0, 0.85, 0.3, alpha * 0.9));
        self.shapes
            .draw_rect(w / 2.0 - 40.0, center_y + 50.0, 80.0, 3.0);

        self.shapes
            .set_color(Color::new(0.7, 0.8, 1.0, alpha * 0.8));
        self.shapes
            .draw_rect(w / 2.0 - 30.0, center_y + 75.0, 60.0, 3.0);

        if self.game_over_timer > 1.5 {
            let blink = (self.game_over_timer * 3.0).sin();
            if blink > 0.0 {
                self.shapes
                    .set_color(Color::new(0.9, 0.9, 1.0, alpha * 0.5 * blink));
                self.shapes
                    .draw_rect(w / 2.0 - 60.0, center_y + 110.0, 120.0, 3.0);
            }
        }
    }

    fn render_title_stars_decoration(&mut self) {
        let w = self.gl.width() as f32;
        let h = self.gl.height() as f32;
        let t = self.title_pulse;

        for i in 0..8 {
            let angle = t * 0.3 + i as f32 * std::f32::consts::TAU / 8.0;
            let radius = 120.0 + (t * 0.5 + i as f32).sin() * 20.0;
            let x = w / 2.0 + angle.cos() * radius;
            let y = h * 0.35 + angle.sin() * radius * 0.4;
            let alpha = 0.3 + (t + i as f32).sin().abs() * 0.3;
            let size = 2.0 + (t * 2.0 + i as f32).sin().abs() * 2.0;

            self.shapes.set_color(Color::new(0.4, 0.6, 1.0, alpha));
            self.shapes
                .draw_rect(x - size * 0.5, y - size * 0.5, size, size);
        }
    }

    fn render(&mut self, _alpha: f32) {
        self.gl.resize();
        let w = self.gl.width() as f32;
        let h = self.gl.height() as f32;
        self.camera.set_viewport(w, h);

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

        self.camera.position = Vec2::new(w / 2.0 + shake_x, h / 2.0 + shake_y);

        let bg_r = 0.03 + self.bg_hue.sin() * 0.01;
        let bg_g = 0.03 + (self.bg_hue + 2.0).sin() * 0.01;
        let bg_b = 0.06 + (self.bg_hue + 4.0).sin() * 0.015;
        self.gl.clear(bg_r, bg_g, bg_b, 1.0);
        self.gl.enable_blend();

        let vp = self.camera.view_projection();
        self.shapes.begin();

        self.render_stars();

        match self.state {
            GameState::Title => {
                self.render_title_stars_decoration();
                self.render_title();
            }
            GameState::Playing => {
                self.render_particles();
                self.render_powerups();
                self.render_bullets();
                self.render_enemies();
                self.render_player();
                self.render_hud();
            }
            GameState::GameOver => {
                self.render_particles();
                self.render_bullets();
                self.render_enemies();
                self.render_hud();
                self.render_game_over();
            }
        }

        self.shapes.flush(self.gl.gl(), &vp);
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    let mut game = SpaceBlitz::new().expect("Failed to create Space Blitz");
    game.time.init();

    let game = Rc::new(RefCell::new(game));
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
