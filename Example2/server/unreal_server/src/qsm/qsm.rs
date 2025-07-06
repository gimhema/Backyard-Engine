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