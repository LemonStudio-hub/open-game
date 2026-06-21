use glow::HasContext;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShaderHandle(u32);

pub struct Shader {
    program: glow::Program,
    uniforms: HashMap<String, glow::UniformLocation>,
}

impl Shader {
    pub fn new(gl: &glow::Context, vertex_src: &str, fragment_src: &str) -> Result<Self, String> {
        let vertex = compile_shader(gl, glow::VERTEX_SHADER, vertex_src)?;
        let fragment = compile_shader(gl, glow::FRAGMENT_SHADER, fragment_src)?;

        let program = unsafe { gl.create_program().map_err(|e| e.to_string())? };
        unsafe {
            gl.attach_shader(program, vertex);
            gl.attach_shader(program, fragment);
            gl.link_program(program);

            if !gl.get_program_link_status(program) {
                let log = gl.get_program_info_log(program);
                gl.delete_program(program);
                gl.delete_shader(vertex);
                gl.delete_shader(fragment);
                return Err(format!("Shader link error: {}", log));
            }

            gl.detach_shader(program, vertex);
            gl.detach_shader(program, fragment);
            gl.delete_shader(vertex);
            gl.delete_shader(fragment);
        }

        Ok(Self {
            program,
            uniforms: HashMap::new(),
        })
    }

    pub fn bind(&self, gl: &glow::Context) {
        unsafe {
            gl.use_program(Some(self.program));
        }
    }

    pub fn unbind(&self, gl: &glow::Context) {
        unsafe {
            gl.use_program(None);
        }
    }

    pub fn get_uniform_location(
        &mut self,
        gl: &glow::Context,
        name: &str,
    ) -> Option<&glow::UniformLocation> {
        if !self.uniforms.contains_key(name) {
            let loc = unsafe { gl.get_uniform_location(self.program, name) };
            if let Some(loc) = loc {
                self.uniforms.insert(name.to_string(), loc);
            }
        }
        self.uniforms.get(name)
    }

    pub fn set_uniform_1f(&mut self, gl: &glow::Context, name: &str, value: f32) {
        if let Some(loc) = self.get_uniform_location(gl, name) {
            unsafe {
                gl.uniform_1_f32(Some(loc), value);
            }
        }
    }

    pub fn set_uniform_2f(&mut self, gl: &glow::Context, name: &str, x: f32, y: f32) {
        if let Some(loc) = self.get_uniform_location(gl, name) {
            unsafe {
                gl.uniform_2_f32(Some(loc), x, y);
            }
        }
    }

    pub fn set_uniform_4f(
        &mut self,
        gl: &glow::Context,
        name: &str,
        x: f32,
        y: f32,
        z: f32,
        w: f32,
    ) {
        if let Some(loc) = self.get_uniform_location(gl, name) {
            unsafe {
                gl.uniform_4_f32(Some(loc), x, y, z, w);
            }
        }
    }

    pub fn set_uniform_mat4(&mut self, gl: &glow::Context, name: &str, matrix: &glam::Mat4) {
        if let Some(loc) = self.get_uniform_location(gl, name) {
            unsafe {
                gl.uniform_matrix_4_f32_slice(Some(loc), false, &matrix.to_cols_array());
            }
        }
    }

    pub fn set_uniform_mat3(&mut self, gl: &glow::Context, name: &str, matrix: &glam::Mat3) {
        if let Some(loc) = self.get_uniform_location(gl, name) {
            unsafe {
                gl.uniform_matrix_3_f32_slice(Some(loc), false, &matrix.to_cols_array());
            }
        }
    }

    pub fn set_uniform_1i(&mut self, gl: &glow::Context, name: &str, value: i32) {
        if let Some(loc) = self.get_uniform_location(gl, name) {
            unsafe {
                gl.uniform_1_i32(Some(loc), value);
            }
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {}
}

fn compile_shader(
    gl: &glow::Context,
    shader_type: u32,
    source: &str,
) -> Result<glow::Shader, String> {
    unsafe {
        let shader = gl.create_shader(shader_type).map_err(|e| e.to_string())?;
        gl.shader_source(shader, source);
        gl.compile_shader(shader);

        if !gl.get_shader_compile_status(shader) {
            let log = gl.get_shader_info_log(shader);
            gl.delete_shader(shader);
            return Err(format!("Shader compile error: {}", log));
        }

        Ok(shader)
    }
}

pub const SPRITE_VERTEX_SHADER: &str = r#"#version 300 es
precision mediump float;
layout(location = 0) in vec2 a_position;
layout(location = 1) in vec2 a_texcoord;
layout(location = 2) in vec4 a_color;
uniform mat4 u_projection;
out vec2 v_texcoord;
out vec4 v_color;
void main() {
    gl_Position = u_projection * vec4(a_position, 0.0, 1.0);
    v_texcoord = a_texcoord;
    v_color = a_color;
}
"#;

pub const SPRITE_FRAGMENT_SHADER: &str = r#"#version 300 es
precision mediump float;
in vec2 v_texcoord;
in vec4 v_color;
uniform sampler2D u_texture;
out vec4 frag_color;
void main() {
    frag_color = texture(u_texture, v_texcoord) * v_color;
}
"#;

pub const SHAPE_VERTEX_SHADER: &str = r#"#version 300 es
precision mediump float;
layout(location = 0) in vec2 a_position;
uniform mat4 u_projection;
uniform vec4 u_color;
out vec4 v_color;
void main() {
    gl_Position = u_projection * vec4(a_position, 0.0, 1.0);
    v_color = u_color;
}
"#;

pub const SHAPE_FRAGMENT_SHADER: &str = r#"#version 300 es
precision mediump float;
in vec4 v_color;
out vec4 frag_color;
void main() {
    frag_color = v_color;
}
"#;
