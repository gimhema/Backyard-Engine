

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

    pub fn get_total_health_point(self) -> i64 {
        return self.total_health_point.clone()
    }

    pub fn get_total_ability_point(self) -> i64 {
        return self.total_ability_point.clone()
    }

    pub fn get_total_stamina_point(self) -> i64 {
        return self.total_stamina_point.clone()
    }

    pub fn set_total_health_point(&mut self, val : i64) {
        self.total_health_point = val;
    }

    pub fn set_total_ability_point(&mut self, val : i64) {
        self.total_ability_point = val;
    }

    pub fn set_total_stamina_point(&mut self, val : i64) {
        self.total_ability_point = val;
    }

    pub fn reset_health_point(&mut self) {
        // 파라미터 업데이트 시 수치 중첩을 방지하기위한 리셋
        self.total_health_point = 0;
    }

    pub fn reset_ability_point(&mut self) {
        // 파라미터 업데이트 시 수치 중첩을 방지하기위한 리셋
        self.total_ability_point = 0;
    }

    pub fn reset_stamina_point(&mut self) {
        // 파라미터 업데이트 시 수치 중첩을 방지하기위한 리셋
        self.total_stamina_point = 0;
    }
}

impl VECharcater 
{
    pub fn get_total_health_point(self) -> i64 {
        return self.player_parameta.get_total_health_point()
    }

    pub fn get_total_ability_point(self) -> i64 {
        return self.player_parameta.get_total_ability_point()
    }

    pub fn get_total_stamina_pont(self) -> i64 {
        return self.player_parameta.get_total_stamina_point()
    }

    pub fn update_parameta(&mut self) {
        // 장비 교체시 호출

        // 전부 리셋
        self.player_parameta.reset_health_point();
        self.player_parameta.reset_ability_point();
        self.player_parameta.reset_stamina_point();
        
        // 체력 합산
        self.calc_total_health_point();

        // 어빌리티 포인트 합산
        self.calc_total_ability_point();

        // 스테미너 합산
        self.calc_total_stamina_point();
    }

    pub fn calc_total_health_point(&mut self) {

    }

    pub fn calc_total_ability_point(&mut self) {

    }
    
    pub fn calc_total_stamina_point(&mut self) {

    }
}




