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
    numUser: i64,
    step: i64,
    common_info : server_common_info
}

impl server_stream {

    pub fn new() -> Self {
        let mut _common_info = server_common_info::new();

        server_stream {
            numUser: 0,
            step: 0,
            common_info : _common_info
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        env_logger::init();

        let mut poll = Poll::new()?;
        let mut events = Events::with_capacity(128);

        let mut server = TcpListener::bind(TCP_SERVER_CONNECT_INFO.parse().unwrap())?;
    
        // Register the server with poll we can receive events for it.
        poll.registry().register(&mut server, SERVER, Interest::READABLE | Interest::WRITABLE)?;
    
        println!("Run TCP Server . . . .");

        let mut unique_token = Token(SERVER.0 + 1);

        loop {
            poll.poll(&mut events, Some(Duration::from_millis(SERVER_TICK)))?;
    
            for event in events.iter() {
                if event.token() == Token(0) && event.is_writable() {
                    println!("Writeable Event . . .");
                }
    
                match event.token() {
                    SERVER => loop {
                        let (mut connection, address) = match server.accept() {
                            Ok((connection, address)) => (connection, address),
                            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                                break;
                            }
                            Err(e) => {
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
                        
                        // 새 연결을 추가할 때 락을 획득합니다.
                        get_tcp_connection_instance().write().unwrap().new_connection(connection, token);
                        // get_user_connection_info().write().unwrap().push(token);

                        println!("SendGamePacket End");
                    },
                    token => {
                        // handle_connection_event 함수가 이제 Token을 인자로 받도록 변경되었으므로,
                        // 이 스코프 내에서 락을 잡고 해제하는 로직이 필요 없어집니다.
                        // 대신, handle_connection_event 내부에서 락을 잡습니다.
                        let done = handle_connection_event(poll.registry(), token, event)?;

                        if done {
                            println!("Disconn search . . .");
                            // 연결 해제 시에도 락을 획득하여 사용합니다.
                            let mut handler = get_tcp_connection_instance().write().unwrap();
                            if let Some(connection) = handler.get_connetion_by_token(token) {
                                println!("User Disconnected . . 1");
                                // deregister는 TcpStream에 대한 참조가 필요하므로,
                                // handler 락을 잡은 상태에서 호출하는 것이 올바릅니다.
                                poll.registry().deregister(connection)?;

                                let _target_id = handler.get_id_by_token(token);
                                
                                if let Some(id) = _target_id {
                                    get_ve_char_manager_instance().write().unwrap().delete_characeter(id);
                                } else {
                                    eprintln!("Failed to find target id for token: {:?}", token);
                                }
                                
                                handler.del_connection(token);
                            }
                            // handler 락은 이 스코프를 벗어나면서 자동으로 해제됩니다.
                        }
                        // 메시지 큐 처리
            
                    }
                }
            }
            let mut connection_handler = get_tcp_connection_instance().write().unwrap();
            connection_handler.message_queue_process();
        }
    }
}

fn handle_connection_event(
    registry: &Registry,
    token: Token, // Token만 전달하여 connection_handler 락을 외부에서 관리하도록 합니다.
    event: &Event,
) -> io::Result<bool> {
    println!("Handle Connection Event Start . . ");

    // 중요한 변경: 여기서 connection_handler 락을 획득하고, 메시지 읽기 직후 해제합니다.
    let mut connection_handler = get_tcp_connection_instance().write().unwrap();
    let mut connection_closed = false;
    let mut received_data = vec![0; 4096];
    let mut bytes_read = 0;

    // 해당 토큰의 TcpStream을 가져옵니다.
    let connection_stream_obj_option = connection_handler.connections.get_mut(&token);

    if let Some(connection_stream_obj) = connection_stream_obj_option {
        let connection = &mut connection_stream_obj.tcpStream;

        if event.is_readable() {
            loop {
                match connection.read(&mut received_data[bytes_read..]) {
                    Ok(0) => {
                        connection_closed = true;
                        break;
                    }
                    Ok(n) => {
                        bytes_read += n;
                        if bytes_read == received_data.len() {
                            received_data.resize(received_data.len() + 1024, 0);
                        }
                    }
                    Err(ref err) if would_block(err) => break,
                    Err(ref err) if interrupted(err) => continue,
                    Err(err) => return Err(err),
                }
            }
        }
    } else {
        eprintln!("Connection not found for token: {:?}", token);
        return Ok(true); // 연결이 없으므로 닫힌 것으로 간주합니다.
    }
    
    // 이 시점에서 `connection_handler` 락은 자동으로 해제됩니다 (함수 스코프를 벗어남).
    // 만약 읽은 데이터가 있다면, 락이 해제된 상태에서 메시지 처리 함수를 호출합니다.
    if bytes_read != 0 {
        let received_data_slice = &received_data[..bytes_read];
        // 여기서 EventHeader::action이 G_TCP_CONNECTION_HANDLER 락을 잡지 않도록 합니다.
        // handle_quicksot_message가 이 버퍼를 받아 처리할 것입니다.
        EventHeader::action(received_data_slice); // `handle_quicksot_message`를 직접 호출할 수도 있습니다.
        println!("Event Actio End");
    }

    if connection_closed {
        println!("Connection closed");
        return Ok(true);
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