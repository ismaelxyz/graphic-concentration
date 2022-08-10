mod first;
mod second;
mod shader;

pub mod prelude {
    pub use crate::{shader::*, Program, ProgramData};
}

use glfw::{Action, Context, Key, OpenGlProfileHint, WindowHint};
use std::sync::mpsc::Receiver;

const WINDOW_NAME: &str = "Programs";
const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

pub struct ProgramData {
    id: u32,
    vao: u32,
    vbo: u32,
    ibo: u32,
}

impl Drop for ProgramData {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

pub trait Program {
    fn create() -> Self;
    fn data(&self) -> &ProgramData;
    fn run(&mut self);
    fn execute(&mut self) {
        unsafe {
            gl::UseProgram(self.data().id);
            self.run();
        }
    }
}

fn main() {
    // glfw: initialize and configure
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(WindowHint::ContextVersion(3, 3));
    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));
    glfw.window_hint(WindowHint::AlphaBits(Some(u32::MAX - 1)));
    glfw.window_hint(WindowHint::DepthBits(Some(403726925)));
    glfw.window_hint(WindowHint::DoubleBuffer(true));
    // glfw.window_hint(WindowHint::TransparentFramebuffer(true));

    // glfw window creation
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

    // gl: load all OpenGL function pointers
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let (mut first_program, mut second_program) = unsafe {
        println!(
            "GLFW version: {}\nOpenGL version: {}",
            glfw::get_version_string(),
            std::ffi::CStr::from_ptr(gl::GetString(gl::VERSION) as *const i8)
                .to_str()
                .unwrap()
        );

        (first::First::create(), second::Second::create())
    };

    while !window.should_close() {
        process_events(&mut window, &events);

        unsafe {
            gl::ClearColor(0.9, 0.9, 0.9, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            first_program.execute();
            second_program.execute();
        }

        window.swap_buffers();
        glfw.poll_events();
    }
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
