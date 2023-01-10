use sdl2::{
    event::Event,
    keyboard::Keycode,
    render::{Texture, TextureCreator},
    rwops::RWops,
    surface::Surface,
    video::WindowContext,
};

const X_IMAGE: &[u8] = include_bytes!("../resources/x.bmp");

/// Take a string describing a path and use it to load
/// an image, and return its surface.
fn load_image(bytes: &[u8]) -> Surface<'static> {
    let mut source = RWops::from_bytes(bytes).unwrap();
    match Surface::load_bmp_rw(&mut source) {
        Ok(surface) => surface,
        Err(err) => panic!("Could not load image: {err}"),
    }
}

/// Take a string describing a path and use it to
/// load an image, and return its texture
fn load_texture(bytes: &[u8], renderer: &TextureCreator<WindowContext>) -> Texture {
    let image = load_image(bytes);
    match renderer.create_texture_from_surface(&image) {
        Ok(tex) => tex,
        Err(err) => panic!("Could not load texture: {err}"),
    }
}

fn main() {
    // Initialize SDL2
    let sdl_ctx = sdl2::init().unwrap();
    let video = sdl_ctx.video().unwrap();

    sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "1");
    sdl2::hint::set("SDL_VIDEO_X11_NET_WM_BYPASS_COMPOSITOR", "0");

    // Create the window
    let window = match video
        .window("SDL Tutorial 03", 640, 480)
        .position_centered()
        .opengl()
        .build()
    {
        Ok(window) => window,
        Err(err) => panic!("Failed to create Window!: {err}",),
    };

    let mut canvas = match window.into_canvas().build() {
        Ok(canvas) => canvas,
        Err(err) => panic!("Could not obtain renderer: {err}"),
    };

    let creator = canvas.texture_creator();
    // Load the image
    let image_texture = load_texture(X_IMAGE, &creator);

    // Get a handle to the SDL2 event pump
    let mut event_pump = match sdl_ctx.event_pump() {
        Ok(event_pump) => event_pump,
        Err(err) => panic!("Could not obtain event pump: {err}"),
    };

    lazy_foo::setup_mainloop(-1, true, move || {
        // Extract any pending events from from the event pump and process them
        for event in event_pump.poll_iter() {
            // pattern match on the type of event
            if let Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } = event
            {
                return false;
            }
        }
        // render the texture each pass through the loop
        canvas.clear();
        canvas.copy(&image_texture, None, None).unwrap();
        canvas.present();

        true
    });
}
