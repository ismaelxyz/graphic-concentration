use sdl2::{
    event::Event,
    image::{InitFlag, LoadSurface},
    keyboard::Keycode,
    mixer::{open_audio, Channel, Chunk, Music, DEFAULT_FORMAT},
    pixels::Color,
    rect::Rect,
    render::{Canvas, Texture, TextureCreator},
    surface::Surface,
    video::{Window, WindowContext},
};

pub struct LTexture {
    texture: Texture,
    width: u32,
    height: u32,
}

impl LTexture {
    fn from_file(path: &str, creator: &TextureCreator<WindowContext>) -> LTexture {
        let mut surface = Surface::from_file(path).unwrap();

        surface
            .set_color_key(true, Color::RGB(0, 0xff, 0xff))
            .unwrap();

        let texture = creator.create_texture_from_surface(&surface).unwrap();

        LTexture {
            texture,
            width: surface.width(),
            height: surface.height(),
        }
    }

    fn render(&self, canvas: &mut Canvas<Window>, x: i32, y: i32) {
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

struct Media {
    prompt: LTexture,
    scratch: Chunk,
    high: Chunk,
    medium: Chunk,
    low: Chunk,
}

impl Media {
    fn load(creator: &TextureCreator<WindowContext>) -> Self {
        Media {
            prompt: LTexture::from_file("resources/lesson21/prompt.png", creator),
            scratch: Chunk::from_file("resources/lesson21/scratch.wav").unwrap(),
            high: Chunk::from_file("resources/lesson21/high.wav").unwrap(),
            medium: Chunk::from_file("resources/lesson21/medium.wav").unwrap(),
            low: Chunk::from_file("resources/lesson21/low.wav").unwrap(),
        }
    }
}

fn main() {
    let sdl_ctx = sdl2::init().expect("Unable to initialize SDL!");
    let video = sdl_ctx.video().expect("Could not acquire video context!");
    sdl_ctx.audio().expect("Could not acquire audio context!");
    sdl2::image::init(InitFlag::PNG).expect("Unable to initialize sdl2_image!");

    sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "1");
    sdl2::hint::set("SDL_VIDEO_X11_NET_WM_BYPASS_COMPOSITOR", "0");

    let window = video
        .window("SDL Tutorial 21", 640, 480)
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
    // Initialize SDL_mixer
    open_audio(44100, DEFAULT_FORMAT, 1, 2048).unwrap();

    let beat = Music::from_file("resources/lesson21/beat.wav").unwrap();
    let (media, channel) = (Media::load(&creator), Channel::all());

    // Get a handle to the SDL2 event pump
    let mut event_pump = sdl_ctx
        .event_pump()
        .expect("Unable to obtain event pump handle!");

    lazy_foo::setup_mainloop(-1, true, move || {
        let mut sound = None;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => return false,
                Event::KeyDown { keycode: k, .. } => match k {
                    Some(Keycode::Escape) => return false,
                    Some(Keycode::Num0) => Music::halt(),
                    Some(Keycode::Num1) => sound = Some(&media.high),
                    Some(Keycode::Num2) => sound = Some(&media.medium),
                    Some(Keycode::Num3) => sound = Some(&media.low),
                    Some(Keycode::Num4) => sound = Some(&media.scratch),
                    Some(Keycode::Num9) => {
                        // If there is music playing
                        if Music::is_playing() {
                            // If the music is paused
                            if Music::is_paused() {
                                Music::resume();
                            } else {
                                // Pause the music
                                Music::pause();
                            }
                        } else {
                            // Play the music
                            beat.play(0).expect("Can't pause the music");
                        }
                    }
                    _ => (),
                },
                _ => {}
            }
        }

        if let Some(raw_sound) = sound.take() {
            channel.play_timed(raw_sound, 0, -1).unwrap();
        }

        canvas.clear();
        media.prompt.render(
            &mut canvas,
            (width - media.prompt.width) as i32 / 2,
            (height - media.prompt.height) as i32 / 2,
        );
        canvas.present();

        true
    });
}
