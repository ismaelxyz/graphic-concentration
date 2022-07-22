use sdl2::{
    event::Event,
    image::InitFlag,
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
    render::{Canvas, RenderTarget, Texture, TextureCreator},
    surface::Surface,
    ttf::{Font, Sdl2TtfContext},
    video::Window,
};

// Screen dimension
const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

// Texture wrapper
pub struct Text<'a> {
    // The actual hardware texture
    texture: Texture<'a>,
    // Image dimensions
    width: u32,
    height: u32,
}

impl<'a> Text<'a> {
    fn new(texture: Texture<'a>, width: u32, height: u32) -> Self {
        Self {
            texture,
            width,
            height,
        }
    }

    // Creates image from font string
    fn create<T>(creator: &'a TextureCreator<T>, font: &Font, text: &str) -> Text<'a> {
        let text_surface: Surface = font
            .render(text)
            .solid(Color::BLACK)
            .expect("Could not create text surface!");

        // Now create a texture from the surface using the supplied creator
        let text_texture = creator
            .create_texture_from_surface(&text_surface)
            .expect("Could not convert text surface to texture!");

        let (w, h) = text_surface.size();
        // Return an Text using the given text_texture
        Text::new(text_texture, w, h)
    }

    // Renders texture at given point
    fn render<T: RenderTarget>(&self, canvas: &mut Canvas<T>, x: i32, y: i32) {
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

fn init() -> (sdl2::Sdl, Window, Sdl2TtfContext) {
    let sdl = sdl2::init().expect("Unable to initialize SDL!");
    let video = sdl.video().expect("Could not acquire video context!");
    let ttf = sdl2::ttf::init().expect("Could not acquire video context!");

    // Recuerda es el valor lo que importa no el nombre de la variable
    // #define SDL_HINT_RENDER_SCALE_QUALITY "SDL_RENDER_SCALE_QUALITY"
    sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "1");
    // Fuera del tutorial. Siempre utilizo esto en Linux.
    // #define SDL_HINT_VIDEO_X11_NET_WM_BYPASS_COMPOSITOR "SDL_VIDEO_X11_NET_WM_BYPASS_COMPOSITOR"
    sdl2::hint::set("SDL_VIDEO_X11_NET_WM_BYPASS_COMPOSITOR", "0");

    let win = video
        .window("SDL Tutorial 22", WIDTH, HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .expect("Could not create SDL window!");

    sdl2::image::init(InitFlag::PNG).expect("Unable to initialize sdl2_image!");

    (sdl, win, ttf)
}

fn main() {
    let (context, win, ttf) = init();
    // Initialize TimerSubsystem
    let time = context.timer().unwrap();

    // Obtain the canvas
    let mut canvas = win
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .expect("Unable to obtain canvas!");

    let creator = canvas.texture_creator();
    // Initialize renderer color
    canvas.set_draw_color(Color::WHITE);

    let font = ttf
        .load_font("resources/lazy.ttf", 28)
        .expect("Could not load font context!");

    // Load prompt texture
    let prompt = Text::create(&creator, &font, "Press Enter to Reset Start Time.");

    // Get a handle to the SDL2 event pump
    let mut event_pump = context
        .event_pump()
        .expect("Unable to obtain event pump handle!");

    let mut start_time = 0i32;

    'running: loop {
        // Extract any pending events from from the event pump and process them
        for event in event_pump.poll_iter() {
            // pattern match on the type of event
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown { keycode: k, .. } => match k {
                    Some(Keycode::Escape) => break 'running,
                    Some(Keycode::Return) => start_time = time.ticks() as i32,
                    _ => (),
                },
                _ => (),
            };
        }

        let text: String = format!(
            "Milliseconds since start time {}",
            time.ticks() as i32 - start_time
        );
        let time_text = Text::create(&creator, &font, text.as_str());
        // Clear and render the texture each pass through the loop
        canvas.clear();

        // Render the Image
        prompt.render(&mut canvas, (WIDTH - prompt.get_width()) as i32 / 2, 0);

        time_text.render(
            &mut canvas,
            (WIDTH - prompt.get_width()) as i32 / 2,
            (HEIGHT - prompt.get_height()) as i32 / 2,
        );

        // Update the screen
        canvas.present();
    }
}
