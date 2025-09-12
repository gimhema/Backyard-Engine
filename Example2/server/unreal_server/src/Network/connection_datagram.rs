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

impl Server {

    // --- (옵션) 브로드캐스트 대상 주소만 먼저 뽑아 쓰고 싶을 때 사용할 헬퍼 ---
    pub fn collect_udp_targets(&self) -> Vec<SocketAddr> {
        self.clients
            .values()
            .filter_map(|c| c.udp_addr)
            .collect()
    }

    // --- 즉시 전송(직접 send_to) 버전: 기존 유지 ---
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

    pub fn udp_recv_action(&mut self, buffer: &[u8], len: usize) {
        EventHeader::action(&buffer[..len]);
    }
}
