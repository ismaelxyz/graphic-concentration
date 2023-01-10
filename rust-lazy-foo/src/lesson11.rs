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

    fn render_to(&self, canvas: &mut Canvas<Window>, x: i32, y: i32, clip: Option<Rect>) {
        let clip_rect = match clip {
            Some(rect) => rect,
            None => Rect::new(0, 0, self.width, self.height),
        };

        canvas
            .copy(
                &self.texture,
                Some(clip_rect),
                Some(Rect::new(x, y, clip_rect.width(), clip_rect.height())),
            )
            .unwrap();
    }
}

fn main() {
    let sdl_ctx = sdl2::init().unwrap();
    let video = sdl_ctx.video().unwrap();

    sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "1");
    sdl2::hint::set("SDL_VIDEO_X11_NET_WM_BYPASS_COMPOSITOR", "0");

    let window = video
        .window("SDL Tutorial 11", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let _image = sdl2::image::init(InitFlag::PNG).unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let creator = canvas.texture_creator();

    let sprite_sheet = LTexture::from_file(
        &creator,
        std::path::Path::new("./resources/lesson11/dots.png"),
    );

    let sprite_clips = [
        Rect::new(0, 0, 100, 100),
        Rect::new(100, 0, 100, 100),
        Rect::new(0, 100, 100, 100),
        Rect::new(100, 100, 100, 100),
    ];

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

        // Render top left sprite
        sprite_sheet.render_to(&mut canvas, 0, 0, Some(sprite_clips[0]));

        // Render top right sprite
        sprite_sheet.render_to(
            &mut canvas,
            (SCREEN_WIDTH - sprite_clips[1].width()) as i32,
            0,
            Some(sprite_clips[1]),
        );

        // Render bottom left sprite
        sprite_sheet.render_to(
            &mut canvas,
            0,
            (SCREEN_HEIGHT - sprite_clips[2].height()) as i32,
            Some(sprite_clips[2]),
        );

        // Render bottom right sprite
        sprite_sheet.render_to(
            &mut canvas,
            (SCREEN_WIDTH - sprite_clips[3].width()) as i32,
            (SCREEN_HEIGHT - sprite_clips[3].height()) as i32,
            Some(sprite_clips[3]),
        );

        // Update the screen
        canvas.present();

        true
    });
}
