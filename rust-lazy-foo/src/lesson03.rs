use sdl2::{
    event::Event,
    render::{Texture, TextureCreator},
    surface::Surface,
    video::Window,
    Sdl,
};

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;
const X_IMAGE: &str = "resources/x.bmp";

/// Break out initialization into a separate function, which
/// returns only the Window (we don't need the video context)
fn init() -> (Sdl, Window) {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    // Create the window
    let win = match video
        .window("SDL Tutorial 03", WIDTH, HEIGHT)
        .position_centered()
        .opengl()
        .build()
    {
        Ok(window) => window,
        Err(err) => panic!("Failed to create Window!: {}", err),
    };
    (sdl, win)
}

/// Take a string describing a path and use it to load
/// an image, and return its surface.
fn load_image(path: &str) -> Surface<'static> {
    use std::path::Path;
    match Surface::load_bmp(&Path::new(path)) {
        Ok(surface) => surface,
        Err(err) => panic!("Could not load image: {}", err),
    }
}

/// Take a string describing a path and use it to
/// load an image, and return its texture
fn load_texture<'a, T>(path: &'static str, renderer: &'a TextureCreator<T>) -> Texture<'a> {
    let image = load_image(path);
    match renderer.create_texture_from_surface(&image) {
        Ok(tex) => tex,
        Err(err) => panic!("Could not load texture: {}", err),
    }
}

fn main() {
    // Initialize SDL2
    let (context, window) = init();

    let mut canvas = match window.into_canvas().build() {
        Ok(canvas) => canvas,
        Err(err) => panic!("Could not obtain renderer: {}", err),
    };

    let creator = canvas.texture_creator();
    // Load the image
    let image_texture = load_texture(X_IMAGE, &creator);

    // Get a handle to the SDL2 event pump
    let mut event_pump = match context.event_pump() {
        Ok(event_pump) => event_pump,
        Err(err) => panic!("Could not obtain event pump: {}", err),
    };

    'running: loop {
        // Extract any pending events from from the event pump and process them
        for event in event_pump.poll_iter() {
            // pattern match on the type of event
            if let Event::Quit { .. } = event {
                break 'running;
            }
        }
        // render the texture each pass through the loop
        canvas.clear();
        canvas.copy(&image_texture, None, None).unwrap();
        canvas.present();
    }
}
