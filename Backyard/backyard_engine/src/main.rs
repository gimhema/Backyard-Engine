use Network::server::get_tcp_server_instance;
use Network::message_queue::get_callback_msg_queue_instance;
use Network::server_common::get_user_connection_info;
use Network::server_common::get_connection_handler;

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

    // Run TCP
    let server_instance = Arc::clone(get_tcp_server_instance());
    thread::spawn(move || {
        get_tcp_server_instance().write().unwrap().run();
        // let server = server_instance.read().unwrap();
        // server.run();
    });

    thread::spawn(move || {
        // listen message . . .
        loop 
        {
            if false == get_callback_msg_queue_instance().read().unwrap().empty() 
            {
                thread::sleep(Duration::from_secs(1)); // Listen Tick
                // pop message
                let mut _msg = get_callback_msg_queue_instance().write().unwrap().pop();

                let mut _targetId = 0; // fetch from parsing msg

                let mut _targetToken = get_user_connection_info().read().unwrap().get_token(_targetId);

                // Test TCP
                // 값 반환 이외에 단순 함수 호출도 read로 가능한가?
                // get_connection_handler()
            }
        }
    });

    loop {
        // println!("Main thread is running...");
        thread::sleep(Duration::from_secs(1));
    }
    // get_tcp_server_instance().write().unwrap().run();
}
