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
use std::path::Path;

const IMG_ARROW: &str = "resources/lesson15/arrow.png";

struct LTexture {
    texture: Texture,
    width: u32,
    height: u32,
}

impl LTexture {
    fn from_file(creator: &TextureCreator<WindowContext>, path: &Path) -> LTexture {
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

    // Renders a texture to a given point using a provided renderer
    // provide additional arguments for rotation and flipping
    // Rust doesn't provide default arguments, and it seems overkill
    // to provide additional function signatures for this, so we're
    // going to wrap rotation and flipping args in Option<> so we can
    // provide None when we don't care about it.
    fn render_to(
        &self,
        canvas: &mut Canvas<Window>,
        (x, y): (i32, i32),
        rotation: f64,
        (flip_h, flip_v): (bool, bool),
    ) {
        canvas
            .copy_ex(
                &self.texture,
                Some(Rect::new(0, 0, self.width, self.height)),
                Some(Rect::new(x, y, self.width, self.height)),
                rotation,
                None,
                flip_h,
                flip_v,
            )
            .unwrap();
    }
}

fn main() {
    let sdl_ctx = sdl2::init().expect("Could not initialize SDL!");
    let video = sdl_ctx
        .video()
        .expect("Could not obtain video from sdl context!");

    sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "1");
    sdl2::hint::set("SDL_VIDEO_X11_NET_WM_BYPASS_COMPOSITOR", "0");

    let window = video
        .window("SDL Tutorial 15", 640, 480)
        .position_centered()
        .opengl()
        .build()
        .expect("Could not create Window!");

    let (width, height) = window.size();

    sdl2::image::init(InitFlag::PNG).expect("Could not initialize sdl2_image!");

    let mut canvas = window.into_canvas().build().unwrap();
    let creator = canvas.texture_creator();

    let arrow = LTexture::from_file(&creator, Path::new(IMG_ARROW));

    let mut event_pump = sdl_ctx.event_pump().expect("Could not obtain event pump!");

    // Track current rotation and flips
    let mut degrees: f64 = 0.0;
    let mut flip_vertical: bool = false;
    let mut flip_horizontal: bool = false;

    lazy_foo::setup_mainloop(-1, true, move || {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => return false,
                Event::KeyDown { keycode: k, .. } => match k {
                    Some(Keycode::A) => {
                        degrees -= 60.0;
                    }
                    Some(Keycode::D) => {
                        degrees += 60.0;
                    }
                    Some(Keycode::Q) => {
                        flip_horizontal = !flip_horizontal;
                    }
                    Some(Keycode::W) => {
                        flip_horizontal = false;
                        flip_vertical = false;
                    }
                    Some(Keycode::E) => {
                        flip_vertical = !flip_vertical;
                    }
                    Some(Keycode::Escape) => return false,
                    _ => {}
                },
                _ => {}
            }
        }
        // Clear and render the texture each pass through the loop
        canvas.set_draw_color(Color::WHITE);
        canvas.clear();

        // Render the arrow
        arrow.render_to(
            &mut canvas,
            (
                (width - arrow.width) as i32 / 2,
                (height - arrow.height) as i32 / 2,
            ),
            degrees,
            (flip_horizontal, flip_vertical),
        );

        // Update the screen
        canvas.present();

        true
    });
}
