use sdl2::{
    event::Event,
    image::LoadSurface,
    keyboard::{Keycode, Mod, TextInputUtil},
    pixels::Color,
    rect::Rect,
    render::{Canvas, Texture, TextureCreator},
    surface::Surface,
    ttf::Font,
    video::{Window, WindowContext},
};

// used of it is better that Clipboard instance of sld2::lipboard::ClipboardUtil use arboard::Clipboard;

// Screen dimension
const WIDTH: i32 = 650;
const HEIGHT: i32 = 480;

pub struct LTexture {
    texture: Texture,
    width: u32,
    height: u32,
}

impl LTexture {
    /// Loads image at specified path
    pub fn from_file(path: &str, creator: &TextureCreator<WindowContext>) -> LTexture {
        let mut surf = Surface::from_file(path).expect("Could not load surface from file!");

        // Color key image
        surf.set_color_key(true, Color::RGB(0, 0xFF, 0xFF))
            .expect("Can't set color key");

        // Create texture from surface pixels
        let texture = creator.create_texture_from_surface(&surf).unwrap();

        // Get image dimensions
        let (width, height) = surf.size();

        LTexture {
            texture,
            width,
            height,
        }
    }

    fn from_text(
        creator: &TextureCreator<WindowContext>,
        font: &Font,
        text: &str,
        color: Color,
    ) -> LTexture {
        let surface: Surface = font
            .render(text)
            .solid(color)
            .expect("Could not create text surface!");

        // Now create a texture from the surface using the supplied creator
        let texture = creator
            .create_texture_from_surface(&surface)
            .expect("Could not convert text surface to texture!");

        LTexture {
            texture,
            width: surface.width(),
            height: surface.height(),
        }
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

fn main() {
    let sdl_ctx = sdl2::init().expect("Unable to initialize SDL!");
    let ttf_ctx = sdl2::ttf::init().expect("Could not acquire ttf context!");
    let video = sdl_ctx.video().expect("Could not acquire video context!");
    let clipboard = video.clipboard();
    let text_util: TextInputUtil = video.text_input();
    text_util.start();

    sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "1");
    sdl2::hint::set("SDL_VIDEO_X11_NET_WM_BYPASS_COMPOSITOR", "0");

    let window = video
        .window("SDL Tutorial 32", WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .opengl()
        .build()
        .expect("Could not create SDL window!");

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .expect("Unable to obtain canvas!");

    let creator = canvas.texture_creator();

    // Get a handle to the SDL2 event pump
    let mut event_pump = sdl_ctx
        .event_pump()
        .expect("Unable to obtain event pump handle!");

    // Load a font
    let font_path = std::path::Path::new("./resources/lesson16/lazy.ttf");

    let prompt = {
        let font = ttf_ctx.load_font(font_path, 28).unwrap();
        LTexture::from_text(&creator, &font, "Enter Text:", Color::BLACK)
    };

    // The current input text.
    let mut input_text = String::from("Some Text");

    lazy_foo::setup_mainloop(-1, true, move || {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => return false,
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
                    (Keycode::Escape, _) => return false,
                    _ => (),
                },
                // Special text input event
                Event::TextInput { text, .. } => {
                    input_text += &text;
                }
                _ => (),
            }
        }

        let font = ttf_ctx.load_font(font_path, 28).unwrap();
        canvas.set_draw_color(Color::WHITE);
        canvas.clear();

        prompt.render(&mut canvas, (WIDTH - prompt.width as i32) / 2, 0, None);

        // Rerender text if input_text not is empty
        if !input_text.is_empty() {
            let input_texture =
                LTexture::from_text(&creator, &font, &input_text, Color::from((0, 0, 0, 0xFF)));

            let x = (WIDTH - input_texture.width as i32) / 2;
            let y = prompt.height as i32;

            text_util.set_rect(Rect::new(x, y, input_texture.width, input_texture.height));
            input_texture.render(&mut canvas, x, y, None);
        }

        canvas.present();
        true
    });
}
