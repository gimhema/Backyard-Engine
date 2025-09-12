
use std::net::SocketAddr;
use std::sync::Arc;
use crossbeam_queue::ArrayQueue;


pub type SharedUdpMessageQueue = Arc<ArrayQueue<(SocketAddr, Arc<[u8]>)>>;


pub trait NetSender: Send + Sync {
    fn send_udp(&self, addr: SocketAddr, data: Vec<u8>) -> Result<(), ()>;

    fn broadcast_udp_all(&self, data: Vec<u8>) -> usize;
}

#[derive(Clone)]
pub struct UdpTx {
    queue: SharedUdpMessageQueue,
    targets_fn: Arc<dyn Fn() -> Vec<SocketAddr> + Send + Sync>,
}

impl UdpTx {
    pub fn new(queue: SharedUdpMessageQueue,
               targets_fn: Arc<dyn Fn() -> Vec<SocketAddr> + Send + Sync>) -> Self {
        Self { queue, targets_fn }
    }
}

impl NetSender for UdpTx {
    #[inline]
    fn send_udp(&self, addr: SocketAddr, data: Vec<u8>) -> Result<(), ()> {
        let data = Arc::<[u8]>::from(data); // 1회만 할당
        self.queue.push((addr, data)).map_err(|_| ())
    }

    fn broadcast_udp_all(&self, data: Vec<u8>) -> usize {
        let data = Arc::<[u8]>::from(data); // 1회만 할당
        let addrs = (self.targets_fn)();
        let mut pushed = 0;
        for addr in addrs {
            if self.queue.push((addr, data.clone())).is_ok() {
                pushed += 1;
            }
        }
        pushed
}
}
