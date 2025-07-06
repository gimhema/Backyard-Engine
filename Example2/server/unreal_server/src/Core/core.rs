use super::Core;

use super::Network::message_queue::get_callback_msg_queue_instance;
use super::Network::server_common::get_user_connection_info;
// use super::Network::server_common::get_connection_handler;

use std::thread;
use std::sync::Arc;
use std::time::Duration;


pub fn SpawnWorkerServerRun() {
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

//                get_connection_handler().write().unwrap().send_message_to_stream(_targetToken, _send_msg);
                println!("Completed Send Message . . .");
            }
        }
    });

}


pub fn MainLogic() {

    SpawnWorkerServerRun();


    loop {
        thread::sleep(Duration::from_secs(1));
    }
}

