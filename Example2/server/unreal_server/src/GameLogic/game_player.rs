use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::{RwLock, RwLockReadGuard};
use super::game_geometry::*;


lazy_static! {
    static ref G_VE_CHARACTER_MANAGER_INSTANCE: Arc<RwLock<VECharacterManager>> = 
Arc::new(RwLock::new(VECharacterManager::new()));
}


pub fn get_ve_char_manager_instance() -> &'static Arc<RwLock<VECharacterManager>> {
    &G_VE_CHARACTER_MANAGER_INSTANCE
}

#[derive(Debug, Clone)]
pub struct VEPlayerNetWorkStatus
{
    session_id : i64,
    ip_addr : String
}

#[derive(Debug, Clone)]
pub struct VEPlayerPersonalInfo
{
    player_name : String
}

impl VEPlayerPersonalInfo
{
    pub fn new_zero() -> VEPlayerPersonalInfo {
        return VEPlayerPersonalInfo { player_name: "".to_string() }
    }

    pub fn new(_name : String) -> VEPlayerPersonalInfo {
        return VEPlayerPersonalInfo { player_name: _name }
    }
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

#[derive(Debug, Clone)]
pub struct VECharcater
{
    player_network_config : VEPlayerNetWorkStatus,
    player_personal_info : VEPlayerPersonalInfo,
    transform : Transform
}

impl VECharcater {
    pub fn new_zero() -> Self {
        return VECharcater { 
            player_network_config: VEPlayerNetWorkStatus::new_zero(),
            player_personal_info: VEPlayerPersonalInfo::new_zero(),
            transform: Transform::new_zero() }
    }

}

pub struct VECharacterManager
{
    // index = session_id
    // player_container : Vec<VECharcater>
    player_container_vec : Vec<Arc<Mutex<VECharcater>>>,
    player_container_search_map : HashMap<i64, Arc<Mutex<VECharcater>>>
}

impl VECharacterManager
{
    pub fn new() -> VECharacterManager {
        let mut vec: Vec<Arc<Mutex<VECharcater>>> = Vec::new();
        let mut map: HashMap<i64, Arc<Mutex<VECharcater>>> = HashMap::new();

        return VECharacterManager { 
            player_container_vec: vec, 
            player_container_search_map: map
         }
    }

    pub fn new_character(&mut self, _new_char : VECharcater) {
        
        let _char_arc = Arc::new(Mutex::new(_new_char));

        self.player_container_vec.push(Arc::clone(&_char_arc));
        // let mut _current_top = self.player_container_vec.len() - 1;
        self.player_container_search_map.insert(0, Arc::clone(&_char_arc));
    }
}
