
use std::vec;

pub mod battle_interface
{
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
        let mut reduced_damage = calc_endurance(target_id, damage);

        hit_process(target_id, reduced_damage);        
    }

    pub fn calc_endurance(target_id : i64, damage : i64) -> i64
    {
        // id를 통해 캐릭터의 방어력을 검사하고 데미지를 감쇠
        return 0
    }

    pub fn hit_process(target_id : i64, damage : i64) 
    {
        // id를 통해 실질적으로 데미지를 적용하여 캐릭터의 체력 감소
    }
}


