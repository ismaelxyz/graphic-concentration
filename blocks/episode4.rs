use blocks::{Matrix, Shader};
use glfw::{Action, Context, Key, OpenGlProfileHint, WindowHint};
use glow::HasContext;

use std::{
    sync::{
        mpsc::{channel, Receiver},
        Arc,
    },
    thread::sleep,
    time::Duration,
};

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
    shader_matrix_location: Option<glow::NativeUniformLocation>,
    x: f32,
    rx: Receiver<f32>,
}

impl Manager {
    fn new(gl: &Arc<glow::Context>) -> Self {
        unsafe {
            // create vertex array object
            let vertex_array = Some(gl.create_vertex_array().unwrap());
            gl.bind_vertex_array(vertex_array);

            // create vertex buffer object
            let vertex_buffer = Some(gl.create_buffer().unwrap());
            gl.bind_buffer(glow::ARRAY_BUFFER, vertex_buffer);

            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                bytemuck::cast_slice(&VERTEX_POSITIONS),
                glow::STATIC_DRAW,
            );

            gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, 0, 0);
            gl.enable_vertex_attrib_array(0);

            // create index buffer object
            let index_buffer = Some(gl.create_buffer().unwrap());
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, index_buffer);

            gl.buffer_data_u8_slice(
                glow::ELEMENT_ARRAY_BUFFER,
                bytemuck::cast_slice(&INDICES),
                glow::STATIC_DRAW,
            );

            // create shader
            let shader = Shader::new(gl, "vert4", "frag4");
            // get the shader matrix uniform location
            let shader_matrix_location = shader.find_uniform("matrix");
            shader.bind();

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
        mv_matrix.rotate_2d(
            self.x + std::f32::consts::TAU / 4.0,
            (self.x / 3.0 * 2.0).sin() / 2.,
        );

        // multiply the two matrices together and send to the shader program
        let mvp_matrix = p_matrix * mv_matrix;
        self.shader
            .uniform_matrix(self.shader_matrix_location.as_ref(), mvp_matrix)
    }
}

unsafe fn process_events(
    window: &mut glfw::Window,
    gl: &Arc<glow::Context>,
    events: &Receiver<(f64, glfw::WindowEvent)>,
) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                // make sure the viewport matches the new window dimensions; note that width and
                // height will be significantly larger than specified on retina displays.
                gl.viewport(0, 0, width, height);
            }
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
        .create_window(800, 600, "Minecraft Clone", glfw::WindowMode::Windowed)
        .unwrap();

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    // gl: load all OpenGL function pointers
    let gl = unsafe {
        Arc::new(glow::Context::from_loader_function(|name| {
            window.get_proc_address(name) as *const _
        }))
    };
    unsafe {
        let mut manager = Manager::new(&gl);
        while !window.should_close() {
            process_events(&mut window, &gl, &events);

            manager.draw(&window);

            gl.clear_color(0.0, 0.0, 0.0, 1.0);
            gl.clear(glow::DEPTH_BUFFER_BIT | glow::COLOR_BUFFER_BIT);

            gl.draw_elements(glow::TRIANGLES, INDICES.len() as i32, glow::UNSIGNED_INT, 0);

            window.swap_buffers();
            glfw.poll_events();
        }
    };
}
