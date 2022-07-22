mod util;
use sdl2::{
    event::Event,
    image::{InitFlag, LoadSurface, Sdl2ImageContext},
    keyboard::Keycode,
    mixer::{open_audio, Channel, Chunk, Music, DEFAULT_FORMAT},
    pixels::Color,
    rect::Rect,
    render::{BlendMode, Canvas, RenderTarget, Texture, TextureCreator},
    surface::Surface,
    ttf::Font,
    video::{Window, WindowContext},
    AudioSubsystem, VideoSubsystem,
};
use util::BoxData;

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

#[allow(dead_code)]
impl<'a> LTexture<'a> {
    fn new(texture: Texture<'a>, width: u32, height: u32) -> Self {
        Self {
            texture,
            width,
            height,
        }
    }

    // Loads image at specified path
    fn from_file<T>(path: &str, creator: &'a TextureCreator<T>) -> LTexture<'a> {
        let mut surf = Surface::from_file(path).expect("Could not load surface from file!");

        // Color key image
        surf.set_color_key(true, Color::RGB(0, 0xFF, 0xFF))
            .expect("Can't set color key");

        // Create texture from surface pixels
        let texture = creator.create_texture_from_surface(&surf).unwrap();

        // Get image dimensions
        let (w, h) = surf.size();

        // Ismaelxyz: No es nesesario pero a modo de demostración
        // liberamos la surface
        drop(surf);

        LTexture::new(texture, w, h)
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

    // Set color modulation
    fn set_color(&mut self, red: u8, green: u8, blue: u8) {
        // Modulate texture rgb
        self.texture.set_color_mod(red, green, blue);
    }

    // Set blending
    fn set_blend_mode(&mut self, blending: BlendMode) {
        self.texture.set_blend_mode(blending);
    }

    // Set alpha modulation
    fn set_alpha(&mut self, alpha: u8) {
        // Modulate texture alpha
        self.texture.set_alpha_mod(alpha);
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
    fn get_width(&self) -> u32 {
        self.width
    }
    fn get_height(&self) -> u32 {
        self.height
    }
}

fn init() -> (
    sdl2::Sdl,
    Window,
    VideoSubsystem,
    AudioSubsystem,
    Sdl2ImageContext,
) {
    let sdl = sdl2::init().expect("Unable to initialize SDL!");
    let video = sdl.video().expect("Could not acquire video context!");
    let audio = sdl.audio().expect("Could not acquire audio context!");

    // Recuerda es el valor lo que importa no el nombre de la variable
    // #define SDL_HINT_RENDER_SCALE_QUALITY "SDL_RENDER_SCALE_QUALITY"
    sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "1");
    // Fuera del tutorial pero siempre utilizo esto en linux
    // #define SDL_HINT_VIDEO_X11_NET_WM_BYPASS_COMPOSITOR "SDL_VIDEO_X11_NET_WM_BYPASS_COMPOSITOR"
    sdl2::hint::set("SDL_VIDEO_X11_NET_WM_BYPASS_COMPOSITOR", "0");

    let win = video
        .window("SDL Tutorial 21", WIDTH, HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .expect("Could not create SDL window!");

    let image = sdl2::image::init(InitFlag::PNG).expect("Unable to initialize sdl2_image!");

    (sdl, win, video, audio, image)
}

fn load_media(creator: &TextureCreator<WindowContext>) -> BoxData<'_> {
    let prompt: LTexture = LTexture::from_file("lesson21/resources/prompt.png", creator);
    let beat: Music = Music::from_file("lesson21/resources/beat.wav").unwrap();
    let scratch: Chunk = Chunk::from_file("lesson21/resources/scratch.wav").unwrap();
    let high: Chunk = Chunk::from_file("lesson21/resources/high.wav").unwrap();
    let medium: Chunk = Chunk::from_file("lesson21/resources/medium.wav").unwrap();
    let low: Chunk = Chunk::from_file("lesson21/resources/low.wav").unwrap();
    BoxData::new(prompt, beat, scratch, high, medium, low)
}

fn main() {
    let (context, win, _video, _audio, _image) = init();
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
    // Initialize SDL_mixer
    open_audio(44100, DEFAULT_FORMAT, 1, 2048).unwrap();
    let bxdata = load_media(&creator);

    // Get a handle to the SDL2 event pump
    let mut event_pump = context
        .event_pump()
        .expect("Unable to obtain event pump handle!");
    let channel = Channel::all();

    // Main loop
    'running: loop {
        // Extract any pending events from from the event pump and process them
        for event in event_pump.poll_iter() {
            // pattern match on the type of event
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown { keycode: k, .. } => match k {
                    Some(Keycode::Escape) => break 'running,
                    Some(Keycode::Num0) => sdl2::mixer::Music::halt(),
                    // EL "unwrap" es divido a que los canales libres son
                    // limitados
                    Some(Keycode::Num1) => {
                        channel.play_timed(&bxdata.hint, 0, -1).unwrap();
                    }
                    Some(Keycode::Num2) => {
                        channel.play_timed(&bxdata.medium, 0, -1).unwrap();
                    }
                    Some(Keycode::Num3) => {
                        channel.play_timed(&bxdata.low, 0, -1).unwrap();
                    }
                    Some(Keycode::Num4) => {
                        channel.play_timed(&bxdata.scratch, 0, -1).unwrap();
                    }
                    Some(Keycode::Num9) => {
                        // If there is music playing
                        if sdl2::mixer::Music::is_playing() {
                            // If the music is paused
                            if sdl2::mixer::Music::is_paused() {
                                sdl2::mixer::Music::resume();
                            } else {
                                // Pause the music
                                sdl2::mixer::Music::pause();
                            }
                        } else {
                            // Play the music
                            bxdata.beat.play(0).expect("Can't pause the music");
                        }
                    }
                    // Nothing
                    Some(_) => (),
                    None => (),
                },
                _ => {}
            }
        }

        // Clear and render the texture each pass through the loop
        canvas.clear();

        // Render the Image
        bxdata.picture.render(
            &mut canvas,
            (WIDTH - bxdata.picture.width) as i32 / 2,
            (HEIGHT - bxdata.picture.height) as i32 / 2,
        );

        // Update the screen
        canvas.present();
    }
}
