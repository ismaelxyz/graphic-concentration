pub mod enemy;
pub mod player;

use sdl2::rect::{Point, Rect};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// The current position of a given entity
#[repr(transparent)]
pub struct Position(pub Point);

/// The current speed and direction of a given entity
pub struct Velocity {
    pub speed: i32,
    pub direction: Direction,
}

#[derive(Clone)]
pub struct Sprite {
    /// The specific spritesheet to render from
    pub spritesheet: usize,
    /// The current region of the spritesheet to be rendered
    pub region: Rect,
}

pub struct MovementAnimation {
    // The current frame in the animation of the direction this entity is moving in
    pub current_frame: usize,
    pub up_frames: Vec<Sprite>,
    pub down_frames: Vec<Sprite>,
    pub left_frames: Vec<Sprite>,
    pub right_frames: Vec<Sprite>,
}

#[allow(dead_code)]
pub struct Object<T> {
    pub(crate) this: T, // This field is not used.
    pub(crate) position: Position,
    pub(crate) velocity: Velocity,
    pub(crate) animation: MovementAnimation,
    pub(crate) frame: Sprite,
}

impl<T> Object<T> {
    /// Anima al objeto según la dirección a la cual se dirige.
    pub(crate) fn animator(&mut self) {
        use self::Direction::*;

        if self.velocity.speed > 0 {
            let frames = match self.velocity.direction {
                Left => &self.animation.left_frames,
                Right => &self.animation.right_frames,
                Up => &self.animation.up_frames,
                Down => &self.animation.down_frames,
            };

            self.animation.current_frame = (self.animation.current_frame + 1) % frames.len();
            self.frame = frames[self.animation.current_frame].clone();
        }
    }

    pub(crate) fn physics(&mut self) {
        use self::Direction::*;

        self.position.0 = match self.velocity.direction {
            Left => self.position.0.offset(-self.velocity.speed, 0),
            Right => self.position.0.offset(self.velocity.speed, 0),
            Up => self.position.0.offset(0, -self.velocity.speed),
            Down => self.position.0.offset(0, self.velocity.speed),
        };
    }
}

