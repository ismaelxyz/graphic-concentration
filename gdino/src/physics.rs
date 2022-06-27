use crate::components::*;

impl<T> Object<T> {
    pub(crate) fn physics(&mut self) {
        use self::Direction::*;

        self.position.0 = match self.velocity.direction {
            Left => self.position.0.offset(-self.velocity.speed, 0),
            Right => self.position.0.offset(self.velocity.speed, 0),
            Up => self.position.0.offset(0, -self.velocity.speed),
            Down => self.position.0.offset(0, self.velocity.speed),
        };
    }
}
