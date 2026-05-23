use glow::HasContext;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;

pub struct GlBackend {
    gl: glow::Context,
    canvas: HtmlCanvasElement,
    width: u32,
    height: u32,
}

impl GlBackend {
    pub fn new(canvas_id: &str) -> Result<Self, String> {
        let window = web_sys::window().ok_or("No window")?;
        let document = window.document().ok_or("No document")?;
        let canvas = document
            .get_element_by_id(canvas_id)
            .ok_or("Canvas not found")?
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| "Element is not a canvas")?;

        let webgl2_context = canvas
            .get_context("webgl2")
            .map_err(|_| "Failed to get webgl2 context")?
            .ok_or("WebGL2 not supported")?
            .dyn_into::<web_sys::WebGl2RenderingContext>()
            .map_err(|_| "Failed to cast to WebGL2RenderingContext")?;

        let gl = glow::Context::from_webgl2_context(webgl2_context);

        let width = canvas.client_width() as u32;
        let height = canvas.client_height() as u32;
        canvas.set_width(width);
        canvas.set_height(height);

        Ok(Self {
            gl,
            canvas,
            width,
            height,
        })
    }

    pub fn gl(&self) -> &glow::Context {
        &self.gl
    }

    pub fn canvas(&self) -> &HtmlCanvasElement {
        &self.canvas
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn resize(&mut self) -> bool {
        let new_width = self.canvas.client_width() as u32;
        let new_height = self.canvas.client_height() as u32;
        if new_width != self.width || new_height != self.height {
            self.width = new_width;
            self.height = new_height;
            self.canvas.set_width(new_width);
            self.canvas.set_height(new_height);
            unsafe {
                self.gl.viewport(0, 0, new_width as i32, new_height as i32);
            }
            true
        } else {
            false
        }
    }

    pub fn clear(&self, r: f32, g: f32, b: f32, a: f32) {
        unsafe {
            self.gl.clear_color(r, g, b, a);
            self.gl.clear(glow::COLOR_BUFFER_BIT);
        }
    }

    pub fn enable_blend(&self) {
        unsafe {
            self.gl.enable(glow::BLEND);
            self.gl
                .blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);
        }
    }

    pub fn disable_blend(&self) {
        unsafe {
            self.gl.disable(glow::BLEND);
        }
    }

    pub fn set_viewport(&self, x: i32, y: i32, width: i32, height: i32) {
        unsafe {
            self.gl.viewport(x, y, width, height);
        }
    }
}
