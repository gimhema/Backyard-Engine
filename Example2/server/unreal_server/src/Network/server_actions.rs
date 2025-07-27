use crate::qsm;
use crate::Network::server::*;
use crate::Network::connection::*;
use super::qsm::user_message::message_allow_connect::*;
use super::Event::event_handler::*;
use crate::Network::server_common::ServerActionType::*;
use mio::Token;


impl Server {

    // 대기열의 스트림으로부터 인증을 요구하는 함수
    pub fn processing_waiting_queue(&mut self) {
        // 대기열에서 토큰을 처리하는 로직
        while let Some(token) = self.player_waiting_queue.lock().unwrap().pop() {
            if let Some(client) = self.clients.get_mut(&token) {

                println!("Processing client with token: {:?}", token);

                let mut _allow_connect_message = AllowConnectGame::new(
                    EventHeader::ALLOW_CONNECT_GAME as u32,
                    0,
                    Token as u32,
                    "TEST_ACCOUNT".to_string(),
                    "TEST_CONNECT_NAME".to_string(),
                    "TEST_CONNECT_INFO".to_string()
                );

                let mut _send_msg = _allow_connect_message.serialize();

                let _req_enter_message = MessageToSend::Single(token, _send_msg);
                if let Err(_) = self.send_tcp_message(_req_enter_message) {
                    eprintln!("Failed to send message to client with token: {:?}", token);
                }
            } else {
                eprintln!("Client with token {:?} not found in clients map.", token);
            }
        }
    }

    // 인증을 받은 플레이어를 실질적으로 통과시키는 함수
    pub fn server_action_enter_player(&mut self, 
        _pid : Token, _account_id: String, _player_name: String, _conn_info: String) {
        // Create Character by Token
    }
}
