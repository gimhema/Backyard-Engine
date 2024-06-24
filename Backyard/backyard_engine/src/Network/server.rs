use crate::Event::event_handler::listen_event;

use super::Network;
use std::str::from_utf8;
use mio::event::Event;
use std::sync::Mutex;
use std::collections::{HashMap, HashSet};
use std::time::Duration;
use std::{thread, time};
use std::sync::{RwLock, Arc, RwLockReadGuard};
use super::connection::*;
use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interest, Poll, Registry, Token};
use std::io::{self, Read, Write};

use super::Event::Event::*;

const SERVER: Token = Token(0);
const SERVER_TICK: u64 = 1000;

lazy_static! {
    static ref G_SERVER_INSTANCE: Arc<RwLock<server_stream>> = Arc::new(RwLock::new(server_stream::new()));
}

pub fn get_server_instance() -> &'static Arc<RwLock<server_stream>> {
    &G_SERVER_INSTANCE
}

pub struct server_stream {
    connectionHandler: stream_handler,
    numUser: i64,
    step: i64,
    server_address : String,
    port : i64,
}

impl server_stream {

    pub fn new() -> Self {
        let mut _connectionHandler = stream_handler::new();

        server_stream {
            connectionHandler: _connectionHandler,
            numUser: 0,
            step: 0,
            server_address : "".to_string(),
            port : 0
        }
    }

    pub fn run(&mut self) -> io::Result<()> 
    {
        env_logger::init();

        let mut userCount: i64 = 0;
        let mut poll = Poll::new()?;
        let mut events = Events::with_capacity(128);

        let addr = "127.0.0.1:9000".parse().unwrap();
        let mut server = TcpListener::bind(addr)?;
    
        // Register the server with poll we can receive events for it.
        poll.registry().register(&mut server, SERVER, Interest::READABLE | Interest::WRITABLE)?;
    
        // Map of `Token` -> `TcpStream`.
        // let mut connections = HashMap::new();

        let mut unique_token = Token(SERVER.0 + 1);


        loop {
            // println!("Set Poll Step : {}", self.step);
            poll.poll(&mut events, Some(Duration::from_millis(SERVER_TICK)))?;
    
            // println!("Iterate Event For Loop");
            for event in events.iter() {
                if event.token() == Token(0) && event.is_writable() {
                    println!("Writeable Event . . .");
                }
    
                match event.token() {
                    SERVER => loop {
                        // Received an event for the TCP server socket, which
                        // indicates we can accept an connection.
                        let (mut connection, address) = match server.accept() {
                            Ok((connection, address)) =>  (connection, address),
                            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                                // If we get a `WouldBlock` error we know our
                                // listener has no more incoming connections queued,
                                // so we can return to polling and wait for some
                                // more.
                                break;
                            }
                            Err(e) => {
                                // If it was any other kind of error, something went
                                // wrong and we terminate with an error.
                                return Err(e);
                            }
                        };
                        println!("Accepted connection from: {}", address);
    
                        let token = next(&mut unique_token);
                        poll.registry().register(
                            &mut connection,
                            token,
                            Interest::READABLE.add(Interest::WRITABLE),
                        )?;
                        println!("Add New Player");
                        let mut sendConnect = connection;
                        
                        self.add_new_connect(sendConnect, token);                        
                    

                        println!("SendGamePacket End");
                    },
                    token => {
                       let done = if let Some(connection)  = self.get_user_connetions_by_token(token) 
                        {
                            println!("Handle Connection Event");
                            handle_connection_event(poll.registry(), connection, event)?
                        } 
                        else 
                        {
                            // Sporadic events happen, we can safely ignore them.
                            false
                        };
 
                       if done {
                            println!("Disconn search . . .");
                            if let Some(mut connection)  = self.get_user_connetions_by_token(token)
                            {
                                println!("User Disconnected . . 1");
                                poll.registry().deregister(connection);
                                
                                self.remove_connection(token);
                            }
                       }
                    }
                }
            }

            self.step += 1;

        }

    }

    pub fn get_id_list(&mut self) -> HashSet<i64> {
        self.connectionHandler.get_id_set_clone()
    }

    pub fn remove_connection(&mut self, token : Token) 
    {
        self.connectionHandler.del_connection(token);
    }

    pub fn add_new_connect(&mut self, _tcpStream : TcpStream, _token: Token) 
    {
        self.connectionHandler.new_connection(_tcpStream, _token);
    }

    pub fn get_user_connetions_by_token(&mut self, token: Token) -> Option<&mut TcpStream>
    {
        self.connectionHandler.get_connetion_by_token(token)
    }

    pub fn get_user_connection_by_id(&mut self, id : i64) -> Option<&mut TcpStream>
    {
        self.connectionHandler.get_connection_by_id(id)
    }

}

fn handle_connection_event(
    registry: &Registry,
    connection: &mut TcpStream,
    event: &Event,
) -> io::Result<bool> {
    println!("Handle Connection Event Start . . ");

    if event.is_readable() {
        let mut connection_closed = false;
        let mut received_data = vec![0; 4096];
        let mut bytes_read = 0;
        // We can (maybe) read from the connection.
        loop {
            match connection.read(&mut received_data[bytes_read..]) {
                Ok(0) => {
                    // Reading 0 bytes means the other side has closed the
                    // connection or is done writing, then so are we.
                    connection_closed = true;
                    break;
                }
                Ok(n) => {
                    bytes_read += n;
                    if bytes_read == received_data.len() {
                        received_data.resize(received_data.len() + 1024, 0);
                    }
                }
                // Would block "errors" are the OS's way of saying that the
                // connection is not actually ready to perform this I/O operation.
                Err(ref err) if would_block(err) => break,
                Err(ref err) if interrupted(err) => continue,
                // Other errors we'll consider fatal.
                Err(err) => return Err(err),
            }
        }

        if bytes_read != 0 {

            let received_data = &received_data[..bytes_read];
            if let Ok(str_buf) = from_utf8(received_data) {

                let vec_of_bytes: Vec<u8> = received_data.to_vec();
                let recvMsg = String::from(from_utf8(received_data).unwrap());
                listen_event(recvMsg);
                
            } else {
                println!("Received (none UTF-8) data: {:?}", received_data);
            }
        }

        if connection_closed {
            println!("Connection closed");
            return Ok(true);
        }
    }
    println!("Handle Connection Event End . . ");
    Ok(false)
}

fn next(current: &mut Token) -> Token {
    let next = current.0;
    current.0 += 1;
    Token(next)
}

fn would_block(err: &io::Error) -> bool {
    err.kind() == io::ErrorKind::WouldBlock
}

fn interrupted(err: &io::Error) -> bool {
    err.kind() == io::ErrorKind::Interrupted
}
