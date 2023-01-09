use sdl2::{
    event::Event,
    image::{InitFlag, LoadSurface},
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
    render::{Canvas, Texture, TextureCreator},
    surface::Surface,
    video::{Window, WindowContext},
};

// The dimensions of the level
const LEVEL_WIDTH: i32 = 1280;
const LEVEL_HEIGHT: i32 = 960;

// Screen dimension
const WIDTH: i32 = 650;
const HEIGHT: i32 = 480;

pub struct LTexture {
    texture: Texture,
    width: u32,
    height: u32,
}

impl LTexture {
    pub fn from_file(path: &str, creator: &TextureCreator<WindowContext>) -> LTexture {
        let mut surf = Surface::from_file(path).expect("Could not load surface from file!");

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
    pub fn render(&self, canvas: &mut Canvas<Window>, x: i32, y: i32, clip: Option<Rect>) {
        let quad = match clip {
            Some(rect) => Rect::new(x, y, rect.width(), rect.height()),
            None => Rect::new(x, y, self.width, self.height),
        };

        canvas
            .copy_ex(&self.texture, clip, quad, 0.0, None, false, false)
            .expect("Could not blit texture to render target!");
    }
}

#[derive(Default)]
struct Dot {
    pos: (i32, i32),
    vel: (i32, i32),
}

impl Dot {
    // The dimensions of the dot
    const WIDTH: i32 = 20;
    const HEIGHT: i32 = 20;

    /// Maximum axis velocity of the dot
    const VEL: i32 = 10;

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
    fn r#move(&mut self) {
        // Move the dot left or right
        self.pos.0 += self.vel.0;

        // If the dot went too far to the left or right
        if self.pos.0 < 0 || self.pos.0 + Dot::WIDTH > LEVEL_WIDTH {
            // Move back
            self.pos.0 -= self.vel.0;
        }

        // Move the dot up or down
        self.pos.1 += self.vel.1;

        // If the dot went too far up or down
        if self.pos.1 < 0 || self.pos.1 + Dot::HEIGHT > LEVEL_HEIGHT {
            // Move back
            self.pos.1 -= self.vel.1;
        }
    }

    /// Shows the dot on the screen
    fn render(&self, canvas: &mut Canvas<Window>, cam_x: i32, cam_y: i32, texture: &LTexture) {
        texture.render(canvas, self.pos.0 - cam_x, self.pos.1 - cam_y, None);
    }
}

fn main() {
    let sdl_ctx = sdl2::init().expect("Unable to initialize SDL!");
    let video = sdl_ctx.video().expect("Could not acquire video context!");
    sdl2::image::init(InitFlag::PNG).unwrap();

    sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "1");
    sdl2::hint::set("SDL_VIDEO_X11_NET_WM_BYPASS_COMPOSITOR", "0");

    let window = video
        .window("SDL Tutorial 30", WIDTH as u32, HEIGHT as u32)
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

    let texture = LTexture::from_file("resources/lesson30/dot.bmp", &creator);
    let background = LTexture::from_file("resources/lesson30/bg.png", &creator);

    let mut dot = Dot::default();
    let mut camera = Rect::new(0, 0, WIDTH as u32, HEIGHT as u32);

    main_loop::setup_mainloop(-1, true, move || {
        for event in event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                return false;
            }

            dot.handle_event(&event);
        }

        // Move the dot
        dot.r#move();

        // Center the camera over the dot
        camera.x = dot.pos.0 + Dot::WIDTH / 2 - WIDTH / 2;
        camera.y = dot.pos.1 + Dot::HEIGHT / 2 - HEIGHT / 2;

        // Keep the camera in bounds
        if camera.x < 0 {
            camera.x = 0;
        }
        if camera.y < 0 {
            camera.y = 0;
        }
        if camera.x > LEVEL_WIDTH - camera.w {
            camera.x = LEVEL_WIDTH - camera.width() as i32;
        }
        if camera.y > LEVEL_HEIGHT - camera.h {
            camera.y = LEVEL_HEIGHT - camera.height() as i32;
        }

        canvas.set_draw_color(Color::WHITE);
        canvas.clear();

        // Render background
        background.render(&mut canvas, 0, 0, Some(camera));

        // Render objects
        dot.render(&mut canvas, camera.x, camera.y, &texture);

        // Update the screen
        canvas.present();

        true
    });
}
