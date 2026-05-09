use super::cache::AssetCache;
use crate::renderer::texture::TextureHandle;

pub struct ImageAsset {
    pub handle: TextureHandle,
    pub width: u32,
    pub height: u32,
}

pub type ImageCache = AssetCache<ImageAsset>;
