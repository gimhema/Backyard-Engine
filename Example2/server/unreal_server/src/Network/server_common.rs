use crossbeam_queue::ArrayQueue;
use mio::Token;
use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use std::io;
use std::net::SocketAddr;
use std::collections::VecDeque;
use crate::Network::server::*;
use crate::Network::connection::*;
use std::time::{Instant};
use crate::Event::event_handler::EventHeader;


#[derive(Debug, Clone)]
pub enum ServerActionType
{
    EnterPlayer(Token) // 이 유저는 접속했으니까 대기큐에서 삭제해도 괜찮다.
}

lazy_static! {
    pub static ref GLOBAL_SERVER_ACTION_QUEUE : Arc<ArrayQueue<ServerActionType>> = Arc::new(ArrayQueue::new(1024));
}


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

        self.ping();

        while let Some(action) = GLOBAL_SERVER_ACTION_QUEUE.pop() {
            match action {
                ServerActionType::EnterPlayer(token) => { self.server_action_enter_player(token);  }
            }
        }

        self.processing_waiting_queue();
    }

    pub fn ping(&mut self) {
                    // --- 주기적인 UDP Ping 전송 확인 ---
            if self.last_ping_time.elapsed() >= self.ping_interval {
                println!("Sending periodic UDP Ping to all connected clients (where UDP address is known)...");
                let ping_message_data = "UDP_Ping".as_bytes().to_vec(); // "UDP_Ping" 문자열을 바이트 벡터로 변환

                let clients_for_udp_ping: Vec<(Token, SocketAddr)> = self.clients.iter()
                    .filter_map(|(&token, client)| {
                        // is_udp_client가 true이고 udp_addr이 Some인 경우에만 핑을 보냅니다.
                        if client.is_udp_client && client.udp_addr.is_some() {
                            Some((token, client.udp_addr.unwrap())) // unwrap()은 Some임을 확인했으므로 안전
                        } else {
                            None
                        }
                    })
                    .collect();

                for (token, target_udp_addr) in clients_for_udp_ping {
                    if let Err(_) = self.send_udp_message(target_udp_addr, ping_message_data.clone()) {
                        eprintln!("Failed to queue UDP ping message for client {:?} ({}).", token, target_udp_addr);
                    }
                }
                self.last_ping_time = Instant::now(); // 마지막 Ping 전송 시간 업데이트
            }
    }


}

