use crate::{components::*, MovementCommand};

const PLAYER_MOVEMENT_SPEED: i32 = 20;

impl Object<Player> {
    /// Actualiza el movimiento del jugador
    pub(crate) fn xmove(&mut self, movement_command: Option<MovementCommand>) {
        if let Some(movement_command) = movement_command {
            match movement_command {
                MovementCommand::Move(direction) => {
                    self.velocity.speed = PLAYER_MOVEMENT_SPEED;
                    self.velocity.direction = direction;
                }
                MovementCommand::Stop => self.velocity.speed = 0,
            }
        }
    }
}
