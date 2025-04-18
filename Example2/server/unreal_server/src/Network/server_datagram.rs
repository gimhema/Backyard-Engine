use mio::{Events, Poll, Token, Interest, Registry};
use mio::net::UdpSocket;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::str;
use super::connection::connection_handle;
use super::Event::event_handler::*;
use std::sync::{RwLock, Arc, RwLockReadGuard};
use super::connection_datagram::*;
use std::collections::HashSet;
use super::server_common::*;
use std::time::Duration;

const SERVER_TOKEN: Token = Token(0);
const UDP_SERVER_TICK: u64 = 1000;
const UDP_SERVER_CONNECT_INFO : &str = "127.0.0.1:8081";

lazy_static! {
    static ref G_SERVER_DATAGRAM_INSTANCE: Arc<RwLock<server_datagram>> = Arc::new(RwLock::new(server_datagram::new()));
}

pub fn get_udp_server_instance() -> &'static Arc<RwLock<server_datagram>> {
    &G_SERVER_DATAGRAM_INSTANCE
}

pub struct server_datagram {
    connectionHandler : datagram_handler,
    socket: UdpSocket,
    poll: Poll,
    clients: HashMap<Token, SocketAddr>,
    token_counter: usize,
    common_info : server_common_info
}

impl server_datagram {

    pub fn new() -> server_datagram {

        let mut _common_info = server_common_info::new();

        let mut socket = UdpSocket::bind(UDP_SERVER_CONNECT_INFO.parse().unwrap()).unwrap();
        let poll = Poll::new().unwrap();
        
        let mut registry = poll.registry();
        registry.register(&mut socket, SERVER_TOKEN, Interest::READABLE | Interest::WRITABLE).unwrap();

        let mut _connectionHandler = datagram_handler::new();

        server_datagram {
            connectionHandler: _connectionHandler,
            socket,
            poll,
            clients: HashMap::new(),
            token_counter: 1,
            common_info : _common_info
        }
    }

    pub fn handle_read_event(&mut self, token: Token) {
        let mut buf = [0; 1024];
        match self.socket.recv_from(&mut buf) {
            Ok((size, src_addr)) => {
                
                EventHeader::action(&buf);

            }
            Err(e) => {
                eprintln!("Failed to receive UDP message: {}", e);
            }
        }
    }

    pub fn run(&mut self) {

        println!("Run UDP Server . . . .");

        let mut events = Events::with_capacity(1024);
        loop {
            self.poll.poll(&mut events, Some(Duration::from_millis(UDP_SERVER_TICK))).unwrap();
//            self.poll.poll(&mut events, None).unwrap();
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

    pub fn get_id_list(&mut self) -> HashSet<i64> {
        self.connectionHandler.get_id_set_clone()
    }

    pub fn remove_connection(&mut self, token : Token) 
    {
        self.connectionHandler.del_connection(token);
    }
//
    pub fn add_new_connect(&mut self, _udpSocket : UdpSocket, _token: Token) 
    {
        self.connectionHandler.new_connection(_udpSocket, _token);
    }
//
    pub fn get_user_connetions_by_token(&mut self, token: Token) -> Option<&mut UdpSocket>
    {
        self.connectionHandler.get_connetion_by_token(token)
    }
//
    pub fn get_user_connection_by_id(&mut self, id : i64) -> Option<&mut UdpSocket>
    {
        self.connectionHandler.get_connection_by_id(id)
    }
}

/*
fn main() {
    let addr: SocketAddr = "127.0.0.1:9000".parse().unwrap();
    let mut server = server_datagram::new(addr);

    println!("UDP echo server running on {}", addr);

    server.run();
}
*/
