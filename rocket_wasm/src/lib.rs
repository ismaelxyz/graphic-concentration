#[macro_use]
extern crate lazy_static;

use wasm_bindgen::prelude::wasm_bindgen;

mod controllers;
mod game_state;
mod geometry;
mod models;
mod util;

use std::sync::Mutex;

use pcg_rand::{seeds::PcgSeeder, Pcg32Basic};
use rand::SeedableRng;

use self::controllers::{Actions, CollisionsController, TimeController};
use self::game_state::GameState;
use self::geometry::Size;

lazy_static! {
    static ref DATA: Mutex<GameData> = Mutex::new(new_game_data(1024.0, 600.0));
}

struct GameData {
    state: GameState,
    actions: Actions,
    time_controller: TimeController<Pcg32Basic>,
}

fn new_game_data(width: f64, height: f64) -> GameData {
    GameData {
        state: GameState::new(Size::new(width, height)),
        actions: Actions::default(),
        time_controller: TimeController::new(Pcg32Basic::from_seed(PcgSeeder::seed_with_stream(
            42, 42,
        ))),
    }
}

// These functions are provided by the runtime
#[wasm_bindgen(module = "/js/draw.js")]
extern "C" {
    fn clear_screen();
    fn draw_player(x: f64, y: f64, angle: f64);
    fn draw_enemy(x: f64, y: f64);
    fn draw_bullet(x: f64, y: f64);
    fn draw_particle(x: f64, y: f64, radius: f64);
    fn draw_score(score: f64);
}

#[wasm_bindgen]
pub fn resize(width: f64, height: f64) {
    *DATA.lock().unwrap() = new_game_data(width, height);
}

#[wasm_bindgen]
pub fn draw() {
    use geometry::{Advance, Position};
    let data = &mut DATA.lock().unwrap();
    let world = &data.state.world;

    clear_screen();
    for particle in &world.particles {
        draw_particle(particle.x(), particle.y(), 5.0 * particle.ttl);
    }

    for bullet in &world.bullets {
        draw_bullet(bullet.x(), bullet.y());
    }

    for enemy in &world.enemies {
        draw_enemy(enemy.x(), enemy.y());
    }

    draw_player(world.player.x(), world.player.y(), world.player.direction());
    draw_score(data.state.score as f64);
}

#[wasm_bindgen]
pub fn update(time: f64) {
    let data: &mut GameData = &mut DATA.lock().unwrap();
    data.time_controller
        .update_seconds(time, &data.actions, &mut data.state);
    CollisionsController::handle_collisions(&mut data.state);
}

#[wasm_bindgen]
pub fn toggle_shoot(b: i32) {
    let data = &mut DATA.lock().unwrap();
    data.actions.shoot = b != 0;
}

#[wasm_bindgen]
pub fn toggle_boost(b: i32) {
    let data = &mut DATA.lock().unwrap();
    data.actions.boost = b != 0;
}

#[wasm_bindgen]
pub fn toggle_turn_left(b: i32) {
    let data = &mut DATA.lock().unwrap();
    data.actions.rotate_left = b != 0;
}

#[wasm_bindgen]
pub fn toggle_turn_right(b: i32) {
    let data = &mut DATA.lock().unwrap();
    data.actions.rotate_right = b != 0;
}
