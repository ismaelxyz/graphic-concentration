#![feature(mutex_unlock)]
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
    sync::{Arc, Mutex},
    thread::sleep,
    time::Duration,
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

fn consumer(g_data: Arc<Mutex<i32>>) -> i32 {
    println!("Consumer started...");

    // Seed thread random
    let mut rng = thread_rng();

    for _ in 0..5 {
        // Wait
        sleep(Duration::from_millis(rng.gen_range(0..1000)));

        // Consume
        consume(g_data.clone());
    }

    println!("Consumer finished!");

    0
}

fn consume(g_data: Arc<Mutex<i32>>) {
    // Lock
    // If the buffer is empty

    if g_data.is_poisoned() {
        // Wait for buffer to be filled

        println!("Consumer encountered empty buffer, waiting for producer to fill buffer...");
    }
    // If the buffer is full
    let mut data = loop {
        match g_data.try_lock() {
            Ok(inner) => break inner,
            Err(..) => (),
        }
    };
    //Show and empty buffer
    println!("Consumed {data}");
    *data = -1;

    // Unlock
    Mutex::unlock(data);
}

fn produce(g_data: Arc<Mutex<i32>>) {
    let mut rng = thread_rng();
    if g_data.is_poisoned() {
        println!("Producer encountered full buffer, waiting for consumer to empty buffer...");
    }

    // If the buffer is full
    let mut data = loop {
        match g_data.try_lock() {
            Ok(inner) => break inner,
            Err(..) => (),
        }
    };

    // Fill and show buffer
    *data = rng.gen_range(0..256);
    println!("Produced {data}");
}

fn producer(g_data: Arc<Mutex<i32>>) {
    println!("Producer started..");

    // Seed thread random
    let mut rng = thread_rng();

    // Produce
    for _ in 0..5 {
        // Wait
        sleep(Duration::from_millis(rng.gen_range(0..1000)));

        // Produce
        produce(g_data.clone());
    }

    println!("Producer finished!");
}

fn main() {
    let sdl_ctx = sdl2::init().unwrap();
    let video = sdl_ctx.video().unwrap();
    let mut event_pump = sdl_ctx.event_pump().unwrap();

    sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "1");
    sdl2::hint::set("SDL_VIDEO_X11_NET_WM_BYPASS_COMPOSITOR", "0");

    let window = video
        .window("SDL Tutorial 49", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let _image = sdl2::image::init(InitFlag::PNG).unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let creator = canvas.texture_creator();

    let splash_texture = LTexture::from_file(
        &creator,
        std::path::Path::new("./resources/lesson49/splash.png"),
    );

    // The "data buffer" in an "access semaphore"
    let g_data = Arc::new(Mutex::new(-1));
    let g_data_clone = g_data.clone();

    // rust-sdl2 not have support for threads, Us used rust threads becaus it
    // Run the thread
    std::thread::Builder::new()
        .name("Producer".to_string())
        .spawn(|| producer(g_data))
        .unwrap();

    let mut rng = thread_rng();
    sleep(Duration::from_millis(16 + rng.gen_range(0..32)));

    std::thread::Builder::new()
        .name("Consumer".to_string())
        .spawn(|| consumer(g_data_clone))
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
