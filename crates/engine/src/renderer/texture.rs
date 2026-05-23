use glow::HasContext;
use web_sys::HtmlImageElement;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TextureHandle(pub u32);

pub struct Texture {
    handle: glow::Texture,
    width: u32,
    height: u32,
}

impl Texture {
    pub fn from_image(gl: &glow::Context, image: &HtmlImageElement) -> Result<Self, String> {
        unsafe {
            let texture = gl.create_texture().map_err(|e| e.to_string())?;
            gl.bind_texture(glow::TEXTURE_2D, Some(texture));

            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_WRAP_S,
                glow::CLAMP_TO_EDGE as i32,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_WRAP_T,
                glow::CLAMP_TO_EDGE as i32,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MIN_FILTER,
                glow::NEAREST as i32,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MAG_FILTER,
                glow::NEAREST as i32,
            );

            gl.tex_image_2d_with_html_image(
                glow::TEXTURE_2D,
                0,
                glow::RGBA as i32,
                glow::RGBA,
                glow::UNSIGNED_BYTE,
                image,
            );

            let width = image.natural_width();
            let height = image.natural_height();

            Ok(Self {
                handle: texture,
                width,
                height,
            })
        }
    }

    pub fn from_color(gl: &glow::Context, r: u8, g: u8, b: u8, a: u8) -> Result<Self, String> {
        unsafe {
            let texture = gl.create_texture().map_err(|e| e.to_string())?;
            gl.bind_texture(glow::TEXTURE_2D, Some(texture));

            let data = [r, g, b, a];
            gl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                glow::RGBA as i32,
                1,
                1,
                0,
                glow::RGBA,
                glow::UNSIGNED_BYTE,
                glow::PixelUnpackData::Slice(Some(&data)),
            );

            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_WRAP_S,
                glow::CLAMP_TO_EDGE as i32,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_WRAP_T,
                glow::CLAMP_TO_EDGE as i32,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MIN_FILTER,
                glow::NEAREST as i32,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MAG_FILTER,
                glow::NEAREST as i32,
            );

            Ok(Self {
                handle: texture,
                width: 1,
                height: 1,
            })
        }
    }

    pub fn bind(&self, gl: &glow::Context, unit: u32) {
        unsafe {
            gl.active_texture(glow::TEXTURE0 + unit);
            gl.bind_texture(glow::TEXTURE_2D, Some(self.handle));
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}

pub struct TextureManager {
    textures: Vec<Option<Texture>>,
    white_texture: Option<TextureHandle>,
}

impl TextureManager {
    pub fn new() -> Self {
        Self {
            textures: Vec::new(),
            white_texture: None,
        }
    }

    pub fn init(&mut self, gl: &glow::Context) {
        if let Ok(tex) = Texture::from_color(gl, 255, 255, 255, 255) {
            let handle = TextureHandle(self.textures.len() as u32);
            self.textures.push(Some(tex));
            self.white_texture = Some(handle);
        }
    }

    pub fn add(&mut self, texture: Texture) -> TextureHandle {
        let handle = TextureHandle(self.textures.len() as u32);
        self.textures.push(Some(texture));
        handle
    }

    pub fn get(&self, handle: TextureHandle) -> Option<&Texture> {
        self.textures.get(handle.0 as usize)?.as_ref()
    }

    pub fn white_texture(&self) -> TextureHandle {
        self.white_texture.unwrap_or(TextureHandle(0))
    }

    pub fn bind(&self, gl: &glow::Context, handle: TextureHandle, unit: u32) {
        if let Some(tex) = self.get(handle) {
            tex.bind(gl, unit);
        }
    }
}

impl Default for TextureManager {
    fn default() -> Self {
        Self::new()
    }
}
