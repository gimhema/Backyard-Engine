

use super::app::*;


impl ServerMonitorApp {

    pub fn add_log_message(&mut self, message: String) {
        self.log_messages.push(message);
    }

    pub fn clear_logs(&mut self) {
        self.log_messages.clear();
    }

}
