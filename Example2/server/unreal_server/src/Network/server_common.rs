use mio::Token;
use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use std::io;
use std::net::SocketAddr;
use std::collections::VecDeque;


pub struct WaitingQueue {
    pub waiting_queue: Arc<RwLock<VecDeque<Token>>>,
}

impl WaitingQueue {
    pub fn new() -> Self {
        WaitingQueue {
            waiting_queue: Arc::new(RwLock::new(VecDeque::new())),
        }
    }

    pub fn push(&self, token: Token) {
        let mut queue = self.waiting_queue.write().unwrap();
        queue.push_back(token);
    }

    pub fn pop(&self) -> Option<Token> {
        let mut queue = self.waiting_queue.write().unwrap();
        queue.pop_front()
    }

    pub fn is_empty(&self) -> bool {
        let queue = self.waiting_queue.read().unwrap();
        queue.is_empty()
    }
    
}
