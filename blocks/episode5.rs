use blocks::{Block, Matrix, Shader, TextureManager};
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

/// Scene Manager
struct Manager {
    gl: Arc<glow::Context>,
    texture_manager: TextureManager,
    shader: Shader,
    shader_matrix_location: Option<glow::NativeUniformLocation>,
    shader_sampler_location: Option<glow::NativeUniformLocation>,
    x: f32,
    rx: Receiver<f32>,
}

impl Manager {
    fn new(gl: &Arc<glow::Context>, texture_manager: TextureManager, block: &Block) -> Self {
        unsafe {
            // create vertex array object
            let vertex_array = Some(gl.create_vertex_array().unwrap());
            gl.bind_vertex_array(vertex_array);

            // create vertex buffer object
            let vertex_buffer = Some(gl.create_buffer().unwrap());
            gl.bind_buffer(glow::ARRAY_BUFFER, vertex_buffer);

            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                bytemuck::cast_slice(block.vertex_positions()),
                glow::STATIC_DRAW,
            );

            gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, 0, 0);
            gl.enable_vertex_attrib_array(0);

            // create tex coord vbo
            let tex_vertex_buffer = Some(gl.create_buffer().unwrap());
            gl.bind_buffer(glow::ARRAY_BUFFER, tex_vertex_buffer);

            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                bytemuck::cast_slice(block.tex_coords()),
                glow::STATIC_DRAW,
            );

            gl.vertex_attrib_pointer_f32(1, 3, glow::FLOAT, false, 0, 0);
            gl.enable_vertex_attrib_array(1);

            // create index buffer object
            let index_buffer = Some(gl.create_buffer().unwrap());
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, index_buffer);

            gl.buffer_data_u8_slice(
                glow::ELEMENT_ARRAY_BUFFER,
                bytemuck::cast_slice(block.indices()),
                glow::STATIC_DRAW,
            );

            // create shader
            let shader = Shader::new(gl, "vert5", "frag5");
            // get the shader matrix uniform location
            let shader_matrix_location = shader.find_uniform("matrix");
            // find our texture array sampler's uniform
            let shader_sampler_location = shader.find_uniform("texture_array_sampler");
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
                gl: gl.clone(),
                texture_manager,
                shader,
                shader_matrix_location,
                shader_sampler_location,
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
        mv_matrix.translate(0.0, 0.0, -3.0);
        mv_matrix.rotate_2d(self.x, (self.x / 3.0 * 2.0).sin() / 2.);

        // multiply the two matrices together and send to the shader program
        let mvp_matrix = p_matrix * mv_matrix;
        self.shader
            .uniform_matrix(self.shader_matrix_location.as_ref(), mvp_matrix);

        // bind textures

        // set our active texture unit to the first texture unit
        self.gl.active_texture(glow::TEXTURE0);
        // bind our texture manager's texture
        self.gl
            .bind_texture(glow::TEXTURE_2D_ARRAY, self.texture_manager.texture_array());
        // tell our sampler our texture is bound to the first texture unit
        self.gl
            .uniform_1_i32(self.shader_sampler_location.as_ref(), 0);
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
    glfw.window_hint(WindowHint::DepthBits(Some(16)));
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
        // create our texture manager (256 textures that are 16 x 16 pixels each)
        let mut texture_manager = TextureManager::new(&gl, 16, 16, 256);

        let grass = texture_manager
            .build_block("grass")
            .top("grass")
            .bottom("dirt")
            .sides("grass_side")
            .build();

        texture_manager.finish();
        let mut manager = Manager::new(&gl, texture_manager, &grass);

        while !window.should_close() {
            process_events(&mut window, &gl, &events);

            manager.draw(&window);

            // enable depth testing so faces are drawn in the right order
            gl.enable(glow::DEPTH_TEST);
            gl.clear_color(0.0, 0.0, 0.0, 1.0);
            gl.clear(glow::DEPTH_BUFFER_BIT | glow::COLOR_BUFFER_BIT);

            gl.draw_elements(
                glow::TRIANGLES,
                grass.indices().len() as i32,
                glow::UNSIGNED_INT,
                0,
            );

            window.swap_buffers();
            glfw.poll_events();
        }
    };
}
