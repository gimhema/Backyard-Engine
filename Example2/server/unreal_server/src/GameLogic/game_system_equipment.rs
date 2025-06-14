
use std::vec;

pub mod battle_interface
{
    pub fn damage_action_player_to_player(attack_id : i64, defense_id : i64, damage : i64)
    {
        let mut reduced_damage = calc_endurance(defense_id, damage);

        hit_process(defense_id, reduced_damage);        
    }

    pub fn damage_action_in_range(defense_id_vec : Vec<i64>, damage : i64)
    {
        
    }

    pub fn calc_endurance(defense_id : i64, damage : i64) -> i64
    {
        return 0
    }

    pub fn hit_process(victim_id : i64, damage : i64) 
    {

    }
}


