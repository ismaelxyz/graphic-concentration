use gl::types::{GLchar, GLenum, GLint, GLuint};
use std::{ffi::CString, io::Read, ptr};

pub unsafe fn compile_shader(kind: GLenum, path: &str) -> GLuint {
    // read shader source
    let shader = gl::CreateShader(kind);
    let mut contents = String::new();
    let mut file = std::fs::File::open(path).unwrap();
    file.read_to_string(&mut contents).unwrap();
    let c_source = CString::new(contents.as_bytes()).unwrap();

    // compile shader
    gl::ShaderSource(shader, 1, &c_source.as_ptr(), ptr::null());
    gl::CompileShader(shader);

    // handle potential errors
    let mut success = gl::FALSE as GLint;
    // subtract 1 to skip the trailing null character
    let mut info_log = vec![0; 512 - 1];
    gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);

    if success != gl::TRUE as GLint {
        gl::GetShaderInfoLog(
            shader,
            512,
            ptr::null_mut(),
            info_log.as_mut_ptr() as *mut GLchar,
        );

        let kind_name = match kind {
            gl::VERTEX_SHADER => "VERTEX_SHADER",
            gl::FRAGMENT_SHADER => "FRAGMENT_SHADER",
            _ => "UNKNOWN_SHADER_TYPE",
        };

        panic!(
            "ERROR[SHADER {} COMPILATION FAILED]:\n    {}",
            kind_name,
            std::str::from_utf8(&info_log).unwrap()
        );
    }

    shader
}

pub unsafe fn link_shader(vertex_shader: u32, fragment_shader: u32) -> u32 {
    // link shaders
    let shader_program = gl::CreateProgram();
    gl::AttachShader(shader_program, vertex_shader);
    gl::AttachShader(shader_program, fragment_shader);
    gl::LinkProgram(shader_program);

    // check for linking errors
    let mut success = gl::FALSE as GLint;
    let mut info_log = Vec::with_capacity(512);
    gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
    if success != gl::TRUE as GLint {
        gl::GetProgramInfoLog(
            shader_program,
            512,
            ptr::null_mut(),
            info_log.as_mut_ptr() as *mut GLchar,
        );
        panic!(
            "ERROR[SHADER PROGRAM COMPILATION FAILED]:\n     {}",
            std::str::from_utf8(&info_log).unwrap()
        );
    }

    shader_program
}
