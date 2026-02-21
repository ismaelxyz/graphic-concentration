mod global;
mod object;
mod wrappers;

use crate::{
    global::{Global, State, game_over, init, text_middle},
    object::{Asteroids, Object, Ship, Size, Text},
};
use sdl3::{event::Event, image::LoadSurface, keyboard::Keycode, rect::Rect, surface::Surface};

fn main() {
    // Setup Pre-Game Logic and Constants
    let icon = Surface::from_file("assets/img/icon.bmp").unwrap();
    let (context, mut win) = init();

    win.set_icon(icon);

    let mut event_pump = context
        .event_pump()
        .expect("unable to obtain event pump handle!");

    win.set_fullscreen(true)
        .expect("unable to set fullscreen mode!");

    let mut global = Global::new(win.size());
    let screen = global.screen;

    let mut canvas = win.into_canvas();
    let creator = canvas.texture_creator();

    let sprite_sheet =
        wrappers::load_texture("assets/img/sprite-sheet.bmp", &creator, (0x0, 0x0, 0x0));
    let font_large = wrappers::load_texture("assets/fonts/fontx64.bmp", &creator, (0x0, 0x0, 0x0));

    let mut asteroids = Asteroids::new(&sprite_sheet);
    let mut ship = Object::new(
        Ship {
            lives: 3,
            bullets: Vec::new(),
        },
        &sprite_sheet,
        0,
        Rect::new(0, 0, 32, 32),
        1.0,
    );

    let canvas = &mut canvas;

    ship.position((screen.width / 2, screen.height / 2), canvas);
    global.timer.bullet = sdl3::timer::ticks();

    while !global.exit {
        let frame_start = sdl3::timer::ticks();
        global.timer.global = frame_start;

        canvas.clear();

        // Extract any pending events from from the event pump and process them
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => global.exit = true,
                Event::KeyDown {
                    repeat: false,
                    keycode: Some(kode),
                    ..
                } => match kode {
                    Keycode::P => {
                        global.state = if global.state == State::Default {
                            State::Pause
                        } else {
                            State::Default
                        };
                    }
                    Keycode::Escape => global.exit = true,
                    _ => (),
                },
                _ => (),
            }
        }

        /* Load Event Stack */
        event_pump.pump_events();

        /* Have a function to update all timers */
        global.timer.game = sdl3::timer::ticks() / 1000; // TODO: El tiempo continua en estando de pausa.

        match global.state {
            State::Default => {
                if ship.lives < 1 {
                    global.exit = true;
                    continue;
                }

                ship.update((screen, global.speed, &mut global.timer, &event_pump, canvas));
                asteroids.update(screen, &mut global.speed, canvas);
                global.collision(&mut ship, &mut asteroids);
                global.hud(&ship, &font_large, canvas);
            }
            State::Pause => {
                text_middle(
                    Text::new(&font_large, "Pause", Size::Large, 1.0),
                    screen,
                    canvas,
                );
                global.hud(&ship, &font_large, canvas);
            }
            State::Menu => {
                //
            }
            _ => unimplemented!(),
        }

        canvas.present();

        // Frames Per Second Delay
        global.delay(frame_start);
    }

    /* Game Over */
    if ship.lives < 1 {
        game_over(&font_large, screen, canvas);
    }
}
