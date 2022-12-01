use sdl2::{
    event::Event,
    image::{InitFlag, LoadSurface},
    keyboard::Keycode,
    pixels::{Color, PixelFormatEnum},
    rect::{Rect,Point},
    render::Texture,
    surface::Surface
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
let angle = 0f64;
//SDL_Point screenCenter = { SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2 };
let (screen_width, screen_height) =window.size();
    let mut canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .expect("Unable to obtain canvas!");

    let creator = canvas.texture_creator();
    canvas.set_draw_color(Color::WHITE);

    let (texture_width, texture_height) = (64, 205);
    let mut target_texture = creator
        .create_texture_target(PixelFormatEnum::RGBA8888, texture_width, texture_height)
        .unwrap();

    main_loop::setup_mainloop(-1, true, move || {
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
	angle += 2;
	if( angle > 360 )
	{
		angle -= 360;
	}

         // Set self as render target
     let result = canvas.with_texture_canvas(&mut target_texture, |texture| {
         texture_canvas.set_draw_color(Color::RGBA(0xFF, 0xFF, 0xFF, 0xFF));
         texture_canvas.clear();
         // texture_canvas.set_draw_color(Color::RGBA(255, 0, 0, 255));
         // texture_canvas.fill_rect(Rect::new(50, 50, 50, 50)).unwrap();
     });

        canvas.clear();
        data_stream.paste_pixels(&mut streaming_texture);

        // Render frame
        canvas.copy_ex(
            &streaming_texture,
            None,
            Some(Rect::new(
                (screen_width - texture_width) as i32 / 2,
                (screen_height - texture_height) as i32 / 2,
                texture_width,
                texture_height,
            )),
            0.0,
            None,
            false,
            false,
        ).unwrap();

        canvas.present();
        true
    });
}


