use super::serverinfo::*;
use super::Crypto::packet_crypto::*;
use std::net::SocketAddr;

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

    pub fn get_socket_addr(&mut self) -> SocketAddr {
        self.connect_info.get_socket_addr()
    }
}

