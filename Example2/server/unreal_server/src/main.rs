use Network::server_datagram::get_udp_server_instance;


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
    get_udp_server_instance().write().unwrap().run();

}
