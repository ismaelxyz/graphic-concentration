use gl::types::*;
use glfw::{Action, Context, Key, OpenGlProfileHint, WindowHint};

mod shader;
use shader::{compile_shader, link_shader};

use std::sync::mpsc::Receiver;
use std::{mem::size_of, os::raw::c_void, ptr};

const WINDOW_NAME: &str = "Minecraft Clone";
const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

/// 3d coordinates for each vertex
#[rustfmt::skip]
const VERTEX_POSITIONS: [f32; 12] = [ 
    // X    Y    Z
    -0.5,  0.5, 1.0,
    -0.5, -0.5, 1.0,
     0.5, -0.5, 1.0,
     0.5,  0.5, 1.0,
];

/// Indices for the first and second triangles
#[rustfmt::skip]
const INDICES: [u32; 6] = [
    0, 1, 2,
    0, 2, 3,
];

fn main() {
    // glfw: initialize and configure
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(WindowHint::ContextVersion(3, 3));
    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));
    glfw.window_hint(WindowHint::AlphaBits(Some(u32::MAX - 1)));
    glfw.window_hint(WindowHint::DepthBits(Some(403726925)));
    glfw.window_hint(WindowHint::DoubleBuffer(true));
    glfw.window_hint(WindowHint::TransparentFramebuffer(true));

    // glfw window creation
    let (mut window, events) = glfw
        .create_window(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            WINDOW_NAME,
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    // gl: load all OpenGL function pointers
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let (program, _) = unsafe {
        let vertex_shader = compile_shader(gl::VERTEX_SHADER, "episode3/vert.glsl");
        let fragment_shader = compile_shader(gl::FRAGMENT_SHADER, "episode3/frag.glsl");

        let shader_program = link_shader(vertex_shader, fragment_shader);

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        // Create vertex array object
        let mut vao = 0;
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        // Create vertex buffer object
        let mut vbo = 0;
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (VERTEX_POSITIONS.len() * size_of::<GLfloat>()) as GLsizeiptr,
            VERTEX_POSITIONS.as_ptr() as *const c_void,
            gl::STATIC_DRAW,
        );

        // Position attribute
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, ptr::null());
        gl::EnableVertexAttribArray(0);

        // Create index buffer object
        let mut ibo: GLuint = 0;
        gl::GenBuffers(1, &mut ibo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ibo);

        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (size_of::<GLuint>() * INDICES.len()) as isize,
            INDICES.as_ptr() as *const c_void,
            gl::STATIC_DRAW,
        );

        gl::UseProgram(shader_program);
        (shader_program, vao)
    };

    while !window.should_close() {
        process_events(&mut window, &events);

        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // draw our first triangle
            gl::DrawElements(
                gl::TRIANGLES,
                INDICES.len() as i32,
                gl::UNSIGNED_INT,
                ptr::null(),
            );
        }

        window.swap_buffers();
        glfw.poll_events();
    }

    unsafe { gl::DeleteProgram(program) };
}

fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                // make sure the viewport matches the new window dimensions; note that width and
                // height will be significantly larger than specified on retina displays.
                unsafe { gl::Viewport(0, 0, width, height) }
            }
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                window.set_should_close(true);
            }
            _ => {}
        }
    }
}
