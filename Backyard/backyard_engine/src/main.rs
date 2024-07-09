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

fn main() {
    println!("Server Start");

    // Run TCP
    get_tcp_server_instance().write().unwrap().run();
}
