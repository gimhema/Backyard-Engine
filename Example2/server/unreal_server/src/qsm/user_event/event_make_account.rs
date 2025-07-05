use crate::get_tcp_server_instance;
use crate::qsm::user_message::message_make_account::{self, MakeAccount};
use crate::qsm::user_message::message_verify_account::{self, VerifyAccount};
use crate::qsm::user_message::message_allow_connect::{self, AllowConnectGame};
use crate::Event::event_handler::EventHeader;
use crate::GameLogic::game_player::{get_ve_char_manager_instance, VECharcater};

use crate::Network::connection::get_tcp_connection_instance;
use crate::Network::message_queue::get_callback_msg_queue_instance;
use crate::Network::server_common::get_user_connection_info;
use crate::Network::connection::connection_handle;

use super::GameLogic::*;

pub fn CallBack_MakeAccount(buffer: &[u8])
{
    match MakeAccount::deserialize(buffer) {
        Ok(make_account_message) => {
            // 여기에 MakeAccount 메시지 처리 로직을 추가하세요.
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
            let _account_id = verify_account_message.userId.clone();
            let _password = verify_account_message.password.clone();
            let _player_name = verify_account_message.userName.clone();
            let _conn_info = verify_account_message.connect_info.clone(); 

            println!("CallBack_VerifyAccount : Account ID : {}, PassWord : {}, Player Name : {}, Conn: {}",
                _account_id, _password, _player_name, _conn_info);

            // 여기에 단 한 번만 락을 획득합니다.
            let connection_info = get_user_connection_info().read().unwrap();

            println!("Step 1: Acquired connection_handler lock.");
            
            let _send_token = connection_info.get_token_by_ip(&_conn_info.clone());
            if _send_token.is_none() {
                eprintln!("No connection found for address: {}", _conn_info);
                return; // 연결이 없는 경우, 함수 종료
            }
            let _send_id = connection_info.find_id_by_token(_send_token.unwrap());
            if _send_id.is_none() {
                eprintln!("No ID found for token: {:?}", _send_token);
                return; 
            }
            let _pid = _send_id.unwrap();

                
            let message_id = EventHeader::ALLOW_CONNECT_GAME as u32;
            let session_id = 0; 
                
            let mut _allow_connect_game = AllowConnectGame::new(
                message_id,
                session_id, 
                _pid as u32,
                _account_id, 
                _player_name, 
                _conn_info);

            println!("Step 3: Prepared AllowConnectGame message.");

            let _send_msg = _allow_connect_game.serialize();
                
            //     // 메시지 전송
            get_callback_msg_queue_instance().write().unwrap()
                .push_message(_send_token.unwrap(), _send_msg);

            println!("Step 4: Sent AllowConnectGame message to target.");
        }
        Err(e) => {
            eprintln!("Failed to deserialize VerifyAccount: {}", e);
        }
    }
}