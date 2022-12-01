use sdl2::{
    event::Event,
    image::{InitFlag, LoadSurface},
    pixels::Color,
    rect::{Point, Rect},
    render::{Canvas, Texture, TextureCreator},
    surface::Surface,
    video::{Window, WindowContext},
};

/// Create a struct that will track texture data
struct LTexture {
    /// The actual texture.
    texture: Texture,
    // Image dimensions
    width: u32,
    height: u32,
}

/// Implement a few functions for the Texture struct
/// Note that Rust doesn't put much focus on data hiding
/// or other OOP concepts so we don't care about having
/// getters and setters or the like.
///
/// Instead, since Rust treats values as immutable by
/// default, we don't have to worry about accidentally
/// setting a struct field unless we create an LTexture
/// using 'mut', in which case we take on the responsibility
/// of ensuring the fields don't get messed with.
///
/// This 'hands off' by default approach helps eliminate
/// a lot of problems that, in OOP, are handled by boilerplate code.
/// The result is cleaner, more consise and ultimately more safe.
impl LTexture {
    /// Load a texture from a file
    fn from_file(creator: &TextureCreator<WindowContext>, name: &str) -> LTexture {
        // Load the surface first, so we can set the color key
        let mut surface = Surface::from_file(&format!("./resources/lesson10/{name}.png")).unwrap();

        // Now set the color key on the surface
        surface
            .set_color_key(true, Color::RGB(0, 0xff, 0xff))
            .unwrap();

        // Convert the surface to a texture and pass it to
        let texture = creator.create_texture_from_surface(&surface).unwrap();

        LTexture {
            texture,
            width: surface.width(),
            height: surface.height(),
        }
    }

    /// Renders a texture to a given point using a provided canvas
    fn render_to(&self, canvas: &mut Canvas<Window>, p: Option<Point>) {
        canvas
            .copy(
                &self.texture,
                None,
                Some(Rect::new(
                    p.map(|p| p.x()).unwrap_or_default(),
                    p.map(|p| p.y()).unwrap_or_default(),
                    self.width,
                    self.height,
                )),
            )
            .unwrap();
    }
}

fn main() {
    // Initialize SDL2
    let sdl_ctx = sdl2::init().unwrap();
    let video = sdl_ctx.video().unwrap();

    sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "1");
    sdl2::hint::set("SDL_VIDEO_X11_NET_WM_BYPASS_COMPOSITOR", "0");

    let window = video
        .window("SDL Tutorial 10", 640, 480)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let _image = sdl2::image::init(InitFlag::PNG).unwrap();

    // Obtain the canvas
    let mut canvas = window.into_canvas().build().unwrap();
    let creator = canvas.texture_creator();

    // Create the textures we are going to use.
    let foo_texture = LTexture::from_file(&creator, "foo");
    let background_texture = LTexture::from_file(&creator, "background");

    // Set renderer color using the context
    canvas.set_draw_color(Color::BLACK);

    // Get a handle to the SDL2 event pump
    let mut event_pump = sdl_ctx.event_pump().unwrap();

    main_loop::setup_mainloop(-1, true, move || {
        // Extract any pending events from from the event pump and process them
        for event in event_pump.poll_iter() {
            // pattern match on the type of event
            if let Event::Quit { .. } = event {
                return false;
            }
        }

        // Clear and render the texture each pass through the loop
        canvas.clear();
        background_texture.render_to(&mut canvas, None);
        foo_texture.render_to(&mut canvas, Some(Point::new(240, 190)));
        canvas.present();

        true
    });
}
