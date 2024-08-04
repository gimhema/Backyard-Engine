use Network::server::get_tcp_server_instance;
use Network::server_common::get_common_logic_instance;
use crate::Network::server_common::get_connection_handler_instance;

#[macro_use]
extern crate lazy_static;
use std::thread;
use std::sync::Arc;
use std::time::Duration;

mod Agent;
mod Event;
mod Network;
mod qsm;
mod Crypto;
mod Session;

// User Custom
mod UserLogic;

fn main() {
    println!("Server Start");

    // get_connection_handler_instance().write().unwrap().new();
    // Run TCP
    let server_instance = Arc::clone(get_tcp_server_instance());
    thread::spawn(move || {
        get_tcp_server_instance().write().unwrap().run();
        // let server = server_instance.read().unwrap();
        // server.run();
    });

    loop {
        // println!("Main thread is running...");
        thread::sleep(Duration::from_secs(1));
    }
    // get_tcp_server_instance().write().unwrap().run();
}
