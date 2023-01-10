use sdl2::{
    event::Event,
    image::{InitFlag, LoadTexture},
    pixels::Color,
    rect::Rect,
};

fn main() {
    let sdl_ctx = sdl2::init().unwrap();
    let video = sdl_ctx.video().unwrap();

    sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "1");
    sdl2::hint::set("SDL_VIDEO_X11_NET_WM_BYPASS_COMPOSITOR", "0");

    // Create the window
    let window = video
        .window("SDL Tutorial 52", 640, 480)
        .position_centered()
        .resizable()
        .opengl()
        .build()
        .unwrap();

    let _image = sdl2::image::init(InitFlag::JPG | InitFlag::PNG).unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let creator = canvas.texture_creator();

    let bytes = include_bytes!("../resources/landscape.png");
    let landscape_texture = creator.load_texture_bytes(bytes).unwrap();

    let bytes = include_bytes!("../resources/portrait.png");
    let portrait_texture = creator.load_texture_bytes(bytes).unwrap();

    canvas.set_draw_color(Color::WHITE);

    // Get a handle to the SDL2 event pump
    let mut event_pump = sdl_ctx.event_pump().unwrap();

    main_loop::setup_mainloop(-1, true, move || {
        
        for event in event_pump.poll_iter() {

            if let Event::Quit { .. } = event {
                return false;
            }
        }

        let (width, height) = canvas.window().size();
        
        let mut texture = &landscape_texture;
        if width <= height {
            texture = &portrait_texture;
        }

        let query = texture.query();

        canvas.clear();
        canvas
            .copy(
                texture,
                None,
                Rect::new(
                    (width - query.width) as i32 / 2,
                    (height - query.height) as i32 / 2,
                    query.width,
                    query.height,
                ),
            )
            .unwrap();
        canvas.present();

        true
    });
}
