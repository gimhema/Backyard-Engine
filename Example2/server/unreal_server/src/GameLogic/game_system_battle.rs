
use std::vec;
use crate::GameLogic::game_player::{VECharacterManager, VECharcater};

pub mod battle_interface
{
    use crate::GameLogic::game_player::get_ve_char_manager_instance;

    pub fn damage_action_player_to_player(attack_id : i64, target_id : i64, damage : i64)
    {
        damage_common_action(target_id, damage);
    }

    pub fn damage_action_in_range(target_id_vec : Vec<i64>, damage : i64)
    {
        for victims in target_id_vec {
            damage_common_action(victims, damage);
        }
    }

    pub fn damage_common_action(target_id : i64, damage : i64) 
    {
        let mut total_damage = damage;

        total_damage = calc_endurance(target_id, total_damage.clone());
        total_damage = calc_status_effect(target_id, total_damage.clone());

        hit_process(target_id, total_damage);        
    }

    pub fn calc_endurance(target_id : i64, damage : i64) -> i64
    {
        let mut calced_damage = damage;
        // id를 통해 캐릭터의 방어력을 검사하고 데미지를 감쇠
        return calced_damage
    }

    pub fn calc_status_effect(target_id : i64, damage : i64) -> i64
    {
        let mut calced_damage = damage;

        return calced_damage
    }

    pub fn hit_process(target_id : i64, damage : i64) 
    {
        let mut current_target_health = get_ve_char_manager_instance().write().unwrap().get_character_health(target_id);
        let mut updatped_health = current_target_health - damage;

        // id를 통해 실질적으로 데미지를 적용하여 캐릭터의 체력 감소
        if (updatped_health <= 0) {
            get_ve_char_manager_instance().write().unwrap().set_character_health(target_id, 0);
        } 
        else {
            get_ve_char_manager_instance().write().unwrap().set_character_health(target_id, updatped_health);
        }

    }
}


