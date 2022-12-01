use sdl2::{
    event::{Event, WindowEvent},
    image::{InitFlag, LoadSurface},
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
    render::{Canvas, Texture as RTexture, TextureCreator},
    surface::Surface,
    video::{FullscreenType, Window, WindowContext},
};

use std::path::Path;

struct Texture {
    texture: RTexture,
    width: u32,
    height: u32,
}

impl Texture {
    fn from_file(creator: &TextureCreator<WindowContext>, path: &Path) -> Texture {
        // Load the surface first, so we can set the color key
        let mut surface = Surface::from_file(path).expect("Could not load surface from file!");

        // Now set the color key on the surface
        surface
            .set_color_key(true, Color::RGB(0, 0xff, 0xff))
            .expect("Could not set color_key on surface!");

        let texture = creator
            .create_texture_from_surface(&surface)
            .expect("Could not create texture from surface!");

        Texture {
            width: texture.query().width,
            height: texture.query().height,
            texture,
        }
    }

    fn render(&self, canvas: &mut Canvas<Window>, x: i32, y: i32) {
        canvas
            .copy_ex(
                &self.texture,
                None,
                Some(Rect::new(x, y, self.width, self.height)),
                0.0,
                None,
                false,
                false,
            )
            .unwrap();
    }
}

#[derive(Default)]
struct LWindow {
    is_keyboard_focus: bool,
    is_mouse_focus: bool,
    is_minimized: bool,
    width: i32,
    height: i32,
}

fn main() {
    let sdl_ctx = sdl2::init().unwrap();
    let video = sdl_ctx.video().unwrap();

    sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "1");
    sdl2::hint::set("SDL_VIDEO_X11_NET_WM_BYPASS_COMPOSITOR", "0");
    let mut window = LWindow {
        width: 650,
        height: 480,
        ..LWindow::default()
    };
    let canvas_window = video
        .window("SDL Tutorial 35", window.width as u32, window.height as u32)
        .resizable()
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    sdl2::image::init(InitFlag::PNG).unwrap();

    let mut canvas = canvas_window.into_canvas().build().unwrap();

    let creator = canvas.texture_creator();

    let mut event_pump = sdl_ctx.event_pump().unwrap();
    let scene_texture = Texture::from_file(&creator, Path::new("resources/lesson35/window.png"));

    canvas.set_draw_color(Color::WHITE);

    main_loop::setup_mainloop(-1, true, move || {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => return false,
                Event::Window { win_event, .. } => {
                    let mut update_caption: bool = false;

                    match win_event {
                        WindowEvent::SizeChanged(width, height) => {
                            window.width = width;
                            window.height = height;
                            canvas.present()
                        }
                        WindowEvent::Exposed => canvas.present(),
                        WindowEvent::Enter => {
                            window.is_mouse_focus = true;
                            update_caption = true;
                        }
                        WindowEvent::Leave => {
                            window.is_mouse_focus = false;
                            update_caption = true;
                        }
                        WindowEvent::FocusGained => {
                            window.is_keyboard_focus = true;
                            update_caption = true;
                        }
                        WindowEvent::FocusLost => {
                            window.is_keyboard_focus = false;
                            update_caption = true;
                        }
                        WindowEvent::Minimized => window.is_minimized = true,
                        WindowEvent::Maximized => window.is_minimized = false,
                        WindowEvent::Restored => window.is_minimized = false,
                        _ => (),
                    }

                    if update_caption {
                        canvas
                            .window_mut()
                            .set_title(&format!(
                                "SDL Tutorial 35 - MouseFocus: {}; KeyboardFocus: {}",
                                if window.is_mouse_focus { "On" } else { "Off" },
                                if window.is_keyboard_focus {
                                    "On"
                                } else {
                                    "Off"
                                }
                            ))
                            .unwrap();
                    }
                }
                Event::KeyDown {
                    keycode: Some(k), ..
                } => match k {
                    Keycode::Escape => return false,
                    Keycode::Return => {
                        if canvas.window().fullscreen_state() == FullscreenType::Desktop {
                            window.is_minimized = false;
                            canvas.window_mut().set_fullscreen(FullscreenType::Off)
                        } else {
                            canvas.window_mut().set_fullscreen(FullscreenType::Desktop)
                        }
                        .unwrap();
                    }
                    _ => (),
                },
                _ => (),
            }
        }

        if !window.is_minimized {
            canvas.clear();
            scene_texture.render(
                &mut canvas,
                (window.width - scene_texture.width as i32) / 2,
                (window.height - scene_texture.height as i32) / 2,
            );

            canvas.present();
        }

        true
    });
}
