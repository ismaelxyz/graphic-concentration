use glow::HasContext;
use sdl2::{
    event::Event,
    keyboard::Keycode,
    video::{GLProfile, SwapInterval},
};
use std::sync::Arc;
type Matrix = [[f32; 4]; 4];

/// Read shader from source file
pub fn create_shader(
    gl: &Arc<glow::Context>,
    target: u32,
    source: &str,
) -> Result<glow::Shader, String> {
    unsafe {
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

pub struct Program {
    gl: Arc<glow::Context>,
    program: glow::Program,
}

impl Program {
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

            Program {
                gl: gl.clone(),
                program,
            }
        }
    }

    pub fn find_uniform(&self, name: &str) -> Option<glow::NativeUniformLocation> {
        unsafe { self.gl.get_uniform_location(self.program, name) }
    }

    pub fn uniform_matrix(&self, location: Option<&glow::UniformLocation>, matrix: &Matrix) {
        unsafe {
            self.gl.uniform_matrix_4_f32_slice(
                location,
                false,
                bytemuck::cast_slice(&matrix.concat()),
            );
        }
    }

    pub fn bind(&self) {
        unsafe {
            self.gl.use_program(Some(self.program));
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe { self.gl.delete_program(self.program) }
    }
}

const VERTEX_SHADER_SOURCE: &str = "
#version 330 core

layout(location = 0) in vec3 vertex_position;

out vec3 local_position;
// create matrix uniform variable
uniform mat4 matrix;

void main() {
    local_position = vertex_position;
    // multiply matrix by vertex_position vector
    gl_Position = matrix * vec4(vertex_position, 1.0); 
}";

const FRAGMENT_SHADER_SOURCE: &str = "
#version 330 core

out vec4 fragment_colour; 

in vec3 local_position;  

void main() {
    fragment_colour = vec4(local_position / 3.0 + 0.3, 0.7);
}";

#[rustfmt::skip]
const INDEX_DATA: [u32; 6] = [
    0, 1, 2,
    0, 2, 3,
];
/// Initializes matrices and clear color
fn init_gl(gl: &Arc<glow::Context>) -> Program {
    let program = Program::new(gl, VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE);
    unsafe {
        let vertex_array = Some(gl.create_vertex_array().unwrap());
        gl.bind_vertex_array(vertex_array);

        #[rustfmt::skip]
        let vertex_data: [f32; 12] = [
           // X     Y    Z
            -0.5,  0.5, 0.0,
            -0.5, -0.5, 0.0,
             0.5, -0.5, 0.0,
             0.5,  0.5, 0.0,
        ];

        let vertex_buffer = Some(gl.create_buffer().unwrap());
        gl.bind_buffer(glow::ARRAY_BUFFER, vertex_buffer);

        gl.buffer_data_u8_slice(
            glow::ARRAY_BUFFER,
            bytemuck::cast_slice(&vertex_data),
            glow::STATIC_DRAW,
        );

        gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(0);

        let index_buffer = Some(gl.create_buffer().unwrap());
        gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, index_buffer);

        gl.buffer_data_u8_slice(
            glow::ELEMENT_ARRAY_BUFFER,
            bytemuck::cast_slice(&INDEX_DATA),
            glow::STATIC_DRAW,
        );

        program.bind();

        //Initialize clear color
        gl.clear_color(0.0, 0.0, 0.0, 1.0);

        program
    }
}

fn main() {
    let sdl_ctx = sdl2::init().unwrap();

    // Get a handle to the underlying video subsystem
    let video = sdl_ctx.video().unwrap();

    let mut event_pump = sdl_ctx.event_pump().unwrap();

    sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "1");
    sdl2::hint::set("SDL_VIDEO_X11_NET_WM_BYPASS_COMPOSITOR", "0");

    let gl_attr = video.gl_attr();
    // Use OpenGL 3.2 core
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(3, 2);
    gl_attr.set_red_size(8);
    gl_attr.set_green_size(8);
    gl_attr.set_blue_size(8);
    gl_attr.set_alpha_size(8);
    gl_attr.set_double_buffer(true);

    let window = video
        .window("SDL Tutorial 51", 640, 480)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let _gl_sys = window.gl_create_context().unwrap();

    let gl = unsafe {
        Arc::new(glow::Context::from_loader_function(|s| {
            video.gl_get_proc_address(s) as *const _
        }))
    };

    video.gl_set_swap_interval(SwapInterval::VSync).unwrap();
    let program = init_gl(&gl);
    let matrix_location = program.find_uniform("matrix");

    let mut render_quad = true;
    let main_loop = move || {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => return false,
                Event::KeyDown { keycode: k, .. } => match k {
                    Some(Keycode::Escape) => return false,
                    Some(Keycode::Q) => render_quad = !render_quad,
                    _ => (),
                },
                _ => (),
            }
        }

        // Renders quad to the screen
        unsafe {
            // Clear color buffer
            gl.clear(glow::COLOR_BUFFER_BIT);

            // Render quad
            if render_quad {
                program.uniform_matrix(
                    matrix_location.as_ref(),
                    &[
                        [1.0, 0.0, 0.0, 0.0],
                        [0.0, 1.0, 0.0, 0.0],
                        [0.0, 0.0, 1.0, 0.0],
                        [0.0, 0.0, 0.0, 1.0],
                    ],
                );

                gl.draw_elements(
                    glow::TRIANGLES,
                    INDEX_DATA.len() as i32,
                    glow::UNSIGNED_INT,
                    0,
                );
            }
        }

        window.gl_swap_window();

        true
    };

    lazy_foo::setup_mainloop(-1, true, main_loop);
}
