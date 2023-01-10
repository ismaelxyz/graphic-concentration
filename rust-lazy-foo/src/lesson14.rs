//! Note: Starting with this tutorial we will eschew using either
//! .unwrap() or matching on errors, instead we will use
//! .expect(error_string), which essentially works like an unwrap,
//! but will display the error_string as well as panicking.

use sdl2::{
    event::Event,
    image::{InitFlag, LoadSurface},
    pixels::Color,
    rect::Rect,
    render::{Canvas, Texture, TextureCreator},
    surface::Surface,
    video::{Window, WindowContext},
};

use std::path::Path;

/// usize so we can mod the array index with it without having to cast.
const WALKING_FRAMES: usize = 4;

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
        let (width, height) = surface.size();
        LTexture {
            texture,
            width,
            height,
        }
    }

    // Renders a texture to a given point using a provided renderer
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
    let sdl_ctx = sdl2::init().expect("Could not initialize SDL!");
    let video = sdl_ctx
        .video()
        .expect("Could not acquire the video context!");

    sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "1");
    sdl2::hint::set("SDL_VIDEO_X11_NET_WM_BYPASS_COMPOSITOR", "0");

    let window = video
        .window("SDL Tutorial 14", 640, 480)
        .position_centered()
        .opengl()
        .build()
        .expect("Could not create window!");

    let _image = sdl2::image::init(InitFlag::PNG).expect("Could not initialize sdl2_image!");

    let (width, height) = window.size();
    let mut canvas = window.into_canvas().build().unwrap();
    let creator = canvas.texture_creator();

    let sprite_sheet = LTexture::from_file(&creator, Path::new("./resources/lesson14/foo2.png"));
    let clips = [
        Rect::new(0, 0, 64, 205),
        Rect::new(64, 0, 64, 205),
        Rect::new(128, 0, 64, 205),
        Rect::new(196, 0, 64, 205),
    ];

    let mut event_pump = sdl_ctx.event_pump().expect("Could not obtain event_pump!");

    // Set current frame to 0
    let mut frame: usize = 0;

    lazy_foo::setup_mainloop(-1, true, move || {
        // Extract any pending events from from the event pump and process them
        for event in event_pump.poll_iter() {
            // pattern match on the type of event
            if let Event::Quit { .. } = event {
                return false;
            }
        }

        canvas.set_draw_color(Color::WHITE);
        canvas.clear();

        let current_clip: Rect = clips[frame % WALKING_FRAMES];
        sprite_sheet.render_to(
            &mut canvas,
            ((width - current_clip.width()) / 2) as i32,
            ((height - current_clip.height()) / 2) as i32,
            Some(current_clip),
        );

        canvas.present();
        // Increment the frame
        frame += 1;
        // Set the frame to 0 if we're at 4
        if frame == WALKING_FRAMES {
            frame = 0;
        }

        // This isn't in the tutorial, but we're going to pause 100ms
        // to give the animation a more graceful, smooth cadence

        //#[cfg(target_os = "emscripten")]
        //emscripten::sleep(100);

        #[cfg(not(target_os = "emscripten"))]
        std::thread::sleep(std::time::Duration::from_millis(100));

        true
    });
}
