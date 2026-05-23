use super::shader::Shader;
use crate::color::Color;
use crate::math::Mat4;
use glow::HasContext;

const MAX_VERTICES: usize = 65536;
const FLOATS_PER_VERTEX: usize = 2;

pub struct ShapeRenderer {
    shader: Shader,
    vao: Option<glow::VertexArray>,
    vbo: Option<glow::Buffer>,
    vertices: Vec<f32>,
    vertex_count: usize,
    draw_calls: Vec<DrawCall>,
    current_color: Color,
}

struct DrawCall {
    offset: i32,
    count: i32,
    color: Color,
    mode: u32,
}

impl ShapeRenderer {
    pub fn new(gl: &glow::Context) -> Result<Self, String> {
        let shader = Shader::new(
            gl,
            super::shader::SHAPE_VERTEX_SHADER,
            super::shader::SHAPE_FRAGMENT_SHADER,
        )?;

        let buffer_size = MAX_VERTICES * FLOATS_PER_VERTEX * 4;

        let (vao, vbo) = unsafe {
            let vao = gl.create_vertex_array().map_err(|e| e.to_string())?;
            let vbo = gl.create_buffer().map_err(|e| e.to_string())?;

            gl.bind_vertex_array(Some(vao));
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            gl.buffer_data_size(glow::ARRAY_BUFFER, buffer_size as i32, glow::DYNAMIC_DRAW);

            gl.enable_vertex_attrib_array(0);
            gl.vertex_attrib_pointer_f32(0, 2, glow::FLOAT, false, 8, 0);

            gl.bind_vertex_array(None);

            (Some(vao), Some(vbo))
        };

        Ok(Self {
            shader,
            vao,
            vbo,
            vertices: Vec::with_capacity(MAX_VERTICES * FLOATS_PER_VERTEX),
            vertex_count: 0,
            draw_calls: Vec::new(),
            current_color: Color::WHITE,
        })
    }

    pub fn begin(&mut self) {
        self.vertices.clear();
        self.vertex_count = 0;
        self.draw_calls.clear();
    }

    pub fn set_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn draw_rect(&mut self, x: f32, y: f32, width: f32, height: f32) {
        let x2 = x + width;
        let y2 = y + height;
        let color = self.current_color;

        let vertices = [x, y, x2, y, x2, y2, x, y, x2, y2, x, y2];

        let offset = (self.vertex_count) as i32;
        self.vertices.extend_from_slice(&vertices);
        self.vertex_count += 6;

        self.draw_calls.push(DrawCall {
            offset,
            count: 6,
            color,
            mode: glow::TRIANGLES,
        });
    }

    pub fn draw_rect_outline(&mut self, x: f32, y: f32, width: f32, height: f32, thickness: f32) {
        self.draw_rect(x, y, width, thickness);
        self.draw_rect(x, y + height - thickness, width, thickness);
        self.draw_rect(x, y + thickness, thickness, height - 2.0 * thickness);
        self.draw_rect(
            x + width - thickness,
            y + thickness,
            thickness,
            height - 2.0 * thickness,
        );
    }

    pub fn draw_line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, thickness: f32) {
        let dx = x2 - x1;
        let dy = y2 - y1;
        let len = (dx * dx + dy * dy).sqrt();
        if len < f32::EPSILON {
            return;
        }

        let nx = -dy / len * thickness * 0.5;
        let ny = dx / len * thickness * 0.5;

        let vertices = [
            x1 + nx,
            y1 + ny,
            x1 - nx,
            y1 - ny,
            x2 + nx,
            y2 + ny,
            x1 - nx,
            y1 - ny,
            x2 - nx,
            y2 - ny,
            x2 + nx,
            y2 + ny,
        ];

        let offset = self.vertex_count as i32;
        self.vertices.extend_from_slice(&vertices);
        self.vertex_count += 6;

        self.draw_calls.push(DrawCall {
            offset,
            count: 6,
            color: self.current_color,
            mode: glow::TRIANGLES,
        });
    }

    pub fn draw_circle(&mut self, cx: f32, cy: f32, radius: f32, segments: u32) {
        let segments = segments.max(3);
        let color = self.current_color;
        let offset = self.vertex_count as i32;
        let mut count = 0;

        for i in 0..segments {
            let angle1 = (i as f32 / segments as f32) * std::f32::consts::TAU;
            let angle2 = ((i + 1) as f32 / segments as f32) * std::f32::consts::TAU;

            self.vertices.extend_from_slice(&[cx, cy]);
            self.vertices
                .extend_from_slice(&[cx + radius * angle1.cos(), cy + radius * angle1.sin()]);
            self.vertices
                .extend_from_slice(&[cx + radius * angle2.cos(), cy + radius * angle2.sin()]);
            count += 3;
            self.vertex_count += 3;
        }

        self.draw_calls.push(DrawCall {
            offset,
            count,
            color,
            mode: glow::TRIANGLES,
        });
    }

    pub fn draw_circle_outline(
        &mut self,
        cx: f32,
        cy: f32,
        radius: f32,
        thickness: f32,
        segments: u32,
    ) {
        let segments = segments.max(3);
        let color = self.current_color;
        let offset = self.vertex_count as i32;
        let mut count = 0;

        let inner = radius - thickness * 0.5;
        let outer = radius + thickness * 0.5;

        for i in 0..segments {
            let angle1 = (i as f32 / segments as f32) * std::f32::consts::TAU;
            let angle2 = ((i + 1) as f32 / segments as f32) * std::f32::consts::TAU;

            let i1x = cx + inner * angle1.cos();
            let i1y = cy + inner * angle1.sin();
            let o1x = cx + outer * angle1.cos();
            let o1y = cy + outer * angle1.sin();
            let i2x = cx + inner * angle2.cos();
            let i2y = cy + inner * angle2.sin();
            let o2x = cx + outer * angle2.cos();
            let o2y = cy + outer * angle2.sin();

            self.vertices
                .extend_from_slice(&[i1x, i1y, o1x, o1y, i2x, i2y]);
            self.vertices
                .extend_from_slice(&[o1x, o1y, o2x, o2y, i2x, i2y]);
            count += 6;
            self.vertex_count += 6;
        }

        self.draw_calls.push(DrawCall {
            offset,
            count,
            color,
            mode: glow::TRIANGLES,
        });
    }

    pub fn flush(&mut self, gl: &glow::Context, projection: &Mat4) {
        if self.vertex_count == 0 {
            return;
        }

        self.shader.bind(gl);
        self.shader.set_uniform_mat4(gl, "u_projection", projection);

        unsafe {
            gl.bind_vertex_array(self.vao);
            gl.bind_buffer(glow::ARRAY_BUFFER, self.vbo);

            let bytes: &[u8] = std::slice::from_raw_parts(
                self.vertices.as_ptr() as *const u8,
                self.vertices.len() * 4,
            );
            gl.buffer_sub_data_u8_slice(glow::ARRAY_BUFFER, 0, bytes);

            for call in &self.draw_calls {
                self.shader.set_uniform_4f(
                    gl,
                    "u_color",
                    call.color.r,
                    call.color.g,
                    call.color.b,
                    call.color.a,
                );
                gl.draw_arrays(call.mode, call.offset, call.count);
            }

            gl.bind_vertex_array(None);
        }
    }
}
