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

I plan to port most of the provided tutorials, though for some I can't
create a port I lack the resourced - for example, I don't have a
gamepad or forcefeedback device, and at this point I can't create
anything for mobile.

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
* (TODO) Lesson 22 - Timing
* (TODO) Lesson 23 - Advanced Timers
* (TODO) Lesson 24 - Calculating Frame Rate
* (TODO) Lesson 25 - Capping Frame Rate
* (TODO) Lesson 26 - Motion
* (TODO) Lesson 27 - Collision Detection
* (TODO) Lesson 28 - Per-pixel Collision Detection
* (TODO) Lesson 29 - Circular Collision Detection
* (TODO) Lesson 30 - Scrolling
* (TODO) Lesson 31 - Scrolling Backgrounds
* (TODO) Lesson 32 - Text Input and Clipboard Handling
* (TODO) Lesson 33 - File Reading and Writing
* (NOT IMPLEMENTED) Lesson 34 - Audio Recording
* (TODO) Lesson 35 - Window Events
* (TODO) Lesson 36 - Multiple Windows
* (TODO) Lesson 37 - Multiple Displays
* (TODO) Lesson 38 - Particle Engines
* (TODO) Lesson 39 - Tiling
* (TODO) Lesson 40 - Texture Manipulation
* (TODO) Lesson 41 - Bitmap Fonts
* (TODO) Lesson 42 - Texture Streaming
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
    sudo apt-get install libsdl2-image-dev libsdl2-ttf-dev libsdl2-mixer-dev

On OS X, you can use Homebrew:

	brew install sdl2
    brew install sdl2_image
    brew install sdl2_ttf
    brew install sdl2_mixer

On Fedora:

    sudo dnf install SDL2-devel SDL2_ttf-devel SDL2_image-devel SDL2_mixer-devel

On RedHat/Centos:

    sudo yum install SDL2-devel SDL2_ttf-devel SDL2_image-devel SDL2_mixer-devel


For other platforms, refer to your existing package documentation.

## Compiling and Running the Examples

Once you have a version of rust installed, you can build all the
examples with the command

```
cargo build
```

To run a specific lesson, run

```
cargo run --bin lesson<NN>
```

Where <NN> is the lesson number.

