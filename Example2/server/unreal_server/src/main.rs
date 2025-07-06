// use qsm::qsm::get_event_handler;
use Network::server_datagram::get_udp_server_instance;


#[macro_use]
extern crate lazy_static;

mod Agent;
mod Event;
mod Network;
mod qsm;
mod Crypto;
mod Session;
mod GameLogic;

// User Custom
mod UserLogic;

// Core Logic
mod Core;
use crate::GameLogic::game_logic_main::*;
use crate::GameLogic::game_setting::*;
use std::sync::Arc;
use std::thread;
use std::sync::Mutex;
use std::time::{Duration, Instant};
// use tokio::time::Duration;
use crate::Network::message_queue::*;
use crate::Network::server::*;
use tokio::io;
use mio::Token;

// // --- 메인 함수 ---
fn main() -> io::Result<()> {
    // 서버 인스턴스 생성
    let mut server = Server::new("127.0.0.1:8080", "127.0.0.1:8081")?;

    // 서버 시작
    server.start()?;

    Ok(())
}


/*
    // 메시지 송신 예시를 위한 Arc 클론 (다른 스레드에서 호출될 수 있다고 가정)
    let message_sender_arc = Arc::clone(&server.message_tx_queue);
    let client_group_manager_arc = Arc::clone(&server.client_groups);

    std::thread::spawn(move || {
        std::thread::sleep(Duration::from_secs(5)); // 서버 시작 대기

        println!("\n--- Sending test messages ---");
        let token_for_group = Token(2); 
        let group_name = "gamers".to_string();
        let mut groups = client_group_manager_arc.lock().unwrap();
        groups.entry(group_name.clone()).or_insert_with(Vec::new).push(token_for_group);
        drop(groups); // 락 해제
        println!("Token({:?}) added to '{}' group (example)", token_for_group.0, group_name);

        let msg = MessageToSend::Single(token_for_group, "Hello Client 2!".as_bytes().to_vec());
        if let Err(_) = message_sender_arc.push(msg) { // push는 이제 TrySendError를 반환하지만, 우리는 성공/실패 여부만 알면 됩니다.
            eprintln!("Failed to push single message to queue.");
        }
        std::thread::sleep(Duration::from_millis(500));
        let group_msg = MessageToSend::Group(group_name, "Message to Gamers!".as_bytes().to_vec());
        if let Err(_) = message_sender_arc.push(group_msg) {
            eprintln!("Failed to push group message to queue.");
        }
        std::thread::sleep(Duration::from_millis(500));

        let broadcast_msg = MessageToSend::Broadcast("Hello All Clients!".as_bytes().to_vec());
        if let Err(_) = message_sender_arc.push(broadcast_msg) {
            eprintln!("Failed to push broadcast message to queue.");
        }

        std::thread::sleep(Duration::from_secs(5)); // 추가 메시지 전송 대기
    });
*/