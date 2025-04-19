use crate::get_tcp_server_instance;
use crate::qsm::user_message::message_make_account::{self, MakeAccount};
use crate::qsm::user_message::message_verify_account::{self, VerifyAccount};
use crate::GameLogic::game_player::{get_ve_char_manager_instance, VECharcater};

use super::GameLogic::*;

pub fn CallBack_MakeAccount(buffer: &[u8])
{
    match MakeAccount::deserialize(buffer) {
        Ok(make_account_message) => {

        }
        Err(e)=>{
            eprintln!("Failed to deserialize MakeAccount: {}", e);
        }
    }
}

pub fn CallBack_VerifyAccount(buffer: &[u8])
{
    match VerifyAccount::deserialize(buffer) {
        Ok(verify_account_message) => {
            let _accountId = verify_account_message.userId;
            let _password = verify_account_message.password;
            let _player_name = verify_account_message.userName;
            let _conn = verify_account_message.connect_info;

            

        }
        Err(e) => {
            eprintln!("Failed to deserialize VerifyAccount: {}", e);
        }
    }
}