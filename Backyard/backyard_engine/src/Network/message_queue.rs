use std::collections::VecDeque;
use std::sync::{RwLock, Arc};
use mio::Token;

lazy_static!{
    static ref G_CALLBACK_MESSAGE_QUEUE : Arc<RwLock<message_queue_handler>> = Arc::new(RwLock::new(message_queue_handler::new()));
    static ref G_UPDATE_MESSAGE_QUEUE : Arc<RwLock<message_queue_handler>> = Arc::new(RwLock::new(message_queue_handler::new()));
}

pub fn get_callback_msg_queue_instance() -> &'static Arc<RwLock<message_queue_handler>> {
    &G_CALLBACK_MESSAGE_QUEUE
}

pub fn get_update_msg_queue_instance() -> &'static Arc<RwLock<message_queue_handler>> {
    &G_UPDATE_MESSAGE_QUEUE
}

pub struct game_message {
    token : Token,
    message : Vec<u8>
}

impl game_message {
    pub fn new(token : Token, message : Vec<u8>) -> Self {
        game_message { token, message }
    }

    pub fn get_token(&self) -> Token {
        return self.token.clone()
    }

    pub fn get_message(&self) -> Vec<u8> {
        return self.message.clone()
    }
}

pub struct message_queue_handler {
    message_queue : VecDeque<game_message>
}

impl message_queue_handler {
    pub fn new() -> Self {
        message_queue_handler{message_queue : VecDeque::new()}
    }

    pub fn clear(&mut self) {
        self.message_queue.clear();
    }

    pub fn push_message(&mut self, token : Token, message : Vec<u8>) {
        let new_message = game_message::new(token, message);
        self.push(new_message);
    }

    pub fn push(&mut self, message : game_message) {
        self.message_queue.push_back(message)
    }

    pub fn pop(&mut self) -> game_message {
        return self.message_queue.pop_back().unwrap()
    }

    pub fn get_size(&self) -> usize {
        return self.message_queue.len()
    }
    
    pub fn empty(&self) -> bool {
        return self.message_queue.is_empty()       
    }

    
}

