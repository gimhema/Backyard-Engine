// use crate::messages::example_message::ExampleMessage;
use crate::{qsm::messages::ExampleMessage, Event::event_handler::event_header};

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
    let mut _message_header = event_header::from(base_message_id);
    // Example
    match _message_header {
        event_header::DEFAULT => {
            println!("message id 0");
         }
         event_header::SEND_MESSAGE_TO_ALL => {
            println!("message id 1");
            let mut _example_message = ExampleMessage::deserialize(buffer).unwrap();
            println!("id : {}", _example_message.id.clone());
            println!("val : {}", _example_message.val.clone());
            println!("name : {}", _example_message.name.clone());
            println!("nums : {:?}", _example_message.nums.clone());
         }
         event_header::SEND_MESSAGE_TO_TARGET => {
            println!("message id 2");
         }
         _ => {
             println!("Unknown message id: {}", base_message_id);
         }
     }
}