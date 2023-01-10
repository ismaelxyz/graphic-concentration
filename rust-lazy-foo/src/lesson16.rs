use sdl2::{
    event::Event,
    image::InitFlag,
    pixels::Color,
    rect::Rect,
    render::{Canvas, Texture, TextureCreator},
    surface::Surface,
    ttf::Font,
    video::{Window, WindowContext},
};

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;
const FONT_FILE: &str = "resources/lesson16/lazy.ttf";
const FONT_SIZE: u16 = 28;

struct Text {
    texture: Texture,
    width: u32,
    height: u32,
}

impl Text {
    fn new(creator: &TextureCreator<WindowContext>, font: &Font, text: &str, color: Color) -> Self {
        let surface: Surface = font
            .render(text)
            .solid(color)
            .expect("Could not create text surface!");

        let (width, height) = surface.size();

        // Now create a texture from the surface using the supplied creator
        let texture = creator
            .create_texture_from_surface(&surface)
            .expect("Could not convert text surface to texture!");

        Text {
            texture,
            width,
            height,
        }
    }

    // Renders a texture to a given point using a provided creator
    // provide additional arguments for rotation and flipping
    // Rust doesn't provide default arguments, and it seems overkill
    // to provide additional function signatures for this, so we're
    // going to wrap rotation and flipping args in Option<> so we can
    // provide None when we don't care about it.
    fn render_to(&self, canvas: &mut Canvas<Window>, x: i32, y: i32) {
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
}

fn main() {
    let sdl_ctx = sdl2::init().expect("Could not initialize SDL!");
    let video = sdl_ctx.video().expect("Could not acquire video context!");
    sdl2::image::init(InitFlag::PNG).expect("Could not initialize sdl2_image!");
    let ttf_ctx = sdl2::ttf::init().expect("Could not initialize sdl2_ttf!");

    sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "1");
    sdl2::hint::set("SDL_VIDEO_X11_NET_WM_BYPASS_COMPOSITOR", "0");

    let window = video
        .window("SDL Tutorial 16", WIDTH, HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .expect("Could not create SDL window!");

    // Obtain the canvas
    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Could not create creator!");
    let creator = canvas.texture_creator();
    // Load the font, using the font and size specified by the global constants
    let font = ttf_ctx
        .load_font(std::path::Path::new(FONT_FILE), FONT_SIZE)
        .expect("Could not load font from file!");

    let text = Text::new(
        &creator,
        &font,
        "The quick brown fox jumps over the lazy dog",
        Color::BLACK,
    );

    let mut event_pump = sdl_ctx
        .event_pump()
        .expect("Could not obtain handle to event pump!");

    lazy_foo::setup_mainloop(-1, true, move || {
        for event in event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                return false;
            }
        }
        // Clear and render the texture each pass through the loop
        canvas.set_draw_color(Color::WHITE);
        canvas.clear();

        // Render the text
        text.render_to(
            &mut canvas,
            (WIDTH - text.width) as i32 / 2,
            (HEIGHT - text.height) as i32 / 2,
        );

        canvas.present();

        true
    });
}
