use std::net::SocketAddr;
use std::sync::{RwLock, Arc, RwLockReadGuard};
use super::server;


pub struct serverinfo {
    socket_addr : String
}

impl serverinfo {
    pub fn new () -> Self {
        serverinfo { socket_addr: "".to_string() }
    }

    pub fn set_socket_addr(&mut self, _socket_addr: String) {
        self.socket_addr = _socket_addr;
    }

    pub fn get_socket_addr(&mut self) -> String {
        self.socket_addr.clone()
    }

    pub fn init(&mut self) {

        // file read . . .

        // self.socket_addr = "127.0.0.1:8080".parse().;
    }
}
