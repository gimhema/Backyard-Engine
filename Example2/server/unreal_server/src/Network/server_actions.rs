use crate::Network::server::*;
use crate::Network::connection::*;
use mio::Token;


impl Server {

    // 대기열의 스트림으로부터 인증을 요구하는 함수
    pub fn processing_waiting_queue(&mut self) {
        // 대기열에서 토큰을 처리하는 로직
        while let Some(token) = self.player_waiting_queue.lock().unwrap().pop() {
            if let Some(client) = self.clients.get_mut(&token) {
                // 클라이언트 연결 처리 로직
                println!("Processing client with token: {:?}", token);
                // 예: 클라이언트에게 메시지 전송 등
                let _req_enter_message = MessageToSend::Single(token, "WaitingQueue: Enter".as_bytes().to_vec());
                if let Err(_) = self.send_tcp_message(_req_enter_message) {
                    eprintln!("Failed to send message to client with token: {:?}", token);
                }
            } else {
                eprintln!("Client with token {:?} not found in clients map.", token);
            }
        }
    }

    // 인증을 받은 플레이어를 실질적으로 통과시키는 함수
    pub fn server_action_enter_player(&mut self, token: Token) {
        // Create Character by Token
    }
}
