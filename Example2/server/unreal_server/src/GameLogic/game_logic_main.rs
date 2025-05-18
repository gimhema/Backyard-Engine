use crossbeam::queue::SegQueue;
use std::sync::Arc;
use std::sync::Mutex;

use super::game_ecs::*;
use super::game_logic_action::*;

use std::collections::{HashMap};

lazy_static! {
    pub static ref G_GAME_LOGIC : Mutex<GameLogicMain> = Mutex::new(GameLogicMain::new());
}

pub fn push_command_to_game_logic(command : Command) {
    G_GAME_LOGIC.lock().unwrap().push_command(command);
}

#[derive(Debug)]
pub enum Command {
    Create { entity_id: u32 },
    Delete {entity_id: u32},
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

    pub fn world_create(&mut self) {
        let mut new_world = World::new();
        new_world.init_world_info(0, WorldType::MainWorld);
        self.world_container.insert(0, new_world);

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
                Command::Move { entity_id, dx, dy } => {
                    do_command_move(cmd);
                }
                Command::Shoot { entity_id } => {
                    println!("Entity {} shoots!", entity_id);
                }
            }
        }
    }

    // let mut game_logic = G_GAME_LOGIC.lock().unwrap();

    // if let Some(world) = game_logic.get_world(0) {
    //     if let Some(pos) = world.get_position(42) {
    //         println!("Entity 42 position: {:?}", pos);
    //     }
    // }
    pub fn get_world(&self, world_id: i64) -> Option<&World> {
        self.world_container.get(&world_id)
    }


    // if let Some(world) = game_logic.get_world_mut(0) {
    //     world.create_entity_with_components(None, 42);
    // }
    pub fn get_world_mut(&mut self, world_id: i64) -> Option<&mut World> {
        self.world_container.get_mut(&world_id)
    }
}
