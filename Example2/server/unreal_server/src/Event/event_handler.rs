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

enum_from_u32! {
    EventHeader {
        DEFAULT = 0,
        SEND_MESSAGE_TO_ALL = 1,
        SEND_MESSAGE_TO_TARGET = 2,
        ECHO_MESSAGE = 3,
        CHAT_MESSAGE = 4,
        PLAYER_MOVEMENT_UPDATE = 5,
        NEW_PLAYER = 6,
        MAKE_ACCOUNT = 7,
        VERIFY_ACCOUNT = 8,
        END = 9
    }
}


impl EventHeader {
    pub fn action(buffer: &[u8])
    {
        handle_quicksot_message(buffer);
    }
}