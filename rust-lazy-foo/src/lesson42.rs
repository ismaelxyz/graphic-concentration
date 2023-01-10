use sdl2::{
    event::Event,
    image::{InitFlag, LoadSurface},
    keyboard::Keycode,
    pixels::{Color, PixelFormatEnum},
    rect::Rect,
    render::Texture,
    surface::Surface,
};

/// A test animation stream
struct DataStream {
    /// Internal data
    images: Vec<Surface<'static>>,
    current_image: usize,
    delay_frames: usize,
}

impl DataStream {
    /// Initializes internals
    fn new() -> Self {
        DataStream {
            /// Internal data
            images: (0..4)
                .map(|i| {
                    let mut surface =
                        Surface::from_file(format!("resources/lesson42/foo_walk_{i}.png")).unwrap();
                    surface = surface.convert_format(PixelFormatEnum::RGBA8888).unwrap();
                    surface
                })
                .collect(),
            current_image: 0,
            delay_frames: 4,
        }
    }

    /// Gets current frame data
    fn buffer(&mut self) -> &Surface<'static> {
        self.delay_frames -= 1;
        if self.delay_frames == 0 {
            self.current_image += 1;
            self.delay_frames = 4;
        }

        if self.current_image == 4 {
            self.current_image = 0;
        }

        &self.images[self.current_image]
    }

    fn paste_pixels(&mut self, dest: &mut Texture) {
        let mut pixels = self.buffer().with_lock(|ps| ps.to_vec());

        dest.with_lock(None, move |trg_pixels, _| {
            // Color key pixels
            for (src_pixel, trg_pixel) in pixels.iter_mut().zip(trg_pixels.iter_mut()) {
                *trg_pixel = *src_pixel;
            }
        })
        .unwrap();
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

    let (screen_width, screen_height) = (640, 480);
    let window = video
        .window("SDL Tutorial 42", screen_width, screen_height)
        .position_centered()
        .opengl()
        .build()
        .expect("Could not create SDL window!");

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .expect("Unable to obtain canvas!");

    let creator = canvas.texture_creator();
    canvas.set_draw_color(Color::WHITE);

    let mut data_stream = DataStream::new();
    let (texture_width, texture_height) = (64, 205);
    let mut streaming_texture = creator
        .create_texture_streaming(PixelFormatEnum::RGBA8888, texture_width, texture_height)
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

        canvas.clear();
        data_stream.paste_pixels(&mut streaming_texture);

        // Render frame
        canvas
            .copy_ex(
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
            )
            .unwrap();

        canvas.present();
        true
    });
}
