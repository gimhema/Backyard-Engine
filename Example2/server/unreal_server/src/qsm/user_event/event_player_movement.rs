use crate::get_udp_server_instance;
use crate::qsm::user_message::message_movement::{self, PlayerMovement};

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

            println!("sender : {}", sender);
            println!("location x : {}, y : {}, z : {}",
             loc_x, loc_y, loc_z);
            println!("euler rotation roll : {}, pitch : {}, yaw : {}",
             roll, pitch, yaw);

             
             get_udp_server_instance().write().unwrap().send_message_to_all_conn(buffer);
        }
        Err(e) => {
            eprintln!("Failed to deserialize MovementMessage: {}", e);
        }
    }
}



