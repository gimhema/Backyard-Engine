use crate::Event::event_handler::EventHeader;

use super::{game_geometry::{Position, Rotation, Transform}, game_logic_main::*};
use super::qsm::user_message::message_movement;


pub fn do_command_create(_command: Command) {
    if let Command::Create { entity_id } = _command {
        println!("Create entity {}", entity_id);

        // 일단 단일월드라고 가정

        // let mut game_logic = G_GAME_LOGIC.lock().unwrap();
        // game_logic.game_world.create_entity(entity_id);
    }
}

pub fn do_command_delete(_command: Command) {
    if let Command::Create { entity_id } = _command {
        println!("Create entity {}", entity_id);

        // 일단 단일월드라고 가정

        // let mut game_logic = G_GAME_LOGIC.lock().unwrap(); 
        // game_logic.game_world.delete_entity(entity_id);
    }
}

pub fn do_command_move(_command : Command) {
    if let Command::Move { entity_id, loc_x, loc_y, loc_z, q_x, q_y, q_z, q_w } = _command {

        // let mut game_logic = G_GAME_LOGIC.lock().unwrap();

        // let update_mov = Transform::new(
        //     Position::new(loc_x as f64, loc_y as f64, loc_z as f64),
        //     Rotation::new(q_x as f64, q_y as f64, q_z as f64, q_w as f64));

        // game_logic.game_world.update_movement(entity_id, update_mov);

        // // Broadcast to all clients
        // let udp_message = message_movement::PlayerMovement {
        //     mid : EventHeader::PLAYER_MOVEMENT_UPDATE as u32,
        //     id: entity_id,
        //     x: loc_x,
        //     y: loc_y,
        //     z: loc_z,
        //     roll: q_x,
        //     pitch: q_y,
        //     yaw: q_z,
        // };
        // let serialized_message = udp_message.serialize();

        

    }
}
