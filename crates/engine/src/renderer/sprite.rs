use super::shader::Shader;
use super::texture::{TextureHandle, TextureManager};
use crate::color::Color;
use crate::math::{Mat4, Vec2};
use glow::HasContext;

const MAX_SPRITES: usize = 10000;
const VERTICES_PER_SPRITE: usize = 6;
const FLOATS_PER_VERTEX: usize = 8;

#[repr(C)]
#[allow(dead_code)]
struct SpriteVertex {
    position: [f32; 2],
    texcoord: [f32; 2],
    color: [f32; 4],
}

pub struct SpriteParams {
    pub position: Vec2,
    pub size: Vec2,
    pub rotation: f32,
    pub anchor: Vec2,
    pub color: Color,
    pub uv_min: Vec2,
    pub uv_max: Vec2,
    pub flip_x: bool,
    pub flip_y: bool,
}

impl Default for SpriteParams {
    fn default() -> Self {
        Self {
            position: Vec2::ZERO,
            size: Vec2::ONE,
            rotation: 0.0,
            anchor: Vec2::new(0.5, 0.5),
            color: Color::WHITE,
            uv_min: Vec2::ZERO,
            uv_max: Vec2::ONE,
            flip_x: false,
            flip_y: false,
        }
    }
}

pub struct SpriteRenderer {
    shader: Shader,
    vao: Option<glow::VertexArray>,
    vbo: Option<glow::Buffer>,
    vertices: Vec<f32>,
    sprite_count: usize,
}

impl SpriteRenderer {
    pub fn new(gl: &glow::Context) -> Result<Self, String> {
        let shader = Shader::new(
            gl,
            super::shader::SPRITE_VERTEX_SHADER,
            super::shader::SPRITE_FRAGMENT_SHADER,
        )?;

        let vertex_size = (FLOATS_PER_VERTEX * 4) as i32;
        let buffer_size = MAX_SPRITES * VERTICES_PER_SPRITE * FLOATS_PER_VERTEX * 4;

        let (vao, vbo) = unsafe {
            let vao = gl.create_vertex_array().map_err(|e| e.to_string())?;
            let vbo = gl.create_buffer().map_err(|e| e.to_string())?;

            gl.bind_vertex_array(Some(vao));
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            gl.buffer_data_size(glow::ARRAY_BUFFER, buffer_size as i32, glow::DYNAMIC_DRAW);

            let stride = vertex_size;
            gl.enable_vertex_attrib_array(0);
            gl.vertex_attrib_pointer_f32(0, 2, glow::FLOAT, false, stride, 0);
            gl.enable_vertex_attrib_array(1);
            gl.vertex_attrib_pointer_f32(1, 2, glow::FLOAT, false, stride, 8);
            gl.enable_vertex_attrib_array(2);
            gl.vertex_attrib_pointer_f32(2, 4, glow::FLOAT, false, stride, 16);

            gl.bind_vertex_array(None);

            (Some(vao), Some(vbo))
        };

        Ok(Self {
            shader,
            vao,
            vbo,
            vertices: Vec::with_capacity(MAX_SPRITES * VERTICES_PER_SPRITE * FLOATS_PER_VERTEX),
            sprite_count: 0,
        })
    }

    pub fn begin(&mut self) {
        self.vertices.clear();
        self.sprite_count = 0;
    }

    pub fn draw_sprite(&mut self, params: &SpriteParams) {
        if self.sprite_count >= MAX_SPRITES {
            return;
        }

        let hw = params.size.x * 0.5;
        let hh = params.size.y * 0.5;
        let ax = (params.anchor.x - 0.5) * params.size.x;
        let ay = (params.anchor.y - 0.5) * params.size.y;

        let corners = [
            Vec2::new(-hw - ax, -hh - ay),
            Vec2::new(hw - ax, -hh - ay),
            Vec2::new(hw - ax, hh - ay),
            Vec2::new(-hw - ax, hh - ay),
        ];

        let cos_r = params.rotation.cos();
        let sin_r = params.rotation.sin();

        let rotate =
            |v: Vec2| -> Vec2 { Vec2::new(v.x * cos_r - v.y * sin_r, v.x * sin_r + v.y * cos_r) };

        let world_corners: Vec<Vec2> = corners
            .iter()
            .map(|c| rotate(*c) + params.position)
            .collect();

        let mut u_min = params.uv_min.x;
        let mut u_max = params.uv_max.x;
        let mut v_min = params.uv_min.y;
        let mut v_max = params.uv_max.y;

        if params.flip_x {
            std::mem::swap(&mut u_min, &mut u_max);
        }
        if params.flip_y {
            std::mem::swap(&mut v_min, &mut v_max);
        }

        let uvs = [
            [u_min, v_min],
            [u_max, v_min],
            [u_max, v_max],
            [u_min, v_max],
        ];

        let col = params.color.to_array();
        let indices = [0, 1, 2, 0, 2, 3];

        for &i in &indices {
            let p = world_corners[i];
            let uv = uvs[i];
            self.vertices.extend_from_slice(&[p.x, p.y]);
            self.vertices.extend_from_slice(&uv);
            self.vertices.extend_from_slice(&col);
        }

        self.sprite_count += 1;
    }

    pub fn draw_rect(&mut self, position: Vec2, size: Vec2, color: Color) {
        self.draw_sprite(&SpriteParams {
            position,
            size,
            color,
            ..Default::default()
        });
    }

    pub fn flush(
        &mut self,
        gl: &glow::Context,
        projection: &Mat4,
        texture_manager: &TextureManager,
        texture_handle: TextureHandle,
    ) {
        if self.sprite_count == 0 {
            return;
        }

        self.shader.bind(gl);
        self.shader.set_uniform_mat4(gl, "u_projection", projection);
        self.shader.set_uniform_1i(gl, "u_texture", 0);

        texture_manager.bind(gl, texture_handle, 0);

        unsafe {
            gl.bind_vertex_array(self.vao);
            gl.bind_buffer(glow::ARRAY_BUFFER, self.vbo);

            let bytes: &[u8] = std::slice::from_raw_parts(
                self.vertices.as_ptr() as *const u8,
                self.vertices.len() * 4,
            );
            gl.buffer_sub_data_u8_slice(glow::ARRAY_BUFFER, 0, bytes);

            gl.draw_arrays(
                glow::TRIANGLES,
                0,
                (self.sprite_count * VERTICES_PER_SPRITE) as i32,
            );
            gl.bind_vertex_array(None);
        }
    }

    pub fn sprite_count(&self) -> usize {
        self.sprite_count
    }
}
