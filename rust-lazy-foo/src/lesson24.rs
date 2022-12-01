use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
    render::{Canvas, Texture, TextureCreator},
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
    fn new(creator: &TextureCreator<WindowContext>, font: &Font, text: &str, color: Color) -> Self {
        let surface = font
            .render(text)
            .solid(color)
            .expect("Could not create text surface!");

        // Now create a texture from the surface using the supplied creator
        let texture = creator
            .create_texture_from_surface(&surface)
            .expect("Could not convert text surface to texture!");

        let (width, height) = surface.size();

        Text {
            texture,
            width,
            height,
        }
    }

    fn render(&self, canvas: &mut Canvas<Window>, x: i32, y: i32) {
        let clip_rect = Rect::new(0, 0, self.width, self.height);

        canvas
            .copy_ex(
                &self.texture,
                Some(clip_rect),
                Some(Rect::new(x, y, clip_rect.width(), clip_rect.height())),
                0.0,
                None,
                false,
                false,
            )
            .expect("Could not blit texture to render target!");
    }

    // Gets image dimensions
    fn get_width(&self) -> u32 {
        self.width
    }
    fn get_height(&self) -> u32 {
        self.height
    }
}

fn main() {
    let sdl_ctx = sdl2::init().expect("Unable to initialize SDL!");
    let video = sdl_ctx.video().expect("Could not acquire video context!");
    let ttf_ctx = sdl2::ttf::init().expect("Could not acquire video context!");
    let time = sdl_ctx.timer().unwrap();

    sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "1");
    sdl2::hint::set("SDL_VIDEO_X11_NET_WM_BYPASS_COMPOSITOR", "0");

    let window = video
        .window("SDL Tutorial 24", 640, 480)
        .position_centered()
        .opengl()
        .build()
        .expect("Could not create SDL window!");
    let (width, height) = window.size();

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .expect("Unable to obtain canvas!");

    let creator = canvas.texture_creator();

    // Initialize renderer color
    canvas.set_draw_color(Color::WHITE);

    // Set text color as black
    let text_color = Color::BLACK;

    // Get a handle to the SDL2 event pump
    let mut event_pump = sdl_ctx
        .event_pump()
        .expect("Unable to obtain event pump handle!");

    let mut counted_frames = 0f64;
    let start_ticks = time.ticks() as f64; // getTicks

    main_loop::setup_mainloop(-1, true, move || {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => return false,
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return false,
                _ => (),
            };
        }

        let font = ttf_ctx
            .load_font("resources/lesson16/lazy.ttf", 28)
            .expect("Could not load font context!");

        // Calculate and correct fps
        let tm = (time.ticks() as f64) - start_ticks;
        let mut avg_fps = counted_frames / (tm / 1000.0f64);

        if avg_fps > 2000000.0 {
            avg_fps = 0.0;
        }

        // Set text to be rendered
        let text: String = format!("Average Frames Per Second {avg_fps}");

        // Load prompt texture
        let time_text = Text::new(&creator, &font, &text, text_color);

        // Clear and render the texture each pass through the loop
        canvas.clear();

        time_text.render(
            &mut canvas,
            (width - time_text.get_width()) as i32 / 2,
            (height - time_text.get_height()) as i32 / 2,
        );

        canvas.present();
        counted_frames += 1.0;

        true
    });
}
