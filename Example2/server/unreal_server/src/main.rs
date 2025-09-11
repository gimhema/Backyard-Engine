// use qsm::qsm::get_event_handler;
// use Network::server_datagram::get_udp_server_instance;


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

    // Init Game Logic
    // {
    //     let mut game_logic = G_GAME_LOGIC.lock().unwrap();
    //     game_logic.world_create();
    // }

    // // Create Game Logic Thread
    // let game_logic_thread = thread::spawn(|| {
    //     let tick_duration = Duration::from_millis(50); // 20 ticks per second
    //     let mut last_tick = Instant::now();
    //     loop {
    //         let now = Instant::now();
    //         if now.duration_since(last_tick) >= tick_duration {
    //             // Process game logic here
    //             G_GAME_LOGIC.lock().unwrap().process_commands();
    //             last_tick = now;
    //         } else {
    //             // Sleep for a short duration to avoid busy-waiting
    //             thread::sleep(Duration::from_millis(1));
    //         }
    //     }
    // });

    // 서버 인스턴스 생성
    let mut server = Server::new("127.0.0.1:8080", "127.0.0.1:8082")?;
    // 서버 시작
    server.start()?;

    // Wait game logic thread to finish (it won't in this example)
    // game_logic_thread.join().unwrap();

    Ok(())
}