use crate::get_udp_server_instance;
use crate::qsm::user_message::message_new_player::*;
use crate::GameLogic::game_player::{get_ve_char_manager_instance, VECharcater};

use super::GameLogic::*;

pub fn CallBack_CreateNewPlayer(buffer: &[u8])
{
    match NewPlayer::deserialize(buffer) {
        Ok(new_player_message) => {
            let _pid = new_player_message.pid;
            let _name = new_player_message.name;

            println!("pid : {}", _pid);
            println!("player name : {}", _name);

            let mut _new_character = VECharcater::new_zero();
            
            _new_character.set_player_name(_name);
            
            get_ve_char_manager_instance().write().unwrap().new_character(_new_character);
        }
        Err(e)=>{
            eprintln!("Failed to deserialize MovementMessage: {}", e);
        }
    }
}