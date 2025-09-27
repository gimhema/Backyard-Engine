use crate::Event::event_handler::EventHeader;

use super::{game_geometry::{Position, Rotation, Transform}, game_logic_main::*};
use super::qsm::user_message::message_movement;
use super::game_ecs::*;


impl GameLogicMain {

    pub fn do_command_create(&mut self, _command: Command) {
    if let Command::Create { entity_id } = _command {
        println!("Create entity {}", entity_id);

        self.game_world.create_entity(entity_id);
    }
}

pub fn do_command_delete(&mut self, _command: Command) {
    if let Command::Create { entity_id } = _command {
        println!("Create entity {}", entity_id);

        self.game_world.delete_entity(entity_id);
    }
}


}


