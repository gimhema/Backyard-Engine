use mio::{Events, Poll, Token, Interest, Registry};
use mio::net::UdpSocket;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::str;

const SERVER_TOKEN: Token = Token(0);

struct server_datagram {
    socket: UdpSocket,
    poll: Poll,
    clients: HashMap<Token, SocketAddr>,
    token_counter: usize,
}

impl server_datagram {
    fn new(addr: SocketAddr) -> server_datagram {
        let mut socket = UdpSocket::bind(addr).unwrap();
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
                println!("Received {} bytes from {}", size, src_addr);
                println!("Message: {}", str::from_utf8(&buf[..size]).unwrap());
                self.clients.insert(token, src_addr);

                // Echo back to the client
                self.socket.send_to(&buf[..size], src_addr).unwrap();
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

fn main() {
    let addr: SocketAddr = "127.0.0.1:9000".parse().unwrap();
    let mut server = server_datagram::new(addr);

    println!("UDP echo server running on {}", addr);

    server.run();
}
