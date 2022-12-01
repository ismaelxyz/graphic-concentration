use rand::Rng;

use crate::geometry::{Point, Size};

/// A `Vector`
#[derive(Clone, Default)]
pub struct Vector {
    /// The position of the vector
    pub position: Point,
    /// The direction angle, in radians
    pub direction: f64,
}

impl Vector {
    /// Returns a new `Vector`
    pub fn new(position: Point, direction: f64) -> Vector {
        Vector {
            position,
            direction,
        }
    }

    /// Returns a random `Vector` within the given bounds
    pub fn random<R: Rng>(rng: &mut R, bounds: Size) -> Vector {
        Vector::new(Point::random(rng, bounds), rng.gen())
    }

    /// Consumes the vector and returns a new one with inverted direction
    pub fn invert(mut self) -> Vector {
        self.direction -= std::f64::consts::PI;
        self
    }
}

/// A macro to implement `Position` and `Direction` for any type that has a field named `vector`
#[macro_export]
#[allow(clippy::crate_in_macro_def)]
macro_rules! derive_position_direction {
    ($t:ty) => {
        impl crate::geometry::Position for $t {
            fn x(&self) -> f64 {
                self.vector.position.x
            }
            fn x_mut(&mut self) -> &mut f64 {
                &mut self.vector.position.x
            }
            fn y(&self) -> f64 {
                self.vector.position.y
            }
            fn y_mut(&mut self) -> &mut f64 {
                &mut self.vector.position.y
            }
        }

        impl crate::geometry::Advance for $t {
            fn direction(&self) -> f64 {
                self.vector.direction
            }

            fn direction_mut(&mut self) -> &mut f64 {
                &mut self.vector.direction
            }
        }
    };
}
