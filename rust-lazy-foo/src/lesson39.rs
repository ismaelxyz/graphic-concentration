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
use std::sync::{
    atomic::{AtomicI32, Ordering},
    Arc,
};

// Screen dimension
const WIDTH: u32 = 650;
const HEIGHT: u32 = 480;

// The dimensions of the level
const LEVEL_WIDTH: i32 = 1280;
const LEVEL_HEIGHT: i32 = 960;

/// The different tile sprites
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum TileKind {
    Red,
    Green,
    Blue,
    Center,
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
    TopLeft,
}

#[derive(Debug)]
struct Tile {
    /// The attributes of the tile
    bound: Rect,

    /// The tile type
    kind: TileKind,
}

impl Tile {
    const WIDTH: u32 = 80;
    const HEIGHT: u32 = 80;

    /// Initializes position and type
    fn new((x, y): (i32, i32), kind: TileKind) -> Self {
        Tile {
            bound: Rect::new(x, y, Tile::WIDTH, Tile::HEIGHT),
            kind,
        }
    }
}

struct TileMap<'a> {
    tiles: [Tile; TileMap::TOTAL],
    clips: [Rect; TileMap::TOTAL_SPRITES],
    texture: LTexture<'a>,
}

impl<'a> TileMap<'a> {
    const TOTAL: usize = 192;
    const TOTAL_SPRITES: usize = 12;

    #[rustfmt::skip]
    const MAP: [TileKind; TileMap::TOTAL] = [TileKind::Red, TileKind::Green, TileKind::Blue, TileKind::Red, TileKind::Green, TileKind::Blue, TileKind::Red, TileKind::Green, TileKind::Blue, TileKind::Red, TileKind::Green, TileKind::Blue, TileKind::Red, TileKind::Green, TileKind::Blue, TileKind::Red, TileKind::Green, TileKind::Blue, TileKind::Red, TileKind::Green, TileKind::Blue, TileKind::Red, TileKind::Green, TileKind::Blue, TileKind::Red, TileKind::Green, TileKind::Blue, TileKind::Red, TileKind::Green, TileKind::Blue, TileKind::Red, TileKind::Green, TileKind::Blue, TileKind::Red, TileKind::TopLeft, TileKind::Top, TileKind::Top, TileKind::Top, TileKind::Top, TileKind::Top, TileKind::Top, TileKind::Top, TileKind::Top, TileKind::Top, TileKind::Top, TileKind::TopRight, TileKind::Green, TileKind::Blue, TileKind::Red, TileKind::Green, TileKind::Left, TileKind::Center, TileKind::Center, TileKind::Center, TileKind::Center, TileKind::Center, TileKind::Center, TileKind::Center, TileKind::Center, TileKind::Center, TileKind::Center, TileKind::Right, TileKind::Blue, TileKind::Red, TileKind::Green, TileKind::Blue, TileKind::Left, TileKind::Center, TileKind::Bottom, TileKind::Bottom, TileKind::Bottom, TileKind::Bottom, TileKind::Bottom, TileKind::Bottom, TileKind::Bottom, TileKind::Center, TileKind::Center, TileKind::Right, TileKind::Red, TileKind::Green, TileKind::Blue, TileKind::Red, TileKind::Left, TileKind::Right, TileKind::Red, TileKind::Green, TileKind::Blue, TileKind::Red, TileKind::Green, TileKind::Blue, TileKind::Red, TileKind::Left, TileKind::Center, TileKind::Right, TileKind::Green, TileKind::Blue, TileKind::Red, TileKind::Green, TileKind::Left, TileKind::Right, TileKind::Green, TileKind::TopLeft, TileKind::TopRight, TileKind::Green, TileKind::Blue, TileKind::Red, TileKind::Green, TileKind::Left, TileKind::Center, TileKind::Right, TileKind::Blue, TileKind::Red, TileKind::Green, TileKind::Blue, TileKind::Left, TileKind::Right, TileKind::Blue, TileKind::BottomLeft, TileKind::BottomRight, TileKind::Blue, TileKind::Red, TileKind::Green, TileKind::Blue, TileKind::Left, TileKind::Center, TileKind::Right, TileKind::Red, TileKind::Green, TileKind::Blue, TileKind::Red, TileKind::Left, TileKind::Right, TileKind::Red, TileKind::Green, TileKind::Blue, TileKind::Red, TileKind::Green, TileKind::Blue, TileKind::Red, TileKind::Left, TileKind::Center, TileKind::Right, TileKind::Green, TileKind::Blue, TileKind::Red, TileKind::Green, TileKind::Left, TileKind::Center, TileKind::Top, TileKind::Top, TileKind::Top, TileKind::TopRight, TileKind::Blue, TileKind::Red, TileKind::Green, TileKind::BottomLeft, TileKind::Bottom, TileKind::BottomRight, TileKind::Blue, TileKind::Red, TileKind::Green, TileKind::Blue, TileKind::BottomLeft, TileKind::Bottom, TileKind::Bottom, TileKind::Bottom, TileKind::Bottom, TileKind::BottomRight, TileKind::Red, TileKind::Green, TileKind::Blue, TileKind::Red, TileKind::Green, TileKind::Blue, TileKind::Red, TileKind::Green, TileKind::Blue, TileKind::Red, TileKind::Green, TileKind::Blue, TileKind::Red, TileKind::Green, TileKind::Blue, TileKind::Red, TileKind::Green, TileKind::Blue, TileKind::Red, TileKind::Green, TileKind::Blue, TileKind::Red, TileKind::Green, TileKind::Blue];

    fn new(texture: LTexture<'a>) -> Self {
        // The tile offsets
        let x = Arc::new(AtomicI32::new(0));
        let y = Arc::new(AtomicI32::new(0));
        TileMap {
            // Initialize the tiles
            tiles: Self::MAP
                .iter()
                .map(move |kind| {
                    let tile = Tile::new(
                        (x.load(Ordering::Relaxed), y.load(Ordering::Relaxed)),
                        *kind,
                    );

                    // Move to next tile spot
                    x.fetch_add(Tile::WIDTH as i32, Ordering::SeqCst);

                    // If we've gone too far
                    if x.load(Ordering::Relaxed) >= LEVEL_WIDTH {
                        // Move back
                        x.store(0, Ordering::Relaxed);

                        // Move to the next row
                        y.fetch_add(Tile::HEIGHT as i32, Ordering::SeqCst);
                    }

                    tile
                })
                .collect::<Vec<Tile>>()
                .try_into()
                .unwrap(),
            clips: [
                (0, 0),
                (0, 80),
                (0, 160),
                (160, 80),
                (160, 0),
                (240, 0),
                (240, 80),
                (240, 160),
                (160, 160),
                (80, 160),
                (80, 80),
                (80, 0),
            ]
            .iter()
            .map(move |(x, y)| Rect::new(*x, *y, Tile::WIDTH, Tile::HEIGHT))
            .collect::<Vec<Rect>>()
            .try_into()
            .unwrap(),
            texture,
        }
    }

    /// Shows the tiles
    fn render(&self, canvas: &mut Canvas<Window>, camera: Rect) {
        for tile in &self.tiles {
            // If the tile is on screen
            if check_collision(camera, tile.bound) {
                // Show the tile
                self.texture.render(
                    canvas,
                    tile.bound.x - camera.x,
                    tile.bound.y - camera.y,
                    Some(self.clips[tile.kind as usize]),
                );
            }
        }
    }

    fn touches_wall(&self, bound: Rect) -> bool {
        // Go through the tiles
        self.tiles.iter().any(move |tile| {
            // If the tile is a wall type tile and the collision bound touches the wall tile
            tile.kind >= TileKind::Center
                && tile.kind <= TileKind::TopLeft
                && check_collision(bound, tile.bound)
        })
    }
}

fn check_collision(a: Rect, b: Rect) -> bool {
    // Calculate the sides of rect A
    let left_a = a.x;
    let right_a = a.x + a.w;
    let top_a = a.y;
    let bottom_a = a.y + a.h;

    // Calculate the sides of rect B
    let left_b = b.x;
    let right_b = b.x + b.w;
    let top_b = b.y;
    let bottom_b = b.y + b.h;

    // If any of the sides from A are outside of B return false.
    !(bottom_a <= top_b || top_a >= bottom_b || right_a <= left_b || left_a >= right_b)
}

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
    pub fn from_file(file: &str, creator: &'a TextureCreator<WindowContext>) -> LTexture<'a> {
        let mut surf = Surface::from_file(format!("resources/{file}"))
            .expect("Could not load surface from file!");

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
    pub fn render(&self, canvas: &mut Canvas<Window>, x: i32, y: i32, clip: Option<Rect>) {
        // Set rendering space and render to screen
        let render_quad = match clip {
            Some(clip) => Rect::new(x, y, clip.width(), clip.height()),
            None => Rect::new(x, y, self.width, self.height),
        };

        canvas
            .copy_ex(
                &self.texture,
                clip,
                Some(render_quad),
                0.0,
                None,
                false,
                false,
            )
            .expect("Could not blit texture to render target!");
    }
}

// The dot that will move around on the screen
struct Dot<'a> {
    /// Collision box of the dot
    bound: Rect,

    /// The X and Y velocity of the dot
    vel: (i32, i32),

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
        // Initialize the collision bound and the velocity
        Dot {
            bound: Rect::new(0, 0, Dot::WIDTH as u32, Dot::HEIGHT as u32),
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

    /// Moves the dot and check collision against tiles
    fn r#move(&mut self, tiles: &TileMap) {
        // Move the dot left or right
        self.bound.x += self.vel.0;

        // If the dot went too far to the left or right or touched a wall
        if self.bound.x < 0
            || self.bound.x + Dot::WIDTH > LEVEL_WIDTH
            || tiles.touches_wall(self.bound)
        {
            // move back
            self.bound.x -= self.vel.0;
        }

        // Move the dot up or down
        self.bound.y += self.vel.1;

        // If the dot went too far up or down or touched a wall
        if self.bound.y < 0
            || self.bound.y + Dot::HEIGHT > LEVEL_HEIGHT
            || tiles.touches_wall(self.bound)
        {
            // Move back
            self.bound.y -= self.vel.1;
        }
    }

    /// Centers the camera over the dot
    fn set_camera(&self, camera: &mut Rect) {
        // Center the camera over the dot
        camera.x = (self.bound.x + Dot::WIDTH / 2) - (WIDTH as i32) / 2;
        camera.y = (self.bound.y + Dot::HEIGHT / 2) - (HEIGHT as i32) / 2;

        // Keep the camera in bounds
        if camera.x < 0 {
            camera.x = 0;
        }
        if camera.y < 0 {
            camera.y = 0;
        }
        if camera.x > LEVEL_WIDTH - camera.w {
            camera.x = LEVEL_WIDTH - camera.w;
        }
        if camera.y > LEVEL_HEIGHT - camera.h {
            camera.y = LEVEL_HEIGHT - camera.h;
        }
    }

    /// Shows the dot on the screen
    fn render(&self, canvas: &mut Canvas<Window>, camera: Rect) {
        // Show image
        self.texture.render(
            canvas,
            self.bound.x - camera.x,
            self.bound.y - camera.y,
            None,
        );
    }
}

fn main() {
    let sdl_ctx = sdl2::init().expect("Unable to initialize SDL!");
    let video = sdl_ctx.video().expect("Could not acquire video context!");

    sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "1");
    sdl2::hint::set("SDL_VIDEO_X11_NET_WM_BYPASS_COMPOSITOR", "0");

    let window = video
        .window("SDL Tutorial 39", WIDTH, HEIGHT)
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

    canvas.set_draw_color(Color::WHITE);
    let creator = canvas.texture_creator();

    let dot_texture = LTexture::from_file("dot.bmp", &creator);
    let tile_texture = LTexture::from_file("tiles.png", &creator);

    let mut event_pump = sdl_ctx
        .event_pump()
        .expect("Unable to obtain event pump handle!");

    let mut dot = Dot::new(dot_texture);
    let mut camera = Rect::new(0, 0, WIDTH, HEIGHT);
    let tiles = TileMap::new(tile_texture);

    'running: loop {
        // Extract any pending events from from the event pump and process them
        for event in event_pump.poll_iter() {
            // Pattern match on the Quit event
            if let Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } = event
            {
                break 'running;
            }

            // Handle input for the dot
            dot.handle_event(&event);
        }

        // Move the dot
        dot.r#move(&tiles);
        dot.set_camera(&mut camera);
        // Clear and render the texture each pass through the loop
        canvas.clear();
        // Render level
        tiles.render(&mut canvas, camera);
        // Render objects
        dot.render(&mut canvas, camera);
        // Update the screen
        canvas.present();
    }
}
