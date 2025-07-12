use std::collections::HashMap;
use std::collections::HashSet;
use mio::net::TcpStream;
use mio::Token;
use std::io::{self, Read, Write};
use std::net::IpAddr; // SocketAddr 대신 IpAddr만 사용하는 경우
use std::sync::{RwLock, Arc, RwLockReadGuard};
use std::net::SocketAddr;
use std::sync::{Mutex};

// --- 클라이언트 연결 구조체 ---
// --- 클라이언트 연결 구조체 ---
pub struct ClientConnection {
    pub stream: TcpStream,
    pub addr: SocketAddr, // 이 addr은 TCP 주소
    pub write_queue: Arc<Mutex<Vec<u8>>>,
    pub is_udp_client: bool, // 클라이언트가 UDP 통신을 지원하는지 여부
    pub udp_addr: Option<SocketAddr>, // 클라이언트의 UDP 수신 주소를 저장할 필드 (새로 추가)
}
