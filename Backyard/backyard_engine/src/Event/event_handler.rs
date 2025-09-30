use crate::qsm::QuickShotMessage;
use super::Event;
use super::qsm::qsm::*;
use super::Network::*;
// use crate::Network::server_send::send_message_to_all_conn_TEST;


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
        END = 1
    }
}


impl EventHeader {
    pub fn action(buffer: &[u8])
    {
        // handle_quicksot_message(buffer);
    }
}
