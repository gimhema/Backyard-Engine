// for udp connection
use std::collections::HashMap;
use std::collections::HashSet;
use mio::net::{UdpSocket, TcpStream};
use mio::Token;
use super::connection::*;
use std::sync::{RwLock, Arc, RwLockReadGuard};
use super::message_queue::*;
use crate::Network::server::*;
use std::io::{self, Read, Write};
use std::net::SocketAddr;
use crate::Event::event_handler::EventHeader;

impl Server{
    // --- 서버 내부 UDP 메시지 큐 처리 및 실제 전송 수행 (새로 추가) ---
    pub fn process_outgoing_udp_messages(&mut self) -> io::Result<()> {
        while let Some((target_addr, data)) = self.udp_message_tx_queue.pop() {
            match self.udp_socket.send_to(&data, target_addr) {
                Ok(n) => {
                    println!("Sent {} bytes UDP message to {} from queue.", n, target_addr);
                }
                Err(e) => {
                    eprintln!("Error sending UDP message to {} from queue: {}", target_addr, e);
                    // 개별 UDP 전송 오류는 루프를 중단하지 않습니다.
                }
            }
        }
        Ok(())
    }

    // --- UDP 메시지 송신 함수 (외부에서 호출 가능) ---
    pub fn send_udp_message(&self, target_addr: SocketAddr, data: Vec<u8>) -> Result<(), ()> {
        if let Err(e) = self.udp_message_tx_queue.push((target_addr, data)) {
            eprintln!("Failed to push UDP message to queue: {:?}", e);
            Err(())
        } else {
            Ok(())
        }
    }

    // --- UDP 클라이언트 대상 메시지 전송 (UDP 전용) ---
    pub fn send_udp_message_to_token(&self, token: Token, addr: SocketAddr, data: Vec<u8>) -> io::Result<()> {
        match self.udp_socket.send_to(&data, addr) {
            Ok(n) => {
                println!("Sent {} bytes UDP message to client {:?} ({})", n, token, addr);
                Ok(())
            }
            Err(e) => {
                eprintln!("Error sending UDP message to client {:?} ({}): {}", token, addr, e);
                Err(e)
            }
        }
    }

        pub fn udp_recv_action(&mut self, buffer : &[u8], len : usize) {
        // 수신된 UDP 메시지 처리 로직
        EventHeader::action(&buffer[..len]); // 수신된 UDP 메시지 처리
    }
}