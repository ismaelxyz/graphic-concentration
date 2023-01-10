use sdl2::{
    event::Event,
    image::{InitFlag, LoadTexture},
    pixels::Color,
    rect::Rect,
};

const IMG_NAME: &[u8] = include_bytes!("../resources/viewport.png");

fn main() {
    // Initialize SDL2
    let sdl_ctx = sdl2::init().unwrap();
    let video = sdl_ctx.video().unwrap();

    sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "1");
    sdl2::hint::set("SDL_VIDEO_X11_NET_WM_BYPASS_COMPOSITOR", "0");

    let window = video
        .window("SDL Tutorial 09", 640, 480)
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
    let (width, height) = canvas.window().size();

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

        // Create the top left viewport
        let top_left_viewport = Rect::new(0, 0, width / 2, height / 2);
        // Set the viewport.  Note that set_viewport takes an Option<Rect>,
        // so we have to pass either Some(rect) or None (whole window)
        canvas.set_viewport(Some(top_left_viewport));
        // Now render a texture to that viewport
        canvas.copy(&image_texture, None, None).unwrap();

        // Top right viewport.  Same process
        let top_right_viewport = Rect::new(width as i32 / 2, 0, width / 2, height / 2);
        canvas.set_viewport(Some(top_right_viewport));
        canvas.copy(&image_texture, None, None).unwrap();

        // Bottom viewport
        let bottom_viewport = Rect::new(0, height as i32 / 2, width, height / 2);
        canvas.set_viewport(Some(bottom_viewport));
        canvas.copy(&image_texture, None, None).unwrap();
        // Update the screen
        canvas.present();

        true
    });
}
