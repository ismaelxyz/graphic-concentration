use sdl2::{
    event::Event,
    image::InitFlag,
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
    render::{Canvas, Texture, TextureCreator},
    surface::Surface,
    ttf::Font,
    video::{Window, WindowContext},
};

pub struct Text {
    texture: Texture,
    width: u32,
    height: u32,
}

impl Text {
    // Creates image from font string
    fn create(creator: &TextureCreator<WindowContext>, font: &Font, text: &str) -> Text {
        let surface: Surface = font
            .render(text)
            .solid(Color::BLACK)
            .expect("Could not create text surface!");

        // Now create a texture from the surface using the supplied creator
        let texture = creator
            .create_texture_from_surface(&surface)
            .expect("Could not convert text surface to texture!");

        let (width, height) = surface.size();
        // Return an Text using the given text_texture
        Self {
            texture,
            width,
            height,
        }
    }

    // Renders texture at given point
    fn render(&self, canvas: &mut Canvas<Window>, x: i32, y: i32) {
        canvas
            .copy_ex(
                &self.texture,
                Some(Rect::new(0, 0, self.width, self.height)),
                Some(Rect::new(x, y, self.width, self.height)),
                0.0,
                None,
                false,
                false,
            )
            .expect("Could not blit texture to render target!");
    }

    // Gets image dimensions
    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }
}

fn main() {
    let sdl_ctx = sdl2::init().expect("Unable to initialize SDL!");
    let video = sdl_ctx.video().expect("Could not acquire video context!");
    let ttf_ctx = sdl2::ttf::init().expect("Could not acquire video context!");

    sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "1");
    sdl2::hint::set("SDL_VIDEO_X11_NET_WM_BYPASS_COMPOSITOR", "0");

    let window = video
        .window("SDL Tutorial 22", 640, 480)
        .position_centered()
        .opengl()
        .build()
        .expect("Could not create SDL window!");

    sdl2::image::init(InitFlag::PNG).expect("Unable to initialize sdl2_image!");
    // Initialize TimerSubsystem
    let time = sdl_ctx.timer().unwrap();
    let (width, height) = window.size();
    let mut canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .expect("Unable to obtain canvas!");

    let creator = canvas.texture_creator();
    canvas.set_draw_color(Color::WHITE);

    // Load prompt texture
    let prompt = {
        let font = ttf_ctx
            .load_font(std::path::Path::new("resources/lesson16/lazy.ttf"), 28)
            .expect("Could not load font context!");

        Text::create(&creator, &font, "Press Enter to Reset Start Time.")
    };

    // Get a handle to the SDL2 event pump
    let mut event_pump = sdl_ctx
        .event_pump()
        .expect("Unable to obtain event pump handle!");

    let mut start_time = 0i32;

    main_loop::setup_mainloop(-1, true, move || {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => return false,
                Event::KeyDown { keycode: k, .. } => match k {
                    Some(Keycode::Escape) => return false,
                    Some(Keycode::Return) => start_time = time.ticks() as i32,
                    _ => (),
                },
                _ => (),
            };
        }

        let font = ttf_ctx
            .load_font(std::path::Path::new("resources/lesson16/lazy.ttf"), 28)
            .expect("Could not load font context!");

        let text: String = format!(
            "Milliseconds since start time {}",
            time.ticks() as i32 - start_time
        );
        let time_text = Text::create(&creator, &font, text.as_str());
        // Clear and render the texture each pass through the loop
        canvas.clear();

        // Render the Image
        prompt.render(&mut canvas, (width - prompt.get_width()) as i32 / 2, 0);

        time_text.render(
            &mut canvas,
            (width - prompt.get_width()) as i32 / 2,
            (height - prompt.get_height()) as i32 / 2,
        );

        canvas.present();
        true
    });
}
