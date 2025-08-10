
use super::app::*;


impl ServerMonitorApp {

    pub fn command_action(&mut self, command: String) {
        match command.as_str() {
            "clear" => self.clear_logs(),
            _ => self.add_log_message(format!("[CMD] Unknown command: {}", command)),
        }
    }

}