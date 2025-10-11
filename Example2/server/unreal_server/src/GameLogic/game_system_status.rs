
use crate::GameLogic::game_player::{VECharacterManager, VECharcater};

use super::game_setting::*;

// let cfg = GameConfig::get();
// println!("Server: {}, Max Players: {}", cfg.server_name, cfg.max_players);

#[derive(Debug, Clone)]
pub enum ActorStatusMode 
{
    IDLE = 0,
    ALIVE = 1,
    DEATH = 2
}

#[derive(Debug, Clone)]
pub struct ActorStatus
{
    actor_mode : ActorStatusMode,
    health_point : i64,
    ability_point : i64,
    stamina : i64
}

impl ActorStatus {
    pub fn new_zero() -> Self {
        return ActorStatus { 
            actor_mode : ActorStatusMode::IDLE,
            health_point : 0,
            ability_point : 0,
            stamina : 0
         }
    }

    pub fn init(&mut self) {
        // 캐릭터의 기본 상태정보를 초기화함
        // 장비로 인한 보너스는 init이후 장비 초기화 메소드로 추가해준다.
        let cfg = GameConfig::get();
        self.health_point = cfg.init_health_point;
        self.ability_point = cfg.init_ability_point;
        self.stamina = cfg.init_stamina_point;
        self.actor_mode = ActorStatusMode::ALIVE;
    }

    pub fn set_health_point(&mut self, val : i64) {
        self.health_point = val;
    }

    pub fn set_ability_point(&mut self, val : i64) {
        self.ability_point = val;
    }

    pub fn set_stamina(&mut self, val : i64) {
        self.stamina = val;
    }

    pub fn set_actor_mode(&mut self, val : ActorStatusMode) {
        self.actor_mode = val;
    }

    pub fn get_health_point(&self) -> i64 {
        return self.health_point
    }

    pub fn get_ability_point(&self) -> i64 {
        return self.ability_point
    }

    pub fn get_stamina(&self) -> i64 {
        return self.stamina
    }

    pub fn get_actor_mode(&self) -> ActorStatusMode {
        return self.actor_mode.clone()
    }

}


// Character Method Status Action Implementation
impl VECharcater {
    pub fn set_character_health(&mut self, val : i64) {
//        self.player_status.set_health_point(val);
    }

    pub fn set_character_ability_point(&mut self, val : i64) {
//        self.player_status.set_ability_point(val);
    }

    pub fn set_character_stamina(&mut self, val : i64) {
//        self.player_status.set_stamina(val);
    }

    pub fn set_character_mode(&mut self, val : ActorStatusMode) {
//        self.player_status.set_actor_mode(val);
    }

//     pub fn get_character_health(&self) -> i64 {
// //        return self.player_status.get_health_point()
//     }

//     pub fn get_character_ability_point(&self) -> i64 {
// //        return self.player_status.get_ability_point()
//     }

//     pub fn get_character_stamina(&self) -> i64 {
// //        return self.player_status.get_stamina()
//     }

    // pub fn get_character_mode(&self) -> ActorStatusMode {
    //     return self.player_status.get_actor_mode()
    // }
}

// Character Manager Method Status Action Implementation
impl VECharacterManager {
    pub fn set_character_health(&mut self, id: i64, val: i64) {
        if let Some(char_arc) = self.player_container_search_map.get(&id) {
            if let Ok(mut character) = char_arc.lock() {
                character.set_character_health(val);
            } else {
                eprintln!("Failed to acquire lock for set_character_health, id={}", id);
            }
        } else {
            eprintln!("Character not found for set_character_health, id={}", id);
        }
    }

    pub fn set_character_ability_point(&mut self, id: i64, val: i64) {
        if let Some(char_arc) = self.player_container_search_map.get(&id) {
            if let Ok(mut character) = char_arc.lock() {
                character.set_character_ability_point(val);
            } else {
                eprintln!("Failed to acquire lock for set_character_ability_point, id={}", id);
            }
        } else {
            eprintln!("Character not found for set_character_ability_point, id={}", id);
        }
    }

    pub fn set_character_stamina(&mut self, id: i64, val: i64) {
        if let Some(char_arc) = self.player_container_search_map.get(&id) {
            if let Ok(mut character) = char_arc.lock() {
                character.set_character_stamina(val);
            } else {
                eprintln!("Failed to acquire lock for set_character_stamina, id={}", id);
            }
        } else {
            eprintln!("Character not found for set_character_stamina, id={}", id);
        }
    }

    pub fn set_character_mode(&mut self, id: i64, val: ActorStatusMode) {
        if let Some(char_arc) = self.player_container_search_map.get(&id) {
            if let Ok(mut character) = char_arc.lock() {
                character.set_character_mode(val);
            } else {
                eprintln!("Failed to acquire lock for set_character_mode, id={}", id);
            }
        } else {
            eprintln!("Character not found for set_character_mode, id={}", id);
        }
    }

    // pub fn get_character_health(&self, id: i64) -> i64 {
    //     if let Some(char_arc) = self.player_container_search_map.get(&id) {
    //         if let Ok(character) = char_arc.lock() {
    //             return character.get_character_health();
    //         } else {
    //             eprintln!("Failed to acquire lock for get_character_health, id={}", id);
    //         }
    //     } else {
    //         eprintln!("Character not found for get_character_health, id={}", id);
    //     }
    //     return 0
    // }

    // pub fn get_character_ability_point(&self, id: i64) -> i64 {
    //     if let Some(char_arc) = self.player_container_search_map.get(&id) {
    //         if let Ok(character) = char_arc.lock() {
    //             return character.get_character_ability_point();
    //         } else {
    //             eprintln!("Failed to acquire lock for get_character_ability_point, id={}", id);
    //         }
    //     } else {
    //         eprintln!("Character not found for get_character_ability_point, id={}", id);
    //     }
    //     return 0
    // }

    // pub fn get_character_stamina(&self, id: i64) -> i64 {
    //     if let Some(char_arc) = self.player_container_search_map.get(&id) {
    //         if let Ok(character) = char_arc.lock() {
    //             return character.get_character_stamina();
    //         } else {
    //             eprintln!("Failed to acquire lock for get_character_stamina, id={}", id);
    //         }
    //     } else {
    //         eprintln!("Character not found for get_character_stamina, id={}", id);
    //     }
    //     return 0
    // }

    // pub fn get_character_mode(&self, id: i64) -> ActorStatusMode {
    //     if let Some(char_arc) = self.player_container_search_map.get(&id) {
    //         if let Ok(character) = char_arc.lock() {
    //             return character.get_character_mode();
    //         } else {
    //             eprintln!("Failed to acquire lock for get_character_mode, id={}", id);
    //         }
    //     } else {
    //         eprintln!("Character not found for get_character_mode, id={}", id);
    //     }
    //     return ActorStatusMode::IDLE
    // }
}
