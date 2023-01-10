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

    /// Renders texture at given point
    pub fn render(&self, canvas: &mut Canvas<Window>, x: i32, y: i32) {
        let clip_rect = Rect::new(x, y, self.width, self.height);

        canvas
            .copy_ex(&self.texture, None, clip_rect, 0.0, None, false, false)
            .expect("Could not blit texture to render target!");
    }
}

/// The dot that will move around on the screen
struct Dot {
    /// The X and Y offsets of the dot
    pos: (i32, i32),

    /// The X and Y velocity of the dot
    vel: (i32, i32),

    /// Dot's collision boxes
    colliders: Vec<Rect>,
}

impl Dot {
    // The dimensions of the dot
    const WIDTH: i32 = 20;
    const HEIGHT: i32 = 20;

    /// Maximum axis velocity of the dot
    const VEL: i32 = 1;

    /// Initializes the variables
    fn new(x: i32, y: i32) -> Self {
        let pos = (x, y);

        let colliders = vec![
            Rect::new(0, 0, 6, 1),
            Rect::new(0, 0, 10, 1),
            Rect::new(0, 0, 14, 1),
            Rect::new(0, 0, 16, 2),
            Rect::new(0, 0, 18, 2),
            Rect::new(0, 0, 20, 6),
            Rect::new(0, 0, 18, 2),
            Rect::new(0, 0, 16, 2),
            Rect::new(0, 0, 14, 1),
            Rect::new(0, 0, 10, 1),
            Rect::new(0, 0, 6, 1),
        ];

        //Initialize the offsets and the velocity
        let mut dot = Dot {
            pos,
            vel: (0, 0),
            colliders,
        };

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
    fn r#move(&mut self, colliders: &[Rect]) {
        //Move the dot left or right
        self.pos.0 += self.vel.0;
        self.shift_colliders();

        //If the dot collided or went too far to the left or right
        if self.pos.0 < 0
            || self.pos.0 + Dot::WIDTH > WIDTH as i32
            || check_collision(&self.colliders, colliders)
        {
            //Move back
            self.pos.0 -= self.vel.0;
            self.shift_colliders();
        }

        //Move the dot up or down
        self.pos.1 += self.vel.1;
        self.shift_colliders();

        //If the dot collided or went too far up or down
        if self.pos.1 < 0
            || self.pos.1 + Dot::HEIGHT > HEIGHT as i32
            || check_collision(&self.colliders, colliders)
        {
            //Move back
            self.pos.1 -= self.vel.1;
            self.shift_colliders();
        }
    }

    /// Shows the dot on the screen
    fn render(&self, canvas: &mut Canvas<Window>, texture: &LTexture) {
        texture.render(canvas, self.pos.0, self.pos.1);
    }

    /// Gets the collision boxes
    pub fn colliders(&self) -> &Vec<Rect> {
        &self.colliders
    }

    /// Moves the collision boxes relative to the dot's offset
    fn shift_colliders(&mut self) {
        //The row offset
        let mut r = 0i32;

        //Go through the dot's collision boxes
        for set in 0..self.colliders.len() {
            //Center the collision box
            let width = self.colliders[set].width() as i32;
            self.colliders[set].set_x(self.pos.0 + (Dot::WIDTH - width) / 2);

            //Set the collision box at its row offset
            self.colliders[set].set_y(self.pos.1 + r);

            //Move the row offset down the height of the collision box
            r += self.colliders[set].height() as i32;
        }
    }
}

fn check_collision(a: &[Rect], b: &[Rect]) -> bool {
    //The sides of the rectangles

    //Go through the A boxes
    for a_box in a {
        //Calculate the sides of rect A
        let left_a = a_box.x();
        let right_a = a_box.x() + a_box.width() as i32;
        let top_a = a_box.y();
        let bottom_a = a_box.y() + a_box.height() as i32;

        //Go through the B boxes
        for b_box in b {
            //Calculate the sides of rect B
            let left_b = b_box.x();
            let right_b = b_box.x() + b_box.width() as i32;
            let top_b = b_box.y();
            let bottom_b = b_box.y() + b_box.height() as i32;

            //If no sides from A are outside of B
            if !(bottom_a <= top_b || top_a >= bottom_b || right_a <= left_b || left_a >= right_b) {
                //A collision is detected
                return true;
            }
        }
    }

    //If neither set of collision boxes touched
    false
}

fn main() {
    let sdl_ctx = sdl2::init().expect("Unable to initialize SDL!");
    let video = sdl_ctx.video().expect("Could not acquire video context!");

    sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "1");
    sdl2::hint::set("SDL_VIDEO_X11_NET_WM_BYPASS_COMPOSITOR", "0");

    let window = video
        .window("SDL Tutorial 28", WIDTH, HEIGHT)
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
    let mut event_pump = sdl_ctx
        .event_pump()
        .expect("Unable to obtain event pump handle!");

    let texture = LTexture::from_file("resources/lesson26/dot.bmp", &creator);

    let mut dot = Dot::new(0, 0);
    let other_dot = Dot::new(WIDTH as i32 / 4, HEIGHT as i32 / 4);

    lazy_foo::setup_mainloop(-1, true, move || {
        for event in event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                return false;
            }

            // Handle input for the dot
            dot.handle_event(&event);
        }

        dot.r#move(other_dot.colliders());

        canvas.set_draw_color(Color::WHITE);
        canvas.clear();

        dot.render(&mut canvas, &texture);
        other_dot.render(&mut canvas, &texture);
        canvas.present();

        true
    });
}
