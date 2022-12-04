/*
/*
Nota: Hasta no terminar RIC (Rust Image Crate) no podre terminar texture_manager.py.
Hasta entonces, esto queda en pausa.
*/

mod texture_manager;
#[rustfmt::skip]
mod numbers;
mod block;
mod matrix;
mod shader;

use block::BlockBuild;
use gl::types::*;
use glfw::{Action, Context, Key, OpenGlProfileHint, WindowHint};
use numbers::{INDICES, VERTEX_POSITIONS};
use shader::Shader;
use matrix::Matrix;
use std::{mem::size_of, sync::mpsc::{Receiver, channel}, thread::sleep, time::Duration};
use texture_manager::TextureManager;

const WINDOW_NAME: &str = "Minecraft Clone";
const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

struct Manager {
    textures: TextureManager,
    shader: Shader,
    shader_matrix_location: i32,
    shader_sampler_location: i32,
    rx: Receiver<f32>,
    x: f32
}

impl Manager {
    unsafe fn new() -> Self {
        // create blocks

        // create our texture manager (256 textures that are 16 x 16 pixels each)
        let mut textures = TextureManager::new(16, 16, 256);
        // let textures = &mut texture_manager;

        // create each one of our blocks with the texture manager and a list of
        //textures per face
        let cobblestone = textures.build_block("cobblestone").all_build();
        let grass = textures
            .build_block("grass")
            .top("grass")
            .bottom("dirt")
            .sides_build("grass_side");
        let dirt = textures.build_block("dirt").all_build();
        let stone = textures.build_block("stone").all_build();
        let sand = textures.build_block("sand").all_build();
        let planks = textures.build_block("planks").all_build();
        let log = textures
            .build_block("log")
            .top("log_top")
            .bottom("log_top")
            .sides_build("log_side");

        // generate mipmaps for our texture manager's texture
        textures.generate_mipmaps();

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

        // create tex coord vbo

        let mut tex_coord_vbo = 0;
        gl::GenBuffers(1, &mut tex_coord_vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, tex_coord_vbo);

        gl::BufferData(
            gl::ARRAY_BUFFER,
            (size_of::<GLfloat>() * grass.tex_coords.len()) as isize,
            // use grass block's texture coordinates positions
            grass.tex_coords.as_ptr() as _,
            gl::STATIC_DRAW,
        );

        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
        gl::EnableVertexAttribArray(1);

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
        let shader = Shader::new("episode5/vert.glsl", "episode5/frag.glsl");
        // get the shader matrix uniform location
        let shader_matrix_location = shader.find_uniform("matrix");
        // find our texture array sampler's uniform
        let shader_sampler_location = shader.find_uniform("texture_array_sampler");
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
            textures,
            shader,
            shader_matrix_location,
            shader_sampler_location,
            rx,
            x: 0.0,
        }
    }

    fn draw(&mut self, window: &glfw::Window) {
        if let Ok(delta_time) = self.rx.recv() {
            self.x += delta_time;
        }

        let (width, height) = window.get_size();

        // create projection matrix

        let mut p_matrix = Matrix::identity();
        p_matrix.perspective(90., (width / height) as f32, 0.1, 500.);

        // create modelview matrix
        let mut mv_matrix = Matrix::identity();
        mv_matrix.translate(0.0, 0.0, -3.0);
        mv_matrix.rotate_2d(self.x, (self.x / 3.0 * 2.0).sin() / 2.0);

        // modelviewprojection matrix

        let mut mvp_matrix = p_matrix * mv_matrix;
        self.shader
            .uniform_matrix(self.shader_matrix_location, mvp_matrix);

        // bind textures
        unsafe {
	        // set our active texture unit to the first texture unit
	        gl::ActiveTexture(gl::TEXTURE0);
	        // bind our texture manager's texture
	        gl::BindTexture(gl::TEXTURE_2D_ARRAY, self.textures.texture_array);
	        // tell our sampler our texture is bound to the first texture unit
	        gl::Uniform1i(self.shader_sampler_location, 0);
	    }
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

            // enable depth testing so faces are drawn in the right order
            gl::Enable(gl::DEPTH_TEST);
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::DrawElements(
                gl::TRIANGLES,
                INDICES.len() as i32,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );

            window.swap_buffers();
            glfw.poll_events();
        }
    }
}
*/

use image::open;

fn main() {
    let texture_image = open("/home/Overlord/Projects/blocks/episode5/textures/grass.png")
    .unwrap().into_rgb8();
    //let data = 
    texture_image.iter().for_each(|x| println!("{}", x));
    // println!("{:?}", data);
}