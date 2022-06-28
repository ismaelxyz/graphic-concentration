use crate::global::{rand, Screen, Speed, Timer};
use sdl2::{
    keyboard::Scancode,
    rect::{Point, Rect},
    render::{Texture, WindowCanvas},
    EventPump, TimerSubsystem,
};

use std::ops::{Deref, DerefMut};

type ScreenItems<'a> = (
    Screen,
    Speed,
    &'a mut Timer,
    &'a EventPump,
    &'a mut WindowCanvas,
    &'a TimerSubsystem,
);

#[derive(Copy, Clone, Eq, PartialEq)]
#[allow(dead_code)]
pub enum Size {
    Tiny = 16, //  field is never read
    Small = 32,
    Medium = 48,
    Large = 64,
}

/// Definition for game objects
pub struct Object<T> {
    this: T,
    // kind: ObjKind,
    // lives: i16,
    // next: Option<Box<Object>>
    image: *const Texture,
    pub clip: Rect,
    // sub_image,
    sub_image: u16,            // Necesario para text?, sera movido de otra forma!
    pub sub_image_number: u16, // Necesario?
    //     x    y
    pub pos: (i32, i32),
    pub scale: f32,
}

impl<T> Object<T> {
    pub fn new(this: T, image: &Texture, sub_image: u16, clip: Rect, scale: f32) -> Self {
        Object {
            this,
            image,
            sub_image, // : 0?
            sub_image_number: 3,
            pos: (0, 0),
            scale,
            clip,
        }
    }

    pub fn clip(&self) -> Rect {
        self.clip
    }

    pub fn scale(&self) -> f32 {
        self.scale
    }

    pub fn image(&self) -> &Texture {
        unsafe { &*self.image }
    }

    pub fn sub_image(&self) -> u16 {
        self.sub_image
    }

    /// Move an object relative coordinates (x and y) and set new object coordinates
    pub fn position(&mut self, pos: (i32, i32), canvas: &mut WindowCanvas) {
        let mut clip = self.clip;
        self.pos.0 += pos.0;
        self.pos.1 += pos.1;
        clip.x += clip.w * self.sub_image as i32;
        self.apply_texture(clip, canvas);
    }

    /// Check object collisions
    pub fn is_collision<R>(&self, rhs: &Object<R>) -> bool {
        self.pos.0 as f32 + self.clip.w as f32 * self.scale >= rhs.pos.0 as f32
            && self.pos.1 as f32 + self.clip.h as f32 * self.scale >= rhs.pos.1 as f32
            && rhs.pos.0 as f32 + rhs.clip.w as f32 * rhs.scale >= self.pos.0 as f32
            && rhs.pos.1 as f32 + rhs.clip.h as f32 * rhs.scale >= self.pos.1 as f32
    }

    fn apply_texture_ex(
        &self,
        clip: Rect,
        angle: f64,
        center: Option<Point>,
        flip: (bool, bool),
        canvas: &mut WindowCanvas,
    ) {
        let mut offset = clip;

        offset.x = self.pos.0;
        offset.y = self.pos.1;
        offset.w *= self.scale as i32;
        offset.h *= self.scale as i32;

        canvas
            .copy_ex(
                self.image(),
                Some(clip),
                Some(offset),
                angle,
                center,
                flip.0,
                flip.1,
            )
            .expect("Could not blit texture to render target!");
    }

    pub fn apply_texture(&self, clip: Rect, canvas: &mut WindowCanvas) {
        self.apply_texture_ex(clip, 0.0, None, (false, false), canvas);
    }
}

impl<T: Sized> Deref for Object<T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &T {
        &self.this
    }
}

impl<T: Sized> DerefMut for Object<T> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut T {
        &mut self.this
    }
}

fn get_x(chr: &u8) -> i32 {
    match *chr as i32 {
        c if c <= 43 => c - 32,
        c if c <= 55 => c - 44,
        c if c <= 67 => c - 56,
        c if c <= 79 => c - 68,
        c if c <= 91 => c - 80,
        c if c <= 103 => c - 92,
        c if c <= 115 => c - 104,
        c => c - 116,
    }
}

fn get_y(c: &u8) -> i32 {
    match *c as i32 {
        c if c <= 43 => 0,
        c if c <= 55 => 1,
        c if c <= 67 => 2,
        c if c <= 79 => 3,
        c if c <= 91 => 4,
        c if c <= 103 => 5,
        c if c <= 115 => 6,
        _ => 7,
    }
}

pub struct Text {
    chucks: Vec<Object<()>>,
}

impl Text {
    pub fn new(image: &Texture, text: &str, size: Size, scale: f32) -> Self {
        let ([width, height], [w, h]) = ([size as i32; 2], [size as u32; 2]);
        let mut chucks = Vec::new();

        for chr in text.as_bytes() {
            chucks.push(Object::new(
                (),
                image,
                0,
                Rect::new(get_x(chr) * width, get_y(chr) * height, w, h),
                scale,
            ));
        }

        Text { chucks }
    }

    pub fn position(&mut self, (x, y): (i32, i32), canvas: &mut WindowCanvas) {
        let (mut i, x2) = (0.0f32, x as f32);

        for obj in &mut self.chucks {
            let mut clip = obj.clip();
            obj.pos.0 = (x2 + ((obj.clip().width() as f32 * 0.5 * obj.scale) * i)) as i32;
            i += 1.0;
            obj.pos.1 = y;
            clip.x += clip.w * obj.sub_image() as i32;
            obj.apply_texture(clip, canvas);
        }
    }

    pub fn chucks(&self) -> &[Object<()>] {
        &self.chucks
    }
}

pub struct Asteroid {
    pub(crate) lives: i16,
    pub(crate) kind: Size,
}

pub struct Asteroids {
    image: *const Texture,
    pub(crate) objs: Vec<Object<Asteroid>>,
}

impl Asteroids {
    pub fn new(image: &Texture) -> Self {
        Asteroids {
            image,
            objs: Vec::new(),
        }
    }

    pub fn update(&mut self, screen: Screen, speed: &mut Speed, canvas: &mut WindowCanvas) {
        let Speed { asteroid, ship, .. } = speed;
        *asteroid *= if *asteroid < *ship * 2.0 {
            1.00025
        } else {
            1.0
        };

        let val = if *asteroid < 30.0 { *asteroid } else { 30.0 };

        if rand() % val as i32 == 0 {
            let (clip, lives, kind) = match rand() % 6 {
                random if random >= 3 => (Rect::new(0, 32, 32, 32), 1, Size::Small),
                random if random >= 1 => (Rect::new(32, 32, 64, 64), 3, Size::Medium),
                _ => (Rect::new(96, 32, 96, 96), 6, Size::Large),
            };

            self.objs.push(Object {
                this: Asteroid { lives, kind },
                pos: ((rand() % screen.width) - (clip.w / 2), -clip.h),
                ..unsafe { Object::new(Asteroid { lives, kind }, &*self.image, 0, clip, 1.0) }
            });
        }

        let mut pos = 0;

        while self.objs.len() > pos {
            let this = &mut self.objs[pos];
            if this.pos.1 > screen.height + this.clip.h || this.lives <= 0 {
                self.objs.remove(pos);
                continue;
            }

            this.position((0, speed.asteroid as _), canvas);

            pos += 1;
        }
    }
}

trait Pressed<T> {
    fn is_pressed(&self, code: T) -> bool;
}

impl Pressed<Scancode> for EventPump {
    fn is_pressed(&self, code: Scancode) -> bool {
        self.keyboard_state().is_scancode_pressed(code)
    }
}

impl Pressed<[Scancode; 2]> for EventPump {
    fn is_pressed(&self, [code, code1]: [Scancode; 2]) -> bool {
        self.is_pressed(code) || self.is_pressed(code1)
    }
}

pub(crate) struct Bullet {
    pub(crate) lives: i16,
}

pub(crate) struct Ship {
    pub(crate) lives: i16,
    pub(crate) bullets: Vec<Object<Bullet>>,
}

impl Object<Ship> {
    pub(crate) fn update(&mut self, (screen, speed, timer, events, canvas, tm_sys): ScreenItems) {
        let (mut ship_x, mut ship_y) = (0i8, 0i8);
        let mut temp: i32;

        /* User Keyboard  */
        if events.is_pressed([Scancode::Left, Scancode::A]) {
            ship_x -= 1;
        }

        if events.is_pressed([Scancode::Right, Scancode::D]) {
            ship_x += 1;
        }

        if events.is_pressed([Scancode::Up, Scancode::W]) {
            ship_y -= 1;
        }

        if events.is_pressed([Scancode::Down, Scancode::S]) {
            ship_y += 1;
        }

        /* Updating Ship Animation */
        if ship_x == 0 {
            self.sub_image = 0;
        } else if ship_x == -1 {
            self.sub_image = 1;
        } else if ship_x == 1 {
            self.sub_image = 2;
        }

        ship_x *= speed.ship as i8;
        ship_y *= speed.ship as i8;

        /* Setting Ship Boundaries */
        if (self.pos.0 + ship_x as i32) < screen.left as i32 {
            temp = screen.left as i32 - self.pos.0;
            ship_x = if temp > 0 { temp as _ } else { 0 };
        }

        if self.pos.0 + self.clip.w + ship_x as i32 > screen.width - screen.right as i32 {
            temp = (screen.width - screen.right as i32) - self.pos.0 - self.clip.w;
            ship_x = if temp > 0 { temp as _ } else { 0 };
        }

        if (self.pos.1 + ship_y as i32) < screen.top as i32 {
            temp = screen.top as i32 - self.pos.1;
            ship_y = if temp > 0 { temp as _ } else { 0 };
        }

        if self.pos.1 + self.clip.h + ship_y as i32 > screen.height - screen.bottom as i32 {
            temp = (screen.height - screen.bottom as i32) - self.pos.1 - self.clip.h;
            ship_y = if temp > 0 { temp as _ } else { 0 };
        }

        self.position((ship_x as _, ship_y as _), canvas);
        self.update_bullets((screen, speed, timer, events, canvas, tm_sys));
    }

    fn update_bullets(&mut self, (screen, speed, timer, events, canvas, tm_sys): ScreenItems) {
        if timer.bullet < tm_sys.ticks() && events.is_pressed([Scancode::Num1, Scancode::Space]) {
            let mut bullet = Object::new(
                Bullet { lives: 1 },
                self.image(),
                0,
                Rect::new(0, 144, 16, 16),
                1.0,
            );

            bullet.pos = (
                self.pos.0 + (bullet.clip.w / 2),
                self.pos.1 - (bullet.clip.w / 2),
            );

            self.bullets.push(bullet);
            timer.bullet = tm_sys.ticks() + 150;
        }

        let mut pos = 0;

        while self.bullets.len() > pos {
            let this = &mut self.bullets[pos];

            if this.pos.1 <= screen.top as i32 || this.lives <= 0 {
                self.bullets.remove(pos);
                continue;
            }

            this.position((0, -(speed.bullet as i32)), canvas);

            pos += 1;
        }
    }
}
