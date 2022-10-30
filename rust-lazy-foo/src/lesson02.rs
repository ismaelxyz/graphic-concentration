use sdl2::surface::Surface;
use std::{path::Path, thread::sleep, time::Duration};

// Screen dimensions
const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

fn main() {
    /* Initialize SDL
    We'll just unwrap these - See lesson01 for an example of how to properly
    handle SDL errors */
    let context = sdl2::init().unwrap();
    let video = context.video().unwrap();

    // Create the window
    let window = video
        .window("SDL Tutorial 02", WIDTH, HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    // Obtain a canvas
    let mut canvas = window.into_canvas().build().unwrap();

    // Load the image as a surface - if we can't load the image, we want to know why
    let image_surface = match Surface::load_bmp(Path::new("resources/hello_world.bmp")) {
        Ok(surface) => surface,
        Err(err) => panic!("Could not load image: {err}"),
    };

    // Obtain a TextureCreator<T> for create textures
    // renderer (SDL_CreateRenderer) == canvas + creator
    let creator = canvas.texture_creator();

    // At this point, we could do any number of transformations on the surface, and
    // then when we're ready, we convert it to a texture for quick blitting
    let image_texture = match creator.create_texture_from_surface(&image_surface) {
        Ok(tex) => tex,
        Err(err) => panic!("Could not convert image to texture: {err}"),
    };

    // Clear the current window
    canvas.clear();
    // Blit the texture to the window.  We specify None, None because we're blitting
    // the entire image to the origin of the window.  If we just wanted to blit a subset
    // of the image, or to a particular section of the window, we would specify Some(rect),
    // where rect is a Rect representing the area to blit from/to.
    // We match on the result because it could return an error.  Note that we return (), which
    // is Rust's 'nothing' type.
    match canvas.copy(&image_texture, None, None) {
        Ok(()) => (),
        Err(err) => panic!("Could not render texture: {err}"),
    };
    // Flip the screen buffer.
    canvas.present();

    // Pause for two seconds
    sleep(Duration::new(2, 0));
}
