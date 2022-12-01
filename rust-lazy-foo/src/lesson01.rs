use sdl2::{event::Event, keyboard::Keycode, pixels::Color};

// Set Screen dimensions
const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;

fn main() {
    // Initialize SDL
    // Note that we can just call:
    // let sdl_context = sdl2::init(sdl2::INIT_VIDEO).unwrap();
    // because 'unwrap' will panic if the result of the init call is not Ok, otherwise
    // it will unwrap Ok and assign the result to 'sdl_context'
    // We eschew .unwrap() here so we can output the exact SDL error that caused the failure.
    let sdl_context = match sdl2::init() {
        Ok(sdl_context) => sdl_context,
        Err(err) => panic!("SDL could not initialize!  SDL_Error: {err}"),
    };

    // Get a handle to the underlying video subsystem
    let video = match sdl_context.video() {
        Ok(video) => video,
        Err(err) => panic!("Could not obtain handle to the video subsystem! SDL_Error: {err}"),
    };

    let mut event_pump = sdl_context.event_pump().unwrap();

    // #define SDL_HINT_RENDER_SCALE_QUALITY "SDL_RENDER_SCALE_QUALITY"
    sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "1");
    // #define SDL_HINT_VIDEO_X11_NET_WM_BYPASS_COMPOSITOR "SDL_VIDEO_X11_NET_WM_BYPASS_COMPOSITOR"
    sdl2::hint::set("SDL_VIDEO_X11_NET_WM_BYPASS_COMPOSITOR", "0");

    // Create a Window
    // Window::new and sdl2::init (and other funcs return an SdlResult, which
    // is just a wrapper around Result<T, string>.  Result can return one
    // of two values: Ok(T), or Err(string).  Use match to unwrap them.
    let window = match video
        .window("SDL Tutorial 1", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
    {
        Ok(window) => window,
        Err(err) => panic!("SDL could not create a window! SDL_Error: {err}"),
    };

    // There's a major deviation from Lazy Foo's Lesson 1 here,
    // because rust-sdl2 (presumably for safety reasons) doesn't let you access
    // the window's Surface without going through a properties
    // function, which in turn requires that you pass it an event
    // pump so it can verify it's not running.  :-/
    // Instead, we'll obtain a canvas, and use that to update
    // the main window.
    let mut canvas = match window.into_canvas().build() {
        Ok(canvas) => canvas,
        Err(err) => panic!("SDL could not create a renderer! SDL_Error: {err}"),
    };

    let main_loop = move || {
        // We blit the image to the screen corresponding to the keypress,
        // or 'press' otherwise.  Using 'Esc' or 'q' will quit the program.
        for event in event_pump.poll_iter() {
            if let Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } = event
            {
                return false;
            }
        }

        // Use the canvas it to clear and render the screen
        canvas.set_draw_color(Color::RGB(140, 200, 200));
        // Clear the current window
        canvas.clear();
        // Flip the screen buffer.
        canvas.present();

        true
    };

    main_loop::setup_mainloop(
        -1,   // call the function as fast as the browser wants to render (typically 60fps)
        true, // call the function repeatedly
        main_loop,
    );
}
