use crossbeam::queue::SegQueue;
use std::sync::Arc;
use std::sync::Mutex;

use super::game_ecs::*;
use super::game_logic_action::*;

use std::collections::{HashMap};


#[derive(Debug)]
pub enum Command {
    Create { entity_id: u32 },
    Delete {entity_id: u32},
    Move { entity_id: u32, loc_x: f32, loc_y: f32, loc_z: f32, q_x: f32, q_y: f32, q_z: f32, q_w: f32 },
    Shoot { entity_id: u32 },
}

pub struct GameLogicMain {
    pub command_queue: Arc<SegQueue<Command>>,
//    pub world_container : HashMap<i64, World>
    pub game_world : World,
}

impl GameLogicMain {
    pub fn new() -> Self {
        GameLogicMain {
            game_world : World::new(),
            command_queue: Arc::new(SegQueue::new()),
        }
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
                }
            }
        }
    }
}
