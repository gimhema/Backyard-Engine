use server::get_tcp_server_instance;

use crate::qsm::QuickShotMessage;

use super::Event;
use super::qsm::qsm::*;
use super::Network::*;
use crate::Network::server_send::send_message_to_all_conn_TEST;

macro_rules! enum_from_u32 {
    ($name:ident { $($variant:ident = $value:expr),* $(,)? }) => {
        #[repr(u32)]
        #[derive(Debug, Clone, Copy)]
        pub enum $name {
            $($variant = $value),*
        }

        impl From<u32> for $name {
            fn from(header: u32) -> Self {
                match header {
                    $($value => $name::$variant),*,
                    _ => $name::END,
                }
            }
        }
    };
}


// pub enum event_header {
//     DEFAULT,
//     SEND_MESSAGE_TO_ALL,
//     SEND_MESSAGE_TO_TARGET,
//     ECHO_MESSAGE,
//     END
// }


// impl From<u32> for event_header {
//     fn from(header: u32) -> Self {
//         match header {
//             0 => event_header::DEFAULT,
//             1 => event_header::SEND_MESSAGE_TO_ALL,
//             2 => event_header::SEND_MESSAGE_TO_TARGET,
//             3 => event_header::ECHO_MESSAGE, // Explicit pattern for 3_i64
//             _ => event_header::END, // Wildcard pattern for all other cases
//         }
//     }
// }

enum_from_u32! {
    EventHeader {
        DEFAULT = 0,
        SEND_MESSAGE_TO_ALL = 1,
        SEND_MESSAGE_TO_TARGET = 2,
        ECHO_MESSAGE = 3,
        END = 4
    }
}



pub fn listen_event(msg : String) {

}

impl EventHeader {
    pub fn action(buffer: &[u8])
    {
        handle_quicksot_message(buffer);
    }
}