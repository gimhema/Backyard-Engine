use super::Network;
use std::str::from_utf8;
use mio::event::Event;
use std::sync::Mutex;
use std::collections::HashMap;
use std::time::Duration;
use std::{thread, time};
use std::sync::{RwLock, Arc, RwLockReadGuard};
use super::connection::*;
use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interest, Poll, Registry, Token};
use std::io::{self, Read, Write};

const SERVER: Token = Token(0);
const SERVER_TICK: u64 = 1000;

lazy_static! {
    static ref G_SERVER_INSTANCE: Arc<RwLock<server>> = Arc::new(RwLock::new(server::new()));
}

pub struct server {
    connectionHandler: connection_handler,
    numUser: i64,
    step: i64,
    server_address : String,
    port : i64,
}

impl server {

    pub fn get_server_instance() -> &'static Arc<RwLock<server>> {
        &G_SERVER_INSTANCE
    }

    pub fn new() -> Self {
        let mut _connectionHandler = connection_handler::new();

        server {
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
    
                        // let token = next(&mut unique_token);
                        // poll.registry().register(
                        //     &mut connection,
                        //     token,
                        //     Interest::READABLE.add(Interest::WRITABLE),
                        // )?;
                        // println!("Add New Player");
                        // let mut sendConnect = connection;
                        
                        // self.AddNewPlayer(sendConnect, token);                        
                    

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
                            // println!("User Disconnected . . 2 2");
                            // Sporadic events happen, we can safely ignore them.
                            false
                        };
    // 
                       // if done {
                       //      //  GetGameLogic().write().unwrap()
                       //      // self.clientHandler.GetConnetionByToken(token)
                       //      println!("Disconn search . . .");
                       //      if let Some(mut connection)  = GetGameLogic().write().unwrap().GetUserConnectionsByToken(token)
                       //      {
                       //          println!("User Disconnected . . 1");
                       //          // poll.registry().deregister(connection);
                       //          // let removeID = self.clientHandler.GetIDByConnection(token);
                       //          // 두 과정은 하나의 함수로 표현해야함
                       //          // self.clientHandler.RemoveConnectionByToken(token);
                       //          // self.clientHandler.RemoveTokenPairByID(removeID);
                       //          // self.RemovePlayerByID(removeID);
                       //          // self.DecreaseNumUser();
                       //      }
                       // }
                    }
                }
            }

            self.step += 1;

        }

    }

    pub fn add_new_connect(&mut self, _tcpStream : TcpStream, _token: Token) 
    {
        self.connectionHandler.new_connection(_tcpStream, _token);
    }

    pub fn get_user_connetions_by_token(&mut self, token: Token) -> Option<&mut TcpStream>
    {
        self.connectionHandler.get_connetion_by_token(token)
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

                // let bytes_slice: &[u8] = &[65, 66, 67, 68, 69];
                let vec_of_bytes: Vec<u8> = received_data.to_vec();
                // Json -> Message Pack
                // MsgPackEventProcess(vec_of_bytes);

                

                // println!("Received data: {}", str_buf.trim_end());
                // // 받은 데이터 처리
                // // 데이터를 수신전용 버퍼에 추가한다.
                // let recvMsg = String::from(from_utf8(received_data).unwrap());
                // CallServerActionByFunctionHeader(Some(recvMsg));

                
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



fn would_block(err: &io::Error) -> bool {
    err.kind() == io::ErrorKind::WouldBlock
}

fn interrupted(err: &io::Error) -> bool {
    err.kind() == io::ErrorKind::Interrupted
}
