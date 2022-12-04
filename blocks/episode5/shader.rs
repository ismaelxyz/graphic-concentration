use std::{ffi::CString, fs::File, io::Read};

//import ctypes
//import pyglet.gl as gl
/*
class Shader_error(Exception):
    def __init__(self, message):
        self.message = message
*/

pub unsafe fn create_shader(target: u32, source_path: &str) {
    // Read shader source
    let mut source = String::new();
    File::open(source_path)
        .unwrap()
        .read_to_string(&mut source)
        .unwrap();

    let source_length = source.len() as i32 + 1;
    let source_buffer = CString::new(source).unwrap();

    // Compile shader
    gl::ShaderSource(target, 1, &source_buffer.as_ptr(), &source_length);
    gl::CompileShader(target);

    // Handle potential errors
    let mut log_length = 0;
    gl::GetShaderiv(target, gl::INFO_LOG_LENGTH, &mut log_length);

    // let log_buffer = ctypes.create_string_buffer(log_length.value)
    let mut c_string = CString::new(format!("{:width$}", width = log_length as usize)).unwrap();
    let log_buffer = c_string.into_raw();
    gl::GetShaderInfoLog(target, log_length, std::ptr::null_mut(), log_buffer);

    if log_length > 1 {
        c_string = CString::from_raw(log_buffer);
        panic!("{:?}", c_string.into_string().unwrap());
    }
}

pub struct Shader {
    program: u32,
}

impl Shader {
    pub unsafe fn new(vert_path: &str, frag_path: &str) -> Self {
        let program = gl::CreateProgram();

        // create vertex shader
        let vert_shader = gl::CreateShader(gl::VERTEX_SHADER);
        create_shader(vert_shader, vert_path);
        gl::AttachShader(program, vert_shader);

        // create fragment shader
        let frag_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        create_shader(frag_shader, frag_path);
        gl::AttachShader(program, frag_shader);

        // link program and clean up
        gl::LinkProgram(program);

        gl::DeleteShader(vert_shader);
        gl::DeleteShader(frag_shader);

        Shader { program }
    }

    pub fn find_uniform(&self, name: &str) -> i32 {
        let c_name = CString::new(name).unwrap();
        let result = unsafe { gl::GetUniformLocation(self.program, c_name.as_ptr()) };
        result
    }

    pub fn uniform_matrix(&self, location: i32, matrix: crate::matrix::Matrix) {
        unsafe {
            let value = matrix.value();
            gl::UniformMatrix4fv(location, 1, gl::FALSE, value.as_ptr());
        }
    }

    pub fn using(&self) {
        unsafe { gl::UseProgram(self.program) }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.program) }
    }
}

/*




*/
