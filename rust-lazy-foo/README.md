# Lazy Foo's SDL2 Tutorials, in Rust #

This repo contains ports of the
[Lazy Foo SDL2](http://lazyfoo.net/tutorials/SDL/index.php) tutorials
to the [Rust](http://www.rust-lang.org) programming language, using
the Rust wrappers kindly provided by [Rust-SDL2](https://github.com/Rust-SDL2/rust-sdl2)

The original author of this project is [ysgard](https://github.com/ysgard), the problem is
that his version is not adapted to the current version of rust-sdl2 and lacks examples.
This version is the most up to date and although it is not finished, it is still under
development.

The examples assume 0.34.3 at a minimum.

## Tutorial Index

I plan to port most of the provided tutorials, though for some I can't create a
port I lack the resourced - for example, I don't have a gamepad or forcefeedback
device, and at this point I can't create anything for mobile.

* Lesson 01 - Hello SDL
* Lesson 02 - Getting an Image on the Screen
* Lesson 03 - Event Driven Programming
* Lesson 04 - Key Presses
* Lesson 05 - Optimized Surface Loading and Soft Stretching
* Lesson 06 - Extension Libraries and Loading Other Image Formats
* Lesson 07 - Texture Loading and Rendering
* Lesson 08 - Geometry Rendering
* Lesson 09 - The Viewport
* Lesson 10 - Color Keying
* Lesson 11 - Clip Rendering and Sprite Sheets
* Lesson 12 - Color Modulation
* Lesson 13 - Alpha Blending
* Lesson 14 - Animated Sprites and Vsync
* Lesson 15 - Rotation and Flipping
* Lesson 16 - True Type Fonts
* Lesson 17 - Mouse Events
* Lesson 18 - Key States
* (NOT IMPLEMENTED) Lesson 19 - Gamepads and Joysticks
* (NOT IMPLEMENTED) Lesson 20 - Force Feedback
* Lesson 21 - Sound Effects and Music
* Lesson 22 - Timing
* Lesson 23 - Advanced Timers
* Lesson 24 - Calculating Frame Rate
* Lesson 25 - Capping Frame Rate
* Lesson 26 - Motion
* Lesson 27 - Collision Detection
* Lesson 28 - Per-pixel Collision Detection
* Lesson 29 - Circular Collision Detection
* Lesson 30 - Scrolling
* Lesson 31 - Scrolling Backgrounds
* Lesson 32 - Text Input and Clipboard Handling
* Lesson 33 - File Reading and Writing
* (NOT IMPLEMENTED) Lesson 34 - Audio Recording
* Lesson 35 - Window Events
* Lesson 36 - Multiple Windows
* Lesson 37 - Multiple Displays
* Lesson 38 - Particle Engines
* Lesson 39 - Tiling
* Lesson 40 - Texture Manipulation
* Lesson 41 - Bitmap Fonts
* Lesson 42 - Texture Streaming
* (TODO) Lesson 43 - Render to Texture
* (TODO) Lesson 44 - Frame Independent Movement
* (TODO) Lesson 45 - Timer Callbacks
* (TODO) Lesson 46 - Multithreading
* (TODO) Lesson 47 - Semaphores
* (TODO) Lesson 48 - Atomic Operations
* (TODO) Lesson 49 - Mutexes and Conditions
* (TODO) Lesson 50 - SDL and OpenGL 2
* (TODO) Lesson 51 - SDL and Modern OpenGL
* (NOT IMPLEMENTED) Lesson 52 - Hello Mobile
* (NOT IMPLEMENTED) Lesson 53 - Extensions and Changing Orientation
* (NOT IMPLEMENTED) Lesson 54 - Touches
* (NOT IMPLEMENTED) Lesson 55 - Multitouch

## Requirements

To run any of these examples, you will need:

1. The [stable build](http://www.rust-lang.org/install.html) of Rust.

2. [The SDL2 Development libraries](https://www.libsdl.org/download-2.0.php). You
will also need the image library, [SDL_Image 2.0](https://www.libsdl.org/projects/SDL_image/), the truetype
library [SDL_TTF 2.0](https://www.libsdl.org/projects/SDL_ttf/), multi-channel audio mixer library [SDL_MIXER 2.0](https://www.libsdl.org/projects/SDL_mixer/).

On Debian and derivatives:
    sudo apt install libsdl2-image-dev libsdl2-ttf-dev libsdl2-mixer-dev

On OS X, you can use Homebrew:
```bash
brew install sdl2
brew install sdl2_image
brew install sdl2_ttf
brew install sdl2_mixer
```

On Fedora:
```bash
sudo dnf install SDL2-devel SDL2_ttf-devel SDL2_image-devel SDL2_mixer-devel
```

On RedHat/Centos:
```bash
sudo yum install SDL2-devel SDL2_ttf-devel SDL2_image-devel SDL2_mixer-devel
```

For other platforms, refer to your existing package documentation.

## Compiling and Running the Examples

Once you have a version of rust installed, you can build all the
examples with the command

```bash
cargo build
```

To run a specific lesson, run

```bash
cargo run --bin lesson<NN>
```

Where <NN> is the lesson number.
