use sdl2::{
    event::Event,
    image::{InitFlag, LoadTexture},
};

fn main() {
    // Initialize SDL2
    let sdl_ctx = sdl2::init().unwrap();
    let video = sdl_ctx.video().unwrap();

    sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "1");
    sdl2::hint::set("SDL_VIDEO_X11_NET_WM_BYPASS_COMPOSITOR", "0");

    // Create the window
    let window = video
        .window("SDL Tutorial 06", 640, 480)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    // As of rust-sdl2 0.27.2, SDL2_IMAGE is now part of the core
    // crate.  So initialize a context for it.  The context by
    // itself is pretty useless, but we need to keep it alive
    // until we're done with it.
    let _image = sdl2::image::init(InitFlag::JPG | InitFlag::PNG).unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let creator = canvas.texture_creator();

    let bytes = include_bytes!("../resources/loaded.png");
    // Load the image
    let image_texture = creator.load_texture_bytes(bytes).unwrap();

    // Get a handle to the SDL2 event pump
    let mut event_pump = sdl_ctx.event_pump().unwrap();

    main_loop::setup_mainloop(-1, true, move || {
        // Extract any pending events from from the event pump and process them
        for event in event_pump.poll_iter() {
            // pattern match on the type of event
            if let Event::Quit { .. } = event {
                return false;
            }
        }
        // Clear and render the texture each pass through the loop
        canvas.clear();
        canvas.copy(&image_texture, None, None).unwrap();
        canvas.present();

        true
    });
}
