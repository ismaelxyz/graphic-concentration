use sdl2::{
    event::Event,
    image::{InitFlag, LoadSurface},
    mouse::MouseState,
    pixels::Color,
    rect::{Point, Rect},
    render::{Canvas, Texture, TextureCreator},
    surface::Surface,
    video::{Window, WindowContext},
};

use std::path::Path;

const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;
const TOTAL_BUTTONS: usize = 4;
const BUTTON_SPRITESHEET: &str = "resources/lesson17/button.png";

/// Rust enums are powerful, allowing you to create algebraic data types,
/// but in the simplest case they can be used like C enums.
/// We derive Copy and Clone for this enum so so we can pull the value out
/// of the struct for conversion into an array index.  If we don't, Rust
/// will try to pull the enum out, which leads to the common "cannot
/// move out of borrowed content" error.
#[derive(Copy, Clone)]
enum ButtonSprite {
    Out,
    OverMotion,
    Down,
    Up,
}

struct LTexture {
    texture: Texture,
    width: u32,
    height: u32,
}

impl LTexture {
    fn from_file(creator: &TextureCreator<WindowContext>, path: &Path) -> LTexture {
        // Load the surface first, so we can set the color key
        let mut surface = Surface::from_file(path).expect("Could not load surface from file!");

        // Now set the color key on the surface
        surface
            .set_color_key(true, Color::RGB(0, 0xff, 0xff))
            .expect("Could not set color_key on surface!");

        // Convert the surface to a texture and pass it to
        // LTexture::new to be wrapped
        let texture = creator
            .create_texture_from_surface(&surface)
            .expect("Could not create texture from surface!");

        LTexture {
            texture,
            width: surface.width(),
            height: surface.height(),
        }
    }

    fn render_to(&self, canvas: &mut Canvas<Window>, x: i32, y: i32, clip: Option<Rect>) {
        let clip_rect = match clip {
            Some(rect) => rect,
            None => Rect::new(0, 0, self.width, self.height),
        };

        canvas
            .copy_ex(
                &self.texture,
                Some(clip_rect),
                Some(Rect::new(x, y, clip_rect.width(), clip_rect.height())),
                0.0,
                None,
                false,
                false,
            )
            .expect("Could not blit texture to render target!");
    }
}

/// Create a struct that will be used to track mouse data
struct Button {
    /// Current position of the mouse
    position: Point,
    /// Currently used sprite
    current_sprite: ButtonSprite,
    pressed: bool,
}

impl Button {
    const WIDTH: u32 = 300;
    const HEIGHT: u32 = 200;

    /// Return a newly initialized Button (not really needed)
    fn new() -> Button {
        Button {
            position: Point::new(0, 0),
            current_sprite: ButtonSprite::Out,
            pressed: false,
        }
    }

    /// Create a new Button with an initial point
    fn new_from_point(p: Point) -> Button {
        Button {
            position: p,
            ..Self::new()
        }
    }

    /// Handle a mouse event.
    fn handle_event(&mut self, s: &MouseState) {
        // The LazyFoo tutorial uses the 'SDL_GetMouseState()' function to
        // obtain the x, y coordinates.  That would require passing the event pump
        // into the function, so we simplify things a little bit by acquiring the
        // mouse state in the main loop and passing in the state to handle_event

        // Check to see if the mouse is inside the button
        if (s.x() < self.position.x())
            || (s.x() > self.position.x() + Button::WIDTH as i32)
            || (s.y() < self.position.y())
            || (s.y() > self.position.y() + Button::HEIGHT as i32)
        {
            self.current_sprite = ButtonSprite::Out;
        } else {
            self.current_sprite = match s.left() {
                true => {
                    self.pressed = true;
                    ButtonSprite::Down
                }
                false => {
                    if self.pressed {
                        ButtonSprite::Up
                    } else {
                        ButtonSprite::OverMotion
                    }
                }
            }
        }
    }

    /// Render a button.  In order to do this, we need the SDL context
    /// as well as the LTexture for the button.
    fn render(&self, canvas: &mut Canvas<Window>, texture: &LTexture, clips: &[Rect]) {
        // This is why we need to derive the Copy trait for the enum.
        let indx = self.current_sprite as usize;
        texture.render_to(
            canvas,
            self.position.x(),
            self.position.y(),
            Some(clips[indx]),
        );
    }
}

fn main() {
    let sdl_ctx = sdl2::init().expect("Unable to initialize SDL!");
    let video = sdl_ctx.video().expect("Could not acquire video context!");
    sdl2::image::init(InitFlag::PNG).expect("Unable to initialize sdl2_image!");

    sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "1");
    sdl2::hint::set("SDL_VIDEO_X11_NET_WM_BYPASS_COMPOSITOR", "0");
    let window = video
        .window("SDL Tutorial 17", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .expect("Could not create SDL window!");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Unable to obtain canvas!");

    let creator = canvas.texture_creator();
    // Load the button sprite
    let button_texture = LTexture::from_file(&creator, Path::new(BUTTON_SPRITESHEET));
    let clip_rects = (0..TOTAL_BUTTONS)
        .map(|i| Rect::new(0, i as i32 * 200, Button::WIDTH, Button::HEIGHT))
        .collect::<Vec<Rect>>();
    // We will create the buttons here.  We will use an array instead of a vec because
    // it makes no difference to the program, but is illustrative.
    let mut buttons: [Button; TOTAL_BUTTONS] = [
        Button::new(),
        Button::new_from_point(Point::new((SCREEN_WIDTH - Button::WIDTH) as i32, 0)),
        Button::new_from_point(Point::new(0, (SCREEN_HEIGHT - Button::HEIGHT) as i32)),
        Button::new_from_point(Point::new(
            (SCREEN_WIDTH - Button::WIDTH) as i32,
            (SCREEN_HEIGHT - Button::HEIGHT) as i32,
        )),
    ];
    let mut event_pump = sdl_ctx
        .event_pump()
        .expect("Unable to obtain event pump handle!");

    main_loop::setup_mainloop(-1, true, move || {
        for event in event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                // Pass off the events to the buttons for handling.
                // Note that unlike the tutorial, we actually check it's
                // a mouse event before handing it off.  Otherwise in an
                // actual program we'd be sending non-mouse events into limbo.
                // UPDATE: This is the old way of doing it.  Now mouse events
                // are not generated events - instead you pull the status of
                // the mouse right from the event pump (see below).
                return false;
            }
        }

        // Check the mouse state, & dispatch it to the buttons
        let state = event_pump.mouse_state();
        buttons
            .iter_mut()
            .for_each(|button| button.handle_event(&state));

        canvas.set_draw_color(Color::WHITE);
        canvas.clear();

        // Render the buttons
        // We don't have globals and Button does not store the button texture,
        // so we need to pass it and the context.
        for button in buttons.iter_mut() {
            button.render(&mut canvas, &button_texture, &clip_rects);
        }

        canvas.present();

        true
    });
}
