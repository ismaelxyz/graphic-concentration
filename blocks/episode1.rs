use glfw::{Action, Context, Key, Window, WindowEvent};
use glow::HasContext;
use std::sync::Arc;

/// handle events
unsafe fn process_events(window: &mut Window, gl: &Arc<glow::Context>, event: WindowEvent) {
    match event {
        WindowEvent::FramebufferSize(width, height) => {
            // make sure the viewport matches the new window dimensions; note that width and
            // height will be significantly larger than specified on retina displays.
            gl.viewport(0, 0, width, height);
        }
        WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true);
        }
        _ => {}
    }
}

/// Create window and content.
fn main() {
    // Get glfw context
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

    // glfw window creation
    let (mut window, events) = glfw
        .create_window(800, 600, "Minecraft Clone", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

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
        while !window.should_close() {
            // Check events
            for (_, event) in glfw::flush_messages(&events) {
                process_events(&mut window, &gl, event);
            }

            // Render:

            // Set clear colour
            gl.clear_color(0.7, 0.5, 0.2, 1.0);
            // clear screen
            gl.clear(glow::COLOR_BUFFER_BIT);

            // glfw: swap buffers
            window.swap_buffers();
            // glfw: poll IO events (keys pressed/released, mouse moved etc.)
            glfw.poll_events();
        }
    }
}
