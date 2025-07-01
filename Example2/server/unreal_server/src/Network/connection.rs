use std::collections::HashMap;
use std::collections::HashSet;
use mio::net::TcpStream;
use mio::Token;
use std::io::{self, Read, Write};
use std::net::SocketAddr;
use std::net::IpAddr;
use std::sync::{RwLock, Arc, RwLockReadGuard};

lazy_static!{
        static ref G_TCP_CONNECTION_HANDLER: Arc<RwLock<stream_handler>> = Arc::new(RwLock::new(stream_handler::new()));
}

pub fn get_tcp_connection_instance() -> &'static Arc<RwLock<stream_handler>> {
    &G_TCP_CONNECTION_HANDLER
}

pub trait connection_handle {
    fn new() -> Self;
    fn get_id_set_clone(&mut self) -> HashSet<i64>;
    fn del_connection(&mut self, token : Token);
    fn get_current_id_sum(&mut self) -> i64;
    fn update_id_sum(&mut self);
    fn send(&mut self, _token :Token, _message : String);

}


pub struct connection_stream
{
    token: Token,
    id: i64,
    tcpStream: TcpStream
}

impl connection_stream {
    pub fn new(_token: Token, _id: i64, _stream: TcpStream) -> Self {
        connection_stream {
            token : _token,
            id : _id,
            tcpStream : _stream
        }
    }

    pub fn write(&mut self, _message : String) {
        // self.tcpStream.write(_message.as_byte());

        let serialized_msg = _message.as_bytes();

        self.tcpStream.write(serialized_msg);
    }
}

pub struct stream_handler {
    id_sum : i64,
    connections: HashMap<Token, connection_stream>,
    tokenIdMap: HashMap<i64, Token>,
    idSet : HashSet<i64>
}

impl connection_handle for stream_handler {
    fn new() -> Self {
        let mut _connetions = HashMap::new();
        let mut _tokenID = HashMap::new();
        let mut _idSet = HashSet::new();
        stream_handler{
            id_sum : 0,
            connections : _connetions,
            tokenIdMap : _tokenID,
            idSet : _idSet
        }
    }

    fn get_id_set_clone(&mut self) -> HashSet<i64> {
        println!("Cloning ID set...");
        self.idSet.clone()
    }

    fn del_connection(&mut self, token : Token) {
        let mut id = self.connections.get(&token).unwrap().id;

        self.connections.remove(&token);
        self.tokenIdMap.remove(&id);
        self.idSet.remove(&id);
    }

    fn get_current_id_sum(&mut self) -> i64 {
        self.id_sum.clone()
    }

    fn update_id_sum(&mut self) {
        self.id_sum += 1;
    }

    fn send(&mut self, _token: Token, _message: String) {
        // Connection을 맵에서 가져옴
        if let Some(connection) = self.connections.get_mut(&_token) {
            // 메시지를 전송
            println!("Write Message Complemeted :{:?}", _message.clone());
            connection.write(_message);
            
        } else {
            // 연결이 없는 경우 처리 (예: 로그 남기기)
            eprintln!("No connection found for token: {:?}", _token);
        }
    }

    
}

impl stream_handler {
    pub fn is_exist_connection_by_address(&mut self, _addr: String) -> bool {
        if let Ok(target_ip) = _addr.parse::<IpAddr>() {
            for connection in self.connections.values_mut() {
                if let Ok(peer_addr) = connection.tcpStream.peer_addr() {
                    if peer_addr.ip() == target_ip {
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn get_connection_by_id (&mut self, id : i64) -> Option<&mut TcpStream>
    {
        let mut _token = self.tokenIdMap.get(&id);

        if let Some(connection) = self.connections.get_mut(_token.unwrap()) {
            Some(&mut connection.tcpStream)
        } else {
            None
        }   
    }

    pub fn get_connetion_by_token(&mut self, token: Token) -> Option<&mut TcpStream>
    {
        if let Some(connection) = self.connections.get_mut(&token) {
            Some(&mut connection.tcpStream)
        } else {
            None
        }
    }

    pub fn get_id_by_token(&self, token: Token) -> Option<i64> {
        self.tokenIdMap.iter()
            .find(|(_, &val)| val == token)
            .map(|(&key, _)| key)
    }

    pub fn get_id_by_connection(&mut self, _addr : String) -> Option<&mut i64> {
        if let Ok(target_ip) = _addr.parse::<IpAddr>() {
            for connection in self.connections.values_mut() {
                if let Ok(peer_addr) = connection.tcpStream.peer_addr() {
                    if peer_addr.ip() == target_ip {
                        return Some(&mut connection.id)
                    }
                }
            }
        }
        None
    }
    

    pub fn new_connection(&mut self, _tcpStream : TcpStream, _token: Token)
    {
        // Id 처리 로직 필요함
        let _id_top = self.get_current_id_sum();
        let _new_connection = connection_stream::new(_token, _id_top, _tcpStream);

        self.connections.insert(_token, _new_connection);
        self.tokenIdMap.insert(_id_top, _token);
        self.idSet.insert(_id_top);

        self.update_id_sum();
    }

    pub fn get_id_top(&self) -> i64 {
        return self.id_sum
    }
}
