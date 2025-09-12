
use std::net::SocketAddr;
use std::sync::Arc;
use crossbeam_queue::ArrayQueue;


pub type SharedUdpMessageQueue = Arc<ArrayQueue<(SocketAddr, Vec<u8>)>>;

pub trait NetSender: Send + Sync {
    fn send_udp(&self, addr: SocketAddr, data: Vec<u8>) -> Result<(), ()>;
}

#[derive(Clone)]
pub struct UdpTx {
    queue: SharedUdpMessageQueue,
}

impl UdpTx {
    pub fn new(queue: SharedUdpMessageQueue) -> Self {
        Self { queue }
    }
}

impl NetSender for UdpTx {
    #[inline]
    fn send_udp(&self, addr: SocketAddr, data: Vec<u8>) -> Result<(), ()> {
        self.queue.push((addr, data)).map_err(|_| ())
    }
}
