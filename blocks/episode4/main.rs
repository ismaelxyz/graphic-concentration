use gl::types::*;
use glfw::{Action, Context, Key, OpenGlProfileHint, WindowHint};

mod matrix;
mod shader;
use matrix::Matrix;
use shader::Shader;

use std::{
    mem::size_of,
    ptr,
    sync::mpsc::{channel, Receiver},
    thread::sleep,
    time::Duration,
};

const WINDOW_NAME: &str = "Minecraft Clone";
const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

/// 3d coordinates for each vertex. Set the Z component to 0.0 so that our
/// object is centered
#[rustfmt::skip]
const VERTEX_POSITIONS: [f32; 12] = [
    // X    Y    Z
    -0.5,  0.5, 0.0,
    -0.5, -0.5, 0.0,
     0.5, -0.5, 0.0,
     0.5,  0.5, 0.0,
];

/// Indices for the first and second triangles
#[rustfmt::skip]
const INDICES: [u32; 6] = [
    0, 1, 2,
    0, 2, 3,
];

// Scene Manager
struct Manager {
    shader: Shader,
    shader_matrix_location: i32,
    x: f32,
    rx: Receiver<f32>,
}

impl Manager {
    unsafe fn new() -> Self {
        // create vertex array object
        let mut vao = 0;
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        // create vertex buffer object
        let mut vbo = 0;
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        gl::BufferData(
            gl::ARRAY_BUFFER,
            (size_of::<GLfloat>() * VERTEX_POSITIONS.len()) as isize,
            VERTEX_POSITIONS.as_ptr() as _,
            gl::STATIC_DRAW,
        );

        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
        gl::EnableVertexAttribArray(0);

        // create index buffer object
        let mut ibo = 0;
        gl::GenBuffers(1, &mut ibo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ibo);

        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (size_of::<GLuint>() * INDICES.len()) as isize,
            INDICES.as_ptr() as _,
            gl::STATIC_DRAW,
        );

        // create shader
        let shader = Shader::new("episode4/vert.glsl", "episode4/frag.glsl");
        // get the shader matrix uniform location
        let shader_matrix_location = shader.find_uniform("matrix");
        shader.using();

        // call update function every 60th of a second
        let (tx, rx) = channel();
        std::thread::Builder::new()
            .name("update".to_string())
            .spawn(move || loop {
                sleep(Duration::from_secs_f32(1.0 / 60.));
                if let Err(..) = tx.send(1.0 / 60.) {
                    break;
                }
            })
            .unwrap();

        Manager {
            shader,
            shader_matrix_location,
            x: 0.,
            rx,
        }
    }

    unsafe fn draw(&mut self, window: &glfw::Window) {
        if let Ok(delta_time) = self.rx.recv() {
            self.x += delta_time;
        }

        let (width, height) = window.get_size();
        // create projection matrix
        let mut p_matrix = Matrix::identity();
        p_matrix.perspective(90., (width / height) as f32, 0.1, 500.);

        // create model view matrix
        let mut mv_matrix = Matrix::identity();
        mv_matrix.translate(0.0, 0.0, -1.0);
        mv_matrix.rotate_2d(self.x + 6.28 / 4.0, (self.x / 3.0 * 2.0).sin() / 2.);

        // multiply the two matrices together and send to the shader program
        let mvp_matrix = p_matrix * mv_matrix;
        self.shader
            .uniform_matrix(self.shader_matrix_location, mvp_matrix)
    }
}

fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
                gl::Viewport(0, 0, width, height)
            },
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                window.set_should_close(true);
            }
            _ => {}
        }
    }
}

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(WindowHint::ContextVersion(3, 3));
    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));
    glfw.window_hint(WindowHint::AlphaBits(Some(u32::MAX - 1)));
    glfw.window_hint(WindowHint::DepthBits(Some(403726925)));
    glfw.window_hint(WindowHint::DoubleBuffer(true));
    glfw.window_hint(WindowHint::TransparentFramebuffer(true));

    let (mut window, events) = glfw
        .create_window(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            WINDOW_NAME,
            glfw::WindowMode::Windowed,
        )
        .unwrap();

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let mut manager = unsafe { Manager::new() };
    while !window.should_close() {
        process_events(&mut window, &events);

        unsafe {
            manager.draw(&window);

            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::DrawElements(
                gl::TRIANGLES,
                INDICES.len() as i32,
                gl::UNSIGNED_INT,
                ptr::null(),
            );

            window.swap_buffers();
            glfw.poll_events();
        }
    }
}
