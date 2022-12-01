//! This module contains the game logic
//!
//! There are three main controllers: collisions, input and time

mod collisions;
mod input;
mod time;

pub use self::{collisions::CollisionsController, input::Actions, time::TimeController};
