use qsm::qsm::get_event_handler;
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

// User Custom
mod UserLogic;

// Core Logic
mod Core;

use std::sync::Arc;
use std::thread;
// use tokio::time::Duration;

fn main() {
    println!("Server Start");

    // Run UDP
    let unreliable_server_instance = Arc::clone(get_udp_server_instance());
    get_udp_server_instance().write().unwrap().set_connect_info("127.0.0.1:8080".to_string());
    let udp_thread = thread::spawn(move || {
        get_udp_server_instance().write().unwrap().run();
    });
    
    // Run TCP
    let reliable_server_instance = Arc::clone(get_tcp_server_instance());

    get_tcp_server_instance().write().unwrap().set_connect_info("127.0.0.1:8080".to_string());
    let tcp_thread = thread::spawn(move || {
        get_tcp_server_instance().write().unwrap().run();
    });

    udp_thread.join().unwrap();
    tcp_thread.join().unwrap();
    
    // get_event_handler().write().unwrap().init_function_map();
    // get_udp_server_instance().write().unwrap().run();

}
