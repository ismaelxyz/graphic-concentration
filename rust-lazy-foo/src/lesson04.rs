use sdl2::{
    event::Event,
    keyboard::Keycode,
    render::{Texture, TextureCreator},
    rwops::RWops,
    surface::Surface,
    video::WindowContext,
};

use std::{cell::RefCell, sync::Arc};

struct Direction {
    up: Arc<RefCell<Texture>>,
    down: Arc<RefCell<Texture>>,
    left: Arc<RefCell<Texture>>,
    right: Arc<RefCell<Texture>>,
    press: Arc<RefCell<Texture>>,
}

/// Take a string describing a path and use it to load
/// an image, and return its surface.
fn load_image(bytes: &[u8]) -> Surface<'static> {
    let mut source = RWops::from_bytes(bytes).unwrap();
    match Surface::load_bmp_rw(&mut source) {
        Ok(surface) => surface,
        Err(err) => panic!("Could not load image: {err}"),
    }
}

/// Take a string describing a path and use it to
/// load an image, and return its texture
fn load_texture(bytes: &[u8], renderer: &TextureCreator<WindowContext>) -> Arc<RefCell<Texture>> {
    let image = load_image(bytes);
    let texture = match renderer.create_texture_from_surface(&image) {
        Ok(tex) => tex,
        Err(err) => panic!("Could not load texture: {err}"),
    };

    Arc::new(RefCell::new(texture))
}

fn main() {
    // Initialize SDL2
    let sdl_ctx = sdl2::init().unwrap();
    let video = sdl_ctx.video().unwrap();

    sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "1");
    sdl2::hint::set("SDL_VIDEO_X11_NET_WM_BYPASS_COMPOSITOR", "0");

    // Create the window
    let window = match video
        .window("SDL Tutorial 04", 640, 480)
        .position_centered()
        .opengl()
        .build()
    {
        Ok(window) => window,
        Err(err) => panic!("Failed to create Window!: {err}"),
    };

    let mut canvas = match window.into_canvas().build() {
        Ok(canvas) => canvas,
        Err(err) => panic!("Could not obtain renderer: {err}"),
    };

    let creator = canvas.texture_creator();
    // Load the textures
    let sprites = Direction {
        up: load_texture(include_bytes!("../resources/up.bmp"), &creator),
        down: load_texture(include_bytes!("../resources/down.bmp"), &creator),
        left: load_texture(include_bytes!("../resources/left.bmp"), &creator),
        right: load_texture(include_bytes!("../resources/right.bmp"), &creator),
        press: load_texture(include_bytes!("../resources/press.bmp"), &creator),
    };
    let mut current_image = Arc::clone(&sprites.press);

    // Obtain the event pump
    let mut event_pump = match sdl_ctx.event_pump() {
        Ok(event_pump) => event_pump,
        Err(err) => panic!("Could not obtain event pump: {err}"),
    };

    // Start up the main loop
    main_loop::setup_mainloop(-1, true, move || {
        // Clear and render the currently selected image
        canvas.clear();

        // We blit the image to the screen corresponding to the keypress,
        // or 'press' otherwise.  Using 'Esc' or 'q' will quit the program.
        for event in event_pump.poll_iter() {
            current_image = Arc::clone(match event {
                Event::Quit { .. } => return false,
                Event::KeyDown { keycode: k, .. } => match k {
                    Some(Keycode::Escape) | Some(Keycode::Q) => return false,
                    Some(Keycode::Up) => &sprites.up,
                    Some(Keycode::Down) => &sprites.down,
                    Some(Keycode::Right) => &sprites.right,
                    Some(Keycode::Left) => &sprites.left,
                    Some(_) => &sprites.press,
                    None => continue,
                },
                _ => continue,
            });
        }

        canvas.copy(&current_image.borrow(), None, None).unwrap();
        canvas.present();

        true
    });
}
