
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

fn main() {
    println!("Server Start");

    Core::core::MainLogic();

    /*
    // Run TCP
    let server_instance = Arc::clone(get_tcp_server_instance());
    thread::spawn(move || {
        get_tcp_server_instance().write().unwrap().run();
    });

    thread::spawn(move || {
        // listen message . . .
        loop 
        {
            if false == get_callback_msg_queue_instance().read().unwrap().empty() 
            {
                println!("Fetch Message . . .");
                // pop message
                let mut _game_msg = get_callback_msg_queue_instance().write().unwrap().pop();
                let mut _targetToken = _game_msg.get_token();
                let mut _send_msg = _game_msg.get_message();

                get_connection_handler().write().unwrap().send_message_to_stream(_targetToken, _send_msg);
                println!("Completed Send Message . . .");
            }
        }
    });

    loop {
        thread::sleep(Duration::from_secs(1));
    }
    */

}
