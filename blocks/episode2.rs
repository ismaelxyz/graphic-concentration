use blocks::{INDICES, VERTEX_POSITIONS};
use glfw::{Action, Context, Key, Window, WindowEvent};
use glow::HasContext;
use std::sync::Arc;

unsafe fn initialize_objects(gl: &Arc<glow::Context>) {
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
}

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    glfw.window_hint(glfw::WindowHint::AlphaBits(Some(u32::MAX - 1)));
    glfw.window_hint(glfw::WindowHint::DepthBits(Some(403726925)));
    glfw.window_hint(glfw::WindowHint::DoubleBuffer(true));
    glfw.window_hint(glfw::WindowHint::TransparentFramebuffer(true));

    let (mut window, events) = glfw
        .create_window(800, 600, "Minecraft Clone", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    let gl = unsafe {
        Arc::new(glow::Context::from_loader_function(|name| {
            window.get_proc_address(name) as *const _
        }))
    };

    unsafe {
        initialize_objects(&gl);

        while !window.should_close() {
            for (_, event) in glfw::flush_messages(&events) {
                process_events(&mut window, &gl, event);
            }

            gl.clear_color(1.0, 0.5, 1.0, 1.0);
            gl.clear(glow::DEPTH_BUFFER_BIT | glow::COLOR_BUFFER_BIT);

            // draw bound buffers to the screen
            gl.draw_elements(glow::TRIANGLES, INDICES.len() as i32, glow::UNSIGNED_INT, 0);

            window.swap_buffers();
            glfw.poll_events();
        }
    }
}

fn process_events(window: &mut Window, gl: &Arc<glow::Context>, event: WindowEvent) {
    match event {
        WindowEvent::FramebufferSize(width, height) => unsafe { gl.viewport(0, 0, width, height) },
        WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true);
        }
        _ => {}
    }
}
