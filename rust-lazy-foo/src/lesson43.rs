use sdl2::{
    event::Event,
    image::InitFlag,
    keyboard::Keycode,
    pixels::{Color, PixelFormatEnum},
    rect::{Point, Rect},
};

fn main() {
    let sdl_ctx = sdl2::init().expect("Unable to initialize SDL!");
    let video = sdl_ctx.video().expect("Could not acquire video context!");
    sdl2::image::init(InitFlag::PNG).expect("Unable to initialize sdl2_image!");
    let mut event_pump = sdl_ctx
        .event_pump()
        .expect("Unable to obtain event pump handle!");

    sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "1");
    sdl2::hint::set("SDL_VIDEO_X11_NET_WM_BYPASS_COMPOSITOR", "0");

    let window = video
        .window("SDL Tutorial 43", 640, 480)
        .position_centered()
        .opengl()
        .build()
        .expect("Could not create SDL window!");
    //Rotation variables
    let mut angle = 0f64;
    let (screen_width, screen_height) = window.size();
    let screen_center = Point::new(screen_width as i32 / 2, screen_height as i32 / 2);
    let mut canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .expect("Unable to obtain canvas!");

    let creator = canvas.texture_creator();

    let mut target_texture = creator
        .create_texture_target(PixelFormatEnum::RGBA8888, screen_width, screen_height)
        .unwrap();

    lazy_foo::setup_mainloop(-1, true, move || {
        for event in event_pump.poll_iter() {
            if let Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } = event
            {
                return false;
            }
        }

        // rotate
        angle += 2.0;
        if angle > 360.0 {
            angle -= 360.0;
        }

        canvas.set_draw_color(Color::WHITE);
        canvas.clear();

        // Set self as render target
        canvas
            .with_texture_canvas(&mut target_texture, |canvas| {
                canvas.set_draw_color(Color::RGBA(0xFF, 0xFF, 0xFF, 0xFF));
                canvas.clear();

                // Render red filled quad
                let fill_rect = Rect::new(
                    screen_width as i32 / 4,
                    screen_height as i32 / 4,
                    screen_width / 2,
                    screen_height / 2,
                );
                canvas.set_draw_color(Color::RGBA(255, 0, 0, 255));
                canvas.fill_rect(fill_rect).unwrap();

                // Render green outlined quad
                let outline_rect = Rect::new(
                    screen_width as i32 / 6,
                    screen_height as i32 / 6,
                    screen_width * 2 / 3,
                    screen_height * 2 / 3,
                );
                canvas.set_draw_color(Color::RGBA(0, 255, 0, 255));
                canvas.draw_rect(outline_rect).unwrap();

                // Draw blue horizontal line
                canvas.set_draw_color(Color::RGBA(0, 0, 255, 255));
                canvas
                    .draw_line(
                        (0, screen_height as i32 / 2),
                        (screen_width as i32, screen_height as i32 / 2),
                    )
                    .unwrap();

                // Draw vertical line of yellow dots
                canvas.set_draw_color(Color::RGBA(255, 255, 0, 255));
                for i in (0..screen_height as i32).step_by(4) {
                    canvas.draw_point((screen_width as i32 / 2, i)).unwrap();
                }
            })
            .unwrap();

        // Show rendered to texture
        let width = target_texture.query().width;
        let height = target_texture.query().height;
        canvas
            .copy_ex(
                &target_texture,
                None,
                Some(Rect::new(0, 0, width, height)),
                angle,
                screen_center,
                false,
                false,
            )
            .unwrap();

        canvas.present();
        true
    });
}
