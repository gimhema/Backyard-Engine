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

            if (true == get_tcp_server_instance().write().unwrap().is_exist_connection(_conn)) 
            {
                // 클라이언트에게 답장 회신
                // 아직 id 검증 로직은 넣지말고

                // 클라는 답장을 받으면 캐릭터 생성뷰로 진입한다.
            }

        }
        Err(e) => {
            eprintln!("Failed to deserialize VerifyAccount: {}", e);
        }
    }
}