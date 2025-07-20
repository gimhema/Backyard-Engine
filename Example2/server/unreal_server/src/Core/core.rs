use super::Core;

use super::Network::message_queue::get_callback_msg_queue_instance;
// use super::Network::server_common::get_user_connection_info;
// use super::Network::server_common::get_connection_handler;

use std::thread;
use std::sync::Arc;
use std::time::Duration;


#[derive(Debug, Clone, Copy)]
pub enum ServerMode {
    NONE,
    GARDNER,
    GAME_STATUS_SERVICE,
    GAME_AUTH_SERVICE,
    GAME_CONNECTION_SERVICE,
    GAME_DB_AGENT
}

fn get_first_word<'a>(s: &'a str) -> &'a str {
    s.split_whitespace().next().unwrap_or("")
}

pub fn set_mode_by_prefix(argv: String) -> ServerMode {
    let first_word = get_first_word(&argv);
    match first_word.to_lowercase().as_str() {
        "gardner" => ServerMode::GARDNER,
        "gss" => ServerMode::GAME_STATUS_SERVICE,
        "gas" => ServerMode::GAME_AUTH_SERVICE,
        "gcs" => ServerMode::GAME_CONNECTION_SERVICE,
        "gda" => ServerMode::GAME_DB_AGENT,
        _ => ServerMode::NONE,
    }
}

pub fn read_server_option(mut argv: Vec<String>) -> ServerMode {
    println!("Entering run function with arguments: {:?}", argv);

           if argv.len() < 2 {
            println!("Insufficient arguments.");
            return ServerMode::NONE
        }

    
        let mode = set_mode_by_prefix(argv[1].clone());

        return mode
}