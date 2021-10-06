#![allow(clippy::too_many_arguments)]

use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    rect::{Point, Rect},
    render::{Canvas, RenderTarget, Texture, TextureCreator},
    surface::Surface,
    ttf::{Font, Sdl2TtfContext},
    video::Window,
};

// Screen dimension
const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

// Texture wrapper
pub struct LTexture<'a> {
    // The actual hardware texture
    texture: Texture<'a>,
    // Image dimensions
    width: u32,
    height: u32,
}

impl<'a> LTexture<'a> {
    fn new(texture: Texture<'a>, width: u32, height: u32) -> Self {
        Self {
            texture,
            width,
            height,
        }
    }

    // Creates image from font string
    fn from_creator_text<T>(
        creator: &'a TextureCreator<T>,
        font: &Font,
        text: &str,
        color: Color,
    ) -> LTexture<'a> {
        let text_surface: Surface = font
            .render(text)
            .solid(color)
            .expect("Could not create text surface!");

        // Now create a texture from the surface using the supplied creator
        let text_texture = creator
            .create_texture_from_surface(&text_surface)
            .expect("Could not convert text surface to texture!");

        let (w, h) = text_surface.size();
        // Return an LTexture using the given text_texture
        LTexture::new(text_texture, w, h)
    }

    // Renders texture at given point
    fn render<T: RenderTarget>(
        &self,
        canvas: &mut Canvas<T>,
        x: i32,
        y: i32,
        clip: Option<Rect>,
        rotation: Option<f64>,
        center: Option<Point>,
        flip_h: bool,
        flip_v: bool,
    ) {
        let clip_rect = clip.unwrap_or_else(|| Rect::new(0, 0, self.width, self.height));
        let rot: f64 = rotation.unwrap_or(0.0);

        canvas
            .copy_ex(
                &self.texture,
                Some(clip_rect),
                Some(Rect::new(x, y, clip_rect.width(), clip_rect.height())),
                rot,
                center,
                flip_h,
                flip_v,
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

fn init() -> (sdl2::Sdl, Window, Sdl2TtfContext) {
    let sdl = sdl2::init().expect("Unable to initialize SDL!");
    let video = sdl.video().expect("Could not acquire video context!");
    let ttf = sdl2::ttf::init().expect("Could not acquire video context!");

    sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "1");
    sdl2::hint::set("SDL_VIDEO_X11_NET_WM_BYPASS_COMPOSITOR", "0");

    let win = video
        .window("SDL Tutorial 24", WIDTH, HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .expect("Could not create SDL window!");

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
    canvas.set_draw_color(Color::RGBA(0xFF, 0xFF, 0xFF, 0xFF));

    let font = ttf
        .load_font("lesson22/resources/lazy.ttf", 28)
        .expect("Could not load font context!");

    // Set text color as black
    let text_color = Color::RGBA(0, 0, 0, 255);

    // Get a handle to the SDL2 event pump
    let mut event_pump = context
        .event_pump()
        .expect("Unable to obtain event pump handle!");

    let mut counted_frames = 0f64;
    let start_ticks = time.ticks() as f64; // getTicks

    'running: loop {
        // Extract any pending events from from the event pump and process them
        for event in event_pump.poll_iter() {
            // pattern match on the type of event
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => (),
            };
        }

        // Calculate and correct fps
        let tm = (time.ticks() as f64) - start_ticks;
        let mut avg_fps = counted_frames / (tm / 1000.0f64);

        if avg_fps > 2000000.0 {
            avg_fps = 0.0;
        }

        // Set text to be rendered
        let text: String = format!("Average Frames Per Second {}", avg_fps);

        // Load prompt texture
        let time_text = LTexture::from_creator_text(&creator, &font, &text, text_color);

        // Clear and render the texture each pass through the loop
        canvas.clear();

        time_text.render(
            &mut canvas,
            (WIDTH - time_text.get_width()) as i32 / 2,
            (HEIGHT - time_text.get_height()) as i32 / 2,
            None,
            None,
            None,
            false,
            false,
        );

        // Update the screen
        canvas.present();
        counted_frames += 1.0;
    }
}
