use mio::{Events, Interest, Poll, Token};
use mio::net::{TcpListener, TcpStream, UdpSocket};
use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use crossbeam_queue::ArrayQueue;
use crate::qsm::*;
use crate::Event::event_handler::EventHeader;
use crate::qsm::qsm::{GLOBAL_MESSAGE_TX_QUEUE, GLOBAL_MESSAGE_UDP_QUEUE};
use crate::GameLogic::game_player::VECharacterManager;
use super::qsm::user_message::message_allow_connect::*;

use super::connection::*;
use super::server_common::*;
use crate::Core::core::*;
use std::thread;
use std::time::{Instant};

// --- 토큰 정의 ---
const SERVER_TCP_TOKEN: Token = Token(0);
const SERVER_UDP_TOKEN: Token = Token(1);
const CLIENT_TOKEN_START: Token = Token(2); // 클라이언트 토큰은 2부터 시작

// --- 메시지를 전송할 Lock-Free 큐 타입 정의 ---
pub type SharedTcpMessageQueue = Arc<ArrayQueue<MessageToSend>>;
pub type SharedUdpMessageQueue = Arc<ArrayQueue<(SocketAddr, Vec<u8>)>>; // (대상 주소, 데이터) 튜플 저장




// --- 서버 구조체 ---
pub struct Server {
    pub server_mode: ServerMode,
    pub poll: Poll,
    pub tcp_listener: TcpListener,
    pub udp_socket: Arc<UdpSocket>,
    pub clients: HashMap<Token, ClientConnection>,
    pub next_client_token: Token,
    pub tcp_message_tx_queue: SharedTcpMessageQueue,
    pub udp_message_tx_queue: SharedUdpMessageQueue,
    // 그룹 관리를 위한 HashMap (Mutex로 보호하여 안전한 동시 접근)
    pub client_groups: Arc<Mutex<HashMap<String, Vec<Token>>>>,
    pub last_ping_time: Instant, // 마지막 Ping 전송 시간을 기록
    pub ping_interval: Duration, // Ping 전송 주기 (예: 5초)

    // Game Play Logic
    pub game_character_manager: Arc<Mutex<VECharacterManager>>,
    pub player_waiting_queue: Arc<Mutex<WaitingQueue>>, // 플레이어 대기열
}


impl Server {
    // --- 서버 인스턴스 생성 ---
pub fn new(tcp_addr: &str, udp_addr: &str) -> io::Result<Server> {
        let poll = Poll::new()?;

        let tcp_listener_addr: SocketAddr = tcp_addr.parse().expect("Invalid TCP address");
        let mut tcp_listener = TcpListener::bind(tcp_listener_addr)?;
        poll.registry().register(&mut tcp_listener, SERVER_TCP_TOKEN, Interest::READABLE)?;

        let udp_socket_addr: SocketAddr = udp_addr.parse().expect("Invalid UDP address");
        let mut udp_socket = UdpSocket::bind(udp_socket_addr)?;
        poll.registry().register(&mut udp_socket, SERVER_UDP_TOKEN, Interest::READABLE)?;

        // TCP 메시지 큐 초기화 (기존 GLOBAL_MESSAGE_TX_QUEUE 사용)
        let tcp_queue_for_server = GLOBAL_MESSAGE_TX_QUEUE.clone();
        // UDP 메시지 큐 초기화 (새로운 큐 생성)
        let udp_queue_for_server = GLOBAL_MESSAGE_UDP_QUEUE.clone();

//        let raw_socket = UdpSocket::bind(udp_socket_addr)?;

        let server = Server {
            server_mode: ServerMode::NONE,
            poll,
            tcp_listener,
            udp_socket : Arc::new(udp_socket),
            clients: HashMap::new(),
            next_client_token: CLIENT_TOKEN_START,
            tcp_message_tx_queue: tcp_queue_for_server,
            udp_message_tx_queue: udp_queue_for_server, // 새 큐 할당
            client_groups: Arc::new(Mutex::new(HashMap::new())),
            last_ping_time: Instant::now(),
            ping_interval: Duration::from_secs(5),
            game_character_manager: Arc::new(Mutex::new(VECharacterManager::new())),
            player_waiting_queue: Arc::new(Mutex::new(WaitingQueue::new())),
        };

        Ok(server)
    }

    // --- 서버 시작 및 이벤트 루프 ---
pub fn start(&mut self) -> io::Result<()> {
        let mut events = Events::with_capacity(1024);

        println!("Server started. Listening on TCP {} and UDP {}",
                 self.tcp_listener.local_addr().unwrap(),
                 self.udp_socket.local_addr().unwrap());

    let udp_queue = self.udp_message_tx_queue.clone();
    let udp_socket = self.udp_socket.clone();

    thread::spawn(move || {
        loop {
            while let Some((target_addr, data)) = udp_queue.pop() {
                match udp_socket.send_to(&data, target_addr) {
                    Ok(n) => println!("[UDP Thread] Sent {} bytes to {}", n, target_addr),
                    Err(e) => eprintln!("[UDP Thread] Error sending to {}: {}", target_addr, e),
                }
            }
            thread::sleep(Duration::from_millis(1));
        }
    });


        loop {
            self.poll.poll(&mut events, Some(Duration::from_millis(1)))?;

            self.server_loop_action();

            self.process_outgoing_tcp_messages();

            let mut actions_to_perform: Vec<(Token, ClientAction)> = Vec::new();

            for event in events.iter() {
                match event.token() {
                    SERVER_TCP_TOKEN => {
                        // ... TCP 연결 수락 로직은 동일 ...
                        loop {
                            match self.tcp_listener.accept() {
                                Ok((mut stream, addr)) => {
                                    println!("Accepted new TCP connection from: {}", addr);
                                    let token = self.next_client_token;
                                    self.next_client_token.0 += 1;

                                    self.poll.registry().register(&mut stream, token, Interest::READABLE | Interest::WRITABLE)?;

                                    println!("Create new player conn info : {:?}", token.clone());
                                    self.player_waiting_queue.lock().unwrap().push(token.clone());

                                    self.clients.insert(token, ClientConnection {
                                        stream,
                                        addr, // TCP 주소
                                        write_queue: Arc::new(Mutex::new(Vec::new())),
                                        is_udp_client: true,
                                        udp_addr: None,
                                    });
                                    
                                    let allow_connect_message = AllowConnectGame::new(
                                        EventHeader::ALLOW_CONNECT_GAME as u32,
                                        0,
                                        token.0 as u32,
                                        "TEST_ACCOUNT".to_string(),
                                        "TEST_CONNECT_NAME".to_string(),
                                        "TEST_CONNECT_INFO".to_string()
                                    );

                                    let send_msg = allow_connect_message.serialize();
                                    let req_enter_message = MessageToSend::Single(token.clone(), send_msg);
                                    
                                    if let Err(_) = self.send_tcp_message(req_enter_message) {
                                        eprintln!("Failed to send message to client with token: {:?}", token);
                                    } else {
                                        eprintln!("Client with token {:?} not found in clients map.", token);
                                    }

                                }
                                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                                    break;
                                }
                                Err(e) => {
                                    eprintln!("Error accepting TCP connection: {}", e);
                                    return Err(e);
                                }
                            }
                        }
                    }
                    SERVER_UDP_TOKEN => {
                        // UDP 메시지 수신 (이벤트 기반)
                        let mut buf = [0; 65507];
                        match self.udp_socket.recv_from(&mut buf) {
                            Ok((len, addr)) => {
                                println!("Received UDP message from {}: {:?}", addr, &buf[..len]);
                                // 수신된 UDP 메시지 처
                                self.udp_recv_action(&buf, len);
                            }
                            Err(e) => {
                                eprintln!("Error receiving UDP message: {}", e);
                            }
                        }
                    }
                    token if token.0 >= CLIENT_TOKEN_START.0 => {
                        // 클라이언트 소켓 이벤트 처리 (TCP 전용)
                        if let Some(client) = self.clients.get_mut(&token) {
                            if event.is_readable() {
                                match ClientConnection::handle_read_event(client) {
                                    Ok(disconnected) => {
                                        if disconnected {
                                            actions_to_perform.push((token, ClientAction::Disconnect));
                                        } else {
                                            actions_to_perform.push((token, ClientAction::Reregister));
                                        }
                                    }
                                    Err(e) => {
                                        eprintln!("Error during read for client {:?}: {}", token, e);
                                        actions_to_perform.push((token, ClientAction::Disconnect));
                                    }
                                }
                            }

                            if event.is_writable() {
                                match ClientConnection::handle_write_event(client) {
                                    Ok(queue_empty) => {
                                        if queue_empty {
                                            actions_to_perform.push((token, ClientAction::Reregister));
                                        } else {
                                            actions_to_perform.push((token, ClientAction::Reregister));
                                        }
                                    }
                                    Err(e) => {
                                        eprintln!("Error during write for client {:?}: {}", token, e);
                                        actions_to_perform.push((token, ClientAction::Disconnect));
                                    }
                                }
                            }
                        } else {
                            eprintln!("Received event for unknown client token: {:?}", token);
                        }
                    }
                    _ => { /* 알 수 없는 토큰 */ }
                }
            }

            // ... (클라이언트 액션 처리 로직은 동일) ...
            for (token, action) in actions_to_perform {
                match action {
                    ClientAction::Disconnect => {
                        if let Some(mut removed_client) = self.clients.remove(&token) {
                            if let Err(e) = self.poll.registry().deregister(&mut removed_client.stream) {
                                eprintln!("Error deregistering stream for client {:?}: {}", token, e);
                            }
                            println!("Client disconnected (action): {}", removed_client.addr);
                            // TODO: 클라이언트 그룹에서 제거하는 로직 추가 필요
                        }
                    }
                    ClientAction::Reregister => {
                        if let Some(client) = self.clients.get_mut(&token) {
                            let interest = if client.write_queue.lock().unwrap().is_empty() {
                                Interest::READABLE
                            } else {
                                Interest::READABLE | Interest::WRITABLE
                            };
                            if let Err(e) = self.poll.registry().reregister(&mut client.stream, token, interest) {
                                eprintln!("Error reregistering stream for client {:?}: {}", token, e);
                            }
                        }
                    }
                }
            }
        }
    }


    // --- 클라이언트를 특정 그룹에 추가하는 함수 (Lock-Free) ---
    pub fn add_client_to_group(&self, token: Token, group_name: String) {
        let mut client_groups = self.client_groups.lock().unwrap();
        // `group_name`의 소유권 이동을 피하기 위해 `clone()` 사용
        client_groups.entry(group_name.clone())
                     .or_insert_with(Vec::new)
                     .push(token);
        println!("Client {:?} added to group '{}'", token, group_name);
    }

    // --- 클라이언트를 그룹에서 제거하는 함수 (Lock-Free) ---
    pub fn remove_client_from_group(&self, token: Token, group_name: &str) {
        let mut client_groups = self.client_groups.lock().unwrap();
        if let Some(tokens) = client_groups.get_mut(group_name) {
            tokens.retain(|&t| t != token);
            if tokens.is_empty() {
                client_groups.remove(group_name);
            }
            println!("Client {:?} removed from group '{}'", token, group_name);
        }
    }
}

// ClientConnection의 이벤트 핸들러는 이제 'Server' 인스턴스와 완전히 독립적입니다.
impl ClientConnection {
    // --- 메시지 수신 처리 (읽기 이벤트) ---
    // 이 함수는 'ClientConnection'에 대한 가변 참조만 받습니다.
    fn handle_read_event(client: &mut ClientConnection) -> io::Result<bool> {
        let mut buffer = Vec::new();
        let mut _read_bytes = 0; // 경고 제거: 'read_bytes'는 사용되지 않지만 할당됨

        loop {
            let mut chunk = [0; 4096]; // 4KB 청크
            match client.stream.read(&mut chunk) {
                Ok(0) => {
                    // 연결 종료
                    println!("Client disconnected: {}", client.addr);
                    return Ok(true); // 연결이 끊겼음을 알림
                }
                Ok(n) => {
                    buffer.extend_from_slice(&chunk[..n]);
                    _read_bytes += n;
                    // 읽을 데이터가 더 이상 없으면 루프 종료
                    if n < chunk.len() {
                        break;
                    }
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    // 더 이상 읽을 데이터가 없음
                    break;
                }
                Err(e) => {
                    eprintln!("Error reading from client {}: {}", client.addr, e);
                    return Err(e);
                }
            }
        }

        if !buffer.is_empty() {
            println!("Received message from client {}: {:?}", client.addr, String::from_utf8_lossy(&buffer));
            // TODO: 수신된 메시지 처리 로직 (예: 게임 로직으로 전달, 파싱 등)
            
            EventHeader::action(&buffer);
        }
        Ok(false) // 연결 유지
    }

    // --- 메시지 송신 처리 (쓰기 이벤트) ---
    // 이 함수는 'ClientConnection'에 대한 가변 참조만 받습니다.
    fn handle_write_event(client: &mut ClientConnection) -> io::Result<bool> {
        let mut write_queue = client.write_queue.lock().unwrap(); // Lock 획득

        if write_queue.is_empty() {
            return Ok(true); // 보낼 데이터가 없음 (큐가 비어있음)
        }

        match client.stream.write(&write_queue) {
            Ok(n) => {
                println!("Sent {} bytes to client {}", n, client.addr);
                // 보낸 데이터만큼 큐에서 제거 
                write_queue.drain(..n);
                Ok(write_queue.is_empty()) // 큐가 비었는지 여부 반환
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                // 쓰기 버퍼가 가득 참, 나중에 다시 시도
                Ok(false)
            }
            Err(e) => {
                eprintln!("Error writing to client {}: {}", client.addr, e);
                Err(e)
            }
        }
    }
}

// 클라이언트 연결 상태 변경을 기록하기 위한 Enum
enum ClientAction {
    Disconnect,
    Reregister,
}

