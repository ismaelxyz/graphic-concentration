use sdl2::{
    event::Event,
    image::{InitFlag, LoadSurface},
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
    render::{Canvas, Texture, TextureCreator},
    surface::Surface,
    video::{Window, WindowContext},
};

struct LTexture {
    texture: Texture,
    width: u32,
    height: u32,
}

impl LTexture {
    fn from_file(creator: &TextureCreator<WindowContext>, path: &std::path::Path) -> LTexture {
        let mut surface = Surface::from_file(path).unwrap();

        surface
            .set_color_key(true, Color::RGB(0, 0xff, 0xff))
            .unwrap();

        let texture = creator.create_texture_from_surface(&surface).unwrap();

        LTexture {
            texture,
            width: surface.width(),
            height: surface.height(),
        }
    }

    /// Renders a texture to a given point using a provided renderer
    fn render_to(&self, canvas: &mut Canvas<Window>) {
        canvas
            .copy(
                &self.texture,
                Some(Rect::new(0, 0, self.width, self.height)),
                Some(Rect::new(0, 0, self.width, self.height)),
            )
            .unwrap();
    }

    // Modulate the LTexture using a Color - this will 'tint' the texture
    // Note that LTextures are immutable, so we have to create a new one
    // and return it - we can't mutate ourselves.
    fn set_color(&mut self, color: Color) {
        let (r, g, b) = color.rgb();
        self.texture.set_color_mod(r, g, b);
    }
}

fn main() {
    let sdl_ctx = sdl2::init().unwrap();
    let video = sdl_ctx.video().unwrap();

    sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "1");
    sdl2::hint::set("SDL_VIDEO_X11_NET_WM_BYPASS_COMPOSITOR", "0");

    let window = video
        .window("SDL Tutorial 12", 640, 480)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let _image = sdl2::image::init(InitFlag::PNG).unwrap();

    // obtain the canvas
    let mut canvas = window.into_canvas().build().unwrap();

    let creator = canvas.texture_creator();

    // Create the textures we are going to use.
    let mut texture = LTexture::from_file(
        &creator,
        std::path::Path::new("./resources/lesson12/colors.png"),
    );

    // Get a handle to the SDL2 event pump
    let mut event_pump = sdl_ctx.event_pump().unwrap();

    // Create the Color we're going to use to modulate the texture
    // As we're allowing the user to alter this tint, we will make
    // it mutable
    let (mut red_tint, mut green_tint, mut blue_tint) = (0xff, 0xff, 0xff);

    main_loop::setup_mainloop(-1, true, move || {
        // Extract any pending events from from the event pump and process them
        for event in event_pump.poll_iter() {
            // pattern match on the type of event
            match event {
                Event::Quit { .. } => return false,
                Event::KeyDown { keycode: k, .. } => match k {
                    // The keys 'q', 'w' and 'e' increase the red, green and blue of the tint,
                    // the keys 'a', 's' and 'd' decrease them.  We check to make sure we don't
                    // overflow the 1-byte value for each color channel.
                    Some(Keycode::Escape) => return false,
                    Some(Keycode::Q) => {
                        if red_tint < 224 {
                            red_tint += 32;
                        }
                    }
                    Some(Keycode::W) => {
                        if green_tint < 224 {
                            green_tint += 32;
                        }
                    }
                    Some(Keycode::E) => {
                        if blue_tint < 224 {
                            blue_tint += 32;
                        }
                    }
                    Some(Keycode::A) => {
                        if red_tint > 32 {
                            red_tint -= 32;
                        }
                    }
                    Some(Keycode::S) => {
                        if green_tint > 32 {
                            green_tint -= 32;
                        }
                    }
                    Some(Keycode::D) => {
                        if blue_tint > 32 {
                            blue_tint -= 32;
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        // Clear and render the texture each pass through the loop
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        // Tint the texture
        texture.set_color(Color::RGB(red_tint, green_tint, blue_tint));
        // Blit the texture
        texture.render_to(&mut canvas);

        // Update the screen
        canvas.present();

        true
    });
}
