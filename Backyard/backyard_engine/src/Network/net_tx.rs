// net_tx.rs
use std::net::SocketAddr;
use std::sync::Arc;
use crossbeam_queue::ArrayQueue;
use mio::Token;

pub type SharedUdpMessageQueue = Arc<ArrayQueue<(SocketAddr, Arc<[u8]>)>>;

pub trait NetSender: Send + Sync {
    fn send_udp(&self, addr: SocketAddr, data: Vec<u8>) -> Result<(), ()>;
    fn broadcast_udp_all(&self, data: Vec<u8>) -> usize;

    fn send_udp_to_token(&self, token: Token, data: Vec<u8>) -> Result<(), ()>;
}

#[derive(Clone)]
pub struct UdpTx {
    queue: SharedUdpMessageQueue,
    targets_fn: Arc<dyn Fn() -> Vec<SocketAddr> + Send + Sync>,

    resolve_by_token: Arc<dyn Fn(Token) -> Option<SocketAddr> + Send + Sync>,
}

impl UdpTx {
    pub fn new(
        queue: SharedUdpMessageQueue,
        targets_fn: Arc<dyn Fn() -> Vec<SocketAddr> + Send + Sync>,
        resolve_by_token: Arc<dyn Fn(Token) -> Option<SocketAddr> + Send + Sync>,
    ) -> Self {
        Self { queue, targets_fn, resolve_by_token }
    }
}

impl NetSender for UdpTx {
    #[inline]
    fn send_udp(&self, addr: SocketAddr, data: Vec<u8>) -> Result<(), ()> {
        self.queue.push((addr, Arc::<[u8]>::from(data))).map_err(|_| ())
    }

    fn broadcast_udp_all(&self, data: Vec<u8>) -> usize {
        let data = Arc::<[u8]>::from(data);
        let addrs = (self.targets_fn)();
        let mut pushed = 0usize;
        for addr in addrs {
            if self.queue.push((addr, data.clone())).is_ok() {
                pushed += 1;
            }
        }
        pushed
    }

    fn send_udp_to_token(&self, token: Token, data: Vec<u8>) -> Result<(), ()> {
        if let Some(addr) = (self.resolve_by_token)(token) {
            self.queue.push((addr, Arc::<[u8]>::from(data))).map_err(|_| ())
        } else {
            Err(())
        }
    }
}
