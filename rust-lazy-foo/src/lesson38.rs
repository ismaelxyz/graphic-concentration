use rand::{thread_rng, Rng};
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

/// Particle count
const TOTAL_PARTICLES: usize = 20;

pub struct LTexture {
    texture: Texture,
    width: u32,
    height: u32,
}

impl LTexture {
    pub fn from_file(name: &str, creator: &TextureCreator<WindowContext>) -> LTexture {
        let mut surf = Surface::from_file(format!("resources/lesson38/{name}.bmp"))
            .expect("Could not load surface from file!");

        surf.set_color_key(true, Color::RGB(0, 0xFF, 0xFF))
            .expect("Can't set color key");

        let texture = creator.create_texture_from_surface(&surf).unwrap();
        let (width, height) = surf.size();

        LTexture {
            texture,
            width,
            height,
        }
    }

    /// Renders texture at given point
    pub fn render(&self, canvas: &mut Canvas<Window>, (x, y): (i32, i32)) {
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

struct Particle {
    /// Offsets
    pos: (i32, i32),

    /// Current frame of animation
    frame: usize,

    /// Type of particle
    texture: LTexture,
}

impl Particle {
    /// Initialize position and animation
    fn new((x, y): (i32, i32), creator: &TextureCreator<WindowContext>) -> Self {
        let mut rng = thread_rng();
        let mut texture = match rng.gen_range(0..3) {
            0 => LTexture::from_file("red", creator),
            1 => LTexture::from_file("green", creator),
            2 => LTexture::from_file("blue", creator),
            _ => unreachable!(),
        };

        texture.texture.set_alpha_mod(192);

        Self {
            // Set offsets
            pos: (x - 5 + rng.gen_range(0..25), y - 5 + rng.gen_range(0..25)),
            // Initialize animation
            frame: rng.gen_range(0..5),
            texture,
        }
    }

    fn render(&mut self, canvas: &mut Canvas<Window>, shimmer_texture: &LTexture) {
        // Show image
        self.texture.render(canvas, self.pos);

        // Show shimmer
        if self.frame % 2 == 0 {
            shimmer_texture.render(canvas, self.pos);
        }

        // Animate
        self.frame += 1;
    }

    /// Checks if particle is dead
    fn is_dead(&self) -> bool {
        self.frame > 10
    }
}

/// The dot that will move around on the screen
struct Dot {
    /// The X and Y offsets of the dot
    pos: (i32, i32),

    /// The X and Y velocity of the dot
    vel: (i32, i32),

    pub texture: LTexture,
    particles: Vec<Particle>,
}

impl Dot {
    // The dimensions of the dot
    const WIDTH: i32 = 20;
    const HEIGHT: i32 = 20;

    /// Maximum axis velocity of the dot
    const VEL: i32 = 10;

    /// Initializes the variables
    fn new(creator: &TextureCreator<WindowContext>) -> Self {
        // Initialize the offsets and the velocity
        Dot {
            pos: (0, 0),
            vel: (0, 0),
            texture: LTexture::from_file("dot", creator),
            particles: (0..=TOTAL_PARTICLES)
                .map(move |_| Particle::new((0, 0), creator))
                .collect(),
        }
    }

    #[rustfmt::skip]
    /// Takes key presses and adjusts the dot's velocity
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
    fn render(
        &mut self,
        canvas: &mut Canvas<Window>,
        creator: &TextureCreator<WindowContext>,
        shimmer_texture: &LTexture,
    ) {
        self.texture.render(canvas, self.pos);

        // # Show particles on top of dot

        // Go through particles
        for particle in self.particles.iter_mut() {
            // Replace dead particles
            if particle.is_dead() {
                *particle = Particle::new(self.pos, creator)
            }

            // Show particles
            particle.render(canvas, shimmer_texture);
        }
    }
}

fn main() {
    let sdl_ctx = sdl2::init().expect("Unable to initialize SDL!");
    let video = sdl_ctx.video().expect("Could not acquire video context!");

    sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "1");
    sdl2::hint::set("SDL_VIDEO_X11_NET_WM_BYPASS_COMPOSITOR", "0");

    let window = video
        .window("SDL Tutorial 38", WIDTH, HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .expect("Could not create SDL window!");

    // Obtain the canvas
    let mut canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .expect("Unable to obtain canvas!");

    let creator = canvas.texture_creator();
    // Initialize renderer color
    canvas.set_draw_color(Color::WHITE);

    // Get a handle to the SDL2 event pump
    let mut event_pump = sdl_ctx
        .event_pump()
        .expect("Unable to obtain event pump handle!");

    let mut shimmer_texture = LTexture::from_file("shimmer", &creator);
    shimmer_texture.texture.set_alpha_mod(192);

    //The dot that will be moving around on the screen
    let mut dot = Dot::new(&creator);

    lazy_foo::setup_mainloop(-1, true, move || {
        for event in event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                return false;
            }

            dot.handle_event(&event);
        }

        dot.r#move();
        canvas.clear();
        dot.render(&mut canvas, &creator, &shimmer_texture);
        canvas.present();

        true
    });
}
