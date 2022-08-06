use sdl2::{
    event::{Event, WindowEvent},
    keyboard::Keycode,
    pixels::Color,
    render::Canvas,
    video::Window,
    Sdl, VideoSubsystem,
};

#[derive(Default)]
struct WindowState {
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
        let state = WindowState {
            shown: true,
            ..Default::default()
        };

        let win = video
            .window("SDL Tutorial 36", 650, 480)
            .resizable()
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let mut canvas = win.into_canvas().build().unwrap();
        canvas.set_draw_color(Color::WHITE);

        XWindow { canvas, state }
    }

    fn focus(&mut self) {
        //Restore window if needed
        if !self.state.shown {
            self.canvas.window_mut().show();
        }

        //Move window forward
        self.canvas.window_mut().raise();
    }

    fn render(&mut self) {
        self.canvas.clear();
        self.canvas.present();
    }

    fn handle_event(&mut self, event: &Event) -> bool {
        match event {
            Event::Quit { .. } => return true,
            Event::Window {
                win_event,
                window_id,
                ..
            } if *window_id == self.canvas.window().id() => {
                let mut update_caption: bool = false;

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
                            "SDL Tutorial - ID: {}; MouseFocus: {}; KeyboardFocus: {}",
                            window_id,
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

        false
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
    let mut windows = [
        XWindow::new(&video),
        XWindow::new(&video),
        XWindow::new(&video),
    ];
    let mut event_pump = context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            for window in &mut windows {
                if window.handle_event(&event) {
                    break 'running;
                }
            }

            if let Event::KeyDown {
                keycode: Some(k), ..
            } = event
            {
                match k {
                    Keycode::Escape => break 'running,
                    Keycode::Num1 => windows[0].focus(),
                    Keycode::Num2 => windows[1].focus(),
                    Keycode::Num3 => windows[2].focus(),
                    _ => (),
                }
            }
        }

        if windows
            .iter_mut()
            .map(|window| {
                window.render();
                window.state.shown
            })
            .collect::<Vec<_>>()
            .iter()
            .all(|b| !b)
        {
            break 'running;
        }
    }
}
