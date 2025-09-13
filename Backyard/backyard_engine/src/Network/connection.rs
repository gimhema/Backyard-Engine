// For TCP

use std::collections::HashMap;
use std::collections::HashSet;
use mio::net::TcpStream;
use mio::Token;
use std::io::{self, Read, Write};
use std::net::IpAddr; // SocketAddr 대신 IpAddr만 사용하는 경우
use std::sync::{RwLock, Arc, RwLockReadGuard};
use std::net::SocketAddr;
use std::sync::{Mutex};
use mio::Interest;
use crate::Network::server::*;
use crate::Event::event_handler::EventHeader;

// --- 전송할 메시지 유형 정의 ---
#[derive(Debug)]
pub enum MessageToSend {
    Single(Token, Vec<u8>),      // 단일 소켓 대상
    Group(String, Vec<u8>),       // 특정 그룹 소켓 대상 (그룹 이름으로 식별)
    Broadcast(Vec<u8>),           // 전체 소켓 대상
}

// --- 클라이언트 연결 구조체 ---
pub struct ClientConnection {
    pub stream: TcpStream,
    pub addr: SocketAddr, // 이 addr은 TCP 주소
    pub write_queue: Arc<Mutex<Vec<u8>>>,
    pub is_udp_client: bool, // 클라이언트가 UDP 통신을 지원하는지 여부
    pub udp_addr: Option<SocketAddr>, // 클라이언트의 UDP 수신 주소를 저장할 필드 (새로 추가)
}

impl Server
{
// --- TCP 메시지 송신 함수 (외부에서 호출 가능) ---
    pub fn send_tcp_message(&self, message: MessageToSend) -> Result<(), ()> {
        if let Err(e) = self.tcp_message_tx_queue.push(message) {
            eprintln!("Failed to push TCP message to queue: {:?}", e);
            Err(())
        } else {
            Ok(())
        }
    }


// --- 단일 TCP 소켓 대상 메시지 전송 (TCP 전용) ---
    // 함수 이름 변경: send_tcp_data_to_token
    pub fn send_tcp_data_to_token(&mut self, token: Token, data: Vec<u8>) -> io::Result<()> {
        if let Some(client) = self.clients.get_mut(&token) {
            let mut write_queue = client.write_queue.lock().unwrap();
            write_queue.extend_from_slice(&data);
            self.poll.registry().reregister(&mut client.stream, token, Interest::READABLE | Interest::WRITABLE)?;
            Ok(())
        } else {
            eprintln!("Attempted to send TCP message to non-existent client with token: {:?}", token);
            Ok(())
        }
    }

    // --- 특정 그룹 TCP 소켓 대상 메시지 전송 ---
    // 함수 이름 변경: send_tcp_data_to_group
    pub fn send_tcp_data_to_group(&mut self, group_name: &str, data: Vec<u8>) -> io::Result<()> {
        let client_groups_lock = self.client_groups.lock().unwrap();
        let tokens_to_send: Vec<Token> = client_groups_lock
            .get(group_name)
            .cloned()
            .unwrap_or_else(Vec::new);
        drop(client_groups_lock);

        for &token in tokens_to_send.iter() {
            // 이제 이 send_tcp_message는 TCP 큐에 메시지를 넣습니다.
            if let Err(_) = self.send_tcp_message(MessageToSend::Single(token, data.clone())) {
                eprintln!("Failed to queue group TCP message for token {:?}.", token);
            }
        }
        if tokens_to_send.is_empty() {
             println!("Group '{}' not found or is empty for sending TCP message.", group_name);
        }
        Ok(())
    }

    // --- 전체 TCP 소켓 대상 메시지 전송 (브로드캐스트) ---
    // 함수 이름 변경: broadcast_tcp_message
    pub fn broadcast_tcp_message(&mut self, data: Vec<u8>) -> io::Result<()> {
        let tokens_to_send: Vec<Token> = self.clients.keys().cloned().collect();
        for token in tokens_to_send {
            // 이제 이 send_tcp_message는 TCP 큐에 메시지를 넣습니다.
            if let Err(_) = self.send_tcp_message(MessageToSend::Single(token, data.clone())) {
                eprintln!("Failed to queue broadcast TCP message for token {:?}.", token);
            }
        }
        Ok(())
    }

     // --- 서버 내부 TCP 메시지 큐 처리 및 실제 전송 수행 ---
    // 이름 변경: process_outgoing_tcp_messages로 명확화
    pub fn process_outgoing_tcp_messages(&mut self) -> io::Result<()> {
        while let Some(msg) = self.tcp_message_tx_queue.pop() {
            match msg {
                MessageToSend::Single(token, data) => {
                    self.send_tcp_data_to_token(token, data)?; // 함수 이름 변경
                }
                MessageToSend::Group(group_name, data) => {
                    self.send_tcp_data_to_group(&group_name, data)?; // 함수 이름 변경
                }
                MessageToSend::Broadcast(data) => {
                    self.broadcast_tcp_message(data)?; // 함수 이름 변경
                }
            }
        }
        Ok(())
    }


}
