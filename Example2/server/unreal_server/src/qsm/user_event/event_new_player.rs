// use crate::get_udp_server_instance;

use std::sync::Arc;

use crate::qsm::user_message::message_new_player::*;
use crate::Event::event_handler::EventHeader;
use crate::GameLogic::game_player::{get_ve_char_manager_instance, VECharcater};
use crate::qsm::user_message::message_enter_player_to_game::*;
use crate::qsm::user_message::message_allow_connect::*;
use crate::qsm::user_message::message_server_response::{self, ServerResponse};

use super::GameLogic::game_logic_main::*;

use super::Network::server_common::*;

pub fn CallBack_CreateNewPlayer(buffer: &[u8])
{

}

pub fn CallBack_EnterNewPlayerToGame(buffer: &[u8])
{

}

pub fn CallBack_AllowConnectGame(buffer: &[u8])
{
    match AllowConnectGame::deserialize(buffer) {
        Ok(allow_connect_message) => {
            let _pid = allow_connect_message.pid.to_string();
            let _account_id = allow_connect_message.accountId.clone();
            let _player_name = allow_connect_message.name.clone();
            let _conn_info = allow_connect_message.connect_info.clone();
            
            
            println!("CallBack_AllowConnectGame : PID : {}, Account ID : {}, Player Name : {}, Conn: {}",
                     _pid, _account_id, _player_name, _conn_info);



        }
        Err(e) => {
            eprintln!("Failed to deserialize AllowConnectGame: {}", e);
        }
    }
}