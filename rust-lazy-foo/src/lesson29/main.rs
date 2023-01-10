mod utils;

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

use crate::utils::*;

// Screen dimension
const WIDTH: u32 = 650;
const HEIGHT: u32 = 480;

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
        surf.set_color_key(true, Color::RGB(0, 0xFF, 0xFF)).unwrap();

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

    /// Renders texture at given point
    pub fn render(&self, canvas: &mut Canvas<Window>, x: i32, y: i32) {
        canvas
            .copy_ex(
                &self.texture,
                None,
                Rect::new(x, y, self.width, self.height),
                0.0,
                None,
                false,
                false,
            )
            .expect("Could not blit texture to render target!");
    }
}

struct Dot {
    pos: (i32, i32),
    vel: (i32, i32),
    collider: Circle,
}

impl Dot {
    // The dimensions of the dot
    const WIDTH: i32 = 20;
    const HEIGHT: i32 = 20;

    /// Maximum axis velocity of the dot
    const VEL: i32 = 1;

    /// Initializes the variables
    fn new(x: i32, y: i32) -> Self {
        let mut dot = Dot {
            // Initialize the offsets
            pos: (x, y),
            // Initialize the velocity
            vel: (0, 0),
            // Set collision circle size
            collider: Circle {
                pos: (0, 0),
                rot: Dot::WIDTH / 2,
            },
        };

        // Move collider relative to the circle
        dot.shift_colliders();
        dot
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

    /// Moves the dot and checks collision
    fn r#move(&mut self, square: Rect, circle: Circle) {
        //Move the dot left or right
        self.pos.0 += self.vel.0;
        self.shift_colliders();

        //If the dot collided or went too far to the left or right
        if self.pos.0 - self.collider.rot < 0
            || self.pos.0 + self.collider.rot > WIDTH as i32
            || self.collider.check_collision(square)
            || self.collider.check_collision(circle)
        {
            //Move back
            self.pos.0 -= self.vel.0;
            self.shift_colliders();
        }

        //Move the dot up or down
        self.pos.1 += self.vel.1;
        self.shift_colliders();

        //If the dot collided or went too far up or down
        if self.pos.1 - self.collider.rot < 0
            || self.pos.1 + self.collider.rot > HEIGHT as i32
            || self.collider.check_collision(square)
            || self.collider.check_collision(circle)
        {
            //Move back
            self.pos.1 -= self.vel.1;
            self.shift_colliders();
        }
    }

    /// Shows the dot on the screen
    fn render(&self, canvas: &mut Canvas<Window>, texture: &LTexture) {
        texture.render(
            canvas,
            self.pos.0 - self.collider.rot,
            self.pos.1 - self.collider.rot,
        );
    }

    /// Gets the collision boxes
    pub fn colliders(&self) -> Circle {
        self.collider
    }

    /// Moves the collision boxes relative to the dot's offset
    fn shift_colliders(&mut self) {
        // Align collider to center of dot
        self.collider.pos.0 = self.pos.0;
        self.collider.pos.1 = self.pos.1;
    }
}

fn main() {
    let sdl_ctx = sdl2::init().expect("Unable to initialize SDL!");
    let video = sdl_ctx.video().expect("Could not acquire video context!");

    sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "1");
    sdl2::hint::set("SDL_VIDEO_X11_NET_WM_BYPASS_COMPOSITOR", "0");

    let window = video
        .window("SDL Tutorial 29", WIDTH, HEIGHT)
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
    let mut event_pump = sdl_ctx
        .event_pump()
        .expect("Unable to obtain event pump handle!");

    let texture = LTexture::from_file("resources/lesson26/dot.bmp", &creator);

    let mut dot = Dot::new(Dot::WIDTH / 2, Dot::HEIGHT / 2);
    let other_dot = Dot::new(WIDTH as i32 / 4, HEIGHT as i32 / 4);
    let wall = Rect::new(300, 40, 40, 400);

    lazy_foo::setup_mainloop(-1, true, move || {
        for event in event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                return false;
            }

            dot.handle_event(&event);
        }

        // Move the dot
        dot.r#move(wall, other_dot.colliders());

        canvas.set_draw_color(Color::WHITE);
        canvas.clear();

        // Render wall
        canvas.set_draw_color(Color::BLACK);
        canvas.draw_rect(wall).unwrap();

        // Render objects
        dot.render(&mut canvas, &texture);
        other_dot.render(&mut canvas, &texture);

        canvas.present();

        true
    });
}
