use std::thread::sleep;
use std::time::Duration;

use crate::qsm;
use crate::Network::server::*;
use crate::Network::connection::*;
use super::qsm::user_message::message_allow_connect::*;
use super::Event::event_handler::*;
use crate::Network::server_common::ServerActionType::*;
use crate::GameLogic::game_player::VECharcater;
use mio::Token;
use crate::GameLogic::game_player::GameNetStatus;


impl Server {

    // 대기열의 스트림으로부터 인증을 요구하는 함수
    pub fn processing_waiting_queue(&mut self) {

        let waiting_queue = self.player_waiting_queue.lock().unwrap();
        let container = waiting_queue.waiting_containter.read().unwrap();

        for token in container.iter() {
            if let Some(client) = self.clients.get_mut(token) {
                sleep(Duration::from_millis(100));
                println!("Processing client with token: {:?}", token);

                let allow_connect_message = AllowConnectGame::new(
                    EventHeader::ALLOW_CONNECT_GAME as u32,
                    0,
                    token.0 as u32,
                    "".to_string(),
                    "".to_string(),
                    "".to_string()
                );

                let send_msg = allow_connect_message.serialize();
                let req_enter_message = MessageToSend::Single(*token, send_msg);

                if let Err(_) = self.send_tcp_message(req_enter_message) {
                    eprintln!("Failed to send message to client with token: {:?}", token);
                    waiting_queue.remove(*token);
                }
            } else {
                eprintln!("Client with token {:?} not found in clients map.", token);
                waiting_queue.remove(*token);
                }
                // 
        }
}


    // 인증을 받은 플레이어를 실질적으로 통과시키는 함수
    pub fn server_action_enter_player(&mut self, 
        _pid : u32, _account_id: String, _player_name: String, _conn_info: String) {
        // Create Character by Token
        println!("Entering player with PID: {}, Account ID: {}, Player Name: {}, Conn Info: {}",
                 _pid, _account_id, _player_name, _conn_info);

        // 대기열 큐에서 제거해야함

        let waiting_queue = self.player_waiting_queue.lock().unwrap();
        
        // token is pid

        let target_token = Token(_pid as usize);
        let _pid_token = target_token.clone();
        waiting_queue.remove(target_token);
        
        // Create a new player character
        let mut new_player = VECharcater::new_zero();
        
        // setting network config
        new_player.player_network_config.set_sessionid(0 as i64);
        new_player.player_network_config.set_net_token(_pid_token);
        new_player.player_network_config.set_net_status(GameNetStatus::CONNECTED);

        // setting personal info
        new_player.set_player_name(_player_name.clone());
        new_player.set_player_pid(_pid as i64);
        new_player.set_player_ip_addr(_conn_info);
        new_player.init();


        // add game player to game character manager
        self.game_character_manager.lock().unwrap().new_character(new_player);

        
    }
}
