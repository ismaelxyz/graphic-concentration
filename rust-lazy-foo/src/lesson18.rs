use sdl2::{
    event::Event,
    image::{InitFlag, LoadTexture},
    keyboard::Scancode,
    render::{Texture, TextureCreator},
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

// Take a string describing a path and use it to
// load an image, and return its texture
fn load_texture<'a, T>(path: &'a str, creator: &'a TextureCreator<T>) -> Texture<'a> {
    match creator.load_texture(std::path::Path::new(path)) {
        Ok(tex) => tex,
        Err(err) => panic!("Could not load texture: {}", err),
    }
}

// Load the textures
fn load_media<T>(creator: &TextureCreator<T>) -> Direction<'_> {
    // Path relative to root of crate
    Direction {
        up: load_texture("resources/up.png", creator),
        down: load_texture("resources/down.png", creator),
        left: load_texture("resources/left.png", creator),
        right: load_texture("resources/right.png", creator),
        press: load_texture("resources/press.png", creator),
    }
}

// Break out initialization into a separate function, which
// returns only the Window (we don't need the sdl_context)
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

fn main() {
    // Initialize SDL2
    let (context, window) = init();

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();

    let creator = canvas.texture_creator();

    // Load the sprite textures into an hashmap
    let sprites = load_media(&creator);

    // Blit the initial image to the window.
    //let mut context = canvas.drawer();

    let mut event_pump = context.event_pump().unwrap();

    // Start up the main loop
    'running: loop {
        // We blit the image to the screen corresponding to the keypress,
        // or 'press' otherwise.  Using 'Esc' or 'q' will quit the program.
        for event in event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                break 'running;
            }
        }

        // Clear and render the currently selected image
        canvas.clear();

        // Instead of using keyboard events to toggle the images,
        // use keyboard state.
        let keys = event_pump.keyboard_state();

        // sprites[current_image] yields a Box<Texture>, so we use
        // a '&' to reference it.
        if keys.is_scancode_pressed(Scancode::Up) {
            canvas.copy(&sprites.up, None, None).unwrap();
        } else if keys.is_scancode_pressed(Scancode::Down) {
            canvas.copy(&sprites.down, None, None).unwrap();
        } else if keys.is_scancode_pressed(Scancode::Right) {
            canvas.copy(&sprites.right, None, None).unwrap();
        } else if keys.is_scancode_pressed(Scancode::Left) {
            canvas.copy(&sprites.left, None, None).unwrap();
        } else {
            canvas.copy(&sprites.press, None, None).unwrap();
        }
        canvas.present();
    }
}
