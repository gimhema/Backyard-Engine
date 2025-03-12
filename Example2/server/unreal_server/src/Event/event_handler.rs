use std::collections::HashMap;
use std::sync::{Arc, RwLock, RwLockReadGuard};
use server::get_tcp_server_instance;
use crate::qsm::QuickShotMessage;
use super::Event;
use super::qsm::qsm::*;
use super::Network::*;
use crate::Network::server_send::send_message_to_all_conn_TEST;

lazy_static! {
    static ref G_EVENT_HANDLER: Arc<RwLock<event_handler>> = Arc::new(RwLock::new(event_handler::new()));
}

pub struct event_handler {
    function_map: HashMap<u32, Box<dyn Fn(i32) + Send + Sync>>,  // () 반환
}

impl event_handler {
    pub fn new() -> Self {
        let f_map: HashMap<u32, Box<dyn Fn(i32) + Send + Sync>> = HashMap::new();

        event_handler {
            function_map: f_map,
        }
    }

    pub fn init_function_map(&mut self) {
        // 일반 함수 추가 (리턴값 없음)
        // self.function_map.insert(1, Box::new(some_func));
    }

    pub fn call_func(&self, fid: u32, input: i32) {
        if let Some(func) = self.function_map.get(&fid) {
            func(input);
        } else {
            println!("Function ID {} not found!", fid);
        }
    }
}


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