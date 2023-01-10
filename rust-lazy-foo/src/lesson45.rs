#![feature(box_syntax)]

use sdl2::{
    event::Event,
    image::{InitFlag, LoadSurface},
    pixels::Color,
    rect::Rect,
    render::{Canvas, Texture, TextureCreator},
    surface::Surface,
    video::{Window, WindowContext},
};

const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;

struct LTexture {
    texture: Texture,
    width: u32,
    height: u32,
}

impl LTexture {
    fn from_file(creator: &TextureCreator<WindowContext>, path: &std::path::Path) -> LTexture {
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
            .copy(
                &self.texture,
                None,
                Some(Rect::new(x, y, self.width, self.height)),
            )
            .unwrap();
    }
}

fn callback(param: &str) {
    // Print callback message
    println!("Callback called back with message: {param}");
}

fn main() {
    let sdl_ctx = sdl2::init().unwrap();
    let video = sdl_ctx.video().unwrap();
    let timer = sdl_ctx.timer().unwrap();

    sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "1");
    sdl2::hint::set("SDL_VIDEO_X11_NET_WM_BYPASS_COMPOSITOR", "0");

    let window = video
        .window("SDL Tutorial 45", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let _image = sdl2::image::init(InitFlag::PNG).unwrap();
    let _timer_id = timer.add_timer(3 * 1000, box || {
        callback("3 seconds waited!");
        0
    });

    let mut canvas = window.into_canvas().build().unwrap();
    let creator = canvas.texture_creator();

    let splash_texture = LTexture::from_file(
        &creator,
        std::path::Path::new("./resources/lesson45/splash.png"),
    );

    // Get a handle to the SDL2 event pump
    let mut event_pump = sdl_ctx.event_pump().unwrap();

    lazy_foo::setup_mainloop(-1, true, move || {
        for event in event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                return false;
            }
        }

        // Clear and render the texture each pass through the loop
        canvas.set_draw_color(Color::WHITE);
        canvas.clear();

        splash_texture.render(&mut canvas, 0, 0);

        // Update the screen
        canvas.present();

        true
    });
}
