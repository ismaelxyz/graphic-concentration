use sdl2::{
    event::Event,
    image::{InitFlag, LoadSurface},
    keyboard::Keycode,
    pixels::{Color, PixelFormat, PixelFormatEnum},
    rect::Rect,
    render::{Canvas, Texture, TextureCreator},
    surface::Surface,
    video::{Window, WindowContext},
};

pub struct LTexture {
    texture: Texture,
    width: u32,
    height: u32,
}

impl LTexture {
    fn from_file(
        path: &str,
        creator: &TextureCreator<WindowContext>,
        pixel_format: PixelFormatEnum,
    ) -> LTexture {
        let mut surface = Surface::from_file(path).unwrap();
        surface = surface.convert_format(pixel_format).unwrap();

        let (width, height) = surface.size();

        let pixel_format = PixelFormat::try_from(pixel_format).unwrap();
        // Map colors
        let color_key = Color::RGB(0, 0xFF, 0xFF).to_u32(&pixel_format);
        let transparent = Color::RGBA(0xFF, 0xFF, 0xFF, 0x00).to_u32(&pixel_format);

        surface.with_lock_mut(move |pixels| {
            let pixels = unsafe {
                &mut *core::ptr::slice_from_raw_parts_mut(
                    pixels as *mut [u8] as *mut u32,
                    (width * height) as usize,
                )
            };

            // Color key pixels
            for pixel in pixels.iter_mut() {
                if *pixel == color_key {
                    *pixel = transparent;
                }
            }
        });

        LTexture {
            texture: surface.as_texture(creator).unwrap(),
            width,
            height,
        }
    }

    fn render(&self, canvas: &mut Canvas<Window>, x: i32, y: i32) {
        canvas
            .copy_ex(
                &self.texture,
                Some(Rect::new(0, 0, self.width, self.height)),
                Some(Rect::new(x, y, self.width, self.height)),
                0.0,
                None,
                false,
                false,
            )
            .expect("Could not blit texture to render target!");
    }
}

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
        .window("SDL Tutorial 40", 640, 480)
        .position_centered()
        .opengl()
        .build()
        .expect("Could not create SDL window!");

    let pixel_format = window.window_pixel_format();
    let (width, height) = window.size();
    let mut canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .expect("Unable to obtain canvas!");

    let creator = canvas.texture_creator();
    canvas.set_draw_color(Color::WHITE);

    let stick_texture = LTexture::from_file("resources/lesson40/foo.png", &creator, pixel_format);

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

        canvas.clear();
        // Render stick figure
        stick_texture.render(
            &mut canvas,
            ((width - stick_texture.width) / 2) as i32,
            ((height - stick_texture.height) / 2) as i32,
        );

        canvas.present();
        true
    });
}
