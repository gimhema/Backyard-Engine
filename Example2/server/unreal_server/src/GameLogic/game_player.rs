use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::{RwLock, RwLockReadGuard};
use crate::qsm::user_event::event_delete_player::RequestDeletePlayer;
use crate::GameLogic::game_parameta_action_logic::ActorParameta;
use mio::Token;

use super::game_geometry::*;
use super::Network::server_common::*;
use super::game_logic_main::*;

use super::game_system_battle::*;
use super::game_system_item::*;
use super::game_system_status::*;
use super::game_system_equipment::*;

lazy_static! {
    static ref G_VE_CHARACTER_MANAGER_INSTANCE: Arc<RwLock<VECharacterManager>> = 
Arc::new(RwLock::new(VECharacterManager::new()));
}


pub fn get_ve_char_manager_instance() -> &'static Arc<RwLock<VECharacterManager>> {
    &G_VE_CHARACTER_MANAGER_INSTANCE
}

#[derive(Debug, Clone)]
pub enum GameNetStatus
{
    // 연결된 상태, 게임에 진입하기를 기다리는 중
    CONNECTED = 0,
    // 게임 서버에 연결되었으며 인게임에 진입하기전 유저가 세팅중인상태
    // 예를들면 캐릭터를 선택중이거나 어떤 월드에 접속하고자하는지 등
    IDLE = 1,
    // 게임 서버에 연결되어 실질적으로 플레이중인 상태
    ACTIVE = 2,
}

#[derive(Debug, Clone)]
pub struct VEPlayerNetWorkStatus
{
    session_id : i64,
    net_token : Token,
    net_status : GameNetStatus,
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
        return VEPlayerNetWorkStatus { session_id: 0, net_token: Token(0), net_status: GameNetStatus::CONNECTED }
    }

    pub fn init(&mut self, _session_id: i64, _token: Token) {
        self.session_id = _session_id;
        self.net_token = _token;
    }

    pub fn set_net_status(&mut self, _status : GameNetStatus) {
        self.net_status = _status;
    }

    pub fn set_net_token(&mut self, _token : Token) {
        self.net_token = _token;
    }

    pub fn set_sessionid(&mut self, _id : i64) {
        self.session_id = _id;
    }
}

#[derive(Debug, Clone)]
pub struct VECharcater
{
    pub player_network_config : VEPlayerNetWorkStatus,
    pub player_personal_info : VEPlayerPersonalInfo,
    pub player_status : ActorStatus,
    pub player_equipment : PlayerEquipment,
    pub player_parameta : ActorParameta
}

impl VECharcater {
    pub fn new_zero() -> Self {
        return VECharcater { 
            player_network_config: VEPlayerNetWorkStatus::new_zero(),
            player_personal_info: VEPlayerPersonalInfo::new_zero(),
            player_status : ActorStatus::new_zero(),
            player_equipment : PlayerEquipment::new(),
            player_parameta : ActorParameta::new_zero()
            }
    }

    pub fn set_player_name(&mut self, _name : String) {
        self.player_personal_info.set_name(_name);
    }

    pub fn set_player_pid(&mut self, _id : i64) {
        self.set_player_pid(_id);
    }

    pub fn set_player_ip_addr(&mut self, _addr : String) {
        self.set_player_ip_addr(_addr);
    }

    pub fn init(&mut self) {
        self.player_status.init();
        self.player_equipment.init();
    }

}

pub struct VECharacterManager
{
//    pub player_container_vec : Vec<Arc<Mutex<VECharcater>>>,
    pub player_container_search_map : HashMap<i64, Arc<Mutex<VECharcater>>>,
//    pub id_top : i64
}

impl VECharacterManager
{
    pub fn new() -> VECharacterManager {
//        let mut vec: Vec<Arc<Mutex<VECharcater>>> = Vec::new();
        let mut map: HashMap<i64, Arc<Mutex<VECharcater>>> = HashMap::new();

        return VECharacterManager { 
//            player_container_vec: vec, 
            player_container_search_map: map,
         }
    }

    pub fn new_character(&mut self, _new_char : VECharcater) {
        
        let _char_arc = Arc::new(Mutex::new(_new_char));

//        self.player_container_vec.push(Arc::clone(&_char_arc));
    }

    pub fn delete_characeter(&mut self, _target_id: i64) {
        if let Some(target_arc) = self.player_container_search_map.remove(&_target_id) {
            // vec에서 해당 Arc를 제거
            push_command_to_game_logic(Command::Delete { entity_id: _target_id.clone() as u32 });
//            self.player_container_vec.retain(|item| {
//                !Arc::ptr_eq(item, &target_arc)
//            });

            RequestDeletePlayer(_target_id);

        } else {
            eprintln!("Tried to delete character with id {}, but not found.", _target_id);
        }
    }
}
