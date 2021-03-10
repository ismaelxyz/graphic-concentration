use sdl2::{
    event::Event,
    image::{InitFlag, LoadTexture},
    keyboard::Scancode,
    render::{Texture, TextureCreator},
    video::Window,
    Sdl,
};
use std::collections::HashMap;
use std::path::Path;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

/// Break out initialization into a separate function, which
/// returns only the Window (we don't need the sdl_context)
fn init() -> (Sdl, Window) {
    let sdl = sdl2::init().expect("Unable to initialize SDL!");
    let video = sdl.video().expect("Could not acquire video context!");
    let win = video
        .window("SDL Tutorial 18", WIDTH, HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .expect("Could not create SDL window!");

    sdl2::image::init(InitFlag::PNG).unwrap();

    (sdl, win)
}

/// Take a string describing a path and use it to
/// load an image, and return its texture
fn load_texture<'a, T>(path: &'static str, creator: &'a TextureCreator<T>) -> Texture<'a> {
    match creator.load_texture(Path::new(path)) {
        Ok(tex) => tex,
        Err(err) => panic!("Could not load texture: {}", err),
    }
}

/// Load the textures we're going to use into an
/// easily indexable HashMap.
fn load_media<T>(creator: &TextureCreator<T>) -> HashMap<&'static str, Box<Texture>> {
    let mut map: HashMap<&'static str, Box<Texture>> = HashMap::new();

    // Path relative to root of crate
    map.insert(
        "up",
        Box::new(load_texture("lesson18/resources/up.png", creator)),
    );
    map.insert(
        "down",
        Box::new(load_texture("lesson18/resources/down.png", creator)),
    );
    map.insert(
        "left",
        Box::new(load_texture("lesson18/resources/left.png", creator)),
    );
    map.insert(
        "right",
        Box::new(load_texture("lesson18/resources/right.png", creator)),
    );
    map.insert(
        "press",
        Box::new(load_texture("lesson18/resources/press.png", creator)),
    );
    map
}

fn main() {
    // Initialize SDL2
    let (sdl_context, window) = init();

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();
    let creator = canvas.texture_creator();

    // Load the sprite textures into an hashmap
    let sprites: HashMap<&'static str, Box<Texture>> = load_media(&creator);

    // Blit the initial image to the window.
    //let mut context = canvas.drawer();

    // Start up the game loop
    let mut running: bool = true;
    let mut event_pump = sdl_context.event_pump().unwrap();

    while running {
        // We blit the image to the screen corresponding to the keypress,
        // or 'press' otherwise.  Using 'Esc' or 'q' will quit the program.
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => running = false,
                _ => {}
            }
        }

        // Instead of using keyboard events to toggle the images,
        // use keyboard state.

        //.pressed_scancodes().filter_map(Keycode::from_scancode).collect();

        let keys = event_pump.keyboard_state();
        let current_image = if keys.is_scancode_pressed(Scancode::Up) {
            "up"
        } else if keys.is_scancode_pressed(Scancode::Down) {
            "down"
        } else if keys.is_scancode_pressed(Scancode::Right) {
            "right"
        } else if keys.is_scancode_pressed(Scancode::Left) {
            "left"
        } else {
            "press"
        };

        // Clear and render the currently selected image
        canvas.clear();
        // sprites[current_image] yields a Box<Texture>, so we use
        // a '&' to reference it.
        canvas.copy(&sprites[current_image], None, None).unwrap();
        canvas.present();
    }
}
