use sdl2::{
    event::Event,
    image::LoadSurface,
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
    render::{Canvas, RenderTarget, Texture, TextureCreator},
    surface::Surface,
    video::Window,
};

//The dimensions of the level
const LEVEL_WIDTH: i32 = 1280;
const LEVEL_HEIGHT: i32 = 960;

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

    /// Renders texture at given point
    pub fn render<T: RenderTarget>(
        &self,
        canvas: &mut Canvas<T>,
        x: i32,
        y: i32,
        clip: Option<Rect>,
    ) {
        let clip_rect = match clip {
            Some(rect) => Rect::new(x, y, rect.width(), rect.height()),
            None => Rect::new(x, y, self.width, self.height),
        };

        canvas
            .copy_ex(&self.texture, clip, clip_rect, 0.0, None, true, true)
            .expect("Could not blit texture to render target!");
    }
}

// The dot that will move around on the screen
struct Dot<'a> {
    /// The X and Y offsets of the dot
    pos: (i32, i32),

    /// The X and Y velocity of the dot
    vel: (i32, i32),

    pub texture: &'a LTexture<'a>,
}

impl<'a> Dot<'a> {
    // The dimensions of the dot
    const WIDTH: i32 = 20;
    const HEIGHT: i32 = 20;

    /// Maximum axis velocity of the dot
    const VEL: i32 = 10;

    /// Initializes the variables
    fn new(texture: &'a LTexture<'a>) -> Self {
        Dot {
            // Initialize the offsets
            pos: (0, 0),
            // Initialize the velocity
            vel: (0, 0),
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

    /// Moves the dot and checks collision
    fn r#move(&mut self) {
        //Move the dot left or right
        self.pos.0 += self.vel.0;

        //If the dot went too far to the left or right
        if self.pos.0 < 0 || self.pos.0 + Dot::WIDTH > LEVEL_WIDTH {
            //Move back
            self.pos.0 -= self.vel.0;
        }

        //Move the dot up or down
        self.pos.1 += self.vel.1;

        //If the dot went too far up or down
        if self.pos.1 < 0 || self.pos.1 + Dot::HEIGHT > LEVEL_HEIGHT {
            //Move back
            self.pos.1 -= self.vel.1;
        }
    }

    /// Shows the dot on the screen
    fn render<T: RenderTarget>(&self, canvas: &mut Canvas<T>, cam_x: i32, cam_y: i32) {
        self.texture
            .render(canvas, self.pos.0 - cam_x, self.pos.1 - cam_y, None);
    }
}

fn init() -> (sdl2::Sdl, Window) {
    let sdl = sdl2::init().expect("Unable to initialize SDL!");
    let video = sdl.video().expect("Could not acquire video context!");

    sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "1");
    sdl2::hint::set("SDL_VIDEO_X11_NET_WM_BYPASS_COMPOSITOR", "0");

    let win = video
        .window("SDL Tutorial 30", WIDTH as u32, HEIGHT as u32)
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

    let texture = LTexture::from_file("resources/dot2.bmp", &creator);
    let background = LTexture::from_file("resources/bg.png", &creator);

    let mut dot = Dot::new(&texture);
    let mut camera = Rect::new(0, 0, WIDTH as u32, HEIGHT as u32);

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
        dot.r#move();

        //Center the camera over the dot
        camera.set_x((dot.pos.0 + Dot::WIDTH / 2) - WIDTH / 2);
        camera.set_y((dot.pos.1 + Dot::HEIGHT / 2) - HEIGHT / 2);

        //Keep the camera in bounds
        if camera.x() < 0 {
            camera.set_x(0);
        }
        if camera.y() < 0 {
            camera.set_y(0);
        }
        if camera.x() > LEVEL_WIDTH - camera.w {
            camera.set_x(LEVEL_WIDTH - camera.width() as i32);
        }
        if camera.y() > LEVEL_HEIGHT - camera.h {
            camera.set_y(LEVEL_HEIGHT - camera.height() as i32);
        }

        canvas.set_draw_color(Color::WHITE);
        canvas.clear();

        //Render background
        background.render(&mut canvas, 0, 0, Some(camera));

        // Render objects
        dot.render(&mut canvas, camera.x(), camera.y());

        // Update the screen
        canvas.present();
    }
}
