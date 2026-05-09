pub mod log;
pub mod math;
pub mod time;
pub mod color;
pub mod transform;
pub mod sprite_component;
pub mod event;
pub mod ecs;
pub mod renderer;
#[cfg(target_arch = "wasm32")]
pub mod input;
#[cfg(target_arch = "wasm32")]
pub mod audio;
pub mod physics;
#[cfg(target_arch = "wasm32")]
pub mod asset;
pub mod scene;
#[cfg(target_arch = "wasm32")]
pub mod app;
pub mod profiler;
pub mod debug;

pub mod prelude {
    pub use crate::math::*;
    pub use crate::time::{Time, Timer};
    pub use crate::color::Color;
    pub use crate::transform::Transform2D;
    pub use crate::sprite_component::Sprite;
    pub use crate::ecs::{Entity, World, System, SystemScheduler, EntityBuilder};
    pub use crate::ecs::query::{QuerySingle, QuerySingleMut, QueryDouble, QueryDoubleMut};
    pub use crate::renderer::Camera2D;
    #[cfg(target_arch = "wasm32")]
    pub use crate::renderer::{TextureHandle, TextureManager, SpriteRenderer, ShapeRenderer, BitmapFont};
    #[cfg(target_arch = "wasm32")]
    pub use crate::input::InputManager;
    #[cfg(target_arch = "wasm32")]
    pub use crate::input::keys::{KeyCode, MouseButton};
    #[cfg(target_arch = "wasm32")]
    pub use crate::audio::{AudioEngine, AudioSource, AudioHandle};
    pub use crate::physics::rigid_body::RigidBody;
    pub use crate::physics::collider::Collider;
    pub use crate::physics::PhysicsSystem;
    pub use crate::scene::{Scene, SceneContext, SceneManager};
    #[cfg(target_arch = "wasm32")]
    pub use crate::app::{App, Renderer, Commands};
    pub use crate::profiler::{Profiler, ScopeGuard, ProfileReport};
    pub use crate::debug::{DebugOverlay, DebugPosition};
}

pub use prelude::*;
