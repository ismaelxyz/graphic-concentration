use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
    render::{Canvas, Texture, TextureCreator},
    ttf::Font,
    video::{Window, WindowContext},
};

const SCREEN_FPS: f64 = 60.;
const SCREEN_TICK_PER_FRAME: f64 = 1000. / SCREEN_FPS;

pub struct Text {
    texture: Texture,
    width: u32,
    height: u32,
}

impl Text {
    fn new(creator: &TextureCreator<WindowContext>, font: &Font, text: &str, color: Color) -> Text {
        let surface = font
            .render(text)
            .solid(color)
            .expect("Could not create text surface!");

        let texture = creator
            .create_texture_from_surface(&surface)
            .expect("Could not convert text surface to texture!");

        let (width, height) = surface.size();
        Self {
            texture,
            width,
            height,
        }
    }

    // Renders texture at given point
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
            .unwrap();
    }
}

fn main() {
    let sdl_ctx = sdl2::init().expect("Unable to initialize SDL!");
    let video = sdl_ctx.video().expect("Could not acquire video context!");
    let ttf_ctx = sdl2::ttf::init().expect("Could not acquire video context!");
    let mut time = sdl_ctx.timer().unwrap();

    sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "1");
    sdl2::hint::set("SDL_VIDEO_X11_NET_WM_BYPASS_COMPOSITOR", "0");

    let window = video
        .window("SDL Tutorial 25", 650, 480)
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
    canvas.set_draw_color(Color::WHITE);

    let text_color = Color::BLACK;
    let mut event_pump = sdl_ctx
        .event_pump()
        .expect("Unable to obtain event pump handle!");

    let mut counted_frames = 0f64;
    let start_ticks = time.ticks() as f64; // getTicks

    lazy_foo::setup_mainloop(-1, true, move || {
        let current_ticks = time.ticks() as f64; // getTicks

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
            .load_font("resources/lesson16/lazy.ttf", 20)
            .expect("Could not load font context!");

        // Calculate and correct fps
        let tm = (time.ticks() as f64) - start_ticks;
        let mut avg_fps = counted_frames / (tm / 1000.0f64);

        if avg_fps > 2000000.0 {
            avg_fps = 0.0;
        }

        // Set text to be rendered
        let text = format!("Average Frames Per Second (With Cap) {avg_fps}");
        let time_text = Text::new(&creator, &font, &text, text_color);

        // Clear and render the texture each pass through the loop
        canvas.clear();

        time_text.render(
            &mut canvas,
            (width - time_text.width) as i32 / 2,
            (height - time_text.height) as i32 / 2,
        );

        // Update the screen
        canvas.present();
        counted_frames += 1.0;

        // If frame finished early
        let frame_ticks = (time.ticks() as f64) - current_ticks;
        if frame_ticks < SCREEN_TICK_PER_FRAME {
            // Wait remaining time
            time.delay((SCREEN_TICK_PER_FRAME - frame_ticks) as u32);
        }

        true
    });
}
