use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use super::game_geometry::*;

#[derive(Debug)]
pub struct VEPlayerNetWorkStatus
{
    session_id : i64,
    ip_addr : String
}

impl VEPlayerNetWorkStatus
{
    pub fn new_zero() -> VEPlayerNetWorkStatus {
        return VEPlayerNetWorkStatus { session_id: 0, ip_addr: "".to_string() }
    }

    pub fn new(_id : i64, _addr : String) -> Self {
        return VEPlayerNetWorkStatus { session_id: _id, ip_addr: _addr }
    }
}

#[derive(Debug)]
pub struct VECharcater
{
    player_network_config : VEPlayerNetWorkStatus,
    transform : Transform
}

pub struct VECharacterManager
{
    // index = session_id
    // player_container : Vec<VECharcater>
    player_container_vec : Vec<Arc<RefCell<VECharcater>>>,
    player_container_search_map : HashMap<i64, Arc<RefCell<VECharcater>>>
}

impl VECharacterManager
{
    pub fn new() -> VECharacterManager {
        let mut vec: Vec<Arc<RefCell<VECharcater>>> = Vec::new();
        let mut map: HashMap<i64, Arc<RefCell<VECharcater>>> = HashMap::new();

        return VECharacterManager { 
            player_container_vec: vec, 
            player_container_search_map: map
         }
    }
}
