use crate::{components::*, MovementCommand};

pub struct Player {}

impl Player {
    const MOVEMENT_SPEED: i32 = 20;
}

impl Object<Player> {
    /// Update the player movement
    pub(crate) fn xmove(&mut self, movement_command: Option<MovementCommand>) {
        if let Some(movement_command) = movement_command {
            match movement_command {
                MovementCommand::Move(direction) => {
                    self.velocity.speed = Player::MOVEMENT_SPEED;
                    self.velocity.direction = direction;
                }
                MovementCommand::Stop => self.velocity.speed = 0,
            }
        }
    }
}
