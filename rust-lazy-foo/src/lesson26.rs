use sdl2::{
    event::Event,
    image::LoadSurface,
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
    render::{Canvas, Texture, TextureCreator},
    surface::Surface,
    video::{Window, WindowContext},
};

// Screen dimension
const WIDTH: u32 = 650;
const HEIGHT: u32 = 480;

pub struct LTexture {
    texture: Texture,
    width: u32,
    height: u32,
}

impl LTexture {
    pub fn from_file(path: &str, creator: &TextureCreator<WindowContext>) -> Self {
        let mut surf = Surface::from_file(path).expect("Could not load surface from file!");

        // Color key image
        surf.set_color_key(true, Color::RGB(0, 0xFF, 0xFF))
            .expect("Can't set color key");

        // Create texture from surface pixels
        let texture = creator.create_texture_from_surface(&surf).unwrap();

        // Get image dimensions
        let (width, height) = surf.size();

        Self {
            texture,
            width,
            height,
        }
    }

    pub fn render(&self, canvas: &mut Canvas<Window>, x: i32, y: i32) {
        let clip_rect = Rect::new(0, 0, self.width, self.height);
        canvas
            .copy_ex(
                &self.texture,
                Some(clip_rect),
                Some(Rect::new(x, y, clip_rect.width(), clip_rect.height())),
                0.0,
                None,
                false,
                false,
            )
            .expect("Could not blit texture to render target!");
    }
}

/// The dot that will move around on the screen
struct Dot {
    /// The X and Y offsets of the dot
    pos: (i32, i32),

    /// The X and Y velocity of the dot
    vel: (i32, i32),

    pub texture: LTexture,
}

impl Dot {
    // The dimensions of the dot
    const WIDTH: i32 = 20;
    const HEIGHT: i32 = 20;

    /// Maximum axis velocity of the dot
    const VEL: i32 = 10;

    /// Initializes the variables
    fn new(texture: LTexture) -> Self {
        //Initialize the offsets and the velocity
        Dot {
            pos: (0, 0),
            vel: (0, 0),
            texture,
        }
    }

    /// Takes key presses and adjusts the dot's velocity
    #[rustfmt::skip]
    fn handle_event(&mut self, e: &Event) {
        match e {
            Event::KeyDown { repeat: false, keycode: Some(kode), .. } => match kode {
                Keycode::Up => self.vel.1 -= Dot::VEL,
                Keycode::Down => self.vel.1 += Dot::VEL,
                Keycode::Left  => self.vel.0 -= Dot::VEL,
                Keycode::Right  => self.vel.0 += Dot::VEL,
                _ => (),
            },
            Event::KeyUp { repeat: false, keycode: Some(kode), .. } => match kode {
                Keycode::Up => self.vel.1 += Dot::VEL,
                Keycode::Down => self.vel.1 -= Dot::VEL,
                Keycode::Left  => self.vel.0 += Dot::VEL,
                Keycode::Right  => self.vel.0 -= Dot::VEL,
                _ => (),
            }
            _ => (),
        }
    }

    /// Moves the dot
    fn r#move(&mut self) {
        // Move the dot left or right
        self.pos.0 += self.vel.0;

        // If the dot went too far to the left or right
        if self.pos.0 < 0 || self.pos.0 + Dot::WIDTH > (WIDTH as i32) {
            // Move back
            self.pos.0 -= self.vel.0;
        }

        // Move the dot up or down
        self.pos.1 += self.vel.1;

        // If the dot went too far up or down
        if self.pos.1 < 0 || self.pos.1 + Dot::HEIGHT > (HEIGHT as i32) {
            // Move back
            self.pos.1 -= self.vel.1;
        }
    }

    /// Shows the dot on the screen
    fn render(&self, canvas: &mut Canvas<Window>) {
        self.texture.render(canvas, self.pos.0, self.pos.1);
    }
}

fn main() {
    let sdl_ctx = sdl2::init().expect("Unable to initialize SDL!");
    let video = sdl_ctx.video().expect("Could not acquire video context!");

    sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "1");
    sdl2::hint::set("SDL_VIDEO_X11_NET_WM_BYPASS_COMPOSITOR", "0");

    let window = video
        .window("SDL Tutorial 26", WIDTH, HEIGHT)
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
    canvas.set_draw_color(Color::WHITE);

    let mut event_pump = sdl_ctx
        .event_pump()
        .expect("Unable to obtain event pump handle!");

    let dot_texture = LTexture::from_file("resources/lesson26/dot.bmp", &creator);
    let mut dot = Dot::new(dot_texture);

    lazy_foo::setup_mainloop(-1, true, move || {
        for event in event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                return false;
            }

            // Handle input for the dot
            dot.handle_event(&event);
        }

        dot.r#move();
        canvas.clear();
        dot.render(&mut canvas);
        canvas.present();

        true
    });
}
