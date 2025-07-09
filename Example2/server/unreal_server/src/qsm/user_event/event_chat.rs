// use crate::get_udp_server_instance;
use crate::qsm::user_message::message_chat::{self, ChatMessage};




pub fn CallBack_Chat(buffer: &[u8])
{
    match ChatMessage::deserialize(buffer) {
        Ok(chat_message) => {
            let sender = chat_message.id;
            let chat_content = chat_message.content;

            println!("sender id : {}", sender);
            println!("chat content : {}", chat_content);

//            get_udp_server_instance().write().unwrap().send_message_to_all_conn(buffer);
        }
        Err(e) => {
            eprintln!("Failed to deserialize ChatMessage: {}", e);
        }
    }
}



