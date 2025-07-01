
use crate::Event::event_handler::EventHeader;

use super::Network;
use std::str::from_utf8;
use mio::event::Event;
use std::sync::Mutex;
use std::collections::{HashMap, HashSet};
use std::time::Duration;
use std::{net, thread, time};
use std::sync::{RwLock, Arc, RwLockReadGuard};
use super::connection::*;
use mio::net::{TcpListener, TcpStream};

use mio::{Events, Interest, Poll, Registry, Token};
use std::io::{self, Read, Write};
use super::server_common::*;

use super::Event::Event::*;

use super::GameLogic::game_player::*;

const SERVER: Token = Token(0);
const SERVER_TICK: u64 = 10;
const TCP_SERVER_CONNECT_INFO : &str = "127.0.0.1:8080";

lazy_static! {
    static ref G_SERVER_INSTANCE: Arc<RwLock<server_stream>> = Arc::new(RwLock::new(server_stream::new()));
}

pub fn get_tcp_server_instance() -> &'static Arc<RwLock<server_stream>> {
    println!("Get TCP Server Lock");
    &G_SERVER_INSTANCE
}

/*
1. 클라이언트가 서버로 접속 요청
2. 서버는 우선 일시적으로 수락
3. 클라이언트는 TCP Recv를 수립받았을때 즉시 계정 정보를 전송
4. 만약 타임아웃된다면(=지정된 시간안에 응답을 못받는다면) 클라이언트는 스스로 해지 (서버는 당연히 disconnect 처리됨)
5. 서버는 계정 정보를 조회하고, 네트워크 정보가 현재 접속 리스트에 있는지를 조회한다음, 검증과정을 통과하면? 접속 처리
6. 만약 6번단계에서 검증 실패할경우 강제 접속 해제 요청
*/

pub struct server_stream {
//    connectionHandler: stream_handler,
    numUser: i64,
    step: i64,
    common_info : server_common_info
}

impl server_stream {

    pub fn new() -> Self {
        let mut _connectionHandler = stream_handler::new();

        let mut _common_info = server_common_info::new();

        server_stream {
//            connectionHandler: _connectionHandler,
            numUser: 0,
            step: 0,
            common_info : _common_info
        }
    }

    pub fn run(&mut self) -> io::Result<()>  {
        env_logger::init();

        let mut userCount: i64 = 0;
        let mut poll = Poll::new()?;
        let mut events = Events::with_capacity(128);

        let mut server = TcpListener::bind(TCP_SERVER_CONNECT_INFO.parse().unwrap())?;
        // let mut server = TcpListener::bind("127.0.0.1:8080".parse().unwrap())?;
    
        // Register the server with poll we can receive events for it.
        poll.registry().register(&mut server, SERVER, Interest::READABLE | Interest::WRITABLE)?;
    
        println!("Run TCP Server . . . .");

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
                        
                        get_connection_handler().write().unwrap().new_tcp_connection(sendConnect, token);
                        get_user_connection_info().write().unwrap().push(token);

                        println!("SendGamePacket End");
                    },
                    token => {

                        let done = {
                            let mut handler = get_connection_handler().write().unwrap();
                            if let Some(connection) = handler.get_tcp_connection_by_token(token) {
                                println!("Handle Connection Event");
                                handle_connection_event(poll.registry(), connection, event)?
                            } else {
                                // Sporadic events happen, we can safely ignore them.
                                false
                            }
                        };


                       if done {
                                println!("Disconn search . . .");
                                if let Some(mut connection)  = 
                                get_connection_handler().write().unwrap().get_tcp_connection_by_token(token)
                                {
                                    println!("User Disconnected . . 1");
                                    poll.registry().deregister(connection);

                                    let _taget_id = get_connection_handler().write().unwrap().get_tcp_connection_id_by_token(token);
                                    
                                    if let Some(id) = _taget_id {
                                        get_ve_char_manager_instance().write().unwrap().delete_characeter(id);
                                    } else {
                                        // 로그를 남기거나, 에러 처리를 할 수 있어요.
                                        eprintln!("Failed to find target id for token: {:?}", token);
                                    }
                                    
                                    get_connection_handler().write().unwrap().del_tcp_connection(token);

                                }
                            }
                    }
                }
                // thread::sleep(Duration::from_secs(1));
            }
            // self.step += 1;
            // println!("server run...")
        }

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

                EventHeader::action(received_data);
                
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
