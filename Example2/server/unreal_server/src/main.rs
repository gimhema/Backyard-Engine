// use qsm::qsm::get_event_handler;
use Network::server_datagram::get_udp_server_instance;
use Network::server::get_tcp_server_instance;



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
        
    // Run UDP
    let unreliable_server_instance = Arc::clone(get_udp_server_instance());
    let udp_thread = thread::spawn(move || {
        get_udp_server_instance().write().unwrap().run();
    });
    
    // Run TCP
    let reliable_server_instance = Arc::clone(get_tcp_server_instance());
    let tcp_thread = thread::spawn(move || {
        get_tcp_server_instance().write().unwrap().run();
    });

    {
        G_GAME_LOGIC.lock().unwrap().world_create();
    }

    let game_logic_thread = thread::spawn(||{

        loop {
            let start = Instant::now();
    
            {
                let logic = G_GAME_LOGIC.lock().unwrap();
                logic.process_commands();
            }
    
            let elapsed = start.elapsed();
            if elapsed < Duration::from_millis(16) {
                thread::sleep(Duration::from_millis(16) - elapsed);
            }
        }

    });

    udp_thread.join().unwrap();
    tcp_thread.join().unwrap();
    game_logic_thread.join().unwrap();
    
    // get_event_handler().write().unwrap().init_function_map();
    // get_udp_server_instance().write().unwrap().run();

}
