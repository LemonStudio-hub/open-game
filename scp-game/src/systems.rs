use opengame_engine::ecs::{World, QuerySingle};
use opengame_engine::math::Vec2;
use opengame_engine::physics::{PhysicsSystem, Collider, collider::ColliderShape};
use opengame_engine::transform::Transform2D;

use crate::components::*;
use crate::resources::*;
use crate::{rand, rand_range, PLAYER_SIZE, PLAYER_SPEED, JUMP_FORCE, BULLET_SPEED,
    ENEMY_BULLET_SPEED, SHOOT_INTERVAL, INVINCIBLE_TIME, GROUND_Y, WORLD_W, WORLD_H, LEVEL_W,
    MAX_BULLETS, MAX_ENEMIES, MAX_PARTICLES, CAM_DEAD_ZONE_X, CAM_SMOOTH};

const LAYER_PLAYER: u32 = 1;
const LAYER_ENEMY: u32 = 2;
const LAYER_PLAYER_BULLET: u32 = 4;
const LAYER_ENEMY_BULLET: u32 = 8;
const LAYER_GROUND: u32 = 16;

struct HitEvent { x: f32, y: f32, shake: f32, score: i32, hit_player: bool }

// ── Physics Sync Down: Ground Detection ──────────────────────────────────────

pub fn physics_sync_down(world: &mut World, physics: &PhysicsSystem) {
    // Reset on_ground flags
    {
        let entities: Vec<_> = QuerySingle::<Player>::new(world)
            .map(|q| q.iter().map(|(e, _)| e).collect())
            .unwrap_or_default();
        for e in entities {
            if let Some(p) = world.get_component_mut::<Player>(e) {
                p.on_ground = false;
            }
        }
    }
    {
        let entities: Vec<_> = QuerySingle::<Enemy>::new(world)
            .map(|q| q.iter().map(|(e, _)| e).collect())
            .unwrap_or_default();
        for e in entities {
            if let Some(en) = world.get_component_mut::<Enemy>(e) {
                en.on_ground = false;
            }
        }
    }

    // Check physics collisions for ground contacts
    for col in &physics.collisions {
        let a_ground = world.has_component::<Ground>(col.entity_a);
        let b_ground = world.has_component::<Ground>(col.entity_b);

        if a_ground {
            let other = col.entity_b;
            if let Some(p) = world.get_component_mut::<Player>(other) { p.on_ground = true; }
            if let Some(e) = world.get_component_mut::<Enemy>(other) { e.on_ground = true; }
        }
        if b_ground {
            let other = col.entity_a;
            if let Some(p) = world.get_component_mut::<Player>(other) { p.on_ground = true; }
            if let Some(e) = world.get_component_mut::<Enemy>(other) { e.on_ground = true; }
        }
    }
}

// ── Player Movement ──────────────────────────────────────────────────────────

pub fn player_move_system(world: &mut World, dt: f32) {
    let gs = world.get_resource::<GameStateRes>().unwrap();
    if gs.state != GameState::Playing { return; }

    let input = world.get_resource::<InputState>().unwrap();
    let left = input.left;
    let right = input.right;
    let jump = input.jump_pressed;

    let entities: Vec<_> = QuerySingle::<Player>::new(world)
        .map(|q| q.iter().map(|(e, _)| e).collect())
        .unwrap_or_default();

    for entity in entities {
        let (facing_right, on_ground, shoot_timer) = {
            match world.get_component::<Player>(entity) {
                Some(p) => (p.facing_right, p.on_ground, p.shoot_timer),
                None => continue,
            }
        };

        // Horizontal velocity
        let mut vx: f32 = 0.0;
        let mut new_facing = facing_right;
        if left { vx = -PLAYER_SPEED; new_facing = false; }
        if right { vx = PLAYER_SPEED; new_facing = true; }

        if let Some(rb) = world.get_component_mut::<opengame_engine::physics::RigidBody>(entity) {
            rb.velocity.x = vx;
        }

        // Jump
        if jump && on_ground {
            if let Some(rb) = world.get_component_mut::<opengame_engine::physics::RigidBody>(entity) {
                rb.apply_impulse(Vec2::new(0.0, -JUMP_FORCE));
            }
        }

        // Update player state
        if let Some(p) = world.get_component_mut::<Player>(entity) {
            p.facing_right = new_facing;
            p.shoot_timer = (shoot_timer - dt).max(0.0);
            if jump && on_ground { p.on_ground = false; }
            if p.invincible > 0.0 {
                p.invincible -= dt;
                p.flash += dt * 15.0;
            }
        }
    }
}

// ── Player Shoot ─────────────────────────────────────────────────────────────

pub fn player_shoot_system(world: &mut World, _dt: f32) {
    let gs = world.get_resource::<GameStateRes>().unwrap();
    if gs.state != GameState::Playing { return; }

    let input = world.get_resource::<InputState>().unwrap();
    if !input.shoot_down { return; }

    // Find player
    let player_entity = QuerySingle::<Player>::new(world)
        .and_then(|q| q.iter().next().map(|(e, _)| e));

    let entity = match player_entity {
        Some(e) => e,
        None => return,
    };

    let (facing_right, can_shoot) = {
        match world.get_component::<Player>(entity) {
            Some(p) => (p.facing_right, p.shoot_timer <= 0.0),
            None => return,
        }
    };

    if !can_shoot { return; }

    let (px, py) = {
        match world.get_component::<Transform2D>(entity) {
            Some(t) => (t.position.x, t.position.y),
            None => return,
        }
    };

    // Set cooldown
    if let Some(p) = world.get_component_mut::<Player>(entity) {
        p.shoot_timer = SHOOT_INTERVAL;
    }

    // Count bullets
    let bullet_count = QuerySingle::<Bullet>::new(world)
        .map(|q| q.iter().filter(|(_, b)| b.is_player).count())
        .unwrap_or(0);
    if bullet_count >= MAX_BULLETS { return; }

    let bx = if facing_right { px + PLAYER_SIZE } else { px - 8.0 };
    let bvx = if facing_right { BULLET_SPEED } else { -BULLET_SPEED };

    world.spawn()
        .with(Bullet { x: bx, y: py + PLAYER_SIZE * 0.4, vx: bvx, vy: 0.0, alive: true, is_player: true })
        .with(Transform2D::new(Vec2::new(bx, py + PLAYER_SIZE * 0.4)))
        .with(opengame_engine::physics::RigidBody::dynamic().with_gravity_scale(0.0))
        .with(Collider {
            shape: ColliderShape::Rectangle { width: 10.0, height: 4.0 },
            is_trigger: true, layer: LAYER_PLAYER_BULLET, mask: LAYER_ENEMY,
            offset: Vec2::ZERO, friction: 0.0, restitution: 0.0,
        })
        .build();
}

// ── Enemy Spawn ──────────────────────────────────────────────────────────────

pub fn enemy_spawn_system(world: &mut World, dt: f32) {
    let gs = world.get_resource::<GameStateRes>().unwrap();
    if gs.state != GameState::Playing { return; }

    {
        let spawn = world.get_resource_mut::<SpawnRes>().unwrap();
        spawn.difficulty_timer += dt;
        if spawn.difficulty_timer > 10.0 {
            spawn.difficulty_timer = 0.0;
            spawn.spawn_interval = (spawn.spawn_interval * 0.88).max(0.4);
        }
        spawn.spawn_timer -= dt;
    }

    let should_spawn = world.get_resource::<SpawnRes>().unwrap().spawn_timer <= 0.0;
    if !should_spawn { return; }

    let enemy_count = QuerySingle::<Enemy>::new(world)
        .map(|q| q.len())
        .unwrap_or(0);

    if enemy_count < MAX_ENEMIES {
        let camera_x = world.get_resource::<CameraRes>().unwrap().camera_x;
        let size = rand_range(24.0, 34.0);
        let spawn_x = camera_x - size;

        world.spawn()
            .with(Enemy { hp: 2, alive: true, on_ground: true, shoot_timer: rand_range(0.8, 2.0), ai_timer: rand_range(0.5, 1.5), flash: 0.0, size })
            .with(Transform2D::new(Vec2::new(spawn_x, GROUND_Y - size)))
            .with(opengame_engine::physics::RigidBody::dynamic().with_velocity(Vec2::new(rand_range(60.0, 130.0), 0.0)))
            .with(Collider {
                shape: ColliderShape::Rectangle { width: size, height: size },
                layer: LAYER_ENEMY, mask: LAYER_PLAYER | LAYER_PLAYER_BULLET | LAYER_GROUND,
                offset: Vec2::ZERO, friction: 0.0, restitution: 0.0, is_trigger: false,
            })
            .build();
    }

    let spawn = world.get_resource_mut::<SpawnRes>().unwrap();
    spawn.spawn_timer = spawn.spawn_interval;
}

// ── Enemy AI ─────────────────────────────────────────────────────────────────

pub fn enemy_ai_system(world: &mut World, dt: f32) {
    let gs = world.get_resource::<GameStateRes>().unwrap();
    if gs.state != GameState::Playing { return; }

    // Read player position
    let player_pos = QuerySingle::<Player>::new(world)
        .and_then(|q| q.iter().next().map(|(e, _)| e))
        .and_then(|e| world.get_component::<Transform2D>(e))
        .map(|t| (t.position.x, t.position.y));

    let (player_x, player_y) = match player_pos {
        Some(p) => p,
        None => return,
    };

    struct ShootEvent { x: f32, y: f32, vx: f32, vy: f32 }
    let mut shoot_events: Vec<ShootEvent> = Vec::new();

    // Collect enemy entities
    let entities: Vec<_> = QuerySingle::<Enemy>::new(world)
        .map(|q| q.iter().filter(|(_, e)| e.alive).map(|(e, _)| e).collect())
        .unwrap_or_default();

    for entity in entities {
        // Read enemy state
        let (mut ai_timer, shoot_timer, size, on_ground, flash) = {
            match world.get_component::<Enemy>(entity) {
                Some(e) => (e.ai_timer, e.shoot_timer, e.size, e.on_ground, e.flash),
                None => continue,
            }
        };

        let (ex, ey) = {
            match world.get_component::<Transform2D>(entity) {
                Some(t) => (t.position.x, t.position.y),
                None => continue,
            }
        };

        // AI timer tick
        ai_timer -= dt;
        if ai_timer <= 0.0 {
            ai_timer = rand_range(0.4, 1.0);
            let dx = player_x - ex;
            let dist = dx.abs();

            let vx = if dist > 60.0 {
                if dx > 0.0 { rand_range(70.0, 140.0) } else { -rand_range(70.0, 140.0) }
            } else { 0.0 };

            if let Some(rb) = world.get_component_mut::<opengame_engine::physics::RigidBody>(entity) {
                rb.velocity.x = vx;
            }

            if player_y < ey - 40.0 && on_ground && rand() < 0.35 {
                if let Some(rb) = world.get_component_mut::<opengame_engine::physics::RigidBody>(entity) {
                    rb.apply_impulse(Vec2::new(0.0, -480.0));
                }
            }
        }

        // Shooting
        let mut new_shoot_timer = shoot_timer - dt;
        if new_shoot_timer <= 0.0 {
            new_shoot_timer = rand_range(1.2, 2.8);
            let dx = player_x + PLAYER_SIZE * 0.5 - ex;
            let dy = player_y + PLAYER_SIZE * 0.5 - ey;
            let dist = (dx * dx + dy * dy).sqrt();
            let (bvx, bvy) = if dist > 1.0 {
                (dx / dist * ENEMY_BULLET_SPEED, dy / dist * ENEMY_BULLET_SPEED)
            } else { (-ENEMY_BULLET_SPEED, 0.0) };
            shoot_events.push(ShootEvent { x: ex + size * 0.5, y: ey + size * 0.5, vx: bvx, vy: bvy });
        }

        // Update enemy state
        if let Some(e) = world.get_component_mut::<Enemy>(entity) {
            e.ai_timer = ai_timer;
            e.shoot_timer = new_shoot_timer;
            e.flash = (flash - dt * 5.0).max(0.0);
        }

        // Despawn check
        let cam_x = world.get_resource::<CameraRes>().unwrap().camera_x;
        if ex < cam_x - 200.0 {
            if let Some(e) = world.get_component_mut::<Enemy>(entity) {
                e.alive = false;
            }
        }

        // Clamp to level
        if let Some(t) = world.get_component_mut::<Transform2D>(entity) {
            t.position.x = t.position.x.clamp(0.0, LEVEL_W - size);
        }
    }

    // Spawn enemy bullets
    let bullet_count = QuerySingle::<Bullet>::new(world)
        .map(|q| q.iter().filter(|(_, b)| !b.is_player).count())
        .unwrap_or(0);

    let mut spawned = 0;
    for ev in shoot_events {
        if bullet_count + spawned >= MAX_BULLETS { break; }
        world.spawn()
            .with(Bullet { x: ev.x, y: ev.y, vx: ev.vx, vy: ev.vy, alive: true, is_player: false })
            .with(Transform2D::new(Vec2::new(ev.x, ev.y)))
            .with(opengame_engine::physics::RigidBody::dynamic().with_gravity_scale(0.0))
            .with(Collider {
                shape: ColliderShape::Rectangle { width: 6.0, height: 4.0 },
                is_trigger: true, layer: LAYER_ENEMY_BULLET, mask: LAYER_PLAYER,
                offset: Vec2::ZERO, friction: 0.0, restitution: 0.0,
            })
            .build();
        spawned += 1;
    }
}

// ── Bullet Movement ──────────────────────────────────────────────────────────

pub fn bullet_move_system(world: &mut World, dt: f32) {
    let gs = world.get_resource::<GameStateRes>().unwrap();
    if gs.state != GameState::Playing { return; }

    let entities: Vec<_> = QuerySingle::<Bullet>::new(world)
        .map(|q| q.iter().map(|(e, _)| e).collect())
        .unwrap_or_default();

    for entity in entities {
        let (alive, vx, vy) = {
            match world.get_component::<Bullet>(entity) {
                Some(b) => (b.alive, b.vx, b.vy),
                None => continue,
            }
        };
        if !alive { continue; }

        let (new_x, new_y) = {
            match world.get_component::<Bullet>(entity) {
                Some(b) => (b.x + vx * dt, b.y + vy * dt),
                None => continue,
            }
        };

        // Update bullet position
        if let Some(b) = world.get_component_mut::<Bullet>(entity) {
            b.x = new_x;
            b.y = new_y;
            if new_x < -10.0 || new_x > LEVEL_W + 10.0 || new_y < -10.0 || new_y > WORLD_H + 10.0 {
                b.alive = false;
            }
        }

        // Sync to Transform2D
        if let Some(t) = world.get_component_mut::<Transform2D>(entity) {
            t.position.x = new_x;
            t.position.y = new_y;
        }
    }
}

// ── Particle Update ──────────────────────────────────────────────────────────

pub fn particle_update_system(world: &mut World, dt: f32) {
    let entities: Vec<_> = QuerySingle::<Particle>::new(world)
        .map(|q| q.iter().map(|(e, _)| e).collect())
        .unwrap_or_default();

    for entity in entities {
        if let Some(p) = world.get_component_mut::<Particle>(entity) {
            if p.life <= 0.0 { continue; }
            p.x += p.vx * dt;
            p.y += p.vy * dt;
            p.vx *= 0.98;
            p.vy *= 0.98;
            p.life -= dt;
        }
    }
}

// ── Collision Handler ────────────────────────────────────────────────────────

pub fn collision_handler(world: &mut World, physics: &PhysicsSystem) {
    let gs = world.get_resource::<GameStateRes>().unwrap();
    if gs.state != GameState::Playing { return; }

    let mut events: Vec<HitEvent> = Vec::new();

    for col in &physics.collisions {
        let a_has_bullet = world.has_component::<Bullet>(col.entity_a);
        let b_has_bullet = world.has_component::<Bullet>(col.entity_b);
        let a_has_player = world.has_component::<Player>(col.entity_a);
        let b_has_player = world.has_component::<Player>(col.entity_b);
        let a_has_enemy = world.has_component::<Enemy>(col.entity_a);
        let b_has_enemy = world.has_component::<Enemy>(col.entity_b);
        let a_is_trigger = world.get_component::<Collider>(col.entity_a).map(|c| c.is_trigger).unwrap_or(false);
        let b_is_trigger = world.get_component::<Collider>(col.entity_b).map(|c| c.is_trigger).unwrap_or(false);

        // Trigger collisions (bullets)
        if a_is_trigger && a_has_bullet {
            handle_bullet_hit(world, col.entity_a, col.entity_b, &mut events);
        } else if b_is_trigger && b_has_bullet {
            handle_bullet_hit(world, col.entity_b, col.entity_a, &mut events);
        }

        // Non-trigger: player-enemy contact
        if !a_is_trigger && !b_is_trigger && !a_has_bullet && !b_has_bullet {
            if (a_has_player && b_has_enemy) || (b_has_player && a_has_enemy) {
                let invincible = world.get_component::<Player>(if a_has_player { col.entity_a } else { col.entity_b })
                    .map(|p| p.invincible > 0.0).unwrap_or(false);

                if !invincible {
                    let enemy_entity = if a_has_enemy { col.entity_a } else { col.entity_b };
                    let (ex, ey) = world.get_component::<Transform2D>(enemy_entity)
                        .map(|t| (t.position.x, t.position.y)).unwrap_or((0.0, 0.0));
                    events.push(HitEvent { x: ex, y: ey, shake: 14.0, score: 0, hit_player: true });
                    if let Some(e) = world.get_component_mut::<Enemy>(enemy_entity) {
                        e.alive = false;
                    }
                }
            }
        }
    }

    // Apply events
    for ev in events {
        let count = if ev.score > 0 { 22 } else { 15 };
        let power = if ev.score > 0 { 250.0 } else { 200.0 };
        spawn_explosion_particles(world, ev.x, ev.y, count, power);

        let cam = world.get_resource_mut::<CameraRes>().unwrap();
        cam.shake_amount = (cam.shake_amount + ev.shake).min(18.0);

        if ev.score > 0 {
            world.get_resource_mut::<ScoreRes>().unwrap().score += ev.score;
        }

        if ev.hit_player {
            let lives = world.get_resource_mut::<LivesRes>().unwrap();
            lives.lives -= 1;

            if lives.lives <= 0 {
                // Get player pos for death explosion
                let pos = QuerySingle::<Player>::new(world)
                    .and_then(|q| q.iter().next().map(|(e, _)| e))
                    .and_then(|e| world.get_component::<Transform2D>(e))
                    .map(|t| (t.position.x, t.position.y))
                    .unwrap_or((400.0, 300.0));
                spawn_explosion_particles(world, pos.0, pos.1, 45, 350.0);
                world.get_resource_mut::<GameStateRes>().unwrap().state = GameState::GameOver;
                world.get_resource_mut::<GameStateRes>().unwrap().game_over_timer = 0.0;
            } else {
                let entities: Vec<_> = QuerySingle::<Player>::new(world)
                    .map(|q| q.iter().map(|(e, _)| e).collect())
                    .unwrap_or_default();
                for e in entities {
                    if let Some(p) = world.get_component_mut::<Player>(e) {
                        p.invincible = INVINCIBLE_TIME;
                        p.flash = 0.0;
                    }
                }
            }
        }
    }
}

fn handle_bullet_hit(world: &mut World, bullet_entity: opengame_engine::ecs::Entity, target_entity: opengame_engine::ecs::Entity, events: &mut Vec<HitEvent>) {
    let is_player_bullet = world.get_component::<Bullet>(bullet_entity)
        .map(|b| b.is_player).unwrap_or(false);

    // Mark bullet dead
    if let Some(b) = world.get_component_mut::<Bullet>(bullet_entity) {
        b.alive = false;
    }

    if is_player_bullet {
        // Hit enemy
        if let Some(enemy) = world.get_component_mut::<Enemy>(target_entity) {
            if !enemy.alive { return; }
            enemy.hp -= 1;
            enemy.flash = 1.0;
            if enemy.hp <= 0 {
                enemy.alive = false;
                let (ex, ey) = world.get_component::<Transform2D>(target_entity)
                    .map(|t| (t.position.x, t.position.y)).unwrap_or((0.0, 0.0));
                events.push(HitEvent { x: ex, y: ey, shake: 6.0, score: 100, hit_player: false });
            }
        }
    } else {
        // Hit player
        let invincible = world.get_component::<Player>(target_entity)
            .map(|p| p.invincible > 0.0).unwrap_or(false);
        if !invincible {
            let (px, py) = world.get_component::<Transform2D>(target_entity)
                .map(|t| (t.position.x, t.position.y)).unwrap_or((0.0, 0.0));
            events.push(HitEvent { x: px, y: py, shake: 14.0, score: 0, hit_player: true });
        }
    }
}

// ── Camera ───────────────────────────────────────────────────────────────────

pub fn camera_system(world: &mut World, dt: f32) {
    let gs = world.get_resource::<GameStateRes>().unwrap();
    if gs.state != GameState::Playing && gs.state != GameState::GameOver { return; }

    let target_x = QuerySingle::<Player>::new(world)
        .and_then(|q| q.iter().next().map(|(e, _)| e))
        .and_then(|e| world.get_component::<Transform2D>(e))
        .map(|t| t.position.x + PLAYER_SIZE * 0.5);

    let target_x = match target_x {
        Some(x) => x,
        None => return,
    };

    let cam = world.get_resource_mut::<CameraRes>().unwrap();

    if cam.shake_amount > 0.0 {
        cam.shake_amount = (cam.shake_amount - 6.0 * dt).max(0.0);
    }

    let cam_center = cam.camera_x + WORLD_W * 0.5;
    let diff = target_x - cam_center;

    let desired_x = if diff > CAM_DEAD_ZONE_X {
        target_x - CAM_DEAD_ZONE_X - WORLD_W * 0.5
    } else if diff < -CAM_DEAD_ZONE_X {
        target_x + CAM_DEAD_ZONE_X - WORLD_W * 0.5
    } else {
        cam.camera_x
    };

    cam.camera_x += (desired_x - cam.camera_x) * CAM_SMOOTH * dt;
    cam.camera_x = cam.camera_x.clamp(0.0, LEVEL_W - WORLD_W);
    cam.camera_y = 0.0;
}

// ── Cleanup ──────────────────────────────────────────────────────────────────

pub fn cleanup_system(world: &mut World) {
    let mut to_despawn: Vec<opengame_engine::ecs::Entity> = Vec::new();

    if let Some(q) = QuerySingle::<Bullet>::new(world) {
        for (e, b) in q.iter() { if !b.alive { to_despawn.push(e); } }
    }
    if let Some(q) = QuerySingle::<Enemy>::new(world) {
        for (e, en) in q.iter() { if !en.alive { to_despawn.push(e); } }
    }
    if let Some(q) = QuerySingle::<Particle>::new(world) {
        for (e, p) in q.iter() { if p.life <= 0.0 { to_despawn.push(e); } }
    }

    for e in to_despawn {
        world.despawn(e);
    }
}

// ── Helpers ──────────────────────────────────────────────────────────────────

fn spawn_explosion_particles(world: &mut World, x: f32, y: f32, count: usize, power: f32) {
    let current = QuerySingle::<Particle>::new(world)
        .map(|q| q.len())
        .unwrap_or(0);

    for i in 0..count {
        if current + i >= MAX_PARTICLES { break; }
        let angle = rand() * std::f32::consts::TAU;
        let speed = rand_range(60.0, power);
        let life = rand_range(0.3, 0.9);
        world.spawn()
            .with(Particle {
                x, y,
                vx: angle.cos() * speed,
                vy: angle.sin() * speed,
                life,
                max_life: life,
                size: rand_range(3.0, 7.0),
                color_idx: (i % 7) as u8,
            })
            .build();
    }
}
