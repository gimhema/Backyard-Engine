use mio::{Events, Poll, Token, Interest, Registry};
use mio::net::UdpSocket;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::str;
use super::Event::event_handler::*;
use std::sync::{RwLock, Arc, RwLockReadGuard};
const SERVER_TOKEN: Token = Token(0);

lazy_static! {
    static ref SERVER_DAGAGRAM_ADDR: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    static ref G_SERVER_DATAGRAM_INSTANCE: Arc<RwLock<server_datagram>> = Arc::new(RwLock::new(server_datagram::new()));
}

struct server_datagram {
    socket: UdpSocket,
    poll: Poll,
    clients: HashMap<Token, SocketAddr>,
    token_counter: usize,
}

impl server_datagram {
    fn new() -> server_datagram {
        let mut socket = UdpSocket::bind(*SERVER_DAGAGRAM_ADDR).unwrap();
        let poll = Poll::new().unwrap();
        
        let mut registry = poll.registry();
        registry.register(&mut socket, SERVER_TOKEN, Interest::READABLE | Interest::WRITABLE).unwrap();

        server_datagram {
            socket,
            poll,
            clients: HashMap::new(),
            token_counter: 1,
        }
    }

    fn handle_read_event(&mut self, token: Token) {
        let mut buf = [0; 1024];
        match self.socket.recv_from(&mut buf) {
            Ok((size, src_addr)) => {
                
                let mut msg = str::from_utf8(&buf[..size]).unwrap().to_string();
                listen_event(msg);

            }
            Err(e) => {
                eprintln!("Failed to receive UDP message: {}", e);
            }
        }
    }

    fn run(&mut self) {
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
}

/*
fn main() {
    let addr: SocketAddr = "127.0.0.1:9000".parse().unwrap();
    let mut server = server_datagram::new(addr);

    println!("UDP echo server running on {}", addr);

    server.run();
}
*/
