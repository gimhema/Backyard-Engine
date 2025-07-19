
use crate::qsm::user_message::message_make_account::{self, MakeAccount};
use crate::qsm::user_message::message_verify_account::{self, VerifyAccount};
use crate::qsm::user_message::message_allow_connect::{self, AllowConnectGame};
use crate::Event::event_handler::EventHeader;
use crate::GameLogic::game_player::{get_ve_char_manager_instance, VECharcater};

use crate::Network::message_queue::get_callback_msg_queue_instance;
use super::GameLogic::*;

use crate::qsm::qsm::GLOBAL_MESSAGE_TX_QUEUE; // 전역 큐 임포트
use crate::Network::connection::{MessageToSend}; // Token과 MessageToSend 임포트
use mio::Token;

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

            let client_token_value: u32 = 3; // This needs to come from the message context!
            let client_token = Token(client_token_value.try_into().unwrap());

            // 응답 메시지 생성 (예시: AllowConnectGame)
            let message_id = EventHeader::ALLOW_CONNECT_GAME as u32;
            let session_id = 0; // 세션 ID는 실제 값으로 대체해야 합니다.
            let _pid = 456; // 플레이어 ID는 실제 값으로 대체해야 합니다.

            let mut allow_connect_game = AllowConnectGame::new(
                message_id,
                session_id,
                _pid as u32,
                _account_id,
                _player_name,
                _conn_info,
            );

            let send_msg = allow_connect_game.serialize(); // 메시지를 바이트로 직렬화

            // 전역 큐를 통해 메시지 전송 요청
            if let Err(_) = GLOBAL_MESSAGE_TX_QUEUE.push(MessageToSend::Single(client_token, send_msg)) {
                eprintln!("Failed to queue AllowConnectGame message for client {:?}", client_token);
            } else {
                println!("Queued AllowConnectGame message for client {:?}", client_token);
            }
        }
        Err(e) => {
            eprintln!("Failed to deserialize VerifyAccount: {}", e);
        }
    }
}