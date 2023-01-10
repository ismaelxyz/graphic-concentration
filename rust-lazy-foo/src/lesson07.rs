//! Note that 'renderer.load_texture' makes this example trivial.  See lesson03
//! to show how we can manually load a surface and convert it to a texture.

use sdl2::{
    event::Event,
    image::{InitFlag, LoadTexture},
    pixels::Color,
};

/// Start using Path for filepaths.
const IMG_NAME: &[u8] = include_bytes!("../resources/texture.png");

fn main() {
    // Initialize SDL2
    let sdl_ctx = sdl2::init().unwrap();
    let video = sdl_ctx.video().unwrap();

    sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "1");
    sdl2::hint::set("SDL_VIDEO_X11_NET_WM_BYPASS_COMPOSITOR", "0");

    // Create the window
    let window = video
        .window("SDL Tutorial 07", 640, 480)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let _image = sdl2::image::init(InitFlag::PNG).unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let creator = canvas.texture_creator();

    // Load the image
    let image_texture = creator.load_texture_bytes(IMG_NAME).unwrap();

    // Set renderer color using the context
    canvas.set_draw_color(Color::BLACK);

    // Get a handle to the SDL2 event pump
    let mut event_pump = sdl_ctx.event_pump().unwrap();

    lazy_foo::setup_mainloop(-1, true, move || {
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
