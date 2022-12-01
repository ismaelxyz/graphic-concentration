mod components;
mod renderer;

use sdl2::{
    event::Event,
    // "self" imports the "image" module itself as well as everything else we listed
    image::{self, InitFlag, LoadTexture},
    keyboard::Keycode,
    pixels::Color,
    rect::{Point, Rect},
};

use crate::components::*;
use std::time::Duration;
use enemy::Enemy;
use player::Player;

/// El patético estado global de este pequeño juego.
pub(crate) struct World {
    pub(crate) enemies: Vec<Object<Enemy>>,
    pub(crate) player: Object<Player>,
    movement_command: Option<MovementCommand>,
}

impl World {
    fn new(player_spritesheet: usize) -> Self {
        let player_top_left_frame = Rect::new(0, 0, 94, 100);

        let player_animation = MovementAnimation {
            current_frame: 0,
            up_frames: character_animation_frames(
                player_spritesheet,
                player_top_left_frame,
                Direction::Up,
            ),
            down_frames: character_animation_frames(
                player_spritesheet,
                player_top_left_frame,
                Direction::Down,
            ),
            left_frames: character_animation_frames(
                player_spritesheet,
                player_top_left_frame,
                Direction::Left,
            ),
            right_frames: character_animation_frames(
                player_spritesheet,
                player_top_left_frame,
                Direction::Right,
            ),
        };

        World {
            enemies: Vec::new(),
            movement_command: None,
            player: Object {
                this: Player {},
                position: Position(Point::new(0, 0)),
                velocity: Velocity {
                    speed: 0,
                    direction: Direction::Right,
                },
                frame: player_animation.right_frames[0].clone(),
                animation: player_animation,
            },
        }
    }

    fn add_enemy(&mut self, enemy_spritesheet: usize, position: Point) {
        let enemy_top_left_frame = Rect::new(0, 0, 122, 114);

        let enemy_animation = MovementAnimation {
            current_frame: 0,
            up_frames: character_animation_frames(
                enemy_spritesheet,
                enemy_top_left_frame,
                Direction::Up,
            ),
            down_frames: character_animation_frames(
                enemy_spritesheet,
                enemy_top_left_frame,
                Direction::Down,
            ),
            left_frames: character_animation_frames(
                enemy_spritesheet,
                enemy_top_left_frame,
                Direction::Left,
            ),
            right_frames: character_animation_frames(
                enemy_spritesheet,
                enemy_top_left_frame,
                Direction::Right,
            ),
        };

        self.enemies.push(Object {
            this: Enemy {},
            position: Position(position),
            velocity: Velocity {
                speed: 0,
                direction: Direction::Right,
            },
            frame: enemy_animation.right_frames[0].clone(),
            animation: enemy_animation,
        })
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum MovementCommand {
    Stop,
    Move(Direction),
}

/// Returns the row of the spritesheet corresponding to the given direction
fn direction_spritesheet_row(direction: Direction) -> i32 {
    use self::Direction::*;
    match direction {
        Up => 3,
        Down => 0,
        Left => 1,
        Right => 2,
    }
}

/// Create animation frames for the standard character spritesheet
fn character_animation_frames(
    spritesheet: usize,
    top_left_frame: Rect,
    direction: Direction,
) -> Vec<Sprite> {
    // All assumptions about the spritesheets are now encapsulated in this function instead of in
    // the design of our entire system. We can always replace this function, but replacing the
    // entire system is harder.

    let (frame_width, frame_height) = top_left_frame.size();
    let y_offset = top_left_frame.y() + frame_height as i32 * direction_spritesheet_row(direction);

    let mut frames = Vec::new();
    for i in 0..3 {
        frames.push(Sprite {
            spritesheet,
            region: Rect::new(
                top_left_frame.x() + frame_width as i32 * i,
                y_offset,
                frame_width,
                frame_height,
            ),
        })
    }

    frames
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    // Leading "_" tells Rust that this is an unused variable that we don't care about. It has to
    // stay unused because if we don't have any variable at all then Rust will treat it as a
    // temporary value and drop it right away!
    let _/*image_context*/ = image::init(InitFlag::PNG | InitFlag::JPG)?;

    sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "1");
    sdl2::hint::set("SDL_VIDEO_X11_NET_WM_BYPASS_COMPOSITOR", "0");

    let window = video_subsystem
        .window("game tutorial", 800, 600)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("could not make a canvas");
    let texture_creator = canvas.texture_creator();

    let textures = [
        texture_creator.load_texture("assets/raptor.png")?,
        texture_creator.load_texture("assets/phoenix.png")?,
    ];
    // First and Second texture in textures array
    let (player_spritesheet, enemy_spritesheet) = (0, 1);

    let mut world = World::new(player_spritesheet);

    world.add_enemy(enemy_spritesheet, Point::new(-150, -150));
    world.add_enemy(enemy_spritesheet, Point::new(150, -190));
    world.add_enemy(enemy_spritesheet, Point::new(-150, 170));

    let mut event_pump = sdl_context.event_pump()?;
    let (mut i, mut movement_command) = (0, None);

    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            movement_command = match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    repeat: false,
                    ..
                } => Some(MovementCommand::Move(Direction::Left)),
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    repeat: false,
                    ..
                } => Some(MovementCommand::Move(Direction::Right)),
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    repeat: false,
                    ..
                } => Some(MovementCommand::Move(Direction::Up)),
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    repeat: false,
                    ..
                } => Some(MovementCommand::Move(Direction::Down)),
                Event::KeyUp {
                    keycode: Some(Keycode::Left),
                    repeat: false,
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::Right),
                    repeat: false,
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::Up),
                    repeat: false,
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::Down),
                    repeat: false,
                    ..
                } => Some(MovementCommand::Stop),
                _ => None,
            };
        }

        world.movement_command = movement_command;

        for enemy in &mut world.enemies {
            enemy.ai();
            enemy.physics();
            enemy.animator();
        }

        {
            world.player.xmove(world.movement_command);
            world.player.physics();
            world.player.animator();
        }

        // Render
        i = (i + 1) % 255;
        renderer::render(&mut canvas, Color::RGB(i, 64, 255 - i), &textures, &world)?;

        // Time management!
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 20));
    }

    Ok(())
}
