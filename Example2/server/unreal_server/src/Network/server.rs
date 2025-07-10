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
use crate::qsm::qsm::GLOBAL_MESSAGE_TX_QUEUE;

use super::connection;

use std::time::{Instant};

// --- 토큰 정의 ---
const SERVER_TCP_TOKEN: Token = Token(0);
const SERVER_UDP_TOKEN: Token = Token(1);
const CLIENT_TOKEN_START: Token = Token(2); // 클라이언트 토큰은 2부터 시작

// --- 메시지를 전송할 Lock-Free 큐 타입 정의 ---
pub type SharedMessageQueue = Arc<ArrayQueue<MessageToSend>>;

// --- 전송할 메시지 유형 정의 ---
#[derive(Debug)]
pub enum MessageToSend {
    Single(Token, Vec<u8>),      // 단일 소켓 대상
    Group(String, Vec<u8>),       // 특정 그룹 소켓 대상 (그룹 이름으로 식별)
    Broadcast(Vec<u8>),           // 전체 소켓 대상
}

// --- 서버 구조체 ---
pub struct Server {
    pub poll: Poll,
    pub tcp_listener: TcpListener,
    pub udp_socket: UdpSocket,
    pub clients: HashMap<Token, ClientConnection>,
    pub next_client_token: Token,
    // 외부에서 메시지를 서버로 보낼 수 있는 큐 (Lock-Free)
    pub message_tx_queue: SharedMessageQueue,
    // 그룹 관리를 위한 HashMap (Mutex로 보호하여 안전한 동시 접근)
    pub client_groups: Arc<Mutex<HashMap<String, Vec<Token>>>>,
    last_ping_time: Instant, // 마지막 Ping 전송 시간을 기록
    ping_interval: Duration, // Ping 전송 주기 (예: 5초)
}

// --- 클라이언트 연결 구조체 ---
// pub struct ClientConnection {
//     pub stream: TcpStream,
//     pub addr: SocketAddr,
//     pub write_queue: Arc<Mutex<Vec<u8>>>,
//     pub is_udp_client: bool, // UDP 클라이언트인지 여부 (TCP와 UDP 연결을 구분)
// }

impl Server {
    // --- 서버 인스턴스 생성 ---
    pub fn new(tcp_addr: &str, udp_addr: &str) -> io::Result<Server> {
        let poll = Poll::new()?;

        // TCP 리스너 설정 및 등록
        let tcp_listener_addr: SocketAddr = tcp_addr.parse().expect("Invalid TCP address");
        let mut tcp_listener = TcpListener::bind(tcp_listener_addr)?;
        poll.registry().register(&mut tcp_listener, SERVER_TCP_TOKEN, Interest::READABLE)?;

        // UDP 소켓 설정 및 등록
        let udp_socket_addr: SocketAddr = udp_addr.parse().expect("Invalid UDP address");
        let mut udp_socket = UdpSocket::bind(udp_socket_addr)?;
        poll.registry().register(&mut udp_socket, SERVER_UDP_TOKEN, Interest::READABLE)?;

        let message_queue_for_server = GLOBAL_MESSAGE_TX_QUEUE.clone();

        let server = Server {
            poll,
            tcp_listener,
            udp_socket,
            clients: HashMap::new(),
            next_client_token: CLIENT_TOKEN_START,
            message_tx_queue: message_queue_for_server, // Assign the cloned Arc here
            client_groups: Arc::new(Mutex::new(HashMap::new())),
            last_ping_time: Instant::now(), // 서버 시작 시 현재 시간으로 초기화
            ping_interval: Duration::from_secs(5), // 5초마다 Ping 전송 (원하는 값으로 조정 가능)
        };

        Ok(server)
    }

    // --- 서버 시작 및 이벤트 루프 ---
    pub fn start(&mut self) -> io::Result<()> {
        let mut events = Events::with_capacity(1024);

        println!("Server started. Listening on TCP {} and UDP {}",
                 self.tcp_listener.local_addr().unwrap(),
                 self.udp_socket.local_addr().unwrap());

        loop {
            self.poll.poll(&mut events, Some(Duration::from_millis(100)))?;


            // --- 추가된 부분: 주기적인 Ping 전송 확인 ---
                                    if self.last_ping_time.elapsed() >= self.ping_interval {
                                        println!("Sending periodic Ping to all connected clients...");
                                        let ping_message_data = "Ping".as_bytes().to_vec(); // "Ping" 문자열을 바이트 벡터로 변환

                                        // `send_message_to_token`이 메시지 길이 프리픽스를 붙이므로, 여기서는 raw "Ping"만 보냄.
                                        // 큐에 메시지를 넣어 비동기적으로 처리하도록 합니다.
                                        if let Err(_) = self.send_message(MessageToSend::Broadcast(ping_message_data)) {
                                            eprintln!("Failed to queue ping broadcast message.");
                                        }
                                        self.last_ping_time = Instant::now(); // 마지막 Ping 전송 시간 업데이트
                                    }

                                    // ... (기존 이벤트 처리 루프와 클라이언트 액션 처리) ...

                                    // 서버 내부 메시지 큐 처리 (여기서 Ping 메시지도 전송됨)
                                    self.process_outgoing_messages()?;


            // 이벤트 처리 중 clients 맵을 직접 수정할 수 없으므로,
            // 수정할 내용을 기록한 후 루프 밖에서 일괄 처리합니다.
            let mut actions_to_perform: Vec<(Token, ClientAction)> = Vec::new();

            for event in events.iter() {
                match event.token() {
                    SERVER_TCP_TOKEN => {
                        // 새로운 TCP 연결 수락
                        loop {
                            match self.tcp_listener.accept() {
                                Ok((mut stream, addr)) => {
                                    println!("Accepted new TCP connection from: {}", addr);
                                    let token = self.next_client_token;
                                    self.next_client_token.0 += 1;

                                    // 클라이언트 스트림 등록 (읽기 및 쓰기 관심)
                                    self.poll.registry().register(&mut stream, token, Interest::READABLE | Interest::WRITABLE)?;

                                    

                                    println!("Create new player conn info : {:?}", token.clone());

                                    self.clients.insert(token, ClientConnection {
                                        stream,
                                        addr,
                                        write_queue: Arc::new(Mutex::new(Vec::new())),
                                        is_udp_client: false,
                                    });
                                }
                                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                                    // 더 이상 대기 중인 연결이 없음
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
                        // UDP 메시지 수신
                        let mut buf = [0; 65507]; // UDP 최대 페이로드 크기
                        match self.udp_socket.recv_from(&mut buf) {
                            Ok((len, addr)) => {
                                println!("Received UDP message from {}: {:?}", addr, &buf[..len]);
                                // TODO: UDP 클라이언트는 TCP와 별개로 관리하거나, 연결된 TCP 클라이언트와 맵핑 필요
                                // UDP 메시지 처리 로직 추가
                            }
                            Err(e) => {
                                eprintln!("Error receiving UDP message: {}", e);
                            }
                        }
                    }
                    token if token.0 >= CLIENT_TOKEN_START.0 => {
                        // 클라이언트 소켓 이벤트 처리
                        if let Some(client) = self.clients.get_mut(&token) {
                            if event.is_readable() {
                                match ClientConnection::handle_read_event(client) { // `self` 없이 호출
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
                                match ClientConnection::handle_write_event(client) { // `self` 없이 호출
                                    Ok(queue_empty) => {
                                        if queue_empty {
                                            actions_to_perform.push((token, ClientAction::Reregister));
                                        } else {
                                            actions_to_perform.push((token, ClientAction::Reregister)); // 아직 보낼 데이터가 있으므로 WRITABLE 유지
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

            // 이벤트 처리 후, 클라이언트 연결 상태 변경 작업 일괄 수행
            for (token, action) in actions_to_perform {
                match action {
                    ClientAction::Disconnect => {
                        if let Some(mut removed_client) = self.clients.remove(&token) {
                            if let Err(e) = self.poll.registry().deregister(&mut removed_client.stream) {
                                eprintln!("Error deregistering stream for client {:?}: {}", token, e);
                            }
                            println!("Client disconnected (action): {}", removed_client.addr);
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

            // 서버 내부 메시지 큐 처리 (Lock-Free 큐에서 메시지를 가져와 전송)
            self.process_outgoing_messages()?;
        }
    }

    // --- Lock-Free 메시지 송신 함수 (외부에서 호출 가능) ---
    pub fn send_message(&self, message: MessageToSend) -> Result<(), ()> {
        if let Err(e) = self.message_tx_queue.push(message) {
            eprintln!("Failed to push message to queue: {:?}", e);
            Err(()) // 큐에 메시지를 넣지 못했음을 알림
        } else {
            Ok(())
        }
    }

    // --- 서버 내부 메시지 큐 처리 및 실제 전송 수행 ---
    fn process_outgoing_messages(&mut self) -> io::Result<()> {
        while let Some(msg) = self.message_tx_queue.pop() {
            match msg {
                MessageToSend::Single(token, data) => {
                    self.send_message_to_token(token, data)?;
                }
                MessageToSend::Group(group_name, data) => {
                    self.send_message_to_group(&group_name, data)?;
                }
                MessageToSend::Broadcast(data) => {
                    self.broadcast_message(data)?;
                }
            }
        }
        Ok(())
    }

    // --- 단일 소켓 대상 메시지 전송 ---
    fn send_message_to_token(&mut self, token: Token, data: Vec<u8>) -> io::Result<()> {
        if let Some(client) = self.clients.get_mut(&token) {
            if client.is_udp_client {
                // TODO: UDP 클라이언트 전송 로직 구현 (주소 관리가 필요)
                println!("Attempted to send UDP message to token {:?}, but UDP client address not fully managed.", token);
                Ok(())
            } else {
                let mut write_queue = client.write_queue.lock().unwrap();
                write_queue.extend_from_slice(&data);
                // 중요: 메시지를 큐에 넣었으므로, WRITABLE 이벤트 관심을 다시 등록하여
                // 다음 poll 루프에서 쓰기 이벤트를 받을 수 있도록 합니다.
                self.poll.registry().reregister(&mut client.stream, token, Interest::READABLE | Interest::WRITABLE)?;
                Ok(())
            }
        } else {
            eprintln!("Attempted to send message to non-existent client with token: {:?}", token);
            Ok(()) // 클라이언트가 이미 끊겼을 수 있으므로 에러가 아닌 경우도 있음
        }
    }

    // --- 특정 그룹 소켓 대상 메시지 전송 ---
    fn send_message_to_group(&mut self, group_name: &str, data: Vec<u8>) -> io::Result<()> {
        let client_groups_lock = self.client_groups.lock().unwrap();
        // 그룹에 속한 토큰 리스트를 복사하여 락 해제 후 안전하게 순회
        let tokens_to_send: Vec<Token> = client_groups_lock
            .get(group_name)
            .cloned() // Option<Vec<Token>>을 clone
            .unwrap_or_else(Vec::new); // 없으면 빈 Vec 반환
        drop(client_groups_lock); // client_groups_lock 해제

        for &token in tokens_to_send.iter() {
            // Lock-Free 큐에 메시지 푸시
            // `send_message`를 사용하도록 변경하여 오류 처리 로직을 중앙화
            if let Err(_) = self.send_message(MessageToSend::Single(token, data.clone())) {
                eprintln!("Failed to queue group message for token {:?}.", token);
            }
        }
        if tokens_to_send.is_empty() {
             println!("Group '{}' not found or is empty for sending message.", group_name);
        }
        Ok(())
    }

    // --- 전체 소켓 대상 메시지 전송 (브로드캐스트) ---
    fn broadcast_message(&mut self, data: Vec<u8>) -> io::Result<()> {
        // 현재 연결된 모든 클라이언트의 토큰 리스트를 복사
        let tokens_to_send: Vec<Token> = self.clients.keys().cloned().collect();
        for token in tokens_to_send {
            // Lock-Free 큐에 메시지 푸시
            // `send_message`를 사용하도록 변경하여 오류 처리 로직을 중앙화
            if let Err(_) = self.send_message(MessageToSend::Single(token, data.clone())) {
                eprintln!("Failed to queue broadcast message for token {:?}.", token);
            }
        }
        Ok(())
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

