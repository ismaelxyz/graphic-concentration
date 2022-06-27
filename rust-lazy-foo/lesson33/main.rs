use sdl2::{
    clipboard::ClipboardUtil,
    event::Event,
    image::LoadSurface,
    keyboard::{Keycode, Mod, TextInputUtil},
    pixels::Color,
    rect::Rect,
    rwops::RWops,
    render::{Canvas, Texture, TextureCreator},
    surface::Surface,
    ttf::{Font, Sdl2TtfContext},
    video::Window,
    VideoSubsystem,
};

// Screen dimension
const WIDTH: i32 = 650;
const HEIGHT: i32 = 480;

//Number of data integers
const TOTAL_DATA: usize = 10;

// Create a struct that will track texture data
struct LTexture<'a> {
    // The actual texture.
    texture: Texture<'a>,
    // Image dimensions
    width: u32,
    height: u32,
}

// Note the use of the #[allow(dead_code)] which turns off
// warnings about functions we don't use in this lesson.
#[allow(dead_code)]
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

    /// Renders texture at given point
    fn render(&self, canvas: &mut Canvas<Window>, x: i32, y: i32, clip: Option<Rect>) {
        let clip_rect = match clip {
            Some(rect) => Rect::new(x, y, rect.width(), rect.height()),
            None => Rect::new(x, y, self.width, self.height),
        };

        canvas
            .copy_ex(&self.texture, clip, clip_rect, 0.0, None, false, false)
            .expect("Could not blit texture to render target!");
    }

    //#[cfg(sdl2_ttf)]
    // We only include this function if sdl2_ttf is used
    fn from_text<T>(
        creator: &'a TextureCreator<T>,
        font: &Font,
        text: &str,
        color: Color,
    ) -> Self {
        let text_surface: Surface = font
            .render(text)
            .solid(color)
            .expect("Could not create text surface!");
       
        // Now create a texture from the surface using the supplied creator
        let text_texture = creator
            .create_texture_from_surface(&text_surface)
            .expect("Could not convert text surface to texture!");
       
        // Return an LTexture using the given text_texture
        Self::new(text_texture)
    }
}

fn init() -> (sdl2::Sdl, Sdl2TtfContext, VideoSubsystem, Window) {
    let sdl = sdl2::init().expect("Unable to initialize SDL!");
    let video = sdl.video().expect("Could not acquire video context!");
    let ttf_context = sdl2::ttf::init().expect("Could not acquire ttf context!");

    sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "1");
    sdl2::hint::set("SDL_VIDEO_X11_NET_WM_BYPASS_COMPOSITOR", "0");

    let win = video
        .window("SDL Tutorial 32", WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .opengl()
        .build()
        .expect("Could not create SDL window!");

    (sdl, ttf_context, video, win)
}

fn main() {
    let (context, ttf_ctx, video, win) = init();
    let clipboard: ClipboardUtil = video.clipboard();
    let text_util: TextInputUtil = video.text_input();
    text_util.start();

    // Obtain the canvas
    let mut canvas = win
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .expect("Unable to obtain canvas!");

    let creator = canvas.texture_creator();

    // Get a handle to the SDL2 event pump
    let mut event_pump = context
        .event_pump()
        .expect("Unable to obtain event pump handle!");

    // Load a font
    let font_path = std::path::Path::new("./resources/lazy.ttf");
    let font = ttf_ctx.load_font(font_path, 28).unwrap();

        let text_color = Color::from(( 0, 0, 0, 0xFF));
    let prompt =
       LTexture::from_text(&creator, &font, "Enter Data:", text_color);

    let data_textures = Vec::with_capacity(TOTAL_DATA);
    let mut data = [0u8; TOTAL_DATA];

    // Open file for reading in binary
    match RWops::from_file( "resources/nums.bin", "r+b" ) {
        Ok(file) => {
               //Load data
            println!( "Reading file...!" );
            file.read(&mut data);
        }
        Err(..) => {
            let mut file = RWops::from_file( "assets/nums.bin", "w+b" ).unwrap();
                println!( "New file created!");

            //Initialize data
            file.write(&data).unwrap();
        }
    }

    data_textures.push(LTexture::from_text(&creator, &font, &data[0].to_string(), Color::from((0xFF, 0, 0, 0xFF)));)

    'running: loop {
        for event in event_pump.poll_iter() {
            // Pattern match on the Quit event
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(k),
                    keymod: _,
                    ..
                } => match k {
                    Keycode::Escape => break 'running,
                    _ => (),
                },
                _ => (),
            }
        }
    }
}

// LIBRARY_PATH="/home/Overlord/Programming/usr/lib/sdl2" cargo --offline build --bin lesson33