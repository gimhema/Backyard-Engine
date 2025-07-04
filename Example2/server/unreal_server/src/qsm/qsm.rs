// use crate::messages::example_message::ExampleMessage;
use crate::{qsm::messages::ExampleMessage, Event::event_handler::EventHeader};
use std::sync::{Arc, RwLock, RwLockReadGuard};
use std::collections::HashMap; // HashMap은 더 이상 필요 없을 수 있지만, 혹시 다른 곳에서 사용된다면 유지

use crate::qsm::user_event::*;

// 콜백 함수들은 그대로 유지됩니다.
use super::user_event::event_chat::CallBack_Chat;
use super::user_event::event_new_player::CallBack_CreateNewPlayer;
use super::user_event::event_player_movement::CallBack_PlayerMovementUpdate;
use super::user_event::event_make_account::CallBack_MakeAccount;
use super::user_event::event_make_account::CallBack_VerifyAccount;
use super::user_event::event_new_player::CallBack_EnterNewPlayerToGame;


// G_EVENT_HANDLER와 event_handler 구조체는 더 이상 필요 없으므로 제거하거나,
// 다른 용도로 사용될 경우를 대비해 유지할 수 있습니다.
// 여기서는 `handle_quicksot_message`가 독립적으로 동작하도록 변경하므로,
// G_EVENT_HANDLER와 event_handler는 사용되지 않습니다.
// 만약 다른 곳에서 여전히 `event_handler` 인스턴스가 필요하다면,
// 이 부분은 유지하되 `init_function_map` 내부의 `function_map` 사용을 제거해야 합니다.

/*
// 필요 없어진 코드 (주석 처리 또는 제거)
lazy_static! {
    static ref G_EVENT_HANDLER: Arc<RwLock<event_handler>> = Arc::new(RwLock::new(event_handler::new()));
}

pub fn get_event_handler() -> &'static Arc<RwLock<event_handler>> {
    &G_EVENT_HANDLER
}

pub struct event_handler {
    // function_map: HashMap<u32, Box<dyn Fn(&[u8]) + Send + Sync>>, // 더 이상 필요 없음
}

impl event_handler {
    pub fn new() -> Self {
        event_handler {
            // function_map: HashMap::new(), // 더 이상 필요 없음
        }
    }

    pub fn init_function_map(&mut self) {
        // 이 함수는 이제 비어 있거나 제거됩니다.
        // 콜백 함수 호출 로직이 handle_quicksot_message로 이동했기 때문입니다.
    }

    pub fn call_func(&self, fid: u32, buffer: &[u8]) {
        // 이 함수도 더 이상 사용되지 않습니다.
    }
}
*/


#[repr(packed)]
pub struct BaseMessage {
    id: u32,  // 메시지 타입을 나타냄
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

/// 수신된 메시지를 기반으로 적절한 콜백 함수를 호출합니다.
pub fn handle_quicksot_message(buffer: &[u8]) {
    // BaseMessage의 ID 확인
    let base_message = match BaseMessage::deserialize(buffer) {
        Ok(msg) => msg,
        Err(e) => {
            eprintln!("Failed to deserialize BaseMessage: {}", e);
            return; // 메시지 파싱 실패 시 처리 중단
        }
    };

    let message_header: EventHeader = base_message.id.into();

    println!("CALL HANDLE FUNC for EventHeader::{:?}", message_header);

    // EventHeader에 따라 콜백 함수를 직접 호출합니다.
    match message_header {
        EventHeader::CHAT_MESSAGE => CallBack_Chat(buffer),
        EventHeader::PLAYER_MOVEMENT_UPDATE => CallBack_PlayerMovementUpdate(buffer),
        EventHeader::NEW_PLAYER => CallBack_CreateNewPlayer(buffer),
        EventHeader::MAKE_ACCOUNT => CallBack_MakeAccount(buffer),
        EventHeader::VERIFY_ACCOUNT => CallBack_VerifyAccount(buffer),
        EventHeader::ENTER_NEW_PAYER => CallBack_EnterNewPlayerToGame(buffer),
        // 향후 추가될 다른 EventHeader 값에 대한 처리
        _ => println!("Unhandled EventHeader: {:?}", message_header),
    }
}