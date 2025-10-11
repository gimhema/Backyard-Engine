use crate::GameLogicMain;
use crate::Command;

// Command Actions
impl GameLogicMain {

    pub fn do_command_shoot(&mut self, _command : Command) {
    if let Command::Shoot { entity_id, target_id, damage } = _command {
        println!("Entity {} shoot!", entity_id);

            self.damage_action(entity_id, target_id, damage);

        }
    }
}

// Combat Actions
impl GameLogicMain {
    
    pub fn damage_action(&mut self, attacker_id: u32, target_id: u32, damage: u32) {
        println!("Entity {} damage {} to Entity {}", attacker_id, damage, target_id);


    }


}