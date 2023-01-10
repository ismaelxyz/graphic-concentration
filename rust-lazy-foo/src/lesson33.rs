use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
    render::{Canvas, Texture, TextureCreator},
    rwops::RWops,
    surface::Surface,
    ttf::Font,
    video::{Window, WindowContext},
};

use std::io::{Read, Write};

const DATA_PATH: &str = "resources/lesson33/nums.bin";
// Number of data integers
const TOTAL_DATA: usize = 10;

// Create a struct that will track texture data
struct Text {
    // The actual texture.
    texture: Texture,
    // Image dimensions
    width: u32,
    height: u32,
}

impl Text {
    // create a new texture
    fn new(texture: Texture) -> Text {
        Text {
            width: texture.query().width,
            height: texture.query().height,
            texture,
        }
    }

    /// Renders texture at given point
    fn render(&self, canvas: &mut Canvas<Window>, x: i32, y: i32) {
        let clip_rect = Rect::new(x, y, self.width, self.height);

        canvas
            .copy_ex(&self.texture, None, clip_rect, 0.0, None, false, false)
            .expect("Could not blit texture to render target!");
    }

    // this function required feature sdl2_ttf
    fn from_text(
        creator: &TextureCreator<WindowContext>,
        font: &Font,
        text: &str,
        color: Color,
    ) -> Self {
        let text_surface: Surface = font
            .render(text)
            .solid(color)
            .expect("Could not create text surface!");

        // Now create a texture from the surface using the supplied creator
        let text_texture = creator
            .create_texture_from_surface(&text_surface)
            .expect("Could not convert text surface to texture!");

        // Return an Text using the given text_texture
        Self::new(text_texture)
    }

    fn normal(creator: &TextureCreator<WindowContext>, font: &Font, text: impl ToString) -> Self {
        Self::from_text(creator, font, &text.to_string(), Color::BLACK)
    }

    fn highlight(
        creator: &TextureCreator<WindowContext>,
        font: &Font,
        text: impl ToString,
    ) -> Self {
        Self::from_text(creator, font, &text.to_string(), Color::RED)
    }
}

#[cfg(target_endian = "big")]
fn from_byte(byte: u8) -> i8 {
    i8::from_be(byte as i8)
}

#[cfg(target_endian = "little")]
fn from_byte(byte: u8) -> i8 {
    i8::from_le(byte as i8)
}

#[cfg(target_endian = "big")]
fn to_byte(byte: i8) -> u8 {
    i8::to_be(byte) as u8
}

#[cfg(target_endian = "little")]
fn to_byte(byte: i8) -> u8 {
    i8::to_le(byte) as u8
}

fn onclose(data: [i8; TOTAL_DATA], mut raw_data: [u8; TOTAL_DATA]) {
    let mut file = RWops::from_file("resources/lesson33/nums.bin", "w+b").unwrap();
    for (item, value) in data.iter().zip(raw_data.iter_mut()) {
        *value = to_byte(*item);
    }

    // Save data
    file.write_all(&raw_data).unwrap();
}

fn main() {
    let sdl_ctx = sdl2::init().expect("Unable to initialize SDL!");
    let video = sdl_ctx.video().expect("Could not acquire video context!");
    let ttf_ctx = sdl2::ttf::init().expect("Could not acquire ttf context!");

    sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "1");
    sdl2::hint::set("SDL_VIDEO_X11_NET_WM_BYPASS_COMPOSITOR", "0");

    let window = video
        .window("SDL Tutorial 33", 650, 480)
        .position_centered()
        .opengl()
        .build()
        .expect("Could not create SDL window!");
    let width = window.size().0;

    // Obtain the canvas
    let mut canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .expect("Unable to obtain canvas!");

    let creator = canvas.texture_creator();

    // Get a handle to the SDL2 event pump
    let mut event_pump = sdl_ctx
        .event_pump()
        .expect("Unable to obtain event pump handle!");

    // Load a font
    let font_path = std::path::Path::new("resources/lesson33/lazy.ttf");
    let font = ttf_ctx.load_font(font_path, 28).unwrap();
    let prompt = Text::normal(&creator, &font, "Enter Data:");

    let mut data_textures = Vec::with_capacity(TOTAL_DATA);
    let (mut data, mut raw_data) = ([0i8; TOTAL_DATA], [0u8; TOTAL_DATA]);
    // Current input point
    let mut current_data = 0;

    // Open file for reading in binary
    match RWops::from_file(DATA_PATH, "r+b") {
        Ok(mut file) => {
            // Load data
            println!("Reading file...!");

            file.read_exact(&mut raw_data).unwrap();

            for (item, value) in raw_data.iter().zip(data.iter_mut()) {
                *value = from_byte(*item);
            }
        }
        Err(..) => {
            let mut file = RWops::from_file(DATA_PATH, "w+b").unwrap();
            println!("New file created!");

            // Initialize data
            file.write_all(&raw_data).unwrap();
        }
    }

    // Load textures
    for item in data {
        data_textures.push(Text::normal(&creator, &font, item));
    }

    data_textures[current_data] = Text::highlight(&creator, &font, data[current_data]);
    drop(font);
    let font_height = data_textures[0].height;

    canvas.set_draw_color(Color::WHITE);

    lazy_foo::setup_mainloop(-1, true, move || {
        for event in event_pump.poll_iter() {
            // Pattern match on the Quit event
            match event {
                Event::Quit { .. } => {
                    onclose(data, raw_data);
                    return false;
                }
                Event::KeyDown {
                    keycode: Some(k),
                    keymod: _,
                    ..
                } => {
                    let mut update = true;

                    match k {
                        Keycode::Escape => {
                            onclose(data, raw_data);
                            return false;
                        }
                        code @ (Keycode::Up | Keycode::Down) => {
                            let font = ttf_ctx.load_font(font_path, 28).unwrap();

                            data_textures[current_data] =
                                Text::normal(&creator, &font, data[current_data]);

                            if code == Keycode::Up {
                                if current_data == usize::MIN {
                                    current_data = TOTAL_DATA;
                                }

                                current_data -= 1;
                            } else {
                                if current_data == TOTAL_DATA {
                                    current_data = 0;
                                }

                                current_data += 1;
                            }
                        }
                        Keycode::Left => data[current_data] -= 1,
                        Keycode::Right => data[current_data] += 1,
                        _ => update = false,
                    }

                    if update {
                        let font = ttf_ctx.load_font(font_path, 28).unwrap();
                        data_textures[current_data] =
                            Text::highlight(&creator, &font, data[current_data]);
                    }
                }
                _ => (),
            }
        }

        canvas.clear();
        prompt.render(&mut canvas, (width - prompt.width) as i32 / 2, 0);

        for (index, texture) in data_textures.iter().enumerate() {
            texture.render(
                &mut canvas,
                (width - texture.width) as i32 / 2,
                (prompt.height + 4 + font_height * index as u32) as i32,
            );
        }

        canvas.present();
        true
    });
}
