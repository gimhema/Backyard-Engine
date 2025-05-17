use crate::get_udp_server_instance;
use crate::get_tcp_server_instance;
use crate::qsm::user_message::message_new_player::*;
use crate::GameLogic::game_player::{get_ve_char_manager_instance, VECharcater};
use crate::qsm::user_message::message_enter_player_to_game::*;
use crate::qsm::user_message::message_server_response::{self, ServerResponse};

use super::GameLogic::*;
use super::GameLogic::game_logic_main::*;

pub fn CallBack_CreateNewPlayer(buffer: &[u8])
{
    match CreatePlayer::deserialize(buffer) {
        Ok(new_player_message) => {

            // 클라이언트로부터 캐릭터 생성 정보를 전달받음

            let _pid = new_player_message.id;
            let _name = new_player_message.name;
            let _conn_info = new_player_message.connect_info;

            println!("pid : {}", _pid);
            println!("player name : {}", _name);

            let mut _new_character = VECharcater::new_zero();
            
            _new_character.set_player_name(_name);
            
            get_ve_char_manager_instance().write().unwrap().new_character(_new_character);

            let mut _server_resp = ServerResponse::new(_pid.clone(), 0, "Create Character Sucessfull".to_string());
            let mut _resp_msg = _server_resp.serialize();

            get_tcp_server_instance().write().unwrap().send_message_byte_to_target(_pid.clone() as i64, _resp_msg);
        }
        Err(e)=>{
            eprintln!("Failed to deserialize MovementMessage: {}", e);
        }
    }
}

pub fn CallBack_EnterNewPlayerToGame(buffer: &[u8])
{
    match EnterPlayerToGame::deserialize(buffer) {
        Ok(entered_payer_message) => {
            let _pid = entered_payer_message.pid;
            let _accountId = entered_payer_message.accountId;
            let _name = entered_payer_message.name;
            let _conn_info = entered_payer_message.connect_info;

            {
                // G_GAME_LOGIC.lock().unwrap().
            }

        }
        Err(e) => {
            eprintln!("Failed to deserialize MovementMessage: {}", e);
        }
    }

}
