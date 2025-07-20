use super::Core;

use super::Network::message_queue::get_callback_msg_queue_instance;
// use super::Network::server_common::get_user_connection_info;
// use super::Network::server_common::get_connection_handler;

use std::thread;
use std::sync::Arc;
use std::time::Duration;

pub enum ServerMode {
    GARDNER,
    GAME_STATUS_SERVICE,
    GAME_AUTH_SERVICE,
    GAME_CONNECTION_SERVICE,
    GAME_DB_AGENT
}

pub fn read_server_option(mut argv: Vec<String>) {
    println!("Entering run function with arguments: {:?}", argv);

           if argv.len() < 2 {
            println!("Insufficient arguments.");
 //           self.print_help();
            return;
        }

        // argv[2]와 argv[4]를 상대 경로로 변환
        if argv.len() > 2 {
            // argv[2] = to_relative_path(&exe_dir, &argv[2]);
        }
        if argv.len() > 4 {
            // argv[4] = to_relative_path(&exe_dir, &argv[4]);
        }
    
        // self.mode = self.set_mode_by_prefix(argv[1].clone());
        // match self.mode {
        //     MODE::DEFAULT => {
        //         self.print_help();
        //     }
        //     MODE::TEST => {
        //         self.param_valid(argv.clone());
        //     }
        //     MODE::DIRECTORY => {
        //         println!("Directory Mode . . .");
        //         self.parse(argv.clone());
        //     }
        // }
}