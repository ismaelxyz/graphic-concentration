use sdl2::rect::{Point, Rect};

#[allow(dead_code)]
pub struct Object<T> {
    pub(crate) this: T, // This field is not used.
    pub(crate) position: Position,
    pub(crate) velocity: Velocity,
    pub(crate) animation: MovementAnimation,
    pub(crate) frame: Sprite,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Player {}

pub struct Enemy {}

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
