use std::net::SocketAddr;
use std::sync::{RwLock, Arc, RwLockReadGuard};
use super::server;


pub struct serverinfo {
    socket_addr : SocketAddr
}

impl serverinfo {
    pub fn new () -> Self {
        serverinfo { socket_addr: "".parse().unwrap() }
    }

    pub fn set_socket_addr(&mut self, _socket_addr: SocketAddr) {
        self.socket_addr = _socket_addr;
    }

    pub fn get_socket_addr(&mut self) -> SocketAddr {
        self.socket_addr.clone()
    }

    pub fn init(&mut self) {

        // file read . . .

        self.socket_addr = "127.0.0.1:8080".parse().unwrap();
    }
}