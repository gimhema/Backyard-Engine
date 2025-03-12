// use crate::messages::example_message::ExampleMessage;
use crate::{qsm::messages::ExampleMessage, Event::event_handler::EventHeader};
use std::sync::{Arc, RwLock, RwLockReadGuard};
use std::collections::HashMap;
lazy_static! {
    static ref G_EVENT_HANDLER: Arc<RwLock<event_handler>> = Arc::new(RwLock::new(event_handler::new()));
}

pub fn get_event_handler() -> &'static Arc<RwLock<event_handler>> {
    &G_EVENT_HANDLER
}

pub struct event_handler {
    function_map: HashMap<u32, Box<dyn Fn(&[u8]) + Send + Sync>>,  // &[u8]을 받음
}

impl event_handler {
    pub fn new() -> Self {
        let f_map: HashMap<u32, Box<dyn Fn(&[u8]) + Send + Sync>> = HashMap::new();

        event_handler {
            function_map: f_map,
        }
    }

    pub fn init_function_map(&mut self) {
        // 일반 함수 추가 (버퍼 처리)
        // self.function_map.insert(1, Box::new(print_buffer));
        // self.function_map.insert(2, Box::new(print_buffer_length));
        // self.function_map.insert(3, Box::new(print_first_byte));
    }

    pub fn call_func(&self, fid: u32, buffer: &[u8]) {
        if let Some(func) = self.function_map.get(&fid) {
            func(buffer);
        } else {
            println!("Function ID {} not found!", fid);
        }
    }
}





#[repr(packed)]
pub struct BaseMessage {
    id: u32,   // 메시지 타입을 나타냄
}

impl BaseMessage {
    // 새로운 BaseMessage 생성
    pub fn new(id: u32) -> Self {
        BaseMessage { id }
    }

    // 메시지의 바이너리 직렬화
    pub fn serialize(&self) -> Vec<u8> {
        let mut buffer = Vec::with_capacity(std::mem::size_of::<BaseMessage>());
        buffer.extend(&self.id.to_le_bytes()); // id 값을 리틀 엔디안으로 직렬화
        buffer
    }

    pub fn deserialize(buffer: &[u8]) -> Result<Self, &'static str> {
        if buffer.len() < 4 {
            return Err("Buffer too short");
        }
        let id = u32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
        Ok(BaseMessage { id })
    }
}

pub fn handle_quicksot_message(buffer: &[u8]) {
    // BaseMessage의 ID 확인
    let base_message = BaseMessage::deserialize(buffer).unwrap();

    let base_message_id = base_message.id; // id를 복사

    // Customize . . .
//    let mut _message_header = event_header::from(base_message_id);
    let _message_header : EventHeader = base_message_id.into();

    get_event_handler().write().unwrap().call_func(base_message_id, buffer);

    // Example
    // match _message_header {
    //     EventHeader::DEFAULT => {
    //         println!("message id 0");
    //      }
    //      EventHeader::SEND_MESSAGE_TO_ALL => {
    //         println!("message id 1");
    //         let mut _example_message = ExampleMessage::deserialize(buffer).unwrap();
    //         println!("id : {}", _example_message.id.clone());
    //         println!("val : {}", _example_message.val.clone());
    //         println!("name : {}", _example_message.name.clone());
    //         println!("nums : {:?}", _example_message.nums.clone());
    //      }
    //      EventHeader::SEND_MESSAGE_TO_TARGET => {
    //         println!("message id 2");
    //      }
    //      _ => {
    //          println!("Unknown message id: {}", base_message_id);
    //      }
    //  }
}