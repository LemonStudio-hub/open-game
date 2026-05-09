pub mod rigid_body;
pub mod collider;
pub mod collision;
pub mod solver;
pub mod spatial;

use crate::math::Vec2;
use crate::transform::Transform2D;
use collision::{CollisionInfo, test_collision};
use spatial::SpatialGrid;

pub use rigid_body::RigidBody;
pub use collider::Collider;
pub use solver::Solver;

use crate::ecs::world::World;
use crate::ecs::entity::Entity;
use crate::ecs::system::System;

pub struct PhysicsSystem {
    solver: Solver,
    grid: SpatialGrid,
    pub collisions: Vec<CollisionInfo>,
}

impl PhysicsSystem {
    pub fn new(gravity: Vec2) -> Self {
        Self {
            solver: Solver::new(gravity),
            grid: SpatialGrid::new(128.0),
            collisions: Vec::new(),
        }
    }

    pub fn with_gravity(mut self, gravity: Vec2) -> Self {
        self.solver = Solver::new(gravity);
        self
    }

    fn step(&mut self, world: &mut World, dt: f32) {
        self.collisions.clear();

        let entities: Vec<Entity> = world.entities();

        let mut updates: Vec<(Entity, Vec2, Vec2)> = Vec::new();
        for &entity in &entities {
            let rb = match world.get_component::<RigidBody>(entity) {
                Some(rb) => rb.clone(),
                None => continue,
            };
            let pos = match world.get_component::<Transform2D>(entity) {
                Some(t) => t.position,
                None => continue,
            };

            let mut rb = rb;
            self.solver.apply_gravity(&mut rb);
            let mut new_pos = pos;
            self.solver.integrate(&mut rb, &mut new_pos, dt);
            updates.push((entity, new_pos, rb.velocity));
        }

        for (entity, new_pos, new_vel) in updates {
            if let Some(t) = world.get_component_mut::<Transform2D>(entity) {
                t.position = new_pos;
            }
            if let Some(rb) = world.get_component_mut::<RigidBody>(entity) {
                rb.velocity = new_vel;
            }
        }

        self.grid.clear();
        for &entity in &entities {
            if let (Some(transform), Some(collider)) = (
                world.get_component::<Transform2D>(entity),
                world.get_component::<Collider>(entity),
            ) {
                let center = transform.position + collider.offset;
                let half = collider.half_extents();
                self.grid.insert(entity, center, half);
            }
        }

        for i in 0..entities.len() {
            let entity_a = entities[i];
            let (transform_a, collider_a) = match (
                world.get_component::<Transform2D>(entity_a),
                world.get_component::<Collider>(entity_a),
            ) {
                (Some(t), Some(c)) => (t.clone(), c.clone()),
                _ => continue,
            };

            let center_a = transform_a.position + collider_a.offset;
            let half_a = collider_a.half_extents();
            let candidates = self.grid.query(center_a, half_a + Vec2::splat(1.0));

            for &entity_b in &candidates {
                if entity_a.id() >= entity_b.id() {
                    continue;
                }

                let (transform_b, collider_b) = match (
                    world.get_component::<Transform2D>(entity_b),
                    world.get_component::<Collider>(entity_b),
                ) {
                    (Some(t), Some(c)) => (t.clone(), c.clone()),
                    _ => continue,
                };

                if let Some((normal, depth)) = test_collision(
                    transform_a.position, &collider_a,
                    transform_b.position, &collider_b,
                ) {
                    let mut info = CollisionInfo::new(entity_a, entity_b, normal, depth);
                    info.point = transform_a.position + normal * depth * 0.5;
                    self.collisions.push(info);
                }
            }
        }

        let collisions = self.collisions.clone();
        for collision in &collisions {
            let is_trigger = {
                let trigger_a = world.get_component::<Collider>(collision.entity_a)
                    .map_or(false, |c| c.is_trigger);
                let trigger_b = world.get_component::<Collider>(collision.entity_b)
                    .map_or(false, |c| c.is_trigger);
                trigger_a || trigger_b
            };

            if is_trigger {
                continue;
            }

            let mut pos_a = match world.get_component::<Transform2D>(collision.entity_a) {
                Some(t) => t.position,
                None => continue,
            };
            let mut pos_b = match world.get_component::<Transform2D>(collision.entity_b) {
                Some(t) => t.position,
                None => continue,
            };

            let mut rb_a = world.get_component::<RigidBody>(collision.entity_a)
                .cloned()
                .unwrap_or(RigidBody::static_body());
            let mut rb_b = world.get_component::<RigidBody>(collision.entity_b)
                .cloned()
                .unwrap_or(RigidBody::static_body());

            self.solver.resolve_collision(collision, &mut rb_a, &mut pos_a, &mut rb_b, &mut pos_b);

            if let Some(t) = world.get_component_mut::<Transform2D>(collision.entity_a) {
                t.position = pos_a;
            }
            if let Some(rb) = world.get_component_mut::<RigidBody>(collision.entity_a) {
                rb.velocity = rb_a.velocity;
            }
            if let Some(t) = world.get_component_mut::<Transform2D>(collision.entity_b) {
                t.position = pos_b;
            }
            if let Some(rb) = world.get_component_mut::<RigidBody>(collision.entity_b) {
                rb.velocity = rb_b.velocity;
            }
        }
    }

    pub fn gravity(&self) -> Vec2 {
        self.solver.gravity
    }

    pub fn set_gravity(&mut self, gravity: Vec2) {
        self.solver.gravity = gravity;
    }
}

impl System for PhysicsSystem {
    fn update(&mut self, world: &mut World, dt: f32) {
        self.step(world, dt);
    }

    fn name(&self) -> &str {
        "PhysicsSystem"
    }
}
