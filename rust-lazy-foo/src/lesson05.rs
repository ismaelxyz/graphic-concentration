use sdl2::{
    event::Event,
    rect::Rect,
    render::{Texture, TextureCreator},
    surface::Surface,
    video::Window,
    Sdl,
};

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

// This is a bit of a strange example.  It seems that the copy function
// will automatically stretch a texture if not provided with a clipping
// rectangle to blit too, so the 'stretching' is done by default during
// the render.
//
// However, it's still instructive to show how a surface can be
// stretched into different dimensions for purposes of blitting
// to other surfaces or in preparation to be rendered to a texture.

// Break out initialization into a separate function, which
// returns only the Window (we don't need the sdl_context)
fn init() -> (Sdl, Window) {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    // Create the window
    let win = match video
        .window("SDL Tutorial 05", WIDTH, HEIGHT)
        .position_centered()
        .opengl()
        .build()
    {
        Ok(window) => window,
        Err(err) => panic!("Failed to create Window!: {err}"),
    };
    (sdl, win)
}

// Take a string describing a path and use it to load
// an image, and return its optimized surface.
fn load_image(path: &str) -> Surface {
    use std::path::Path;
    match Surface::load_bmp(Path::new(path)) {
        Ok(surface) => surface,
        Err(err) => panic!("Could not load image: {err}"),
    }
}

// Take a string describing a path and use it to
// load an image, and return its texture
fn surface_to_texture<'a, T>(
    sfc: &'a Surface<'static>,
    renderer: &'a TextureCreator<T>,
) -> Texture<'a> {
    match renderer.create_texture_from_surface(sfc) {
        Ok(tex) => tex,
        Err(err) => panic!("Could not load texture: {err}"),
    }
}

fn main() {
    // Initialize SDL2
    let (context, window) = init();

    // Get a handle to the SDL2 event pump.  This is done here because we
    // used to need to pass the event pump to a function called 'properties_getters
    // on the window in order to retrieve the window's pixel format.  Thank
    // the gods that rust-sdl doesn't require such shenanigans anymore.
    // We still need the event pump for later, so we keep the line.
    let mut event_pump = context.event_pump().unwrap();

    // Load the image.  Note that we do this after we get the event_pump,
    // because in order to optimize the surface we need the PixelFormat
    // used by the window, which requires a WindowProperties, which can
    // only be obtained by passing an event pump handle to ensure no
    // unsafe operations occur while the event pump is running.
    let image_surface = load_image("resources/stretch.bmp");
    // Now optimize it, using the pixel format used by the window
    let pixel_format = window.window_pixel_format();
    let sf_pixel_format = image_surface.pixel_format();
    let optimized_surface = match image_surface.convert(&sf_pixel_format) {
        Ok(surface) => surface,
        Err(err) => panic!("Could not convert surface: {err}"),
    };

    // Now stretch the optimized surface to the dimensions we want
    let dst_rect = Rect::new(0, 0, WIDTH, HEIGHT);
    let mut stretched_surface = match Surface::new(WIDTH, HEIGHT, pixel_format) {
        Ok(surface) => surface,
        Err(err) => panic!("Could not create surface: {err}"),
    };
    // blit_scaled does not return anything, but it does return an SdlResult, so
    // we unwrap to trigger the panic if we can't blit.
    optimized_surface
        .blit_scaled(None, &mut stretched_surface, Some(dst_rect))
        .unwrap();

    let mut canvas = match window.into_canvas().build() {
        Ok(renderer) => renderer,
        Err(err) => panic!("Could not obtain renderer: {err}"),
    };

    let creator = canvas.texture_creator();
    // Convert the surface to a texture
    let image_texture = surface_to_texture(&stretched_surface, &creator);

    // Main loop
    'running: loop {
        // Extract any pending events from from the event pump and process them
        for event in event_pump.poll_iter() {
            // pattern match on the type of event
            if let Event::Quit { .. } = event {
                break 'running;
            }
        }
        // Clear and render the texture each pass through the loop
        canvas.clear();
        canvas.copy(&image_texture, None, None).unwrap();
        canvas.present();
    }
}
