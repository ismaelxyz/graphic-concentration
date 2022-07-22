use sdl2::{
    event::Event,
    image::{InitFlag, LoadSurface},
    keyboard::Keycode,
    mixer::{open_audio, Channel, Chunk, Music, DEFAULT_FORMAT},
    pixels::Color,
    rect::Rect,
    render::{Canvas, RenderTarget, Texture, TextureCreator},
    surface::Surface,
    video::{Window, WindowContext},
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
}

struct Media<'a> {
    prompt: LTexture<'a>,
    beat: Music<'a>,
    scratch: Chunk,
    high: Chunk,
    medium: Chunk,
    low: Chunk,
}

impl<'a> Media<'a> {
    fn load(creator: &'a TextureCreator<WindowContext>) -> Self {
        Media {
            prompt: LTexture::from_file("resources/prompt.png", creator),
            beat: Music::from_file("resources/beat.wav").unwrap(),
            scratch: Chunk::from_file("resources/scratch.wav").unwrap(),
            high: Chunk::from_file("resources/high.wav").unwrap(),
            medium: Chunk::from_file("resources/medium.wav").unwrap(),
            low: Chunk::from_file("resources/low.wav").unwrap(),
        }
    }
}

fn init() -> (sdl2::Sdl, Window) {
    let sdl = sdl2::init().expect("Unable to initialize SDL!");
    let video = sdl.video().expect("Could not acquire video context!");
    sdl.audio().expect("Could not acquire audio context!");

    // #define SDL_HINT_RENDER_SCALE_QUALITY "SDL_RENDER_SCALE_QUALITY"
    sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "1");
    // #define SDL_HINT_VIDEO_X11_NET_WM_BYPASS_COMPOSITOR "SDL_VIDEO_X11_NET_WM_BYPASS_COMPOSITOR"
    sdl2::hint::set("SDL_VIDEO_X11_NET_WM_BYPASS_COMPOSITOR", "0");

    let win = video
        .window("SDL Tutorial 21", WIDTH, HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .expect("Could not create SDL window!");

    sdl2::image::init(InitFlag::PNG).expect("Unable to initialize sdl2_image!");

    (sdl, win)
}

fn main() {
    let (context, win) = init();
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
    // Initialize SDL_mixer
    open_audio(44100, DEFAULT_FORMAT, 1, 2048).unwrap();
    let media = Media::load(&creator);

    // Get a handle to the SDL2 event pump
    let mut event_pump = context
        .event_pump()
        .expect("Unable to obtain event pump handle!");
    let channel = Channel::all();
    let mut sound = None;
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
                    Some(Keycode::Num1) => sound = Some(&media.high),
                    Some(Keycode::Num2) => sound = Some(&media.medium),
                    Some(Keycode::Num3) => sound = Some(&media.low),
                    Some(Keycode::Num4) => sound = Some(&media.scratch),
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
                            media.beat.play(0).expect("Can't pause the music");
                        }
                    }
                    // Nothing
                    None | Some(_) => (),
                },
                _ => {}
            }
        }

        if let Some(raw_sound) = sound.take() {
            channel.play_timed(raw_sound, 0, -1).unwrap();
        }

        // Clear and render the texture each pass through the loop
        canvas.clear();

        // Render the Image
        media.prompt.render(
            &mut canvas,
            (WIDTH - media.prompt.width) as i32 / 2,
            (HEIGHT - media.prompt.height) as i32 / 2,
        );

        // Update the screen
        canvas.present();
    }
}
