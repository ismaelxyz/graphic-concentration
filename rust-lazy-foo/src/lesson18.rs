use sdl2::{
    event::Event,
    image::{InitFlag, LoadTexture},
    keyboard::Scancode,
    render::{Texture, TextureCreator},
    video::WindowContext,
};

struct Direction {
    up: Texture,
    down: Texture,
    left: Texture,
    right: Texture,
    press: Texture,
}

/// Take a string describing a path and use it to
/// load an image, and return its texture
#[inline(always)]
fn load_texture(name: &str, creator: &TextureCreator<WindowContext>) -> Texture {
    let relative_path = format!("./resources/lesson18/{name}.png");
    creator
        .load_texture(std::path::Path::new(&relative_path))
        .expect("Could not load texture!")
}

/// Load the textures
#[inline(always)]
fn load_media(creator: &TextureCreator<WindowContext>) -> Direction {
    // Path relative to root of crate
    Direction {
        up: load_texture("up", creator),
        down: load_texture("down", creator),
        left: load_texture("left", creator),
        right: load_texture("right", creator),
        press: load_texture("press", creator),
    }
}

fn main() {
    let sdl_ctx = sdl2::init().expect("Unable to initialize SDL!");
    let video = sdl_ctx.video().expect("Could not acquire video context!");

    sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "1");
    sdl2::hint::set("SDL_VIDEO_X11_NET_WM_BYPASS_COMPOSITOR", "0");
    let window = video
        .window("SDL Tutorial 18", 640, 480)
        .position_centered()
        .opengl()
        .build()
        .expect("Could not create SDL window!");

    sdl2::image::init(InitFlag::PNG).unwrap();

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();

    let creator = canvas.texture_creator();

    // Load the sprite textures into an hashmap
    let sprites = load_media(&creator);

    let mut event_pump = sdl_ctx.event_pump().unwrap();

    lazy_foo::setup_mainloop(-1, true, move || {
        // We blit the image to the screen corresponding to the keypress,
        // or 'press' otherwise.  Using 'Esc' or 'q' will quit the program.
        for event in event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                return false;
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

        true
    });
}
