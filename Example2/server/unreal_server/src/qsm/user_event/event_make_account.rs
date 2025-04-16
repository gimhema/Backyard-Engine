use crate::get_tcp_server_instance;
use crate::qsm::user_message::message_make_account::{self, MakeAccount};
use crate::GameLogic::game_player::{get_ve_char_manager_instance, VECharcater};

use super::GameLogic::*;

pub fn CallBack_MakeAccount(buffer: &[u8])
{
    match MakeAccount::deserialize(buffer) {
        Ok(new_player_message) => {

        }
        Err(e)=>{
            eprintln!("Failed to deserialize MovementMessage: {}", e);
        }
    }
}