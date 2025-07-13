use mio::Token;
use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use std::io;
use std::net::SocketAddr;
use std::collections::VecDeque;
use crate::Network::server::*;


pub struct WaitingQueue {
    pub waiting_queue: Arc<RwLock<VecDeque<Token>>>,
}

impl WaitingQueue {
    pub fn new() -> Self {
        WaitingQueue {
            waiting_queue: Arc::new(RwLock::new(VecDeque::new())),
        }
    }

    pub fn push(&self, token: Token) {
        let mut queue = self.waiting_queue.write().unwrap();
        queue.push_back(token);
    }

    pub fn pop(&self) -> Option<Token> {
        let mut queue = self.waiting_queue.write().unwrap();
        queue.pop_front()
    }

    pub fn is_empty(&self) -> bool {
        let queue = self.waiting_queue.read().unwrap();
        queue.is_empty()
    }
}

impl Server{

    pub fn server_loop_action(&mut self) {

        // 서버 루프에서 대기열 처리
        self.processing_waiting_queue();


    }

    pub fn processing_waiting_queue(&mut self) {
        // 대기열에서 토큰을 처리하는 로직
        while let Some(token) = self.player_waiting_queue.lock().unwrap().pop() {
            if let Some(client) = self.clients.get_mut(&token) {
                // 클라이언트 연결 처리 로직
                println!("Processing client with token: {:?}", token);
                // 예: 클라이언트에게 메시지 전송 등
            } else {
                eprintln!("Client with token {:?} not found in clients map.", token);
            }
        }
    }

}

