use crate::global::{rand, Screen, Speed};
use sdl2::{
    rect::{Point, Rect},
    render::Texture,
    render::WindowCanvas,
};
use std::ops::{Deref, DerefMut};

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Size {
    Tiny = 16,
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
        self.clip.clone()
    }

    pub fn scale(&self) -> f32 {
        self.scale.clone()
    }

    pub fn image(&self) -> &Texture {
        unsafe { &*self.image }
    }

    pub fn sub_image(&self) -> u16 {
        self.sub_image
    }

    /// Move an object relative coordinates (x and y) and set new object coordinates
    pub fn position(&mut self, pos: (i32, i32), canvas: &mut WindowCanvas) {
        let mut clip = self.clip.clone();
        self.pos = pos;
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

struct Asteroid {
    lives: i16,
}

pub struct Asteroids {
    image: *const Texture,
    objs: Vec<Object<Asteroid>>,
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
            let (clip, lives) = match rand() % 6 {
                random if random >= 3 => (Rect::new(0, 32, 32, 32), 1),
                random if random >= 1 => (Rect::new(32, 32, 64, 64), 3),
                _ => (Rect::new(96, 32, 96, 96), 6),
            };

            self.objs.push(Object {
                this: Asteroid { lives },
                pos: ((rand() % screen.width) - (clip.w / 2), -clip.h),
                ..unsafe { Object::new(Asteroid { lives }, &*self.image, 0, clip, 1.0) }
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

crate struct Bullet {
    lives: i16,
}

crate struct Ship {
    crate lives: i16,
    crate bullets: Vec<Object<Bullet>>,
}

impl Object<Ship> {
    crate fn update(&mut self) {
        self.update_bullets();
    }

    fn update_bullets(&mut self) {}
}
