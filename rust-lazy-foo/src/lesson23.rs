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
    /// Creates image from font string
    fn new(creator: &TextureCreator<WindowContext>, font: &Font, text: &str) -> Self {
        let text_surface: Surface = font
            .render(text)
            .solid(Color::BLACK)
            .expect("Could not create text surface!");

        // Now create a texture from the surface using the supplied creator
        let texture = creator
            .create_texture_from_surface(&text_surface)
            .expect("Could not convert text surface to texture!");

        let (width, height) = text_surface.size();
        // Return an Tex using the given text_texture
        Self {
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

/// The application time based timer
struct LTimer {
    //The clock time when the timer started
    start_ticks: i32,

    //The ticks stored when the timer was paused
    paused_ticks: i32,

    //The timer status
    paused: bool,
    started: bool,
}

impl LTimer {
    /// Initializes variables
    fn new() -> Self {
        Self {
            start_ticks: 0,
            paused_ticks: 0,
            paused: false,
            started: false,
        }
    }

    /// The various clock actions
    fn start(&mut self, t: i32) {
        // Get the current clock time
        self.start_ticks = t;
        self.paused_ticks = 0;
        // Unpause the timer
        self.paused = false;
        // Start the timer
        self.started = true;
    }

    fn stop(&mut self) {
        // Clear tick variables
        self.start_ticks = 0;
        self.paused_ticks = 0;
        // Unpause the timer
        self.paused = false;
        // Stop the timer
        self.started = false;
    }

    fn pause(&mut self, t: i32) {
        //If the timer is running and isn't already paused
        if self.started && !self.paused {
            self.paused = true;
            //Calculate the paused ticks
            self.paused_ticks = t - self.start_ticks;
            self.start_ticks = 0;
        }
    }

    fn unpause(&mut self, t: i32) {
        // If the timer is running and paused
        if self.started && self.paused {
            // Unpause the timer
            self.paused = false;

            //Reset the starting ticks
            self.start_ticks = t - self.paused_ticks;

            //Reset the paused ticks
            self.paused_ticks = 0;
        }
    }

    /// Gets the timer's time
    fn get_ticks(&mut self, t: i32) -> i32 {
        // The actual timer time
        let mut time: i32 = 0;

        // If the timer is running
        if self.started {
            // If the timer is paused
            if self.paused {
                // Return the number of ticks when the timer was paused
                time += self.paused_ticks;
            } else {
                // Return the current time minus the start time
                time = t - self.start_ticks;
            }
        }

        time
    }

    /// Checks the status of the timer
    fn is_started(&self) -> bool {
        // Timer is running and paused or unpaused
        self.started
    }

    fn is_paused(&self) -> bool {
        // Timer is running and paused
        self.paused && self.started
    }
}

fn main() {
    let sdl_ctx = sdl2::init().expect("Unable to initialize SDL!");
    let video = sdl_ctx.video().expect("Could not acquire video context!");
    let ttf_ctx = sdl2::ttf::init().expect("Could not acquire video context!");
    sdl2::image::init(InitFlag::PNG).expect("Unable to initialize sdl2_image!");
    // Initialize TimerSubsystem
    let time = sdl_ctx.timer().unwrap();

    sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "1");
    sdl2::hint::set("SDL_VIDEO_X11_NET_WM_BYPASS_COMPOSITOR", "0");

    let window = video
        .window("SDL Tutorial 23", 640, 480)
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

    // Load stop and pause prompt texture
    let (prompt_start, prompt_pause) = {
        let font = ttf_ctx
            .load_font("resources/lesson16/lazy.ttf", 28)
            .expect("Could not load font context!");

        (
            Text::new(&creator, &font, "Press S to Start or Stop the Timer"),
            Text::new(&creator, &font, "Press P to Pause or Unpause the Timer"),
        )
    };

    // Get a handle to the SDL2 event pump
    let mut event_pump = sdl_ctx
        .event_pump()
        .expect("Unable to obtain event pump handle!");

    // The application timer
    let mut ltime = LTimer::new();

    lazy_foo::setup_mainloop(-1, true, move || {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => return false,
                Event::KeyDown { keycode: k, .. } => match k {
                    Some(Keycode::Escape) => return false,
                    // Start/stop
                    Some(Keycode::S) => {
                        if ltime.is_started() {
                            ltime.stop();
                        } else {
                            ltime.start(time.ticks() as i32);
                        }
                    }
                    // Pause/unpause
                    Some(Keycode::P) => {
                        if ltime.is_paused() {
                            ltime.unpause(time.ticks() as i32);
                        } else {
                            ltime.pause(time.ticks() as i32);
                        }
                    }
                    _ => (),
                },
                _ => (),
            };
        }

        let font = ttf_ctx
            .load_font("resources/lesson16/lazy.ttf", 28)
            .expect("Could not load font context!");

        let text: String = format!(
            "Seconds since start time: {}",
            ltime.get_ticks(time.ticks() as i32) as f32 / 1000.0f32
        );

        let time_text = Text::new(&creator, &font, text.as_str());
        // Clear and render the texture each pass through the loop
        canvas.clear();

        // Render the Image
        prompt_start.render(
            &mut canvas,
            (width - prompt_start.get_width()) as i32 / 2,
            0,
        );

        prompt_pause.render(
            &mut canvas,
            (width - prompt_pause.get_width()) as i32 / 2,
            prompt_start.get_height() as i32,
        );

        time_text.render(
            &mut canvas,
            (width - time_text.get_width()) as i32 / 2,
            (height - time_text.get_height()) as i32 / 2,
        );

        // Update the screen
        canvas.present();

        true
    });
}
