use server::get_tcp_server_instance;

use crate::qsm::QuickShotMessage;

use super::Event;
use super::qsm::qsm::*;
use super::Network::*;
use crate::Network::server_send::send_message_to_all_conn_TEST;

pub enum event_header {
    DEFAULT,
    SEND_MESSAGE_TO_ALL,
    SEND_MESSAGE_TO_TARGET,
    ECHO_MESSAGE,
    END
}

impl From<u32> for event_header {
    fn from(header: u32) -> Self {
        match header {
            0 => event_header::DEFAULT,
            1 => event_header::SEND_MESSAGE_TO_ALL,
            2 => event_header::SEND_MESSAGE_TO_TARGET,
            3 => event_header::ECHO_MESSAGE, // Explicit pattern for 3_i64
            _ => event_header::END, // Wildcard pattern for all other cases
        }
    }
}



pub fn listen_event(msg : String) {

}

impl event_header {
    fn action(&self, buffer: &[u8])
    {

        handle_quicksot_message(buffer);
        // match self {
        //     event_header::DEFAULT => {
        //     }
        //     event_header::SEND_MESSAGE_TO_ALL => {
        //        // ServerAction_CHAT_MESSAGE_ALL(val);
        //     }
        //     event_header::SEND_MESSAGE_TO_TARGET => {
        //        // ServerAction_CHAT_MESSAGE_TO_GROUP(val);
        //     }
        //     event_header::ECHO_MESSAGE => {
        //         // TCP Test
        //         let _msg = "ECHO TEST".to_string();
        //         println!("Call Echo Message");
        //         send_message_to_all_conn_TEST(_msg);
        //         // {
        //         //     get_tcp_server_instance().write().unwrap().send_message_to_all_conn(_msg);
        //         // }

        //     }
        //     event_header::END => {
        //        // ServerAction_CHAT_MESSAGE_TO_ONE(val);
        //     }
        // }
    }
}