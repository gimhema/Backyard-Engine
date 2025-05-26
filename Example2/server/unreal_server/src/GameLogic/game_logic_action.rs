use super::{game_geometry::{Position, Rotation, Transform}, game_logic_main::*};


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

        game_logic.game_world.create_entity(entity_id);

        // if let Some(world) = game_logic.get_world_mut(0) {
        //     world.create_entity(entity_id);
        // }
    }
}

pub fn do_command_delete(_command: Command) {
    if let Command::Create { entity_id } = _command {
        println!("Create entity {}", entity_id);

        // 일단 단일월드라고 가정

        let mut game_logic = G_GAME_LOGIC.lock().unwrap();
 
        game_logic.game_world.delete_entity(entity_id);
        // if let Some(world) = game_logic.get_world_mut(0) {
        //     world.delete_entity(entity_id);
        // }
    }
}

// pub fn push_command_move(entity_id: u32, loc_x : f32, loc_y : f32, loc_z : f32,roll : f32, pitch : f32,yaw : f32) {
//     let logic = G_GAME_LOGIC.lock().unwrap();
//     logic.command_queue.push(Command::Move {
//         entity_id,
//         loc_x : loc_x,
//         loc_y : loc_y,
//         loc_z : loc_z,
//         roll : roll,
//         pitch : pitch,
//         yaw : yaw    
//     });
// }

pub fn do_command_move(_command : Command) {
    if let Command::Move { entity_id, loc_x, loc_y, loc_z, q_x, q_y, q_z, q_w } = _command {
//        println!("Moving entity {} by {}, {}", entity_id, dx, dy);
        // 추후에 월드 ID를 받아야함

        // 우선 main 월드로 고정

        let mut game_logic = G_GAME_LOGIC.lock().unwrap();

        let update_mov = Transform::new(
            Position::new(loc_x as f64, loc_y as f64, loc_z as f64),
            Rotation::new(q_x as f64, q_y as f64, q_z as f64, q_w as f64));

        game_logic.game_world.update_movement(entity_id, update_mov);
        // if let Some(world) = game_logic.get_world_mut(0) {
        //     world.update_movement(entity_id, update_mov);
        // }
    }
}
