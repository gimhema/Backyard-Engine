use std::net::SocketAddr;
use std::sync::{RwLock, Arc, RwLockReadGuard};
use super::server;



lazy_static! {
    static ref G_SERVER_INFO: Arc<RwLock<serverinfo>> = Arc::new(RwLock::new(serverinfo::new()));
}

pub fn get_server_info() -> &'static Arc<RwLock<serverinfo>> {
    &G_SERVER_INFO
}

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
}