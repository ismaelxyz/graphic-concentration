use crate::object::{Asteroids, Object, Ship, Size, Text};
use sdl2::{
    pixels::Color,
    rect::Rect,
    render::{Texture, WindowCanvas},
    video::Window,
};

pub(crate) fn rand() -> i32 {
    extern "C" {
        fn rand() -> i32;
    }

    unsafe { rand() }
}

fn srand() -> i32 {
    extern "C" {
        fn time(_: i64) -> i64;
        fn srand(_: i64) -> i32;
    }

    unsafe { srand(time(0)) }
}

pub fn init() -> (sdl2::Sdl, Window) {
    let sdl = sdl2::init().expect("Unable to initialize SDL!");
    let video = sdl.video().expect("Could not acquire video context!");

    sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "1");
    sdl2::hint::set("SDL_VIDEO_X11_NET_WM_BYPASS_COMPOSITOR", "0");

    let build_win = video
        .window("Asterlike", 500, 800)
        .position_centered()
        .opengl()
        .build()
        .expect("Could not create SDL window!");

    srand(); // Voy aquí?
    (sdl, build_win)
}

#[derive(Copy, Clone)]
pub struct Speed {
    pub ship: f32,
    pub asteroid: f32,
    pub bullet: f32,
}

#[derive(Copy, Clone)]
pub struct Screen {
    pub width: i32,
    pub height: i32,
    pub top: f32,
    pub bottom: f32,
    pub left: f32,
    pub right: f32,
}

/// Game enumerated states
#[derive(Eq, PartialEq)]
pub enum State {
    Default,
    CutScene,
    Menu,
    Wave,
    Pause,
}

#[derive(Default)]
crate struct Timer {
    crate global: u32,
    crate bullet: u32,
    crate game: u32,
}

crate struct Global {
    crate speed: Speed,
    crate screen: Screen,
    crate frames_per_second: f32,
    crate tick_ratio: f32,
    crate exit: bool,
    crate state: State,
    crate timer: Timer,
    crate score: u32,
}

impl Global {
    pub fn new() -> Self {
        let frames_per_second = 60.0;
        let tick_ratio = 60.0 / frames_per_second;

        // Constant Logic / initialize
        Global {
            speed: Speed {
                ship: 7.5 * tick_ratio,
                asteroid: 1.5 * tick_ratio,
                bullet: 9.0 * tick_ratio,
            },
            screen: Screen {
                width: 1920,
                height: 1080,
                top: 64.0,
                bottom: 5.0,
                left: 5.0,
                right: 5.0,
            },

            // Make this an option
            frames_per_second,
            tick_ratio,

            /* Initialize Keystates */
            // Global->keystates = SDL_GetKeyboardState(NULL);
            exit: false,
            state: State::Default,
            timer: Timer::default(),

            // Global Score
            score: 0,
        }
    }
}

// static STATIC: Type = init;

/// Display text in middle of the screen
pub fn text_middle(mut text: Text, screen: Screen, canvas: &mut WindowCanvas) {
    let chucks = text.chucks();
    let clip = chucks[0].clip();
    let len = chucks.len() as f32 * clip.w as f32 * 0.5 * chucks[0].scale();

    text.position(
        (
            (((screen.width as f32 - screen.right) / 2.0 + screen.left) - len / 2.0) as i32,
            ((((screen.height as f32 - screen.bottom) / 2.0) + screen.top) - clip.h as f32 / 2.0)
                as i32,
        ),
        canvas,
    );
}

/// Display the game over message
pub fn game_over(font: &Texture, screen: Screen, canvas: &mut WindowCanvas) {
    canvas.clear();
    text_middle(
        Text::new(font, "Game Over", Size::Large, 1.0),
        screen,
        canvas,
    );
    canvas.present();
    std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 * 2));
}

/// Display the user's heads up display
crate fn hud(ship: &Ship, font: &Texture, global: &Global, canvas: &mut WindowCanvas) {
    let Global { screen, score, .. } = global;

    /* Set the HUD bar */
    let bar = Rect::new(0, 0, screen.width as u32, screen.top as u32);
    let mut data = score.to_string();

    canvas.set_draw_color(Color::from((0, 51, 102, 0xFF)));
    canvas.fill_rect(bar);
    canvas.set_draw_color(Color::from((0x0, 0x0, 0x0, 0xFF)));
    let mut tmp = Text::new(font, "Score".into(), Size::Large, 1.0);

    // Display score number
    let mut previous = {
        tmp.position((0, 0), canvas);

        let obj = &tmp.chucks()[0];
        let len = (tmp.chucks().len() + 2 - data.len()) as f32;
        obj.pos.0 as f32 + (len * obj.clip.w as f32 * 0.5 * obj.scale)
    } as i32;

    tmp = Text::new(font, &data, Size::Large, 1.0);
    tmp.position((previous, 0), canvas);

    // Display lives text
    let mut obj = &tmp.chucks()[0];
    previous += ((tmp.chucks().len() + 3) as f32 * obj.clip.w as f32 * 0.5 * obj.scale) as i32;

    tmp = Text::new(font, "Lives", Size::Large, 1.0);
    tmp.position((previous, 0), canvas);

    // Display lives number
    data = ship.lives.to_string();
    obj = &tmp.chucks()[0];

    previous = {
        let len = (tmp.chucks().len() + 3 - data.len()) as f32;
        obj.pos.0 as f32 + (len * obj.clip.w as f32 * 0.5 * obj.scale)
    } as i32;

    Text::new(font, &data, Size::Large, 1.0).position((previous, 0), canvas);

    // Display Timer
    tmp = Text::new(font, &global.timer.game.to_string()[..], Size::Large, 1.0);
    let (w, len) = (global.screen.width as usize, tmp.chucks().len());
    tmp.position(
        ((w - (len * tmp.chucks()[0].clip.w as usize)) as i32, 0),
        canvas,
    );
}

crate fn collision(ship: &Ship, asteroids: &Asteroids) {}
