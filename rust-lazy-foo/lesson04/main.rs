use sdl2::{
    event::Event,
    keyboard::Keycode,
    render::{Texture, TextureCreator},
    surface::Surface,
    video::Window,
    Sdl,
};

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

struct Direction<'a> {
    up: Texture<'a>,
    down: Texture<'a>,
    left: Texture<'a>,
    right: Texture<'a>,
    press: Texture<'a>,
}

// Take a string describing a path and use it to load
// an image, and return its surface.
fn load_image(path: &str) -> Surface<'static> {
    use std::path::Path;
    match Surface::load_bmp(&Path::new(path)) {
        Ok(surface) => surface,
        Err(err) => panic!("Could not load image: {}", err),
    }
}

// Take a string describing a path and use it to
// load an image, and return its texture
fn load_texture<'a, T>(path: &str, renderer: &'a TextureCreator<T>) -> Texture<'a> {
    let image = load_image(path);
    match renderer.create_texture_from_surface(&image) {
        Ok(tex) => tex,
        Err(err) => panic!("Could not load texture: {}", err),
    }
}

// Load the textures
fn load_media<T>(creator: &TextureCreator<T>) -> Direction<'_> {
    // Path relative to root of crate
    Direction {
        up: load_texture("resources/up.bmp", creator),
        down: load_texture("resources/down.bmp", creator),
        left: load_texture("resources/left.bmp", creator),
        right: load_texture("resources/right.bmp", creator),
        press: load_texture("resources/press.bmp", creator),
    }
}

// Break out initialization into a separate function, which
// returns only the Window (we don't need the sdl_context)
fn init() -> (Sdl, Window) {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    // Create the window
    let win = match video
        .window("SDL Tutorial 04", WIDTH, HEIGHT)
        .position_centered()
        .opengl()
        .build()
    {
        Ok(window) => window,
        Err(err) => panic!("Failed to create Window!: {}", err),
    };
    (sdl, win)
}

fn main() {
    // Initialize SDL2
    let (context, window) = init();

    let mut canvas = match window.into_canvas().build() {
        Ok(canvas) => canvas,
        Err(err) => panic!("Could not obtain renderer: {}", err),
    };

    let creator = canvas.texture_creator();
    // Load the sprite textures into an hashmap
    let sprites = load_media(&creator);
    let mut current_image = &sprites.press;

    // Obtain the event pump
    let mut event_pump = match context.event_pump() {
        Ok(event_pump) => event_pump,
        Err(err) => panic!("Could not obtain event pump: {}", err),
    };

    // Start up the main loop
    'running: loop {
        // Clear and render the currently selected image
        canvas.clear();

        // We blit the image to the screen corresponding to the keypress,
        // or 'press' otherwise.  Using 'Esc' or 'q' will quit the program.
        for event in event_pump.poll_iter() {
            current_image = match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown { keycode: k, .. } => match k {
                    Some(Keycode::Escape) | Some(Keycode::Q) => break 'running,
                    Some(Keycode::Up) => &sprites.up,
                    Some(Keycode::Down) => &sprites.down,
                    Some(Keycode::Right) => &sprites.right,
                    Some(Keycode::Left) => &sprites.left,
                    Some(_) => &sprites.press,
                    None => continue,
                },
                _ => continue,
            };
        }

        canvas.copy(current_image, None, None).unwrap();
        canvas.present();
    }
}
