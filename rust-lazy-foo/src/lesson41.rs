use sdl2::{
    event::Event,
    image::{InitFlag, LoadSurface},
    keyboard::Keycode,
    pixels::{Color, PixelFormat, PixelFormatEnum},
    rect::Rect,
    render::{BlendMode, Canvas, Texture, TextureCreator},
    surface::Surface,
    video::{Window, WindowContext},
};

unsafe fn to_vecu32(vecu8: &mut [u8], size: usize) -> &mut [u32] {
    &mut *core::ptr::slice_from_raw_parts_mut(&mut vecu8[..] as *mut [u8] as *mut u32, size)
}

/// Get the pixel requested
fn get_pixel(pixels: &[u32], pitch: usize, x: u32, y: u32) -> u32 {
    pixels[y as usize * (pitch / 4) + x as usize]
}

pub struct LTexture {
    texture: Texture,
    width: u32,
    height: u32,
}

impl LTexture {
    fn from_file(path: &str, creator: &TextureCreator<WindowContext>) -> LTexture {
        let pixel_format_enum = PixelFormatEnum::RGBA8888;
        let mut surface = Surface::from_file(path).unwrap();
        surface = surface.convert_format(pixel_format_enum).unwrap();

        let (width, height) = surface.size();

        let pixel_format = PixelFormat::try_from(pixel_format_enum).unwrap();
        // Map colors
        let color_key = Color::RGB(0, 0xFF, 0xFF).to_u32(&pixel_format);
        let transparent = Color::RGBA(0xFF, 0xFF, 0xFF, 0x00).to_u32(&pixel_format);

        let mut src_pixels = surface.with_lock(move |pixels| pixels.to_vec());

        let mut texture = creator
            .create_texture_streaming(pixel_format_enum, width, height)
            .unwrap();

        // Enable blending on texture
        texture.set_blend_mode(BlendMode::Blend);

        texture
            .with_lock(None, move |trg_pixels, _| {
                let (pixels, trg_pixels) = unsafe {
                    let size = (width * height) as usize;
                    (
                        to_vecu32(&mut src_pixels, size),
                        to_vecu32(trg_pixels, size),
                    )
                };

                // Color key pixels
                for (src_pixel, trg_pixel) in pixels.iter_mut().zip(trg_pixels.iter_mut()) {
                    *trg_pixel = if *src_pixel == color_key {
                        transparent
                    } else {
                        *src_pixel
                    };
                }
            })
            .unwrap();

        LTexture {
            texture,
            width,
            height,
        }
    }

    fn render(&self, canvas: &mut Canvas<Window>, x: i32, y: i32, clip: Rect) {
        canvas
            .copy_ex(
                &self.texture,
                clip,
                Some(Rect::new(x, y, clip.width(), clip.height())),
                0.0,
                None,
                false,
                false,
            )
            .expect("Could not blit texture to render target!");
    }
}

struct BitmapFont {
    /// The font texture
    bitmap: LTexture,

    /// The individual characters in the surface
    chars: [Rect; 256],

    /// Spacing Variables
    new_line: i32,
    space: i32,
}

impl BitmapFont {
    /// Generates the font
    fn build_font(mut bitmap: LTexture) -> Self {
        let (width, height) = (bitmap.width, bitmap.height);

        let (chars, new_line, space) = bitmap
            .texture
            .with_lock(None, move |pixels, pitch| {
                let mut chars = [Rect::new(0, 0, 0, 0); 256];
                let pixels = unsafe { to_vecu32(pixels, (width * height) as usize) };
                // Set the background color
                let bg_color = get_pixel(pixels, pitch, 0, 0);

                // Set the cell dimensions
                let (cell_w, cell_h) = (width / 16, height / 16);

                // New line variables
                let (mut top, mut base_a) = (cell_w, cell_h);

                // The current character we're setting
                let mut current_char = 0;

                // Go through the cell rows
                for rows in 0..16 {
                    // Go through the cell columns
                    for cols in 0..16 {
                        // Set the character offset
                        chars[current_char].x = (cell_w * cols) as i32;
                        chars[current_char].y = (cell_h * rows) as i32;

                        // Set the dimensions of the character
                        chars[current_char].w = cell_w as i32;
                        chars[current_char].h = cell_h as i32;

                        // Find Left Side
                        // Go through pixel columns
                        'top: for pixel_col in 0..cell_w {
                            // Go through pixel rows
                            for pixel_row in 0..cell_h {
                                // Get the pixel offsets
                                let px = cell_w * cols + pixel_col;
                                let py = cell_h * rows + pixel_row;

                                // If a non colorkey pixel is found
                                if get_pixel(pixels, pitch, px, py) != bg_color {
                                    // Set the x offset
                                    chars[current_char].x = px as i32;

                                    break 'top;
                                }
                            }
                        }

                        // Find Right Side
                        // Go through pixel columns
                        'top: for mut pixel_colw in 0..cell_w {
                            pixel_colw = cell_w - pixel_colw - 1;
                            // Go through pixel rows
                            for pixel_roww in 0..cell_h {
                                // Get the pixel offsets
                                let px = cell_w * cols + pixel_colw;
                                let py = cell_h * rows + pixel_roww;

                                // If a non colorkey pixel is found
                                if get_pixel(pixels, pitch, px, py) != bg_color {
                                    // Set the width
                                    chars[current_char].w = px as i32 - chars[current_char].x + 1;

                                    break 'top;
                                }
                            }
                        }

                        // Find Top
                        // Go through pixel rows
                        'top: for pixel_row in 0..cell_h {
                            // Go through pixel columns
                            for pixel_col in 0..cell_w {
                                // Get the pixel offsets
                                let px = cell_w * cols + pixel_col;
                                let py = cell_h * rows + pixel_row;

                                // If a non colorkey pixel is found
                                if get_pixel(pixels, pitch, px, py) != bg_color {
                                    // If new top is found
                                    if pixel_row < top {
                                        top = pixel_row;
                                    }

                                    break 'top;
                                }
                            }
                        }

                        // Find Bottom of A
                        if current_char == 'A' as usize {
                            // Go through pixel rows
                            'top: for mut pixel_row in 0..cell_h {
                                pixel_row = cell_h - pixel_row - 1;
                                // Go through pixel columns
                                for pixel_col in 0..cell_w {
                                    // Get the pixel offsets
                                    let px = cell_w * cols + pixel_col;
                                    let py = cell_h * rows + pixel_row;

                                    // If a non colorkey pixel is found
                                    if get_pixel(pixels, pitch, px, py) != bg_color {
                                        // Bottom of a is found
                                        base_a = pixel_row;

                                        break 'top;
                                    }
                                }
                            }
                        }

                        // Go to the next character
                        current_char += 1;
                    }
                }

                // Calculate space
                let space = cell_w / 2;

                // Calculate new line
                let new_line = base_a - top;

                // Lop off excess top pixels
                for character in chars.iter_mut() {
                    character.y += top as i32;
                    character.h -= top as i32;
                }

                (chars, new_line as i32, space as i32)
            })
            .unwrap();

        BitmapFont {
            bitmap,
            chars,
            new_line,
            space,
        }
    }

    /// Shows the text
    fn render_text(&self, canvas: &mut Canvas<Window>, text: &str) {
        // Temp offsets
        let (mut cur_x, mut cur_y) = (0, 0);

        // Go through the text
        for c in text.chars() {
            match c {
                // If the current character is a space. Move over.
                ' ' => cur_x += self.space,
                //If the current character is a newline
                '\n' => {
                    // Move down
                    cur_y += self.new_line;

                    // Move back
                    cur_x = 0;
                }
                _ => {
                    // Get the ASCII value of the character
                    let ascii = c as usize;

                    // Show the character
                    self.bitmap.render(canvas, cur_x, cur_y, self.chars[ascii]);

                    // Move over the width of the character with one pixel of padding
                    cur_x += self.chars[ascii].w + 1;
                }
            }
        }
    }
}

fn main() {
    let sdl_ctx = sdl2::init().expect("Unable to initialize SDL!");
    let video = sdl_ctx.video().expect("Could not acquire video context!");
    sdl2::image::init(InitFlag::PNG).expect("Unable to initialize sdl2_image!");
    let mut event_pump = sdl_ctx
        .event_pump()
        .expect("Unable to obtain event pump handle!");

    sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "1");
    sdl2::hint::set("SDL_VIDEO_X11_NET_WM_BYPASS_COMPOSITOR", "0");

    let window = video
        .window("SDL Tutorial 41", 640, 480)
        .position_centered()
        .opengl()
        .build()
        .expect("Could not create SDL window!");

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .expect("Unable to obtain canvas!");

    let creator = canvas.texture_creator();
    canvas.set_draw_color(Color::WHITE);

    let font_texture = LTexture::from_file("resources/lesson41/lazyfont.png", &creator);
    let bitmap_font = BitmapFont::build_font(font_texture);

    main_loop::setup_mainloop(-1, true, move || {
        for event in event_pump.poll_iter() {
            if let Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } = event
            {
                return false;
            }
        }

        canvas.clear();

        bitmap_font.render_text(
            &mut canvas,
            "Bitmap Font:\nABDCEFGHIJKLMNOPQRSTUVWXYZ\nabcdefghijklmnopqrstuvwxyz\n0123456789",
        );

        canvas.present();
        true
    });
}
