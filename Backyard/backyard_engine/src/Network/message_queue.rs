use std::collections::VecDeque;



pub trait message_queue_action {
    fn new() -> Self;
    fn push(&mut self, message : String);
    fn pop(&mut self) -> String;
    fn get_size(&self) -> usize;
    fn empty(&self) -> bool;
}

pub struct callback_response_message_queue {
    message_queue : VecDeque<String>
}

impl message_queue_action for callback_response_message_queue {
    fn new() -> Self {
        callback_response_message_queue{message_queue : VecDeque::new()}
    }

    fn push(&mut self, message : String) {
        self.message_queue.push_back(message)
    }

    fn pop(&mut self) -> String {
        return self.message_queue.pop_back().unwrap()
    }

    fn get_size(&self) -> usize {
        return self.message_queue.len()
    }
    
    fn empty(&self) -> bool {
        return self.message_queue.is_empty()       
    }
}

