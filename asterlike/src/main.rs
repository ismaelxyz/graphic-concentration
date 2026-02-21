mod global;
mod object;
mod wrappers;

use crate::{
    global::{Global, State, game_over, init, text_middle},
    object::{Asteroids, Object, Ship, Size, Text},
};
use sdl3::{
    event::{Event, WindowEvent},
    image::LoadSurface,
    keyboard::Keycode,
    rect::Rect,
    render::{Texture, WindowCanvas},
    surface::Surface,
};

#[cfg(target_os = "emscripten")]
mod emscripten {
    use core::ffi::c_void;

    unsafe extern "C" {
        pub fn emscripten_set_main_loop_arg(
            func: extern "C" fn(*mut c_void),
            arg: *mut c_void,
            fps: i32,
            simulate_infinite_loop: i32,
        );

        pub fn emscripten_cancel_main_loop();
    }
}

struct Game {
    _context: sdl3::Sdl,
    event_pump: sdl3::EventPump,
    global: Global,
    canvas: WindowCanvas,
    sprite_sheet: &'static Texture,
    font_large: &'static Texture,
    asteroids: Asteroids<'static>,
    ship: Object<'static, Ship<'static>>,
}

impl Game {
    fn new() -> Self {
        // Setup Pre-Game Logic and Constants
        let icon = Surface::from_file("assets/img/icon.bmp").unwrap();
        let (context, mut win) = init();

        win.set_icon(icon);

        let event_pump = context
            .event_pump()
            .expect("unable to obtain event pump handle!");

        // Fullscreen on the web requires a user gesture; don't hard-fail.
        #[cfg(target_os = "emscripten")]
        {
            let _ = win.set_fullscreen(true);
        }
        #[cfg(not(target_os = "emscripten"))]
        {
            win.set_fullscreen(true)
                .expect("unable to set fullscreen mode!");
        }

        let mut global = Global::new(win.size());
        let mut canvas = win.into_canvas();

        // Sync to the renderer's real output size (important for fullscreen / HiDPI).
        if let Ok(size) = canvas.output_size() {
            global.update_screen_size(size);
        }

        let creator = canvas.texture_creator();

        // Leak textures so we can keep stable references inside game objects.
        let sprite_sheet: &'static Texture = Box::leak(Box::new(wrappers::load_texture(
            "assets/img/sprite-sheet.bmp",
            &creator,
            (0x0, 0x0, 0x0),
        )));
        let font_large: &'static Texture = Box::leak(Box::new(wrappers::load_texture(
            "assets/fonts/fontx64.bmp",
            &creator,
            (0x0, 0x0, 0x0),
        )));

        let asteroids = Asteroids::new(sprite_sheet);
        let mut ship = Object::new(
            Ship {
                lives: 3,
                bullets: Vec::new(),
            },
            sprite_sheet,
            0,
            Rect::new(0, 0, 32, 32),
            1.0,
        );

        let screen = global.screen;
        ship.position((screen.width / 2, screen.height / 2), &mut canvas);
        global.timer.bullet = sdl3::timer::ticks();

        Game {
            _context: context,
            event_pump,
            global,
            canvas,
            sprite_sheet,
            font_large,
            asteroids,
            ship,
        }
    }

    fn tick(&mut self) {
        let frame_start = sdl3::timer::ticks();
        self.global.timer.global = frame_start;

        self.canvas.clear();

        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => self.global.exit = true,
                Event::Window { win_event, .. } => match win_event {
                    WindowEvent::Resized(w, h) | WindowEvent::PixelSizeChanged(w, h) => {
                        let w = u32::try_from(w).unwrap_or(1).max(1);
                        let h = u32::try_from(h).unwrap_or(1).max(1);
                        self.global.update_screen_size((w, h));
                    }
                    _ => (),
                },
                Event::KeyDown {
                    repeat: false,
                    keycode: Some(kode),
                    ..
                } => match kode {
                    Keycode::P => {
                        self.global.state = if self.global.state == State::Default {
                            State::Pause
                        } else {
                            State::Default
                        };
                    }
                    Keycode::Escape => self.global.exit = true,
                    _ => (),
                },
                _ => (),
            }
        }

        self.event_pump.pump_events();
        self.global.timer.game = sdl3::timer::ticks() / 1000;

        match self.global.state {
            State::Default => {
                if self.ship.lives < 1 {
                    self.global.exit = true;
                } else {
                    let screen = self.global.screen;
                    self.ship.update((
                        screen,
                        self.global.speed,
                        &mut self.global.timer,
                        &self.event_pump,
                        &mut self.canvas,
                    ));
                    self.asteroids
                        .update(screen, &mut self.global.speed, &mut self.canvas);
                    self.global.collision(&mut self.ship, &mut self.asteroids);
                    self.global
                        .hud(&self.ship, self.font_large, &mut self.canvas);
                }
            }
            State::Pause => {
                let screen = self.global.screen;
                let text = Text::new(self.font_large, "Pause", Size::Large, 1.0);
                text_middle(text, screen, &mut self.canvas);
                self.global
                    .hud(&self.ship, self.font_large, &mut self.canvas);
            }
            State::Menu => {
                //
            }
            _ => unimplemented!(),
        }

        self.canvas.present();
        self.global.delay(frame_start);

        if self.global.exit && self.ship.lives < 1 {
            game_over(self.font_large, self.global.screen, &mut self.canvas);
        }
    }
}

fn main() {
    #[cfg(target_os = "emscripten")]
    {
        use core::ffi::c_void;

        extern "C" fn tick(arg: *mut c_void) {
            let game = unsafe { &mut *(arg as *mut Game) };
            game.tick();
            if game.global.exit {
                unsafe { crate::emscripten::emscripten_cancel_main_loop() };
            }
        }

        let game = Box::new(Game::new());
        let game_ptr = Box::into_raw(game) as *mut c_void;

        unsafe {
            // fps = 0 => requestAnimationFrame
            // simulate_infinite_loop = 1 => the call never returns (emscripten semantics)
            crate::emscripten::emscripten_set_main_loop_arg(tick, game_ptr, 0, 1);
        }

        unreachable!();
    }

    #[cfg(not(target_os = "emscripten"))]
    {
        let mut game = Game::new();
        while !game.global.exit {
            game.tick();
        }
    }
}
