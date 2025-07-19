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

pub type SharedServerActionQueue = Arc<ArrayQueue<ServerActionType>>;

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

        self.ping();
    }

    pub fn ping(&mut self) {
                    // --- 주기적인 UDP Ping 전송 확인 ---
            if self.last_ping_time.elapsed() >= self.ping_interval {
                println!("Sending periodic UDP Ping to all connected clients (where UDP address is known)...");
                let ping_message_data = "UDP_Ping".as_bytes().to_vec(); // "UDP_Ping" 문자열을 바이트 벡터로 변환

                // 현재 연결된 모든 클라이언트에게 UDP Ping 메시지를 큐에 추가
                // 이때, ClientConnection에 저장된 UDP 주소를 사용합니다.
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



}

