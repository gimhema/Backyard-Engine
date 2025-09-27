use crate::GameLogicMain;
use crate::Command;
use crate::qsm::user_message::message_movement;
use crate::GameLogic::game_geometry::Transform;
use crate::GameLogic::game_geometry::Position;
use crate::GameLogic::game_geometry::Rotation;
use crate::Event::event_handler::EventHeader;

impl GameLogicMain {

    pub fn do_command_move(&mut self, _command : Command) {
    if let Command::Move { entity_id, loc_x, loc_y, loc_z, q_x, q_y, q_z, q_w } = _command {

        self.game_world.update_movement(entity_id, Transform {
            position : Position { x: loc_x as f64, y: loc_y as f64, z: loc_z as f64 },
            rotation : Rotation { q_x: q_x as f64, q_y: q_y as f64, q_z: q_z as f64, q_w: q_w as f64 }
        });

        // // Broadcast to all clients
        let udp_message = message_movement::PlayerMovement {
            mid : EventHeader::PLAYER_MOVEMENT_UPDATE as u32,
            id: entity_id,
            x: loc_x,
            y: loc_y,
            z: loc_z,
            roll: q_x,
            pitch: q_y,
            yaw: q_z,
        };
        let serialized_message = udp_message.serialize();

        self.broadcast_msg_udp_all(serialized_message);

    }

    }

}