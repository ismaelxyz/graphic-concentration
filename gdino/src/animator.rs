use crate::components::*;

impl<T> Object<T> {
    /// Anima al objeto según la dirección a la cual se dirige.
    pub(crate) fn animator(&mut self) {
        use self::Direction::*;

        if self.velocity.speed > 0 {
            let frames = match self.velocity.direction {
                Left => &self.animation.left_frames,
                Right => &self.animation.right_frames,
                Up => &self.animation.up_frames,
                Down => &self.animation.down_frames,
            };

            self.animation.current_frame = (self.animation.current_frame + 1) % frames.len();
            self.frame = frames[self.animation.current_frame].clone();
        }
    }
}
