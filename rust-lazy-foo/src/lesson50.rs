use glow::HasContext;
use glu_sys as glu;
use sdl2::{event::Event, keyboard::Keycode, video::SwapInterval};

/// Check for error
unsafe fn get_error(gl: &glow::Context) -> Result<(), String> {
    let error = gl.get_error();
    if error != glow::NO_ERROR {
        let cstr = std::ffi::CStr::from_ptr(glu::gluErrorString(error) as _);
        return Err(String::from_utf8_lossy(cstr.to_bytes()).to_string());
    }

    Ok(())
}

/// Initializes matrices and clear color
fn init_gl(gl: &glow::Context) {
    unsafe {
        // Initialize Projection Matrix
        glu::glMatrixMode(glu::GL_PROJECTION);
        glu::glLoadIdentity();

        get_error(gl).unwrap();

        // Initialize Modelview Matrix
        glu::glMatrixMode(glu::GL_MODELVIEW);
        glu::glLoadIdentity();

        get_error(gl).unwrap();

        // Initialize clear color
        glu::glClearColor(0.0, 0.0, 0.0, 1.0);

        get_error(gl).unwrap();
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
    // Use OpenGL 2.1
    gl_attr.set_context_version(2, 1);

    let window = video
        .window("SDL Tutorial 50", 640, 480)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let _gl_sys = window.gl_create_context().unwrap();

    let gl = unsafe {
        glow::Context::from_loader_function(|s| video.gl_get_proc_address(s) as *const _)
    };

    video.gl_set_swap_interval(SwapInterval::VSync).unwrap();
    init_gl(&gl);

    //let mut canvas = window.into_canvas().build().unwrap();
    let mut render_quad = true;
    let main_loop = move || {
        // We blit the image to the screen corresponding to the keypress,
        // or 'press' otherwise.  Using 'Esc' or 'q' will quit the program.
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
            glu::glClear(glu::GL_COLOR_BUFFER_BIT);

            // Render quad
            if render_quad {
                glu::glBegin(glu::GL_QUADS);

                glu::glVertex2f(-0.5, -0.5);
                glu::glVertex2f(0.5, -0.5);
                glu::glVertex2f(0.5, 0.5);
                glu::glVertex2f(-0.5, 0.5);

                glu::glEnd();
            }
        }

        window.gl_swap_window();

        true
    };

    main_loop::setup_mainloop(
        -1,   // call the function as fast as the browser wants to render (typically 60fps)
        true, // call the function repeatedly
        main_loop,
    );
}
