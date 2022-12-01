use crate::components::*;
use rand::prelude::{thread_rng, Rng};

pub struct Enemy {
}

impl Enemy {
    const MOVEMENT_SPEED: i32 = 10;
}

impl Object<Enemy> {
    /// Movimiento "Inteligente" de los personajes.
    pub(crate) fn ai(&mut self) {
        let mut rng = thread_rng();

        if rng.gen_range(0..10usize) == 0 {
            self.velocity.speed = Enemy::MOVEMENT_SPEED;
            // Safety: La cantidad de valores representables iguala al rango dado.
            self.velocity.direction =
                unsafe { *(&rng.gen_range(0..4usize) as *const _ as *const Direction) };
        }
    }
}
