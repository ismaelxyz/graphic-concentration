use std::path::Path;

use sdl2::{
    event::Event,
    image::{InitFlag, LoadTexture, Sdl2ImageContext},
    pixels::Color,
    rect::Rect,
    video::Window,
    Sdl,
};

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

const IMG_NAME: &'static str = "resources/viewport.png";

// Note that 'renderer.load_texture' makes this example trivial.  See lesson03
// to show how we can manually load a surface and convert it to a texture.

/// Break out initialization into a separate function, which
/// returns only the Window (we don't need the sdl_context)
fn init() -> (Sdl, Window, Sdl2ImageContext) {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let win = match video
        .window("SDL Tutorial 09", WIDTH, HEIGHT)
        .position_centered()
        .opengl()
        .build()
    {
        Ok(window) => window,
        Err(err) => panic!("Failed to create Window!: {}", err),
    };

    let image = sdl2::image::init(InitFlag::PNG).unwrap();

    (sdl, win, image)
}

fn main() {
    // Initialize SDL2
    let (sdl_context, window, _image) = init();

    let mut canvas = match window.into_canvas().build() {
        Ok(canvas) => canvas,
        Err(err) => panic!("Could not obtain canvas: {}", err),
    };
    let creator = canvas.texture_creator();

    // Load the image
    let image_texture = match creator.load_texture(Path::new(IMG_NAME)) {
        Ok(texture) => texture,
        Err(err) => panic!("Could not load texture: {}", err),
    };

    // Set renderer color using the context
    canvas.set_draw_color(Color::RGB(0, 0, 0));

    // running is 'mut' because we will want to 'flip' it to false when we're ready
    // to exit the game loop.
    let mut running: bool = true;

    // Get a handle to the SDL2 event pump
    let mut event_pump = sdl_context.event_pump().unwrap();

    // game loop
    while running {
        // Extract any pending events from from the event pump and process them
        for event in event_pump.poll_iter() {
            // pattern match on the type of event
            match event {
                Event::Quit { .. } => running = false,
                _ => {}
            }
        }
        // Clear and render the texture each pass through the loop
        canvas.clear();

        // Create the top left viewport
        let top_left_viewport = Rect::new(0, 0, WIDTH / 2, HEIGHT / 2);
        // Set the viewport.  Note that set_viewport takes an Option<Rect>,
        // so we have to pass either Some(rect) or None (whole window)
        canvas.set_viewport(Some(top_left_viewport));
        // Now render a texture to that viewport
        canvas.copy(&image_texture, None, None).unwrap();

        // Top right viewport.  Same process
        let top_right_viewport = Rect::new(WIDTH as i32 / 2, 0, WIDTH / 2, HEIGHT / 2);
        canvas.set_viewport(Some(top_right_viewport));
        canvas.copy(&image_texture, None, None).unwrap();

        // Bottom viewport
        let bottom_viewport = Rect::new(0, HEIGHT as i32 / 2, WIDTH, HEIGHT / 2);
        canvas.set_viewport(Some(bottom_viewport));
        canvas.copy(&image_texture, None, None).unwrap();

        // Update the screen
        canvas.present();
    }
}
