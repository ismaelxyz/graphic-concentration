mod global;
mod object;
mod wrappers;

use crate::{
    global::{game_over, init, text_middle, Global, State},
    object::{Asteroids, Object, Ship, Size, Text},
};
use sdl2::{
    event::Event, image::LoadSurface, keyboard::Keycode, rect::Rect, surface::Surface,
    video::FullscreenType,
};

fn main() {
    // Setup Pre-Game Logic and Constants
    let icon = Surface::from_file("assets/img/icon.bmp").unwrap();
    let (context, mut win) = init();

    // Initialize TimerSubsystem
    let time = context.timer().unwrap();

    win.set_icon(icon);

    let mut event_pump = context
        .event_pump()
        .expect("Unable to obtain event pump handle!");
    win.set_fullscreen(FullscreenType::Desktop).unwrap();

    let mut global = Global::new(win.size());
    let screen = global.screen;

    let mut cv = win
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .expect("Unable to obtain canvas!");
    let creator = cv.texture_creator();

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

    let canvas = &mut cv;

    ship.position((screen.width / 2, screen.height / 2), canvas);
    global.timer.bullet = time.ticks();

    while !global.exit {
        global.timer.global = time.ticks();

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
        global.timer.game = time.ticks() / 1000; // TODO: El tiempo continua en estando de pausa.

        match global.state {
            State::Default => {
                if ship.lives < 1 {
                    global.exit = true;
                    continue;
                }

                ship.update((
                    screen,
                    global.speed,
                    &mut global.timer,
                    &event_pump,
                    canvas,
                    &time,
                ));
                asteroids.update(screen, &mut global.speed, canvas);
                global.collision(&mut ship, &mut asteroids);
                global.hud(&*ship, &font_large, canvas);
            }
            State::Pause => {
                text_middle(
                    Text::new(&font_large, "Pause", Size::Large, 1.0),
                    screen,
                    canvas,
                );
                global.hud(&*ship, &font_large, canvas);
            }
            State::Menu => {
                //
            }
            _ => unimplemented!(),
        }

        canvas.present();

        // Frames Per Second Delay
        global.delay(time.ticks(), &time);
    }

    /* Game Over */
    if ship.lives < 1 {
        game_over(&font_large, screen, canvas);
    }

    unsafe {
        sprite_sheet.destroy();
        font_large.destroy();
    }
}
