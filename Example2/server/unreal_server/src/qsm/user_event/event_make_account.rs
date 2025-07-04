use crate::get_tcp_server_instance;
use crate::qsm::user_message::message_make_account::{self, MakeAccount};
use crate::qsm::user_message::message_verify_account::{self, VerifyAccount};
use crate::qsm::user_message::message_allow_connect::{self, AllowConnectGame};
use crate::Event::event_handler::EventHeader;
use crate::GameLogic::game_player::{get_ve_char_manager_instance, VECharcater};

use crate::Network::connection::get_tcp_connection_instance;
use crate::Network::connection::connection_handle;

use super::GameLogic::*;

pub fn CallBack_MakeAccount(buffer: &[u8])
{
    match MakeAccount::deserialize(buffer) {
        Ok(make_account_message) => {

        }
        Err(e)=>{
            eprintln!("Failed to deserialize MakeAccount: {}", e);
        }
    }
}

pub fn CallBack_VerifyAccount(buffer: &[u8])
{
    match VerifyAccount::deserialize(buffer) {
        Ok(verify_account_message) => {
            let _accountId = verify_account_message.userId.clone();
            let _password = verify_account_message.password.clone();
            let _player_name = verify_account_message.userName.clone();
            let _conn = verify_account_message.connect_info.clone();

            println!("CallBack_VerifyAccount : Account ID : {}, PassWord : {}, Player Name : {}, Conn: {}",
        _accountId.clone(), _password.clone(), _player_name.clone(), _conn.clone());

            let mut _pid = get_tcp_connection_instance().write().unwrap().get_id_by_connection(_conn.clone()).unwrap();
            
            if(true == get_tcp_connection_instance().write().unwrap().is_exist_connection_by_address(_conn.clone()))
            {
                let message_id = EventHeader::ALLOW_CONNECT_GAME as u32;
                let session_id = 0; // 서버가 지정
                let _account_id = verify_account_message.userId.clone();
                let _player_name = verify_account_message.userName.clone();
                let _conn_info = verify_account_message.connect_info.clone();

                let mut _allow_connect_game = AllowConnectGame::new(
                    message_id,
                    session_id, 
                    _pid.clone() as u32,
                    _account_id, 
                    _player_name, 
                    _conn_info);

                let mut _send_msg = _allow_connect_game.serialize();
                get_tcp_connection_instance().write().unwrap().send_message_byte_to_target(
                    _pid.clone() as i64,
                    _send_msg);

            }
            else
            {
                println!("Connection not found for address: {}", _conn);
            }

        }
        Err(e) => {
            eprintln!("Failed to deserialize VerifyAccount: {}", e);
        }
    }
}