use crate::GameLogicMain;
use crate::Command;


impl GameLogicMain {

    pub fn do_command_shoot(&mut self, _command : Command) {
    if let Command::Shoot { entity_id } = _command {
        println!("Entity {} shoot!", entity_id);

        // . . .
        }
    }
}