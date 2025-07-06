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


fn main() {
    println!("Server Start");

    // get_event_handler().write().unwrap().init_function_map();

    GameConfig::init("");

    get_callback_msg_queue_instance().write().unwrap().clear();
        

}
