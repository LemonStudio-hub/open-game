use opengame_engine::ecs::{World, QuerySingle, QuerySingleMut};
use opengame_engine::ecs::system::SystemScheduler;

use crate::components::*;
use crate::resources::*;
use crate::{rand, rand_range, GRAVITY, PLAYER_SIZE, PLAYER_SPEED, JUMP_FORCE, BULLET_SPEED,
    ENEMY_BULLET_SPEED, SHOOT_INTERVAL, INVINCIBLE_TIME, GROUND_Y, WORLD_W, WORLD_H, LEVEL_W,
    MAX_BULLETS, MAX_ENEMIES, MAX_PARTICLES, CAM_DEAD_ZONE_X, CAM_SMOOTH};

/// Register all game systems with the scheduler.
pub fn register_systems(scheduler: &mut SystemScheduler) {
    scheduler.add_system(player_move_system);
    scheduler.add_system(player_shoot_system);
    scheduler.add_system(enemy_spawn_system);
    scheduler.add_system(enemy_ai_system);
    scheduler.add_system(bullet_move_system);
    scheduler.add_system(particle_update_system);
    scheduler.add_system(collision_system);
    scheduler.add_system(camera_system);
    scheduler.add_system(cleanup_system);
}

// ── Player Movement ─────────────────────────────────────────────────────────

fn player_move_system(world: &mut World, dt: f32) {
    let state = world.get_resource::<GameStateRes>().unwrap();
    if state.state != GameState::Playing { return; }

    let input = world.get_resource::<InputState>().unwrap();
    let left = input.left;
    let right = input.right;
    let jump = input.jump_pressed;

    let mut query = QuerySingleMut::<Player>::new(world).unwrap();
    for (_entity, player) in query.iter_mut() {
        // Horizontal movement
        let move_speed = PLAYER_SPEED * dt;
        if left {
            player.x -= move_speed;
            player.facing_right = false;
        }
        if right {
            player.x += move_speed;
            player.facing_right = true;
        }
        player.x = player.x.clamp(0.0, LEVEL_W - PLAYER_SIZE);

        // Jump
        if jump && player.on_ground {
            player.vy = -JUMP_FORCE;
            player.on_ground = false;
        }

        // Gravity
        player.vy += GRAVITY * dt;
        player.y += player.vy * dt;

        // Ground collision
        if player.y >= GROUND_Y - PLAYER_SIZE {
            player.y = GROUND_Y - PLAYER_SIZE;
            player.vy = 0.0;
            player.on_ground = true;
        }

        // Shoot cooldown
        player.shoot_timer = (player.shoot_timer - dt).max(0.0);

        // Invincibility
        if player.invincible > 0.0 {
            player.invincible -= dt;
            player.flash += dt * 15.0;
        }
    }
}

// ── Player Shoot ────────────────────────────────────────────────────────────

fn player_shoot_system(world: &mut World, _dt: f32) {
    let state = world.get_resource::<GameStateRes>().unwrap();
    if state.state != GameState::Playing { return; }

    let input = world.get_resource::<InputState>().unwrap();
    if !input.shoot_down { return; }

    // Read player state
    let (px, py, facing_right, can_shoot) = {
        let query = QuerySingle::<Player>::new(world);
        match query {
            Some(q) => {
                if let Some((_e, p)) = q.iter().next() {
                    (p.x, p.y, p.facing_right, p.shoot_timer <= 0.0)
                } else { return; }
            }
            None => return,
        }
    };

    if !can_shoot { return; }

    // Set cooldown
    {
        let mut query = QuerySingleMut::<Player>::new(world).unwrap();
        for (_e, player) in query.iter_mut() {
            player.shoot_timer = SHOOT_INTERVAL;
        }
    }

    // Count existing player bullets
    let bullet_count = {
        let query = QuerySingle::<Bullet>::new(world);
        match query {
            Some(q) => q.iter().filter(|(_, b)| b.is_player).count(),
            None => 0,
        }
    };

    if bullet_count >= MAX_BULLETS { return; }

    let bx = if facing_right { px + PLAYER_SIZE } else { px - 8.0 };
    let bvx = if facing_right { BULLET_SPEED } else { -BULLET_SPEED };

    world.spawn()
        .with(Bullet {
            x: bx,
            y: py + PLAYER_SIZE * 0.4,
            vx: bvx,
            vy: 0.0,
            alive: true,
            is_player: true,
        })
        .build();
}

// ── Enemy Spawn ─────────────────────────────────────────────────────────────

fn enemy_spawn_system(world: &mut World, dt: f32) {
    let state = world.get_resource::<GameStateRes>().unwrap();
    if state.state != GameState::Playing { return; }

    // Difficulty ramp
    {
        let spawn = world.get_resource_mut::<SpawnRes>().unwrap();
        spawn.difficulty_timer += dt;
        if spawn.difficulty_timer > 10.0 {
            spawn.difficulty_timer = 0.0;
            spawn.spawn_interval = (spawn.spawn_interval * 0.88).max(0.4);
        }
        spawn.spawn_timer -= dt;
    }

    let should_spawn = {
        let spawn = world.get_resource::<SpawnRes>().unwrap();
        spawn.spawn_timer <= 0.0
    };

    if !should_spawn { return; }

    let enemy_count = {
        let query = QuerySingle::<Enemy>::new(world);
        match query {
            Some(q) => q.len(),
            None => 0,
        }
    };

    if enemy_count < MAX_ENEMIES {
        let camera_x = world.get_resource::<CameraRes>().unwrap().camera_x;
        let size = rand_range(24.0, 34.0);

        world.spawn()
            .with(Enemy {
                x: camera_x - size,
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
            })
            .build();
    }

    // Reset timer
    let spawn = world.get_resource_mut::<SpawnRes>().unwrap();
    spawn.spawn_timer = spawn.spawn_interval;
}

// ── Enemy AI ────────────────────────────────────────────────────────────────

fn enemy_ai_system(world: &mut World, dt: f32) {
    let state = world.get_resource::<GameStateRes>().unwrap();
    if state.state != GameState::Playing { return; }

    // Read player position
    let (player_x, player_y) = {
        let query = QuerySingle::<Player>::new(world);
        match query {
            Some(q) => {
                if let Some((_e, p)) = q.iter().next() {
                    (p.x, p.y)
                } else { return; }
            }
            None => return,
        }
    };

    // Collect enemy shoot events (can't spawn bullets while iterating enemies)
    struct ShootEvent { x: f32, y: f32, vx: f32, vy: f32 }
    let mut shoot_events: Vec<ShootEvent> = Vec::new();

    {
        let mut query = QuerySingleMut::<Enemy>::new(world).unwrap();
        for (_entity, enemy) in query.iter_mut() {
            if !enemy.alive { continue; }

            enemy.flash = (enemy.flash - dt * 5.0).max(0.0);

            // AI timer
            enemy.ai_timer -= dt;
            if enemy.ai_timer <= 0.0 {
                enemy.ai_timer = rand_range(0.4, 1.0);
                let dx = player_x - enemy.x;
                let dist = dx.abs();

                if dist > 60.0 {
                    enemy.vx = if dx > 0.0 { rand_range(70.0, 140.0) } else { -rand_range(70.0, 140.0) };
                } else {
                    enemy.vx = 0.0;
                }

                if player_y < enemy.y - 40.0 && enemy.on_ground && rand() < 0.35 {
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

            enemy.x = enemy.x.clamp(0.0, LEVEL_W - enemy.size);

            // Shooting
            enemy.shoot_timer -= dt;
            if enemy.shoot_timer <= 0.0 {
                enemy.shoot_timer = rand_range(1.2, 2.8);
                let dx = player_x + PLAYER_SIZE * 0.5 - enemy.x;
                let dy = player_y + PLAYER_SIZE * 0.5 - enemy.y;
                let dist = (dx * dx + dy * dy).sqrt();
                let (bvx, bvy) = if dist > 1.0 {
                    let speed = ENEMY_BULLET_SPEED;
                    (dx / dist * speed, dy / dist * speed)
                } else {
                    (-ENEMY_BULLET_SPEED, 0.0)
                };
                shoot_events.push(ShootEvent {
                    x: enemy.x + enemy.size * 0.5,
                    y: enemy.y + enemy.size * 0.5,
                    vx: bvx,
                    vy: bvy,
                });
            }

            // Despawn if far behind camera
            let cam_x = 0.0; // placeholder — read below
            if enemy.x < cam_x - 200.0 {
                enemy.alive = false;
            }
        }
    }

    // Fix despawn: read camera_x and re-check (can't read resource while iterating)
    let camera_x = world.get_resource::<CameraRes>().unwrap().camera_x;
    {
        let mut query = QuerySingleMut::<Enemy>::new(world).unwrap();
        for (_entity, enemy) in query.iter_mut() {
            if enemy.alive && enemy.x < camera_x - 200.0 {
                enemy.alive = false;
            }
        }
    }

    // Spawn enemy bullets
    let bullet_count = {
        let query = QuerySingle::<Bullet>::new(world);
        match query {
            Some(q) => q.iter().filter(|(_, b)| !b.is_player).count(),
            None => 0,
        }
    };

    let mut spawned = 0;
    for ev in shoot_events {
        if bullet_count + spawned >= MAX_BULLETS { break; }
        world.spawn()
            .with(Bullet {
                x: ev.x,
                y: ev.y,
                vx: ev.vx,
                vy: ev.vy,
                alive: true,
                is_player: false,
            })
            .build();
        spawned += 1;
    }
}

// ── Bullet Movement ─────────────────────────────────────────────────────────

fn bullet_move_system(world: &mut World, dt: f32) {
    let state = world.get_resource::<GameStateRes>().unwrap();
    if state.state != GameState::Playing { return; }

    let mut query = QuerySingleMut::<Bullet>::new(world).unwrap();
    for (_entity, bullet) in query.iter_mut() {
        if !bullet.alive { continue; }
        bullet.x += bullet.vx * dt;
        bullet.y += bullet.vy * dt;

        if bullet.x < -10.0 || bullet.x > LEVEL_W + 10.0
            || bullet.y < -10.0 || bullet.y > WORLD_H + 10.0
        {
            bullet.alive = false;
        }
    }
}

// ── Particle Update ─────────────────────────────────────────────────────────

fn particle_update_system(world: &mut World, dt: f32) {
    let mut query = QuerySingleMut::<Particle>::new(world).unwrap();
    for (_entity, p) in query.iter_mut() {
        if p.life <= 0.0 { continue; }
        p.x += p.vx * dt;
        p.y += p.vy * dt;
        p.vx *= 0.98;
        p.vy *= 0.98;
        p.life -= dt;
    }
}

// ── Collision Detection ─────────────────────────────────────────────────────

fn collision_system(world: &mut World, _dt: f32) {
    let state = world.get_resource::<GameStateRes>().unwrap();
    if state.state != GameState::Playing { return; }

    // Read player state
    let (px, py, player_invincible) = {
        let query = QuerySingle::<Player>::new(world);
        match query {
            Some(q) => {
                if let Some((_e, p)) = q.iter().next() {
                    (p.x, p.y, p.invincible)
                } else { return; }
            }
            None => return,
        }
    };

    // Collect events (can't mutate multiple storages simultaneously)
    struct HitEvent { x: f32, y: f32, shake: f32, score: i32, hit_player: bool }
    let mut events: Vec<HitEvent> = Vec::new();

    // Player bullets vs enemies
    {
        // Collect bullet hits
        let mut bullet_hits: Vec<(usize, usize)> = Vec::new(); // (bullet_idx, enemy_idx)
        let bullets = QuerySingle::<Bullet>::new(world);
        let enemies = QuerySingle::<Enemy>::new(world);
        if let (Some(bq), Some(eq)) = (bullets, enemies) {
            let bullet_vec: Vec<_> = bq.iter().collect();
            let enemy_vec: Vec<_> = eq.iter().collect();
            for (bi, (_be, bullet)) in bullet_vec.iter().enumerate() {
                if !bullet.alive || !bullet.is_player { continue; }
                for (ei, (_ee, enemy)) in enemy_vec.iter().enumerate() {
                    if !enemy.alive { continue; }
                    let es = enemy.size * 0.5;
                    if bullet.x + 4.0 > enemy.x - es
                        && bullet.x - 4.0 < enemy.x + es
                        && bullet.y + 3.0 > enemy.y - es
                        && bullet.y - 3.0 < enemy.y + es
                    {
                        bullet_hits.push((bi, ei));
                        break;
                    }
                }
            }
        }

        // Apply hits
        if !bullet_hits.is_empty() {
            // Mark bullets as dead
            {
                let mut bq = QuerySingleMut::<Bullet>::new(world).unwrap();
                let all_bullets: Vec<_> = bq.iter().map(|(e, _b)| e).collect();
                for (bi, _) in &bullet_hits {
                    if let Some(e) = all_bullets.get(*bi) {
                        if let Some(b) = bq.get_mut(*e) {
                            b.alive = false;
                        }
                    }
                }
            }

            // Process enemy hits
            {
                let mut eq = QuerySingleMut::<Enemy>::new(world).unwrap();
                let all_enemies: Vec<_> = eq.iter().map(|(e, _en)| e).collect();
                for (_, ei) in &bullet_hits {
                    if let Some(e) = all_enemies.get(*ei) {
                        if let Some(enemy) = eq.get_mut(*e) {
                            if !enemy.alive { continue; }
                            enemy.hp -= 1;
                            enemy.flash = 1.0;
                            if enemy.hp <= 0 {
                                enemy.alive = false;
                                events.push(HitEvent {
                                    x: enemy.x, y: enemy.y,
                                    shake: 6.0, score: 100,
                                    hit_player: false,
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    // Enemy bullets vs player
    if player_invincible <= 0.0 {
        let mut hit_bullet: Option<usize> = None;
        {
            let query = QuerySingle::<Bullet>::new(world);
            if let Some(q) = query {
                for (i, (_e, bullet)) in q.iter().enumerate() {
                    if !bullet.alive || bullet.is_player { continue; }
                    if bullet.x + 3.0 > px && bullet.x - 3.0 < px + PLAYER_SIZE
                        && bullet.y + 3.0 > py && bullet.y - 3.0 < py + PLAYER_SIZE
                    {
                        hit_bullet = Some(i);
                        break;
                    }
                }
            }
        }
        if let Some(idx) = hit_bullet {
            // Mark bullet dead
            let mut bq = QuerySingleMut::<Bullet>::new(world).unwrap();
            let all: Vec<_> = bq.iter().map(|(e, _b)| e).collect();
            if let Some(e) = all.get(idx) {
                if let Some(b) = bq.get_mut(*e) { b.alive = false; }
            }
            events.push(HitEvent {
                x: px, y: py, shake: 14.0, score: 0,
                hit_player: true,
            });
        }
    }

    // Enemy contact vs player
    if player_invincible <= 0.0 {
        let mut hit_enemy: Option<usize> = None;
        {
            let query = QuerySingle::<Enemy>::new(world);
            if let Some(q) = query {
                for (i, (_e, enemy)) in q.iter().enumerate() {
                    if !enemy.alive { continue; }
                    let es = enemy.size * 0.5;
                    if px + PLAYER_SIZE > enemy.x - es && px < enemy.x + es
                        && py + PLAYER_SIZE > enemy.y - es && py < enemy.y + es
                    {
                        hit_enemy = Some(i);
                        break;
                    }
                }
            }
        }
        if let Some(idx) = hit_enemy {
            let (ex, ey) = {
                let q = QuerySingle::<Enemy>::new(world).unwrap();
                let all: Vec<_> = q.iter().collect();
                if let Some((_e, enemy)) = all.get(idx) {
                    (enemy.x, enemy.y)
                } else { (0.0, 0.0) }
            };
            // Mark enemy dead
            {
                let mut eq = QuerySingleMut::<Enemy>::new(world).unwrap();
                let all: Vec<_> = eq.iter().map(|(e, _en)| e).collect();
                if let Some(e) = all.get(idx) {
                    if let Some(en) = eq.get_mut(*e) { en.alive = false; }
                }
            }
            events.push(HitEvent {
                x: ex, y: ey, shake: 14.0, score: 0,
                hit_player: true,
            });
        }
    }

    // Apply events
    for ev in events {
        // Spawn explosion particles
        let count = if ev.score > 0 { 22 } else { 15 };
        let power = if ev.score > 0 { 250.0 } else { 200.0 };
        spawn_explosion_particles(world, ev.x, ev.y, count, power);

        // Screen shake
        let cam = world.get_resource_mut::<CameraRes>().unwrap();
        cam.shake_amount = (cam.shake_amount + ev.shake).min(18.0);

        // Score
        if ev.score > 0 {
            let score_res = world.get_resource_mut::<ScoreRes>().unwrap();
            score_res.score += ev.score;
        }

        // Player hit
        if ev.hit_player {
            let lives = world.get_resource_mut::<LivesRes>().unwrap();
            lives.lives -= 1;

            if lives.lives <= 0 {
                // Death explosion
                spawn_explosion_particles(world, px, py, 45, 350.0);
                let gs = world.get_resource_mut::<GameStateRes>().unwrap();
                gs.state = GameState::GameOver;
                gs.game_over_timer = 0.0;
            } else {
                // Set invincible
                let mut pq = QuerySingleMut::<Player>::new(world).unwrap();
                for (_e, player) in pq.iter_mut() {
                    player.invincible = INVINCIBLE_TIME;
                    player.flash = 0.0;
                }
            }
        }
    }
}

// ── Camera ──────────────────────────────────────────────────────────────────

fn camera_system(world: &mut World, dt: f32) {
    let state = world.get_resource::<GameStateRes>().unwrap();
    if state.state != GameState::Playing && state.state != GameState::GameOver { return; }

    // Read player center
    let target_x = {
        let query = QuerySingle::<Player>::new(world);
        match query {
            Some(q) => {
                if let Some((_e, p)) = q.iter().next() {
                    p.x + PLAYER_SIZE * 0.5
                } else { return; }
            }
            None => return,
        }
    };

    let cam = world.get_resource_mut::<CameraRes>().unwrap();

    // Shake decay
    if cam.shake_amount > 0.0 {
        cam.shake_amount = (cam.shake_amount - 6.0 * dt).max(0.0);
    }

    // Dead zone
    let cam_center = cam.camera_x + WORLD_W * 0.5;
    let diff = target_x - cam_center;

    let desired_x = if diff > CAM_DEAD_ZONE_X {
        target_x - CAM_DEAD_ZONE_X - WORLD_W * 0.5
    } else if diff < -CAM_DEAD_ZONE_X {
        target_x + CAM_DEAD_ZONE_X - WORLD_W * 0.5
    } else {
        cam.camera_x
    };

    // Smooth follow
    cam.camera_x += (desired_x - cam.camera_x) * CAM_SMOOTH * dt;

    // Boundary clamp
    cam.camera_x = cam.camera_x.clamp(0.0, LEVEL_W - WORLD_W);
    cam.camera_y = 0.0;
}

// ── Cleanup ─────────────────────────────────────────────────────────────────

fn cleanup_system(world: &mut World, _dt: f32) {
    // Collect entities to despawn
    let mut to_despawn: Vec<opengame_engine::ecs::Entity> = Vec::new();

    {
        let query = QuerySingle::<Bullet>::new(world);
        if let Some(q) = query {
            for (e, b) in q.iter() {
                if !b.alive { to_despawn.push(e); }
            }
        }
    }
    {
        let query = QuerySingle::<Enemy>::new(world);
        if let Some(q) = query {
            for (e, en) in q.iter() {
                if !en.alive { to_despawn.push(e); }
            }
        }
    }
    {
        let query = QuerySingle::<Particle>::new(world);
        if let Some(q) = query {
            for (e, p) in q.iter() {
                if p.life <= 0.0 { to_despawn.push(e); }
            }
        }
    }

    for e in to_despawn {
        world.despawn(e);
    }
}

// ── Helper ──────────────────────────────────────────────────────────────────

fn spawn_explosion_particles(world: &mut World, x: f32, y: f32, count: usize, power: f32) {
    let current = {
        let query = QuerySingle::<Particle>::new(world);
        match query {
            Some(q) => q.len(),
            None => 0,
        }
    };

    for i in 0..count {
        if current + i >= MAX_PARTICLES { break; }
        let angle = rand() * std::f32::consts::TAU;
        let speed = rand_range(60.0, power);
        world.spawn()
            .with(Particle {
                x, y,
                vx: angle.cos() * speed,
                vy: angle.sin() * speed,
                life: rand_range(0.3, 0.9),
                max_life: rand_range(0.3, 0.9),
                size: rand_range(3.0, 7.0),
                color_idx: (i % 7) as u8,
            })
            .build();
    }
}
