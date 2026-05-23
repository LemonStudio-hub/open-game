pub mod camera;
#[cfg(target_arch = "wasm32")]
pub mod gl_backend;
pub mod shader;
#[cfg(target_arch = "wasm32")]
pub mod shape;
#[cfg(target_arch = "wasm32")]
pub mod sprite;
#[cfg(target_arch = "wasm32")]
pub mod text;
#[cfg(target_arch = "wasm32")]
pub mod texture;

pub use camera::Camera2D;
#[cfg(target_arch = "wasm32")]
pub use gl_backend::GlBackend;
pub use shader::Shader;
#[cfg(target_arch = "wasm32")]
pub use shape::ShapeRenderer;
#[cfg(target_arch = "wasm32")]
pub use sprite::SpriteRenderer;
#[cfg(target_arch = "wasm32")]
pub use text::BitmapFont;
#[cfg(target_arch = "wasm32")]
pub use texture::{Texture, TextureHandle, TextureManager};
