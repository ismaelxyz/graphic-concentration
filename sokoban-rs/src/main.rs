// This file is part of sokoban-rs
// Copyright 2015 Sébastien Watteau
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! This is an implementation of Sokoban in Rust.

#[macro_use]
extern crate bitflags;

use clap::{CommandFactory, Parser, error::ErrorKind};
use sdl2::Sdl;
use sdl2::event::Event;
use sdl2::image::InitFlag;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::{Window, WindowContext};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use xml::reader::{EventReader, XmlEvent};

pub mod error;
pub mod game;
pub mod painter;
pub mod shadow;
pub mod tileset;

use game::{Direction, Level};
use painter::Painter;
use tileset::Tileset;

#[derive(Parser, Debug)]
#[command(
    name = "sokoban-rs",
    version,
    about = "An implementation of Sokoban in the Rust programming language.",
    author = "Sébastien Watteau",
    arg_required_else_help = true,
    disable_help_flag = true
)]
struct Cli {
    /// a Sokoban level collection (SLC) file
    slc_file: PathBuf,

    /// Launches the game in fullscreen mode
    #[arg(short = 'f', long = "fullscreen")]
    fullscreen: bool,

    /// The width of the window in pixels
    #[arg(short = 'w', long = "width", value_parser = clap::value_parser!(u32))]
    width: Option<u32>,

    /// The height of the window in pixels
    #[arg(short = 'h', long = "height", value_parser = clap::value_parser!(u32))]
    height: Option<u32>,

    /// Print help information
    #[arg(long = "help", action = clap::ArgAction::Help)]
    help: Option<bool>,
}

pub fn main() -> Result<(), Box<dyn Error>> {
    // Read command line arguments
    let cli = Cli::parse();
    let (width, height) = match (cli.width, cli.height) {
        (Some(w), Some(h)) => (w, h),
        (None, None) => (1024, 768),
        _ => {
            Cli::command()
                .error(
                    ErrorKind::MissingRequiredArgument,
                    "Both --width and --height must be provided together",
                )
                .exit();
        }
    };
    let fullscreen = cli.fullscreen;
    let slc_file = cli.slc_file;

    // Load the level collection file
    let levels = load_slc_file(slc_file)?;

    // Initialize SDL components
    let sdl = sdl2::init()?;
    let _ = sdl2::image::init(InitFlag::PNG)?;
    let ttf_context = sdl2::ttf::init()?;

    let window = create_window(&sdl, width, height, fullscreen)?;
    let mut canvas = window.into_canvas().build()?;
    let texture_creator = canvas.texture_creator();

    let mut painter = {
        let big_set = load_tileset(
            &texture_creator,
            "assets/image/tileset.png",
            101,
            171,
            83,
            40,
        )?;
        let small_set = load_tileset(
            &texture_creator,
            "assets/image/tileset-small.png",
            50,
            85,
            41,
            20,
        )?;
        let font = ttf_context.load_font("assets/font/RujisHandwritingFontv.2.0.ttf", 20)?;
        Painter::new(&mut canvas, big_set, small_set, font)
    };

    mainloop(&sdl, levels.iter(), &mut painter, &mut canvas);

    Ok(())
}

/// Builds levels from a level collection file in the SLC format.
fn load_slc_file<P: AsRef<Path>>(path: P) -> Result<Vec<Level>, error::SokobanError> {
    let mut levels = Vec::new();

    let parser = {
        let file = File::open(path.as_ref())?;
        EventReader::new(BufReader::new(file))
    };

    let mut level_title = String::new();
    let mut level_data = String::new();
    let mut reading_level = false;
    for event in parser {
        match event {
            Ok(XmlEvent::StartElement {
                ref name,
                ref attributes,
                ..
            }) => {
                if name.local_name == "L" {
                    reading_level = true;
                } else if name.local_name == "Level" {
                    if let Some(id) = attributes.iter().find(|&attr| attr.name.local_name == "Id") {
                        level_title = id.value.clone();
                    }
                }
            }
            Ok(XmlEvent::EndElement { name }) => {
                if name.local_name == "Level" {
                    let mut level = Level::from_str(&level_data)?;
                    level.set_title(level_title.clone());
                    levels.push(level);
                    level_data.clear();
                }
            }
            Ok(XmlEvent::Characters(ref data)) => {
                if reading_level {
                    level_data.push_str(data);
                    level_data.push('\n');
                }
            }
            _ => {}
        }
    }

    Ok(levels)
}

/// Creates the SDL window
fn create_window(
    sdl: &Sdl,
    width: u32,
    height: u32,
    fullscreen: bool,
) -> Result<Window, Box<dyn Error>> {
    let mut window_builder = sdl.video()?.window("sokoban-rs", width, height);
    if fullscreen {
        window_builder.fullscreen();
    } else {
        window_builder.position_centered();
    }
    let window = window_builder.opengl().build()?;
    Ok(window)
}

/// Loads a tileset
fn load_tileset<P: AsRef<Path>>(
    texture_creator: &TextureCreator<WindowContext>,
    path: P,
    width: u32,
    height: u32,
    effective_height: u32,
    offset: i32,
) -> Result<Tileset<'_>, Box<dyn Error>> {
    let texture = texture_creator.load_texture(path.as_ref())?;
    let tileset = Tileset::new(texture, width, height, effective_height, offset);
    Ok(tileset)
}

/// Main game event loop
fn mainloop<'a, I: Iterator<Item = &'a Level>>(
    sdl: &Sdl,
    mut levels: I,
    painter: &mut Painter,
    canvas: &mut Canvas<Window>,
) {
    let (mut reference_level, mut level) = match levels.next() {
        Some(l) => (l, l.clone()),
        None => {
            return;
        }
    };

    let mut running = true;
    let mut events = sdl.event_pump().unwrap();
    let mut skip = false;
    while running {
        if level.is_completed() || skip {
            match levels.next() {
                Some(l) => {
                    reference_level = l;
                    level = l.clone();
                    skip = false;
                }
                None => {
                    break;
                }
            }
        }

        painter.paint(canvas, &level);

        match events.wait_event() {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => running = false,
            Event::KeyDown {
                keycode: Some(Keycode::Left),
                ..
            } => {
                level.step(Direction::Left);
            }
            Event::KeyDown {
                keycode: Some(Keycode::Right),
                ..
            } => {
                level.step(Direction::Right);
            }
            Event::KeyDown {
                keycode: Some(Keycode::Up),
                ..
            } => {
                level.step(Direction::Up);
            }
            Event::KeyDown {
                keycode: Some(Keycode::Down),
                ..
            } => {
                level.step(Direction::Down);
            }
            Event::KeyDown {
                keycode: Some(Keycode::R),
                ..
            } => {
                level = reference_level.clone();
            }
            Event::KeyDown {
                keycode: Some(Keycode::N),
                ..
            } => {
                skip = true;
            }
            _ => {}
        }
    }
}
