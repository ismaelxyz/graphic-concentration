use sdl2::{
    event::Event,
    image::{InitFlag, LoadSurface, Sdl2ImageContext},
    pixels::Color,
    rect::Rect,
    render::{Canvas, RenderTarget, Texture, TextureCreator},
    surface::Surface,
    video::Window,
    Sdl,
};
use std::path::Path;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;
const IMG_DOTS: &str = "resources/dots.png";

// Create a struct that will track texture data
struct LTexture<'a> {
    // The actual texture.
    texture: Texture<'a>,
    // Image dimensions
    width: u32,
    height: u32,
}

// Implement a few functions for the Texture struct
// Note that Rust doesn't put much focus on data hiding
// or other OOP concepts so we don't care about having
// getters and setters or the like.
//
// Instead, since Rust treats values as immutable by
// default, we don't have to worry about accidentally
// setting a struct field unless we create an LTexture
// using 'mut', in which case we take on the responsibility
// of ensuring the fields don't get messed with.
//
// This 'hands off' by default approach helps eliminate
// a lot of problems that, in OOP, are handled by boilerplate code.
// The result is cleaner, more consise and ultimately more safe.
impl<'a> LTexture<'a> {
    // create a new texture
    fn new(tex: Texture<'a>) -> LTexture {
        let w = tex.query().width;
        let h = tex.query().height;
        LTexture {
            texture: tex,
            width: w,
            height: h,
        }
    }

    // Load a texture from a file
    fn new_from_file<T>(ren: &'a TextureCreator<T>, path: &Path) -> LTexture<'a> {
        // Load the surface first, so we can set the color key
        let mut surface = match Surface::from_file(path) {
            Ok(surface) => surface,
            Err(err) => panic!("Could not load surface: {err}"),
        };

        // Now set the color key on the surface
        surface
            .set_color_key(true, Color::RGB(0, 0xff, 0xff))
            .unwrap();

        // Convert the surface to a texture and pass it to
        // LTexture::new to be wrapped
        let tex = match ren.create_texture_from_surface(&surface) {
            Ok(texture) => texture,
            Err(err) => panic!("Could not convert surface to texture: {err}"),
        };
        LTexture::new(tex)
    }

    // Renders a texture to a given point using a provided renderer
    fn render_to<T: RenderTarget>(
        &self,
        canvas: &mut Canvas<T>,
        x: i32,
        y: i32,
        clip: Option<Rect>,
    ) {
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

// Note that 'creator.load_texture' makes this example trivial.  See lesson03
// to show how we can manually load a surface and convert it to a texture.

// Break out initialization into a separate function, which
// returns only the Window (we don't need the sdl_context)
fn init() -> (Sdl, Window, Sdl2ImageContext) {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let win = match video
        .window("SDL Tutorial 11", WIDTH, HEIGHT)
        .position_centered()
        .opengl()
        .build()
    {
        Ok(window) => window,
        Err(err) => panic!("Failed to create Window!: {err}"),
    };

    let image = sdl2::image::init(InitFlag::PNG).unwrap();

    (sdl, win, image)
}

// LoadMedia function
//
// We want to avoid the use of global variables (it's not really
// a Rust, or functional, idiom) so we return a tuple containing
// the data
fn load_media<'a, T>(
    renderer: &'a TextureCreator<T>,
    path: &'a std::path::Path,
) -> (LTexture<'a>, [Rect; 4]) {
    // Return the teuple
    (
        LTexture::new_from_file(renderer, path),
        [
            Rect::new(0, 0, 100, 100),
            Rect::new(100, 0, 100, 100),
            Rect::new(0, 100, 100, 100),
            Rect::new(100, 100, 100, 100),
        ],
    )
}

fn main() {
    // Initialize SDL2
    let (context, window, _image) = init();

    // Obtain the canvas
    let mut canvas = match window.into_canvas().build() {
        Ok(canvas) => canvas,
        Err(err) => panic!("Could not obtain canvas: {err}"),
    };

    let creator = canvas.texture_creator();
    // Create the textures we are going to use.
    let (sprite_sheet, sprite_clips) = load_media(&creator, Path::new(IMG_DOTS));
    // Get a handle to the SDL2 event pump
    let mut event_pump = context.event_pump().unwrap();

    'running: loop {
        // Extract any pending events from from the event pump and process them
        for event in event_pump.poll_iter() {
            // pattern match on the type of event
            if let Event::Quit { .. } = event {
                break 'running;
            }
        }
        // Clear and render the texture each pass through the loop
        canvas.set_draw_color(Color::RGB(0xff, 0xff, 0xff));
        canvas.clear();

        // Render top left sprite
        sprite_sheet.render_to(&mut canvas, 0, 0, Some(sprite_clips[0]));

        // Render top right sprite
        sprite_sheet.render_to(
            &mut canvas,
            (WIDTH - sprite_clips[1].width()) as i32,
            0,
            Some(sprite_clips[1]),
        );

        // Render bottom left sprite
        sprite_sheet.render_to(
            &mut canvas,
            0,
            (HEIGHT - sprite_clips[2].height()) as i32,
            Some(sprite_clips[2]),
        );

        // Render bottom right sprite
        sprite_sheet.render_to(
            &mut canvas,
            (WIDTH - sprite_clips[3].width()) as i32,
            (HEIGHT - sprite_clips[3].height()) as i32,
            Some(sprite_clips[3]),
        );

        // Update the screen
        canvas.present();
    }
}
