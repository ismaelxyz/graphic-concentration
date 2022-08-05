use sdl2::{
    event::Event,
    image::{InitFlag, Sdl2ImageContext},
    pixels::Color,
    rect::Rect,
    render::{Canvas, Texture, TextureCreator},
    surface::Surface,
    ttf::{Font, Sdl2TtfContext},
    video::Window,
    Sdl,
};

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;
const FONT_FILE: &str = "resources/lazy.ttf";
const FONT_SIZE: u16 = 28;

// Create a struct that will track texture data
struct LTexture<'a> {
    // The actual texture.
    texture: Texture<'a>,
    // Image dimensions
    width: u32,
    height: u32,
}

impl<'a> LTexture<'a> {
    // create a new texture
    fn new(tex: Texture<'a>) -> LTexture {
        let w = tex.query().width;
        let h = tex.query().height;
        LTexture {
            texture: tex,
            width: w,
            height: h,
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
        // Return an LTexture using the given text_texture
        LTexture::new(text_texture)
    }
}

// Load the font, and use it to create and return a new texture with
// the creator string
fn load_media<'a, T>(creator: &'a TextureCreator<T>, ttf: &'a Sdl2TtfContext) -> LTexture<'a> {
    // Load the font, using the font and size specified by the global constants
    let font = ttf
        .load_font(std::path::Path::new(FONT_FILE), FONT_SIZE)
        .expect("Could not load font from file!");

    // Now return a new LTexture using the supplied font and creator
    LTexture::from_creator_text(
        creator,
        &font,
        "The quick brown fox jumps over the lazy dog",
        Color::RGB(0, 0, 0),
    )
}

// Break out initialization into a separate function, which
// returns only the Window (we don't need the sdl_context)
// Ugh, the SDL font context name!
fn init() -> (Sdl, Window, Sdl2ImageContext, Sdl2TtfContext) {
    let sdl = sdl2::init().expect("Could not initialize SDL!");
    let video = sdl.video().expect("Could not acquire video context!");
    let win = video
        .window("SDL Tutorial 16", WIDTH, HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .expect("Could not create SDL window!");

    let image = sdl2::image::init(InitFlag::PNG).expect("Could not initialize sdl2_image!");
    let ttf = sdl2::ttf::init().expect("Could not initialize sdl2_ttf!");

    (sdl, win, image, ttf)
}

fn main() {
    // Initialize SDL2
    let (sdl_context, window, _image, ttf_context) = init();

    // Obtain the canvas
    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Could not create creator!");
    let creator = canvas.texture_creator();
    let text = load_media(&creator, &ttf_context);

    // Get a handle to the SDL2 event pump
    let mut event_pump = sdl_context
        .event_pump()
        .expect("Could not obtain handle to event pump!");

    'running: loop {
        // Extract any pending events from from the event pump and process them
        for event in event_pump.poll_iter() {
            // pattern match on the type of event
            if let Event::Quit { .. } = event {
                break 'running;
            }
        }
        // Clear and render the texture each pass through the loop
        canvas.set_draw_color(Color::RGB(0xff, 0xff, 0xff));
        canvas.clear();

        // Render the text
        text.render_to(
            &mut canvas,
            (WIDTH - text.width) as i32 / 2,
            (HEIGHT - text.height) as i32 / 2,
        );

        // Update the screen
        canvas.present();
    }
}
