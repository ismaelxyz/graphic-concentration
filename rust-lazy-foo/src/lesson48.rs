use rand::{thread_rng, Rng};
use sdl2::{
    event::Event,
    image::{InitFlag, LoadSurface},
    pixels::Color,
    rect::Rect,
    render::{Canvas, Texture, TextureCreator},
    surface::Surface,
    video::{Window, WindowContext},
};
use std::{
    sync::{
        atomic::{AtomicI32, Ordering},
        Arc,
    },
    thread::sleep,
    time::Duration,
};

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

/// Our worker thread function
fn worker(data: &str, g_data: Arc<AtomicI32>) {
    println!("{data} starting...");
    let mut rng = thread_rng();

    // Work 5 times
    for _i in 0..5 {
        // Wait randomly
        sleep(Duration::from_millis(16 + rng.gen_range(0..32)));

        // Print pre work data
        println!("{data} gets {g_data:?}");

        // "Work"
        g_data.store(rng.gen_range(0..256), Ordering::Relaxed);

        // Print post work data
        println!("{data} sets {g_data:?}");

        // Wait randomly
        sleep(Duration::from_millis(16 + rng.gen_range(0..640)));
    }

    println!("{data} finished!\n");
}

fn main() {
    let sdl_ctx = sdl2::init().unwrap();
    let video = sdl_ctx.video().unwrap();
    let mut event_pump = sdl_ctx.event_pump().unwrap();

    sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "1");
    sdl2::hint::set("SDL_VIDEO_X11_NET_WM_BYPASS_COMPOSITOR", "0");

    let window = video
        .window("SDL Tutorial 48", 640, 480)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let _image = sdl2::image::init(InitFlag::PNG).unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let creator = canvas.texture_creator();

    let splash_texture = LTexture::from_file(
        &creator,
        std::path::Path::new("./resources/lesson47/splash.png"),
    );

    // The "data buffer" in an "access semaphore"
    let g_data = Arc::new(AtomicI32::new(-1));
    let g_data_clone = g_data.clone();

    // rust-sdl2 not have support for threads, Us used rust threads becaus it
    // Run the thread
    std::thread::Builder::new()
        .name("Thread A".to_string())
        .spawn(|| worker("Thread A", g_data))
        .unwrap();

    let mut rng = thread_rng();
    sleep(Duration::from_millis(16 + rng.gen_range(0..32)));

    std::thread::Builder::new()
        .name("Thread B".to_string())
        .spawn(|| worker("Thread B", g_data_clone))
        .unwrap();

    main_loop::setup_mainloop(-1, true, move || {
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
