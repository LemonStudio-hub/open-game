#[cfg(target_arch = "wasm32")]
pub mod app;
#[cfg(target_arch = "wasm32")]
pub mod asset;
#[cfg(target_arch = "wasm32")]
pub mod audio;
pub mod color;
pub mod debug;
pub mod ecs;
pub mod event;
#[cfg(target_arch = "wasm32")]
pub mod input;
pub mod log;
pub mod math;
pub mod physics;
pub mod profiler;
pub mod renderer;
pub mod scene;
pub mod sprite_component;
pub mod time;
pub mod transform;

pub mod prelude {
    #[cfg(target_arch = "wasm32")]
    pub use crate::app::{App, Commands, Renderer};
    #[cfg(target_arch = "wasm32")]
    pub use crate::audio::{AudioEngine, AudioHandle, AudioSource};
    pub use crate::color::Color;
    pub use crate::debug::{DebugOverlay, DebugPosition};
    pub use crate::ecs::query::{QueryDouble, QueryDoubleMut, QuerySingle, QuerySingleMut};
    pub use crate::ecs::{Entity, EntityBuilder, System, SystemScheduler, World};
    #[cfg(target_arch = "wasm32")]
    pub use crate::input::keys::{KeyCode, MouseButton};
    #[cfg(target_arch = "wasm32")]
    pub use crate::input::InputManager;
    pub use crate::math::*;
    pub use crate::physics::collider::Collider;
    pub use crate::physics::rigid_body::RigidBody;
    pub use crate::physics::PhysicsSystem;
    pub use crate::profiler::{ProfileReport, Profiler, ScopeGuard};
    pub use crate::renderer::Camera2D;
    #[cfg(target_arch = "wasm32")]
    pub use crate::renderer::{
        BitmapFont, ShapeRenderer, SpriteRenderer, TextureHandle, TextureManager,
    };
    pub use crate::scene::{Scene, SceneContext, SceneManager};
    pub use crate::sprite_component::Sprite;
    pub use crate::time::{Time, Timer};
    pub use crate::transform::Transform2D;
}

pub use prelude::*;
