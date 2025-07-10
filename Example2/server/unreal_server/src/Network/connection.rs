use std::collections::HashMap;
use std::collections::HashSet;
use mio::net::TcpStream;
use mio::Token;
use std::io::{self, Read, Write};
use std::net::IpAddr; // SocketAddr 대신 IpAddr만 사용하는 경우
use std::sync::{RwLock, Arc, RwLockReadGuard};

// --- 클라이언트 연결 구조체 ---
pub struct ClientConnection {
    pub stream: TcpStream,
    pub addr: SocketAddr,
    pub write_queue: Arc<Mutex<Vec<u8>>>,
    pub is_udp_client: bool, // UDP 클라이언트인지 여부 (TCP와 UDP 연결을 구분)
}
