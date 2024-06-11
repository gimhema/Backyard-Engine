use crate::qsm::QuickShotMessage;

use super::Event;
use super::qsm::qsm::*;

pub enum event_header {
    DEFAULT,
    SEND_MESSAGE_TO_ALL,
    SEND_MESSAGE_TO_TARGET,
    END
}

impl From<i64> for event_header {
    fn from(header: i64) -> Self {
        match header {
            0 => event_header::DEFAULT,
            1 => event_header::SEND_MESSAGE_TO_ALL,
            2 => event_header::SEND_MESSAGE_TO_TARGET,
            3 => todo!(), // Explicit pattern for 3_i64
            _ => event_header::END, // Wildcard pattern for all other cases
        }
    }
}

impl event_header {
    fn action(&self, msg : Vec<String>)
    {
        //match self {
        //    FunctionHeader::DEFAULT => {
        //    }
        //    FunctionHeader::CHAT_MESSAGE_ALL => {
        //        ServerAction_CHAT_MESSAGE_ALL(val);
        //    }
        //    FunctionHeader::CHAT_MESSAGE_TO_GROUP => {
        //        ServerAction_CHAT_MESSAGE_TO_GROUP(val);
        //    }
        //    FunctionHeader::CHAT_MESSAGE_TO_ONE => {
        //        ServerAction_CHAT_MESSAGE_TO_ONE(val);
        //    }
        //    FunctionHeader::MOVE_TO_LOCATION => {
        //        ServerAction_MOVE_TO_LOCATION(val);
        //    }
        //    FunctionHeader::CONNECTION_SUCESSFUL => {
        //        ServerAction_CONNECTION_SUCESSFUL(val);
        //    }
        //}
    }
}

pub fn listen_event(msg : String) {

    if let Some((id, size, data)) = deseirialize(&msg) {
        let _data_vec = extract_data(data.as_str());
        let mut q_message = QMessage::new(id as i64, size as usize, _data_vec);

        let event = event_header::from(q_message.get_id());

        event.action(q_message.get_data());

        // println!("id = {}", q_message.get_id());
        // println!("size = {}", q_message.get_size());
        // println!("data = {:?}", q_message.get_data());

    } else {
        println!("Invalid input format");
    }
}
