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
            let connection_handler = get_tcp_connection_instance().read().unwrap();

            println!("Step 1: Acquired connection_handler lock.");

            // // 연결 존재 여부 확인
            // if connection_handler.is_exist_connection_by_address(_conn_info.clone()) {
            //     println!("Step 2: Connection exists.");
                
            //     // _pid를 안전하게 얻기
            //     let _pid = match connection_handler.get_id_by_connection(_conn_info.clone()) {
            //         Some(id) => id,
            //         None => {
            //             eprintln!("Failed to get ID by connection for address: {}. Aborting verify account.", _conn_info);
            //             // 이 경우, 연결은 있지만 ID를 찾지 못한 것이므로, 추가적인 처리가 필요할 수 있습니다.
            //             // 예를 들어, 해당 연결을 끊거나, 오류 메시지를 클라이언트에 보낼 수 있습니다.
            //             return; 
            //         }
            //     };
                
            //     let message_id = EventHeader::ALLOW_CONNECT_GAME as u32;
            //     let session_id = 0; 
                
            //     let mut _allow_connect_game = AllowConnectGame::new(
            //         message_id,
            //         session_id, 
            //         _pid as u32,
            //         _account_id, 
            //         _player_name, 
            //         _conn_info);

            //     println!("Step 3: Prepared AllowConnectGame message.");

            //     let _send_msg = _allow_connect_game.serialize();
                
            //     // 메시지 전송
            //     connection_handler.send_message_byte_to_target(
            //         _pid,
            //         _send_msg);

            //     println!("Step 4: Sent AllowConnectGame message to target.");
            // } else {
            //     println!("Connection not found for address: {}", _conn_info);
            //     // 연결이 없는 경우에 대한 추가적인 처리 (예: 에러 로그, 클라이언트에 거부 메시지)
            // }
            // `connection_handler` 변수가 이 함수의 스코프를 벗어나면서 락이 자동으로 해제됩니다.
        }
        Err(e) => {
            eprintln!("Failed to deserialize VerifyAccount: {}", e);
        }
    }
}