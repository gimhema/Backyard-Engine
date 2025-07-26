use crate::Network::server::*;
use crate::Network::connection::*;
use mio::Token;


impl Server {
    pub fn server_action_enter_player(&mut self, token: Token) {
        // 서버 액션 큐에 EnterPlayer 액션을 추가합니다.
        // if let Err(e) = GLOBAL_SERVER_ACTION_QUEUE.push(ServerActionType::EnterPlayer(token)) {
        //     eprintln!("Failed to push EnterPlayer action to global server action queue: {:?}", e);
        // }
    }
}
