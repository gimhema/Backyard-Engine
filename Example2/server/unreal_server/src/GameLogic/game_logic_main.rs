use crossbeam::queue::SegQueue;
use std::sync::Arc;
use std::sync::Mutex;

use super::game_ecs::*;
use super::game_logic_action::*;

use std::collections::{HashMap};

lazy_static! {
    pub static ref G_GAME_LOGIC : Mutex<GameLogicMain> = Mutex::new(GameLogicMain::new());
}

#[derive(Debug)]
pub enum Command {
    Create { entity_id: u32 },
    Move { entity_id: u32, dx: f32, dy: f32 },
    Shoot { entity_id: u32 },
}

pub struct GameLogicMain {
    pub command_queue: Arc<SegQueue<Command>>,
    pub world_container : HashMap<i64, World>
}

impl GameLogicMain {
    pub fn new() -> Self {
        GameLogicMain {
            world_container : HashMap::new(),
            command_queue: Arc::new(SegQueue::new()),
        }
    }

    pub fn process_commands(&self) {
        while let Some(cmd) = self.command_queue.pop() {
            match cmd {
                Command::Create { entity_id } => {
                    do_command_create(cmd);
                }
                Command::Move { entity_id, dx, dy } => {
                    do_command_move(cmd);
                }
                Command::Shoot { entity_id } => {
                    println!("Entity {} shoots!", entity_id);
                }
            }
        }
    }
}
