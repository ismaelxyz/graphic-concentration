use glfw::{Action, Context, Key, Window, WindowEvent};

// Screen dimensions and title
const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;
const WINDOW_NAME: &str = "Minecraft Clone";

// handle events
fn process_events(window: &mut Window, event: WindowEvent) {
    match event {
        WindowEvent::FramebufferSize(width, height) => {
            // make sure the viewport matches the new window dimensions; note that width and
            // height will be significantly larger than specified on retina displays.
            unsafe { gl::Viewport(0, 0, width, height) }
        }
        WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true);
        }
        _ => {}
    }
}

// Create window and content.
fn main() {
    // Get glfw context
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

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

    // Main loop
    while !window.should_close() {
        // Check events
        for (_, event) in glfw::flush_messages(&events) {
            process_events(&mut window, event);
        }

        // Render:
        unsafe {
            // Set clear colour
            gl::ClearColor(1.0, 0.5, 1.0, 1.0);
            // clear screen
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // glfw: swap buffers
        window.swap_buffers();
        // glfw: poll IO events (keys pressed/released, mouse moved etc.)
        glfw.poll_events();
    }
}
