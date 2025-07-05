use std::collections::HashMap;
use std::collections::HashSet;
use mio::net::TcpStream;
use mio::Token;
use std::io::{self, Read, Write};
use std::net::IpAddr; // SocketAddr 대신 IpAddr만 사용하는 경우
use std::sync::{RwLock, Arc, RwLockReadGuard};
use super::message_queue::*;

lazy_static!{
    static ref G_TCP_CONNECTION_HANDLER: Arc<RwLock<stream_handler>> = Arc::new(RwLock::new(stream_handler::new()));
}

pub fn get_tcp_connection_instance() -> &'static Arc<RwLock<stream_handler>> {
    println!("LOCK TCP CONNECTION HANDLE");
    &G_TCP_CONNECTION_HANDLER
}

pub trait connection_handle {
    fn new() -> Self;
    fn get_id_set_clone(&mut self) -> HashSet<i64>;
    fn del_connection(&mut self, token : Token);
    fn get_current_id_sum(&mut self) -> i64;
    fn update_id_sum(&mut self);
    fn send(&mut self, _token :Token, _message : String);
    fn send_message_byte_to_target(&mut self, target : i64, msg_byte : Vec<u8>); // depracated
    fn send_message_byte_to_all(&mut self, msg_byte : Vec<u8>); // depracated
    fn message_queue_process(&mut self);
}


pub struct connection_stream
{
    pub token: Token,
    pub id: i64,
    pub tcpStream: TcpStream,
    pub peer_ip: Option<IpAddr>, 
}

impl connection_stream {
    pub fn new(_token: Token, _id: i64, _stream: TcpStream) -> Self {
        let peer_ip = _stream.peer_addr().ok().map(|addr| addr.ip());
        connection_stream {
            token : _token,
            id : _id,
            tcpStream : _stream,
            peer_ip, // 캐시된 IP 주소 저장
        }
    }

    pub fn write(&mut self, _message : String) {
        let serialized_msg = _message.as_bytes();
        // write 결과에 대한 에러 처리 추가 (optional)
        if let Err(e) = self.tcpStream.write(serialized_msg) {
            eprintln!("Failed to write to TCP stream for token {:?}: {}", self.token, e);
        }
    }
}

pub struct stream_handler {
    pub id_sum : i64,
    pub connections: HashMap<Token, connection_stream>,
    pub tokenIdMap: HashMap<i64, Token>,
    pub idSet : HashSet<i64>
}

impl connection_handle for stream_handler {
    fn new() -> Self {
        stream_handler{
            id_sum : 0,
            connections : HashMap::new(),
            tokenIdMap : HashMap::new(),
            idSet : HashSet::new()
        }
    }

    fn get_id_set_clone(&mut self) -> HashSet<i64> {
        println!("Cloning ID set...");
        self.idSet.clone()
    }

    fn del_connection(&mut self, token : Token) {
        // remove가 Option을 반환하므로 안전하게 처리
        if let Some(conn_stream) = self.connections.remove(&token) {
            self.tokenIdMap.remove(&conn_stream.id);
            self.idSet.remove(&conn_stream.id);
            println!("Connection for token {:?} and ID {} deleted.", token, conn_stream.id);
        } else {
            eprintln!("Attempted to delete non-existent connection for token: {:?}", token);
        }
    }

    fn get_current_id_sum(&mut self) -> i64 {
        self.id_sum
    }

    fn update_id_sum(&mut self) {
        self.id_sum += 1;
    }

    fn send(&mut self, _token: Token, _message: String) {
        if let Some(connection) = self.connections.get_mut(&_token) {
            println!("Write Message Completed :{:?}", _message.clone());
            connection.write(_message);
        } else {
            eprintln!("No connection found for token: {:?}", _token);
        }
    }

    fn send_message_byte_to_target(&mut self, target : i64, msg_byte : Vec<u8>) {
        if let Some(token) = self.tokenIdMap.get(&target) {
            if let Some(connection) = self.connections.get_mut(token) {
                if let Err(e) = connection.tcpStream.write(&msg_byte) {
                    eprintln!("Failed to send message to target {:?} (token {:?}): {}", target, token, e);
                }
            } else {
                eprintln!("No connection found for target ID {} (token {:?})", target, token);
            }
        } else {
            eprintln!("No token found for target ID {}", target);
        }
    }

    fn send_message_byte_to_all(&mut self, msg_byte : Vec<u8>) {
        for connection in self.connections.values_mut() {
            if let Err(e) = connection.tcpStream.write(&msg_byte) {
                eprintln!("Failed to send message to connection: {:?}", e);
            }
        }
    }

    fn message_queue_process(&mut self) {
        // 메시지 큐 처리 로직을 여기에 구현합니다.
        // 예시로, 메시지 큐에서 메시지를 꺼내서 각 연결에 전송하는 로직을 추가할 수 있습니다.
        let mut msg_queue = get_callback_msg_queue_instance().write().unwrap();
        
        while !msg_queue.empty() {
            let message = msg_queue.pop();
            if let Some(connection) = self.connections.get_mut(&message.get_token()) {
                connection.write(message.get_message());
            } else {
                eprintln!("No connection found for token: {:?}", message.get_token());
            }
        }
    }
}

impl stream_handler {
    // is_exist_connection_by_address 메서드 최적화
    pub fn is_exist_connection_by_address(&self, _addr: String) -> bool {
        if let Ok(target_ip) = _addr.parse::<IpAddr>() {
            for connection in self.connections.values() { // read-only access
                if let Some(cached_ip) = connection.peer_ip {
                    if cached_ip == target_ip {
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn get_connection_by_id (&mut self, id : i64) -> Option<&mut TcpStream> {
        self.tokenIdMap.get(&id).and_then(|token| {
            self.connections.get_mut(token).map(|conn_stream| &mut conn_stream.tcpStream)
        })
    }

    pub fn get_connetion_by_token(&mut self, token: Token) -> Option<&mut TcpStream> {
        self.connections.get_mut(&token).map(|conn_stream| &mut conn_stream.tcpStream)
    }

    pub fn get_id_by_token(&self, token: Token) -> Option<i64> {
        // HashMap의 values()를 순회하여 찾도록 변경 (tokenIdMap 사용)
        self.connections.get(&token).map(|conn_stream| conn_stream.id)
    }

    // get_id_by_connection 메서드 최적화 및 불필요한 로그 제거
    pub fn get_id_by_connection (&self, _addr : String) -> Option<i64> {
        // println!("Get ID by Connection : {}", _addr); // 이 로그는 너무 자주 출력될 수 있습니다.

        if let Ok(target_ip) = _addr.parse::<IpAddr>() {
            for connection in self.connections.values() { // read-only access
                if let Some(cached_ip) = connection.peer_ip {
                    if cached_ip == target_ip {
                        // println!("Found connection with ID: {}", connection.id); // 찾았을 때만 로그
                        return Some(connection.id)
                    }
                }
            }
        }
        None
    }
    
    pub fn new_connection(&mut self, _tcpStream : TcpStream, _token: Token) {
        let _id_top = self.get_current_id_sum();
        let _new_connection = connection_stream::new(_token, _id_top, _tcpStream);

        self.connections.insert(_token, _new_connection);
        self.tokenIdMap.insert(_id_top, _token);
        self.idSet.insert(_id_top);

        self.update_id_sum();
    }

    pub fn get_id_top(&self) -> i64 {
        self.id_sum
    }
}