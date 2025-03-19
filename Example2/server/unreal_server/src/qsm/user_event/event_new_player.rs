use crate::get_udp_server_instance;
use crate::qsm::user_message::message_new_player::*;


pub fn CallBack_CreateNewPlayer(buffer: &[u8])
{
    match NewPlayer::deserialize(buffer) {
        Ok(new_player_message) => {
            let _pid = new_player_message.pid;
            let _name = new_player_message.name;

            println!("pid : {}", _pid);
            println!("player name : ", _name);
        }
        Err(e)=>{
            eprintln!("Failed to deserialize MovementMessage: {}", e);
        }
    }
}