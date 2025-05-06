use crossbeam::queue::SegQueue;
use std::sync::Arc;
use std::sync::Mutex;

use super::game_ecs::*;

lazy_static! {
    pub static ref G_GAME_LOGIC : Mutex<GameLogicMain> = Mutex::new(GameLogicMain::new());
}

#[derive(Debug)]
pub enum Command {
    Move { entity_id: u32, dx: f32, dy: f32 },
    Shoot { entity_id: u32 },
}

pub struct GameLogicMain {
    pub command_queue: Arc<SegQueue<Command>>,
}

impl GameLogicMain {
    pub fn new() -> Self {
        GameLogicMain {
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
