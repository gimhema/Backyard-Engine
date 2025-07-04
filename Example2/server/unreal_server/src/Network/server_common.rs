use mio::net::{TcpStream, UdpSocket};
use mio::Token;
use std::vec::Vec;
use std::collections::{HashMap, HashSet};
use std::sync::{RwLock, Arc};
use super::connection::stream_handler;
use super::connection_datagram::datagram_handler;
use super::serverinfo::*;
use super::Crypto::packet_crypto::*;
use std::net::SocketAddr;
use crate::Network::connection::connection_handle;

lazy_static! {
    static ref G_GAME_COMMON_LOGIC_INSTANCE: Arc<RwLock<server_extend_common>> = Arc::new(RwLock::new(server_extend_common::new()));
    static ref G_CONNECTION_HANLDER_INSTANCE: Arc<RwLock<server_common_connetion_handler>> = Arc::new(RwLock::new(server_common_connetion_handler::new()));
    static ref G_SEND_CONNECTION_HANLDER_INSTANCE: Arc<RwLock<server_common_connetion_handler>> = Arc::new(RwLock::new(server_common_connetion_handler::new()));
    static ref G_USER_CONN_INFO_INSTANCE: Arc<RwLock<user_connect_info>> = Arc::new(RwLock::new(user_connect_info::new()));
}

pub fn get_common_logic_instance() -> &'static Arc<RwLock<server_extend_common>> {
    &G_GAME_COMMON_LOGIC_INSTANCE
}

// pub fn get_connection_handler() -> &'static Arc<RwLock<server_common_connetion_handler>> {
//     &G_CONNECTION_HANLDER_INSTANCE
// }

pub fn get_user_connection_info() -> &'static Arc<RwLock<user_connect_info>> {
    &G_USER_CONN_INFO_INSTANCE
}

pub struct server_common_info {
    connect_info : serverinfo,
    crypto_processor : cryption_processor
}

impl server_common_info {
    pub fn new() -> Self {
        let mut _conn_info = serverinfo::new();
        let mut _crypto_processor = cryption_processor::new();

        _conn_info.init();
        _crypto_processor.init();

        server_common_info{connect_info : _conn_info, crypto_processor : _crypto_processor}
    }

    pub fn get_socket_addr(&mut self) -> String {
        self.connect_info.get_socket_addr()
    }
}

pub struct user_common_info {
    pId : i64, // share network info, not index
    idToken  : Token,  // share network info
    ipaddress : String,  // share network info
}

impl user_common_info {
    pub fn new(_pId : i64, _token : Token, _ipaddress : String) -> Self {
        user_common_info { pId: _pId, idToken: _token, ipaddress: _ipaddress }
    }
}

// for extend user customize(UserLogic)
pub struct server_extend_common {
    user_common_container : Vec<user_common_info> 
}

impl server_extend_common {
    pub fn new() -> Self {
        server_extend_common{ user_common_container: Vec::new() }
    }

    pub fn PushNewCommonInfo(&mut self, _new_user_common: user_common_info) {
        self.user_common_container.push(_new_user_common);
    }
}


// for udp
pub struct server_common_connetion_handler {
    tcp_connections : stream_handler,
    udp_connections : datagram_handler
}

impl server_common_connetion_handler {
    pub fn new() -> Self {
        server_common_connetion_handler{ 
            tcp_connections : stream_handler::new(), 
            udp_connections : datagram_handler::new() 
        }
    }

    // TCP
    pub fn new_tcp_connection(&mut self, _tcpStream : TcpStream, _token: Token) 
    {
        self.tcp_connections.new_connection(_tcpStream, _token);
    }

    pub fn get_tcp_connection_by_token(&mut self, _token: Token) -> Option<&mut TcpStream>
    {
        self.tcp_connections.get_connetion_by_token(_token)
    }

    pub fn get_tcp_connection_id_by_token(&mut self, _token : Token) -> Option<i64> {
        return self.tcp_connections.get_id_by_token(_token)
    }

    pub fn get_connection_id_top(&self) -> i64 {
        return self.tcp_connections.get_id_top()
    }

    pub fn get_tcp_connection_by_id(&mut self, _id: i64) -> Option<&mut TcpStream>
    {
        self.tcp_connections.get_connection_by_id(_id)
    }

    pub fn del_tcp_connection(&mut self, _token: Token) {
        self.tcp_connections.del_connection(_token);
    }

    pub fn get_tcp_connection_list(&mut self) -> HashSet<i64> {
        self.tcp_connections.get_id_set_clone()
    }

    pub fn send_message_to_stream(&mut self, _token : Token, _msg : String) {
        self.tcp_connections.send(_token, _msg);
    }


    // UDP
    pub fn new_udp_connection(&mut self, _udpSocket : UdpSocket, _token: Token)
    {
        self.udp_connections.new_connection(_udpSocket, _token);
    }

    pub fn get_udp_connection_by_token(&mut self, _token: Token) -> Option<&mut UdpSocket>
    {
        self.udp_connections.get_connetion_by_token(_token)
    }

    pub fn get_udp_connection_by_id(&mut self, _id: i64) -> Option<&mut UdpSocket>
    {
        self.udp_connections.get_connection_by_id(_id)
    }

    pub fn del_udp_connection(&mut self, _token: Token) {
        self.udp_connections.del_connection(_token);
    }

    pub fn get_udp_connection_list(&mut self) -> HashSet<i64> {
        self.udp_connections.get_id_set_clone()
    }
}


pub struct user_connect_info {
    user_token_vec : Vec<Token>,
    id_token_map : HashMap<i64, Token>,
    ip_token_map : HashMap<String, Token>
}

impl user_connect_info {
    pub fn new() -> Self {
        return user_connect_info{ 
            user_token_vec : Vec::new(),
            id_token_map : HashMap::new(),
            ip_token_map : HashMap::new() 
        }
    }

    pub fn clear(&mut self) {
        self.user_token_vec.clear();
        self.id_token_map.clear();
        self.ip_token_map.clear();
    }

    pub fn new_connect_info(&mut self, id: i64, token: Token, ip_address: String) {
        println!("New connection info added: id = {}, token = {:?}, ip_address = {}", id, token, ip_address.clone());
        self.user_token_vec.push(token);
        self.id_token_map.insert(id, token);
        self.ip_token_map.insert(ip_address, token);
    }

    pub fn del_connect_info(&mut self, id: i64, token: Token, ip_address: String) {
        self.user_token_vec.retain(|&t| t != token);
        self.id_token_map.remove(&id);
        self.ip_token_map.remove(&ip_address);
    }

    pub fn get_token_by_id(&self, id: i64) -> Option<Token> {
        if let Some(&token) = self.id_token_map.get(&id) {
            return Some(token);
        }
        None
    }

    pub fn get_token_by_ip(&self, ip_address: &str) -> Option<Token> {
        if let Some(&token) = self.ip_token_map.get(ip_address) {
            return Some(token);
        }
        None
    }

    pub fn get_token(&self, idx: usize) -> Option<Token> {
        if let Some(&value) = self.user_token_vec.get(idx) {
            return Some(value);
        }
        None
    }

    pub fn find_id_by_token(&self, token: Token) -> Option<i64> {
        for (id, t) in &self.id_token_map {
            if *t == token {
                return Some(*id);
            }
        }
        None
    }

    pub fn find_ip_by_token(&self, token: Token) -> Option<String> {
        for (ip, t) in &self.ip_token_map {
            if *t == token {
                return Some(ip.clone());
            }
        }
        None
    }

    pub fn get_token_vec_size(&self) -> usize {
        return self.user_token_vec.len()
    }

    pub fn get_token_vec(&self) -> Vec<Token> {
        return self.user_token_vec.clone()
    }
}

