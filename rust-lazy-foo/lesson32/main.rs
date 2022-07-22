use sdl2::{
    clipboard::ClipboardUtil,
    event::Event,
    image::LoadSurface,
    keyboard::{Keycode, Mod, TextInputUtil},
    pixels::Color,
    rect::Rect,
    render::{Canvas, Texture, TextureCreator},
    surface::Surface,
    ttf::{Font, Sdl2TtfContext},
    video::Window,
    VideoSubsystem,
};

// Screen dimension
const WIDTH: i32 = 650;
const HEIGHT: i32 = 480;

/// Texture wrapper
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

    /// Loads image at specified path
    pub fn from_file<T>(path: &str, creator: &'a TextureCreator<T>) -> LTexture<'a> {
        let mut surf = Surface::from_file(path).expect("Could not load surface from file!");

        // Color key image
        surf.set_color_key(true, Color::RGB(0, 0xFF, 0xFF))
            .expect("Can't set color key");

        // Create texture from surface pixels
        let texture = creator.create_texture_from_surface(&surf).unwrap();

        // Get image dimensions
        let (w, h) = surf.size();

        LTexture::new(texture, w, h)
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

        let (w, h) = text_surface.size();

        // Now create a texture from the surface using the supplied creator
        let text_texture = creator
            .create_texture_from_surface(&text_surface)
            .expect("Could not convert text surface to texture!");
        // Return an LTexture using the given text_texture
        LTexture::new(text_texture, w, h)
    }

    /// Renders texture at given point
    pub fn render(&self, canvas: &mut Canvas<Window>, x: i32, y: i32, clip: Option<Rect>) {
        let clip_rect = match clip {
            Some(rect) => Rect::new(x, y, rect.width(), rect.height()),
            None => Rect::new(x, y, self.width, self.height),
        };

        canvas
            .copy_ex(&self.texture, clip, clip_rect, 0.0, None, false, false)
            .expect("Could not blit texture to render target!");
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

/*
    # Nota: El Clipboard de sdl no es el de la pc, es uno propio : (.
    # Nota: El Clipboard de sdl no debería llamarse como tal.
*/
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
    let prompt = LTexture::from_creator_text(&creator, &font, "Enter Text:", Color::BLACK);

    //The current input text.
    let mut input_text = String::from("Some Text");

    'running: loop {
        // Extract any pending events from from the event pump and process them
        for event in event_pump.poll_iter() {
            // Pattern match on the Quit event
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(k),
                    keymod,
                    ..
                } => match (k, keymod) {
                    // lop off character
                    (Keycode::Backspace, _) => {
                        input_text.pop();
                    }
                    // Handle copy
                    (Keycode::C, Mod::LCTRLMOD | Mod::RCTRLMOD) => {
                        clipboard.set_clipboard_text(&input_text).unwrap()
                    }
                    // Handle paste
                    (Keycode::V, Mod::LCTRLMOD | Mod::RCTRLMOD) => {
                        input_text = clipboard.clipboard_text().unwrap()
                    }
                    (Keycode::Escape, _) => break 'running,
                    _ => (),
                },
                // Special text input event
                Event::TextInput { text, .. } => {
                    input_text += &text;
                }
                _ => (),
            }
        }

        // Clear and render the texture each pass through the loop
        canvas.set_draw_color(Color::WHITE);
        canvas.clear();

        prompt.render(&mut canvas, (WIDTH - prompt.width as i32) / 2, 0, None);

        // Rerender text if input_text not is empty
        if !input_text.is_empty() {
            let input_texture = LTexture::from_creator_text(
                &creator,
                &font,
                &input_text,
                Color::from((0, 0, 0, 0xFF)),
            );

            let x = (WIDTH - input_texture.width as i32) / 2;
            let y = prompt.height as i32;

            text_util.set_rect(Rect::new(x, y, input_texture.width, input_texture.height));
            input_texture.render(&mut canvas, x, y, None);
        }

        // Update the screen
        canvas.present();
    }
}
