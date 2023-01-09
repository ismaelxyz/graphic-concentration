use glow::HasContext;
use std::{fs::File, io::Read, sync::Arc};

/// Read shader from source file
pub fn create_shader(
    gl: &Arc<glow::Context>,
    target: u32,
    name: &str,
) -> Result<glow::Shader, String> {
    unsafe {
        let mut source = String::new();
        File::open(format!("shaders/{name}.glsl"))
            .unwrap()
            .read_to_string(&mut source)
            .unwrap();

        let shader = gl.create_shader(target)?;

        gl.shader_source(shader, &source);

        gl.compile_shader(shader);

        if gl.get_shader_compile_status(shader) {
            Ok(shader)
        } else {
            Err(gl.get_shader_info_log(shader))
        }
    }
}

pub struct Shader {
    gl: Arc<glow::Context>,
    program: glow::Program,
}

impl Shader {
    pub fn new(gl: &Arc<glow::Context>, vert_path: &str, frag_path: &str) -> Self {
        unsafe {
            let program = gl.create_program().unwrap();

            // create vertex shader
            let vert_shader = create_shader(gl, glow::VERTEX_SHADER, vert_path).unwrap();
            gl.attach_shader(program, vert_shader);

            // create fragment shader
            let frag_shader = create_shader(gl, glow::FRAGMENT_SHADER, frag_path).unwrap();
            gl.attach_shader(program, frag_shader);

            // link program and clean up
            gl.link_program(program);

            gl.delete_shader(vert_shader);
            gl.delete_shader(frag_shader);

            Shader {
                gl: gl.clone(),
                program,
            }
        }
    }

    pub fn find_uniform(&self, name: &str) -> Option<glow::NativeUniformLocation> {
        unsafe { self.gl.get_uniform_location(self.program, name) }
    }

    pub fn uniform_matrix(&self, location: Option<&glow::UniformLocation>, matrix: crate::Matrix) {
        unsafe {
            let value = matrix.value();
            self.gl
                .uniform_matrix_4_f32_slice(location, false, bytemuck::cast_slice(&value));
        }
    }

    pub fn bind(&self) {
        unsafe {
            self.gl.use_program(Some(self.program));
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { self.gl.delete_program(self.program) }
    }
}
