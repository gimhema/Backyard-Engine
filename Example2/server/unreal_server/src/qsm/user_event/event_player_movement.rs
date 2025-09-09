// use crate::get_udp_server_instance;
use crate::qsm::{qsm::GLOBAL_MESSAGE_UDP_QUEUE, user_message::message_movement::{self, PlayerMovement}};

use super::GameLogic::game_logic_main::*;

pub fn CallBack_PlayerMovementUpdate(buffer: &[u8])
{
    match PlayerMovement::deserialize(buffer ) {
        Ok(movement_message) => {
            let sender = movement_message.id;
            let loc_x = movement_message.x;
            let loc_y = movement_message.y;
            let loc_z = movement_message.z;
            let roll = movement_message.roll;
            let pitch = movement_message.pitch;
            let yaw = movement_message.yaw;


            G_GAME_LOGIC.lock().unwrap().push_command(
                Command::Move { entity_id: sender, loc_x, loc_y, loc_z, q_x: roll, q_y: pitch, q_z: yaw, q_w: 0.0 }
            );


        }
        Err(e) => {
            eprintln!("Failed to deserialize MovementMessage: {}", e);
        }
    }
}



