use sdl2::{
    event::Event,
    pixels::Color,
    rect::{Point, Rect},
};

const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;

fn main() {
    // Initialize SDL2
    let sdl_ctx = sdl2::init().unwrap();
    let video = sdl_ctx.video().unwrap();

    sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "1");
    sdl2::hint::set("SDL_VIDEO_X11_NET_WM_BYPASS_COMPOSITOR", "0");

    // Create the window
    let window = video
        .window("SDL Tutorial 08", 640, 480)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    // Set texture filtering to linear
    let mut canvas = window.into_canvas().build().unwrap();
    // Get a handle to the SDL2 event pump
    let mut event_pump = sdl_ctx.event_pump().unwrap();

    lazy_foo::setup_mainloop(-1, true, move || {
        // Extract any pending events from from the event pump and process them
        for event in event_pump.poll_iter() {
            // pattern match on the type of event
            if let Event::Quit { .. } = event {
                return false;
            }
        }

        // Set renderer color using the sdl_ctx
        canvas.set_draw_color(Color::WHITE);
        // Clear the screen
        canvas.clear();

        // Render red filled quad
        // Here's a bit of a dumb thing - Rect::new expects its
        // coords to be i32 - because you can have negative
        // positions - but its width and height to be u32, because
        // negative size isn't logical (though it can be useful).
        // Because HEIGHT and WIDTH are u32, we have to cast them
        // as i32 for the positions.  Note that casting is
        // unsafe if your SCREEN_HEIGHT / SCREEN_HEIGHT fall in the value range
        // between i32 and u32 (greater than 2,147,483,647)
        // but to date nobody's made a screen with a resolution
        // that large.
        let fill_rect = Rect::new(
            SCREEN_WIDTH as i32 / 4,
            SCREEN_HEIGHT as i32 / 4,
            SCREEN_WIDTH / 2,
            SCREEN_HEIGHT / 2,
        );
        canvas.set_draw_color(Color::RGB(0xff, 0, 0));
        canvas.fill_rect(fill_rect).unwrap();

        // Render green outlined quad
        let outline_rect = Rect::new(
            SCREEN_WIDTH as i32 / 6,
            SCREEN_HEIGHT as i32 / 6,
            (SCREEN_WIDTH * 2) / 3,
            (SCREEN_HEIGHT * 2) / 3,
        );

        canvas.set_draw_color(Color::GREEN);
        canvas.draw_rect(outline_rect).unwrap();
        // Draw Blue horizontal line
        canvas.set_draw_color(Color::BLUE);
        canvas
            .draw_line(
                Point::new(0, SCREEN_HEIGHT as i32 / 2),
                Point::new(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32 / 2),
            )
            .unwrap();

        // Draw vertical line of yellow dots
        canvas.set_draw_color(Color::RGB(0xff, 0xff, 0));
        for i in (0..SCREEN_HEIGHT as i32).step_by(4) {
            canvas
                .draw_point(Point::new(SCREEN_WIDTH as i32 / 2, i))
                .unwrap();
        }

        // Update the screen
        canvas.present();

        true
    });
}
