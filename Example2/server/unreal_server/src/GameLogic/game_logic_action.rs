use super::game_logic_main::*;


pub fn push_command_create(entity_id: u32) {
    let logic = G_GAME_LOGIC.lock().unwrap();
    logic.command_queue.push(Command::Create {
        entity_id
    });    
}

pub fn do_command_create(_command: Command) {
    if let Command::Create { entity_id } = _command {
        println!("Create entity {}", entity_id);

        // 일단 단일월드라고 가정

        let mut game_logic = G_GAME_LOGIC.lock().unwrap();

        if let Some(world) = game_logic.get_world_mut(0) {
            world.create_entity(entity_id);
        }
    }
}

pub fn do_command_delete(_command: Command) {
    if let Command::Create { entity_id } = _command {
        println!("Create entity {}", entity_id);

        // 일단 단일월드라고 가정

        // let mut game_logic = G_GAME_LOGIC.lock().unwrap();
// 
        // if let Some(world) = game_logic.get_world_mut(0) {
        //     world.create_entity(entity_id);
        // }
    }
}

pub fn push_command_move(entity_id: u32, _dx : f32, _dy : f32) {
    let logic = G_GAME_LOGIC.lock().unwrap();
    logic.command_queue.push(Command::Move {
        entity_id,
        dx : _dx,
        dy : _dy
    });
}

pub fn do_command_move(_command : Command) {
    if let Command::Move { entity_id, dx, dy } = _command {
        println!("Moving entity {} by {}, {}", entity_id, dx, dy);
    }
}
