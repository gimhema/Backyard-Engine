use mio::{Events, Poll, Token, Interest, Registry};
use mio::net::UdpSocket;
use std::collections::{HashMap, HashSet};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex, OnceLock};
use super::connection::connection_handle;
use super::Event::event_handler::*;
use super::connection_datagram::*;
use super::server_common::*;

const SERVER_TOKEN: Token = Token(0);

// OnceLock을 사용하여 안전하게 싱글톤 인스턴스를 생성
static G_SERVER_DATAGRAM_INSTANCE: OnceLock<Arc<Mutex<server_datagram>>> = OnceLock::new();

pub fn init_udp_server_instance() -> Arc<Mutex<server_datagram>> {
    G_SERVER_DATAGRAM_INSTANCE.get_or_init(|| Arc::new(Mutex::new(server_datagram::new()))).clone()
}

pub fn get_udp_server_instance() -> Option<Arc<Mutex<server_datagram>>> {
    G_SERVER_DATAGRAM_INSTANCE.get().cloned()
}

pub struct server_datagram {
    connection_handler: datagram_handler,
    socket: UdpSocket,
    poll: Poll,
    clients: HashMap<Token, SocketAddr>,
    token_counter: usize,
    common_info: server_common_info,
}

impl server_datagram {
    pub fn new() -> server_datagram {
        let common_info = server_common_info::new();

        let socket = UdpSocket::bind("127.0.0.1:8080".parse().unwrap()).unwrap();
        let poll = Poll::new().unwrap();

        let registry = poll.registry();
        registry.register(&socket, SERVER_TOKEN, Interest::READABLE | Interest::WRITABLE).unwrap();

        let connection_handler = datagram_handler::new();

        server_datagram {
            connection_handler,
            socket,
            poll,
            clients: HashMap::new(),
            token_counter: 1,
            common_info,
        }
    }

    pub fn handle_read_event(&mut self, _token: Token) {
        let mut buf = [0; 1024];
        match self.socket.recv_from(&mut buf) {
            Ok((_size, _src_addr)) => {
                EventHeader::action(&buf);
            }
            Err(e) => {
                eprintln!("Failed to receive UDP message: {}", e);
            }
        }
    }

    pub fn run(&mut self) {
        let mut events = Events::with_capacity(1024);
        loop {
            self.poll.poll(&mut events, None).unwrap();
            for event in events.iter() {
                match event.token() {
                    SERVER_TOKEN => {
                        if event.is_readable() {
                            self.handle_read_event(SERVER_TOKEN);
                        }
                    }
                    _ => {
                        eprintln!("Unknown token: {:?}", event.token());
                    }
                }
            }
        }
    }

    pub fn get_id_list(&self) -> HashSet<i64> {
        self.connection_handler.get_id_set_clone()
    }

    pub fn remove_connection(&mut self, token: Token) {
        self.connection_handler.del_connection(token);
    }

    pub fn add_new_connect(&mut self, udp_socket: UdpSocket, token: Token) {
        self.connection_handler.new_connection(udp_socket, token);
    }

    pub fn get_user_connections_by_token(&mut self, token: Token) -> Option<&mut UdpSocket> {
        self.connection_handler.get_connection_by_token(token)
    }

    pub fn get_user_connection_by_id(&mut self, id: i64) -> Option<&mut UdpSocket> {
        self.connection_handler.get_connection_by_id(id)
    }
}
