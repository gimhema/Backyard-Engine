use std::collections::VecDeque;
use std::sync::{RwLock, Arc};
use crate::qsm::*;

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

pub struct message_queue_handler {
    message_queue : VecDeque<String>
}

impl message_queue_handler {
    pub fn new() -> Self {
        message_queue_handler{message_queue : VecDeque::new()}
    }

    pub fn push(&mut self, message : String) {
        self.message_queue.push_back(message)
    }

    pub fn pop(&mut self) -> String {
        return self.message_queue.pop_back().unwrap()
    }

    pub fn get_size(&self) -> usize {
        return self.message_queue.len()
    }
    
    pub fn empty(&self) -> bool {
        return self.message_queue.is_empty()       
    }

    
}

