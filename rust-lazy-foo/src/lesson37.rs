use sdl2::{
    event::{Event, WindowEvent},
    keyboard::Keycode,
    pixels::Color,
    render::Canvas,
    video::{Window, WindowPos},
    Sdl, VideoSubsystem,
};

// Screen dimension
const WIDTH: i32 = 640;
const HEIGHT: i32 = 480;

#[derive(Default)]
struct WindowState {
    display_id: usize,
    is_keyboard_focus: bool,
    is_mouse_focus: bool,
    is_minimized: bool,
    shown: bool,
}

struct XWindow {
    canvas: Canvas<Window>,
    state: WindowState,
}

impl XWindow {
    fn new(video: &VideoSubsystem) -> Self {
        let win = video
            .window("SDL Tutorial 37", 650, 480)
            .resizable()
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let mut canvas = win.into_canvas().build().unwrap();
        canvas.set_draw_color(Color::WHITE);

        XWindow {
            state: WindowState {
                shown: true,
                display_id: canvas.window().display_index().unwrap() as usize,
                ..Default::default()
            },
            canvas,
        }
    }

    fn render(&mut self) {
        self.canvas.clear();
        self.canvas.present();
    }

    fn handle_event(&mut self, event: &Event, mut update_caption: bool) {
        match event {
            Event::Window {
                win_event,
                window_id,
                ..
            } if *window_id == self.canvas.window().id() => {
                match win_event {
                    WindowEvent::Shown => self.state.shown = true,
                    WindowEvent::Hidden => self.state.shown = false,
                    WindowEvent::SizeChanged(..) => self.canvas.present(),
                    WindowEvent::Exposed => self.canvas.present(),
                    WindowEvent::Enter => {
                        self.state.is_mouse_focus = true;
                        update_caption = true;
                    }
                    WindowEvent::Leave => {
                        self.state.is_mouse_focus = false;
                        update_caption = true;
                    }
                    WindowEvent::FocusGained => {
                        self.state.is_keyboard_focus = true;
                        update_caption = true;
                    }
                    WindowEvent::FocusLost => {
                        self.state.is_keyboard_focus = false;
                        update_caption = true;
                    }
                    WindowEvent::Minimized => self.state.is_minimized = true,
                    WindowEvent::Maximized => self.state.is_minimized = false,
                    WindowEvent::Restored => self.state.is_minimized = false,
                    WindowEvent::Close => self.canvas.window_mut().hide(),
                    _ => (),
                }

                if update_caption {
                    self.canvas
                        .window_mut()
                        .set_title(&format!(
                            "SDL Tutorial - ID: {} Display: {} MouseFocus: {} KeyboardFocus: {}",
                            window_id,
                            self.state.display_id,
                            if self.state.is_mouse_focus {
                                "On"
                            } else {
                                "Off"
                            },
                            if self.state.is_keyboard_focus {
                                "On"
                            } else {
                                "Off"
                            }
                        ))
                        .unwrap();
                }
            }
            _ => (),
        }
    }
}

fn init() -> (Sdl, VideoSubsystem) {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();

    sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "1");
    sdl2::hint::set("SDL_VIDEO_X11_NET_WM_BYPASS_COMPOSITOR", "0");

    (sdl, video)
}

fn main() {
    let (context, video) = init();

    let total_displays = video.num_video_displays().unwrap() as usize;
    if total_displays < 2 {
        eprintln!("Warning: Only one display connected!");
    }

    let display_bounds = (0..total_displays)
        .map(|i| video.display_bounds(i as i32))
        .collect::<Result<Vec<_>, String>>()
        .unwrap();
    let mut window = XWindow::new(&video);
    let mut event_pump = context.event_pump().unwrap();

    'running: loop {
        let mut update_caption: bool = false;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(k), ..
                } => match k {
                    Keycode::Escape => break 'running,
                    k @ (Keycode::Up | Keycode::Down) => {
                        update_caption = true;

                        if k == Keycode::Up {
                            if window.state.display_id + 1 == total_displays {
                                window.state.display_id = 0;
                            } else {
                                window.state.display_id += 1;
                            }
                        } else if window.state.display_id.overflowing_sub(1).1 {
                            window.state.display_id = total_displays - 1;
                        } else {
                            window.state.display_id -= 1;
                        }

                        let bound = display_bounds[window.state.display_id];
                        window.canvas.window_mut().set_position(
                            WindowPos::Positioned(bound.x + (bound.w - WIDTH) / 2),
                            WindowPos::Positioned(bound.y + (bound.h - HEIGHT) / 2),
                        );
                    }
                    _ => (),
                },
                _ => (),
            }

            window.handle_event(&event, update_caption);
        }

        window.render();
    }
}
