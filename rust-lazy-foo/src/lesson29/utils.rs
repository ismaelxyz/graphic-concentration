use sdl2::rect::Rect;
/// A circle stucture
#[derive(Copy, Clone)]
pub struct Circle {
    //     x,   y;
    pub pos: (i32, i32),
    pub rot: i32,
}

pub trait Collition<T> {
    fn check_collision(&self, _: T) -> bool;
}

impl Collition<Circle> for Circle {
    fn check_collision(&self, b: Circle) -> bool {
        // Calculate total radius squared
        let radius_squared = (self.rot + b.rot).pow(2);

        // If the distance between the centers of the circles is less than the sum of their radii
        distance_squared(self.pos.0, self.pos.1, b.pos.0, b.pos.1) < radius_squared
    }
}

impl Collition<Rect> for Circle {
    fn check_collision(&self, b: Rect) -> bool {
        // Find closest x offset
        let c_x = if self.pos.0 < b.x() {
            b.x()
        } else if self.pos.0 > b.x() + b.width() as i32 {
            b.x() + b.width() as i32
        } else {
            self.pos.0
        };

        // Find closest y offset
        let c_y = if self.pos.1 < b.y() {
            b.y()
        } else if self.pos.1 > b.y() + b.height() as i32 {
            b.y() + b.height() as i32
        } else {
            self.pos.1
        };

        // If the closest point is inside the circle
        distance_squared(self.pos.0, self.pos.1, c_x, c_y) < self.rot * self.rot
    }
}

fn distance_squared(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    let delta_x = x2 - x1;
    let delta_y = y2 - y1;
    delta_x * delta_x + delta_y * delta_y
}
