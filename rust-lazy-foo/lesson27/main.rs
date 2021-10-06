#![allow(clippy::too_many_arguments)]

use sdl2::{
    event::Event,
    image::LoadSurface,
    keyboard::Keycode,
    pixels::Color,
    rect::{Point, Rect},
    render::{Canvas, RenderTarget, Texture, TextureCreator},
    surface::Surface,
    video::Window,
};

// Screen dimension
const WIDTH: u32 = 650;
const HEIGHT: u32 = 480;

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

    /// Renders texture at given point
    pub fn render<T: RenderTarget>(
        &self,
        canvas: &mut Canvas<T>,
        x: i32,
        y: i32,
        clip: Option<Rect>,
        rotation: Option<f64>,
        center: Option<Point>,
        flip_h: bool,
        flip_v: bool,
    ) {
        let clip_rect = match clip {
            Some(rect) => rect,
            None => Rect::new(0, 0, self.width, self.height),
        };
        let rot: f64 = rotation.unwrap_or(0.0);

        canvas
            .copy_ex(
                &self.texture,
                Some(clip_rect),
                Some(Rect::new(x, y, clip_rect.width(), clip_rect.height())),
                rot,
                center,
                flip_h,
                flip_v,
            )
            .expect("Could not blit texture to render target!");
    }
}

// The dot that will move around on the screen
struct Dot<'a> {
    /// The X and Y offsets of the dot
    pos: (i32, i32),

    /// The X and Y velocity of the dot
    vel: (i32, i32),

    //Dot's collision box
    collider: Rect,

    pub texture: LTexture<'a>,
}

impl<'a> Dot<'a> {
    // The dimensions of the dot
    const WIDTH: i32 = 20;
    const HEIGHT: i32 = 20;

    /// Maximum axis velocity of the dot
    const VEL: i32 = 10;

    /// Initializes the variables
    fn new(texture: LTexture<'a>) -> Self {
        //Initialize the offsets and the velocity
        Dot {
            pos: (0, 0),
            vel: (0, 0),
            //Set collision box dimension
            collider: Rect::new(0, 0, Dot::WIDTH as u32, Dot::HEIGHT as u32),
            texture,
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
    fn r#move(&mut self, wall: Rect) {
        // Move the dot left or right
        self.pos.0 += self.vel.0;
        self.collider.x = self.pos.0;

        // If the dot went too far to the left or right
        if self.pos.0 < 0
            || self.pos.0 + Dot::WIDTH > (WIDTH as i32)
            || check_collision(self.collider, wall)
        {
            // Move back
            self.pos.0 -= self.vel.0;
            self.collider.x = self.pos.0;
        }

        // Move the dot up or down
        self.pos.1 += self.vel.1;
        self.collider.y = self.pos.1;

        // If the dot went too far up or down
        if self.pos.1 < 0
            || self.pos.1 + Dot::HEIGHT > (HEIGHT as i32)
            || check_collision(self.collider, wall)
        {
            // Move back
            self.pos.1 -= self.vel.1;
            self.collider.y = self.pos.1;
        }
    }

    /// Shows the dot on the screen
    fn render<T: RenderTarget>(&self, canvas: &mut Canvas<T>) {
        self.texture.render(
            canvas, self.pos.0, self.pos.1, None, None, None, false, false,
        );
    }
}

fn check_collision(a: Rect, b: Rect) -> bool {
    //The sides of the rectangles

    //Calculate the sides of rect A
    let left_a = a.x();
    let right_a = a.x() + a.width() as i32;
    let top_a = a.y();
    let bottom_a = a.y() + a.height() as i32;

    //Calculate the sides of rect B
    let left_b = b.x();
    let right_b = b.x() + b.width() as i32;
    let top_b = b.y();
    let bottom_b = b.y() + b.height() as i32;

    //If any of the sides from A are outside of B
    if bottom_a <= top_b || top_a >= bottom_b || right_a <= left_b || left_a >= right_b {
        return false;
    }

    //If none of the sides from A are outside B
    true
}

fn init() -> (sdl2::Sdl, Window) {
    let sdl = sdl2::init().expect("Unable to initialize SDL!");
    let video = sdl.video().expect("Could not acquire video context!");

    sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "1");
    sdl2::hint::set("SDL_VIDEO_X11_NET_WM_BYPASS_COMPOSITOR", "0");

    let win = video
        .window("SDL Tutorial 27", WIDTH, HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .expect("Could not create SDL window!");

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

    // Get a handle to the SDL2 event pump
    let mut event_pump = context
        .event_pump()
        .expect("Unable to obtain event pump handle!");

    let dot_texture = LTexture::from_file("resources/dot.bmp", &creator);
    let mut dot = Dot::new(dot_texture);

    //Set the wall
    let wall = Rect::new(300, 40, 40, 400);

    'running: loop {
        // Extract any pending events from from the event pump and process them
        for event in event_pump.poll_iter() {
            // Pattern match on the Quit event
            if let Event::Quit { .. } = event {
                break 'running;
            }

            // Handle input for the dot
            dot.handle_event(&event);
        }

        // Move the dot
        dot.r#move(wall);

        canvas.set_draw_color(Color::RGBA(0xFF, 0xFF, 0xFF, 0xFF));
        canvas.clear();

        canvas.set_draw_color(Color::RGBA(0x00, 0x00, 0x00, 0xFF));
        canvas.draw_rect(wall).unwrap();

        // Render objects
        dot.render(&mut canvas);

        // Update the screen
        canvas.present();
    }
}