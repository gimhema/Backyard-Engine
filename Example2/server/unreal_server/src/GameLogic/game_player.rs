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

    pub fn set_name(&mut self, _name : String) {
        self.player_name = _name;
    }
}


impl VEPlayerNetWorkStatus
{
    pub fn new_zero() -> VEPlayerNetWorkStatus {
        return VEPlayerNetWorkStatus { session_id: 0, ip_addr: "".to_string() }
    }

    pub fn set_net_work_info(_id : i64, _addr : String) -> Self {
        return VEPlayerNetWorkStatus { session_id: _id, ip_addr: _addr }
    }
}

#[derive(Debug, Clone)]
pub struct VECharcater
{
    pub player_network_config : VEPlayerNetWorkStatus,
    pub player_personal_info : VEPlayerPersonalInfo,
    transform : Transform
}

impl VECharcater {
    pub fn new_zero() -> Self {
        return VECharcater { 
            player_network_config: VEPlayerNetWorkStatus::new_zero(),
            player_personal_info: VEPlayerPersonalInfo::new_zero(),
            transform: Transform::new_zero(),
            }
    }

    pub fn set_player_name(&mut self, _name : String) {
        self.player_personal_info.set_name(_name);
    }



}

pub struct VECharacterManager
{
    // index = session_id
    // player_container : Vec<VECharcater>
    player_container_vec : Vec<Arc<Mutex<VECharcater>>>,
    player_container_search_map : HashMap<i64, Arc<Mutex<VECharcater>>>,
    id_top : i64
}

impl VECharacterManager
{
    pub fn new() -> VECharacterManager {
        let mut vec: Vec<Arc<Mutex<VECharcater>>> = Vec::new();
        let mut map: HashMap<i64, Arc<Mutex<VECharcater>>> = HashMap::new();

        return VECharacterManager { 
            player_container_vec: vec, 
            player_container_search_map: map,
            id_top : 0
         }
    }

    pub fn increase_id_top(&mut self) {
        self.id_top += 1;
    }

    pub fn new_character(&mut self, _new_char : VECharcater) {
        
        let _char_arc = Arc::new(Mutex::new(_new_char));

        let _new_id = self.id_top.clone();

        self.player_container_vec.push(Arc::clone(&_char_arc));
        self.player_container_search_map.insert(_new_id, Arc::clone(&_char_arc));

        self.increase_id_top();
    }

    pub fn delete_characeter(&mut self, _target_id: i64) {
        if let Some(target_arc) = self.player_container_search_map.remove(&_target_id) {
            // vec에서 해당 Arc를 제거
            self.player_container_vec.retain(|item| {
                !Arc::ptr_eq(item, &target_arc)
            });
        } else {
            eprintln!("Tried to delete character with id {}, but not found.", _target_id);
        }
    }
}
