

use crate::GameLogic::game_player::{VECharacterManager, VECharcater};
use crate::GameLogic::game_system_equipment::*;
use crate::GameLogic::game_system_status::*;


#[derive(Debug, Clone)]
pub struct ActorParameta
{
    total_health_point : i64, // 기본 체력, 장비등으로 합산되는 실질적인 캐릭터의 체력
    total_ability_point : i64, // 기본 어빌리티 포인트, 장비등으로 합산되는 실질적인 캐릭터의 어빌리티 포인트
    // 데미지는 순수하게 무기의 스탯으로 입힌다.
    total_stamina_point : i64 
}

impl ActorParameta
{
    pub fn new_zero() -> Self {
        return ActorParameta { total_health_point: 0, total_ability_point: 0, total_stamina_point: 0 }
    }

    pub fn update_parameta(&mut self) {

    }

    pub fn get_total_health_point(self) -> i64 {
        return self.total_health_point.clone()
    }

    pub fn get_total_ability_point(self) -> i64 {
        return self.total_ability_point.clone()
    }

    pub fn get_total_stamina_point(self) -> i64 {
        return self.total_stamina_point.clone()
    }
}




