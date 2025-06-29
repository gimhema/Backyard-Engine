use crate::get_tcp_server_instance;
use crate::qsm::user_message::message_make_account::{self, MakeAccount};
use crate::qsm::user_message::message_verify_account::{self, VerifyAccount};
use crate::qsm::user_message::message_allow_connect::{self, AllowConnectGame};
use crate::Event::event_handler::EventHeader;
use crate::GameLogic::game_player::{get_ve_char_manager_instance, VECharcater};

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

            let mut server = get_tcp_server_instance().write().unwrap();
            let mut _pid = server.get_pid_by_connection(_conn.clone());
            let mut _pid_copy = _pid.unwrap().clone() as u32;

            if (true == get_tcp_server_instance().write().unwrap().is_exist_connection(_conn)) 
            {
                // 클라이언트에게 답장 회신
                // 아직 id 검증 로직은 넣지말고

                // 클라는 답장을 받으면 캐릭터 생성뷰로 진입한다.

                // 지정된 id를 세팅해서 AllowConnectGame 메세지에 할당한후 전송

                let _message_id = EventHeader::ALLOW_CONNECT_GAME as u32;

                let _session_id = 0; // 서버가 지정

                let _account_id = verify_account_message.userId.clone();
                let _player_name = verify_account_message.userName.clone();
                let _conn_info = verify_account_message.connect_info.clone();

                let mut _allow_connect_game = AllowConnectGame::new(
                    _message_id,
                    _session_id, 
                    _pid_copy, 
                    _account_id, 
                    _player_name, 
                    _conn_info);

                let mut _send_msg = _allow_connect_game.serialize();

                get_tcp_server_instance().write().unwrap().send_message_byte_to_target(
                    _pid_copy as i64,
                     _send_msg);

                // 클라이언트는 검증 메세지를 받은 뒤 캐릭터 선택/생성 화면으로 진입
            }

        }
        Err(e) => {
            eprintln!("Failed to deserialize VerifyAccount: {}", e);
        }
    }
}