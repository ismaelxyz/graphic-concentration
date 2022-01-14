#![feature(crate_visibility_modifier)]
#![allow(unused, dead_code)]
mod global;
mod object;
mod wrappers;

// text::Text
use crate::{
    global::{collision, game_over, hud, init, text_middle, Global, State},
    object::{Asteroids, Object, Ship, Size, Text},
};
use sdl2::{
    event::Event, image::LoadSurface, keyboard::Keycode, rect::Rect, surface::Surface,
    video::FullscreenType,
};
//use std::time::Duration;

fn main() {
    // Setup Pre-Game Logic and Constants
    let icon = Surface::from_file("assets/img/icon.bmp").unwrap();
    let (context, mut win) = init();

    // Initialize TimerSubsystem
    let mut time = context.timer().unwrap();
    let mut global = Global::new();

    win.set_icon(icon);

    let mut event_pump = context
        .event_pump()
        .expect("Unable to obtain event pump handle!");
    win.set_fullscreen(FullscreenType::Desktop);
    let (w, h) = win.size();
    global.screen.width = w as i32;
    global.screen.height = h as i32;
    let screen = global.screen;

    let mut canvas = win
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .expect("Unable to obtain canvas!");
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

    ship.position(
        (
            (((screen.width as f32 - screen.right) / 2. + (ship.clip().w / 2) as f32) + screen.left)
                as i32,
            (((screen.height as f32 - screen.top) * 0.75 + (ship.clip().h / 2) as f32)
                + screen.bottom) as i32,
        ),
        &mut canvas,
    );

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

        hud(&*ship, &font_large, &global, &mut canvas);
        /* Have a function to update all timers */
        global.timer.game = time.ticks() / 1000; // TODO: El tiempo continua en estando de pausa.

        match global.state {
            State::Default => {
                if ship.lives < 1 {
                    global.exit = true;
                    continue;
                }

                ship.update();
                asteroids.update(screen, &mut global.speed, &mut canvas);
                collision(&ship, &asteroids);
            }
            State::Pause => text_middle(
                Text::new(&font_large, "Pause", Size::Large, 1.0),
                screen,
                &mut canvas,
            ),
            State::Menu => {
                canvas.clear(); // Remove hud
                                //
            }
            _ => unimplemented!(),
        }

        canvas.present();

        //std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    /* Game Over */
    if ship.lives < 1 {
        game_over(&font_large, screen, &mut canvas);
    }

    unsafe {
        sprite_sheet.destroy();
        font_large.destroy();
    }
}
