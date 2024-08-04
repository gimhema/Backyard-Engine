use mio::net::{TcpStream, UdpSocket};
use mio::Token;
use std::collections::HashSet;
use std::sync::{RwLock, Arc};
use super::connection::stream_handler;
use super::connection_datagram::datagram_handler;
use super::serverinfo::*;
use super::Crypto::packet_crypto::*;
use std::net::SocketAddr;
use crate::Network::connection::connection_handle;

lazy_static! {
    static ref G_GAME_COMMON_LOGIC_INSTANCE: Arc<RwLock<server_extend_common>> = Arc::new(RwLock::new(server_extend_common::new()));
    static ref G_CONNECTION_HANDLER_INSTANCE: Arc<RwLock<server_common_connetion_handler>> = Arc::new(RwLock::new(server_common_connetion_handler::new()));
}

pub fn get_common_logic_instance() -> &'static Arc<RwLock<server_extend_common>> {
    &G_GAME_COMMON_LOGIC_INSTANCE
}

pub fn get_connection_handler_instance() -> &'static Arc<RwLock<server_common_connetion_handler>> {
    &G_CONNECTION_HANDLER_INSTANCE
}

pub fn get_connection_hanlder_clone() -> Arc<RwLock<server_common_connetion_handler>> {
    G_CONNECTION_HANDLER_INSTANCE.clone()
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

    pub fn get_tcp_connection_by_id(&mut self, _id: i64) -> Option<&mut TcpStream>
    {
        self.tcp_connections.get_connection_by_id(_id)
    }

    pub fn del_tcp_connection(&mut self, _token: Token) {
        self.tcp_connections.del_connection(_token);
    }

    pub fn get_tcp_connection_list(&self) -> HashSet<i64> {
        self.tcp_connections.get_id_set_clone()
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

