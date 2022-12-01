use sdl2::{
    event::Event,
    image::{InitFlag, LoadSurface},
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
    render::{BlendMode, Canvas, Texture, TextureCreator},
    surface::Surface,
    video::{Window, WindowContext},
};

struct LTexture {
    texture: Texture,
    width: u32,
    height: u32,
}

impl LTexture {
    fn from_file(creator: &TextureCreator<WindowContext>, name: &str) -> LTexture {
        let mut surface = Surface::from_file(&format!("./resources/lesson13/{name}.png")).unwrap();

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

    fn render_to(&self, canvas: &mut Canvas<Window>) {
        canvas
            .copy(
                &self.texture,
                Some(Rect::new(0, 0, self.width, self.height)),
                Some(Rect::new(0, 0, self.width, self.height)),
            )
            .unwrap();
    }

    fn set_blend(&mut self, blending: BlendMode) {
        self.texture.set_blend_mode(blending);
    }

    // Set the alpha channel of the texture, controlling its transparency
    fn set_alpha(&mut self, alpha: u8) {
        self.texture.set_alpha_mod(alpha);
    }
}

fn main() {
    let sdl_ctx = sdl2::init().unwrap();
    let video = sdl_ctx.video().unwrap();

    sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "1");
    sdl2::hint::set("SDL_VIDEO_X11_NET_WM_BYPASS_COMPOSITOR", "0");

    let window = video
        .window("SDL Tutorial 13", 640, 480)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let _image = sdl2::image::init(InitFlag::PNG).unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    let creator = canvas.texture_creator();
    // In the Lazy Foo tutorial, this is delegated to loadMedia(), but since
    // it's so easy to load a texture, we'll just do it here.
    let mut modulated_texture = LTexture::from_file(&creator, "fadeout");
    modulated_texture.set_blend(BlendMode::Blend);
    let background_texture = LTexture::from_file(&creator, "fadein");

    // Get a handle to the SDL2 event pump
    let mut event_pump = sdl_ctx.event_pump().unwrap();

    // Set the current alpha to max (255).
    let mut alpha: u8 = 0xff;

    main_loop::setup_mainloop(-1, true, move || {
        // Extract any pending events from from the event pump and process them
        for event in event_pump.poll_iter() {
            // pattern match on the type of event
            match event {
                Event::Quit { .. } => return false,
                // Use 'w' to increase the alpha, and 's' to decrease it
                Event::KeyDown { keycode: k, .. } => match k {
                    Some(Keycode::W) => {
                        if alpha < 224 {
                            alpha += 32;
                        } else {
                            alpha = 255;
                        }
                    }
                    Some(Keycode::S) => {
                        if alpha > 32 {
                            alpha -= 32;
                        } else {
                            alpha = 0;
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        canvas.set_draw_color(Color::WHITE);
        canvas.clear();
        // Blit the background texture
        background_texture.render_to(&mut canvas);
        // Set the alpha on the modulated texture
        modulated_texture.set_alpha(alpha);
        // Blit the modulated texture over the background
        modulated_texture.render_to(&mut canvas);
        // Update the screen
        canvas.present();

        true
    });
}
