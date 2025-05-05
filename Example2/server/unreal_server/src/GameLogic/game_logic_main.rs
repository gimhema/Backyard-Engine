use crossbeam::queue::SegQueue;
use std::sync::Arc;

use super::game_ecs::*;

#[derive(Debug)]
pub enum Command {
    Move { entity_id: u32, dx: f32, dy: f32 },
    Shoot { entity_id: u32 },
}

pub struct GameLogic {
    pub command_queue: Arc<SegQueue<Command>>,
}

impl GameLogic {
    pub fn new() -> Self {
        GameLogic {
            command_queue: Arc::new(SegQueue::new()),
        }
    }

    pub fn process_commands(&self) {
        while let Some(cmd) = self.command_queue.pop() {
            match cmd {
                Command::Move { entity_id, dx, dy } => {
                    println!("Moving entity {} by {}, {}", entity_id, dx, dy);
                }
                Command::Shoot { entity_id } => {
                    println!("Entity {} shoots!", entity_id);
                }
            }
        }
    }
}
