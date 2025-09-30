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


// User Custom
mod UserLogic;

// Core Logic
mod Core;

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
    let mut server = Server::new("127.0.0.1:8080", "127.0.0.1:8082")?;
    // 서버 시작
    server.start()?;
    Ok(())
}