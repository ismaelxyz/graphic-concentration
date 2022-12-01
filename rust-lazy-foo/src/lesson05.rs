//! This is a bit of a strange example.  It seems that the copy function
//! will automatically stretch a texture if not provided with a clipping
//! rectangle to blit too, so the 'stretching' is done by default during
//! the render.
//!
//! However, it's still instructive to show how a surface can be
//! stretched into different dimensions for purposes of blitting
//! to other surfaces or in preparation to be rendered to a texture.

use sdl2::{
    event::Event,
    rect::Rect,
    render::{Texture, TextureCreator},
    rwops::RWops,
    surface::Surface,
    video::WindowContext,
};

const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;

/// Take a string describing a path and use it to load
/// an image, and return its surface.
fn load_image(bytes: &[u8]) -> Surface<'static> {
    let mut source = RWops::from_bytes(bytes).unwrap();
    match Surface::load_bmp_rw(&mut source) {
        Ok(surface) => surface,
        Err(err) => panic!("Could not load image: {err}"),
    }
}

// Take a string describing a path and use it to
// load an image, and return its texture
fn surface_to_texture(sfc: &Surface<'static>, renderer: &TextureCreator<WindowContext>) -> Texture {
    match renderer.create_texture_from_surface(sfc) {
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
    let window = video
        .window("SDL Tutorial 05", 640, 480)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    // Get a handle to the SDL2 event pump.  This is done here because we
    // used to need to pass the event pump to a function called 'properties_getters
    // on the window in order to retrieve the window's pixel format.  Thank
    // the gods that rust-sdl doesn't require such shenanigans anymore.
    // We still need the event pump for later, so we keep the line.
    let mut event_pump = sdl_ctx.event_pump().unwrap();

    // Load the image.  Note that we do this after we get the event_pump,
    // because in order to optimize the surface we need the PixelFormat
    // used by the window, which requires a WindowProperties, which can
    // only be obtained by passing an event pump handle to ensure no
    // unsafe operations occur while the event pump is running.
    let image_surface = load_image(include_bytes!("../resources/stretch.bmp"));

    // Now optimize it, using the pixel format used by the window
    let pixel_format = window.window_pixel_format();
    let sf_pixel_format = image_surface.pixel_format();
    let optimized_surface = image_surface.convert(&sf_pixel_format).unwrap();

    // Now stretch the optimized surface to the dimensions we want
    let dst_rect = Rect::new(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT);
    let mut stretched_surface = Surface::new(SCREEN_WIDTH, SCREEN_HEIGHT, pixel_format).unwrap();
    // blit_scaled does not return anything, but it does return an SdlResult, so
    // we unwrap to trigger the panic if we can't blit.
    optimized_surface
        .blit_scaled(None, &mut stretched_surface, Some(dst_rect))
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let creator = canvas.texture_creator();
    // Convert the surface to a texture
    let image_texture = surface_to_texture(&stretched_surface, &creator);

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
