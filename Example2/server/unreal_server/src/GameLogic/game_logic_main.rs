use crossbeam::queue::SegQueue;
use std::sync::{Arc, Mutex};
use std::net::SocketAddr;

use super::game_ecs::*;
use super::game_logic_action::*;
use std::collections::HashMap;

// 추가: 전송 인터페이스
use crate::Network::net_tx::NetSender;


#[derive(Debug)]
pub enum Command {
    Create { entity_id: u32 },
    Delete { entity_id: u32 },
    Move { entity_id: u32, loc_x: f32, loc_y: f32, loc_z: f32, q_x: f32, q_y: f32, q_z: f32, q_w: f32 },
    Shoot { entity_id: u32 },

    NetSendUdp { addr: SocketAddr, payload: Vec<u8> },
}

pub struct GameLogicMain {
    pub command_queue: Arc<SegQueue<Command>>,
    pub game_world : World,

    net_tx: Option<Arc<dyn NetSender>>,
}

impl GameLogicMain {
    pub fn new() -> Self {
        GameLogicMain {
            game_world : World::new(),
            command_queue: Arc::new(SegQueue::new()),
            net_tx: None,
        }
    }

    pub fn set_net_sender(&mut self, tx: Arc<dyn NetSender>) {
        self.net_tx = Some(tx);
    }

    pub fn world_create(&mut self) {
        
        // let mut new_world = World::new();
        // new_world.init_world_info(0, WorldType::MainWorld);
        // self.world_container.insert(0, new_world);

        // . . .
    }

    pub fn push_command(&mut self, cmd : Command) {
        self.command_queue.push(cmd);
    }

    pub fn process_commands(&self) {
        while let Some(cmd) = self.command_queue.pop() {
            match cmd {
                Command::Create { entity_id } => {
                    do_command_create(cmd);
                }
                Command::Delete { entity_id } => {
                    do_command_delete(cmd);
                }
                Command::Move { entity_id, loc_x, loc_y, loc_z,q_x, q_y,q_z, q_w } => {
                    do_command_move(cmd);
                }
                Command::Shoot { entity_id } => {
                    println!("Entity {} shoots!", entity_id);
                    // Example: Send a UDP message when an entity shoots
                    // self.try_send_udp(addr, payload);
                }
                Command::NetSendUdp { addr, payload } => {
                    self.try_send_udp(addr, payload);
                }
            }
        }
    }

    #[inline]
    fn try_send_udp(&self, addr: SocketAddr, payload: Vec<u8>) {
        if let Some(tx) = &self.net_tx {
            if let Err(_) = tx.send_udp(addr, payload) {
                // 큐가 가득 찬 경우 로깅/백프레셔 전략 택1
                eprintln!("[GameLogic] UDP queue is full; dropping packet to {}", addr);
            }
        } else {
            eprintln!("[GameLogic] NetSender not set; cannot send UDP");
        }
    }
}
