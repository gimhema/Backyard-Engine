// use crate::get_udp_server_instance;

use crate::qsm::user_message::message_new_player::*;
use crate::Event::event_handler::EventHeader;
use crate::GameLogic::game_player::{get_ve_char_manager_instance, VECharcater};
use crate::qsm::user_message::message_enter_player_to_game::*;
use crate::qsm::user_message::message_server_response::{self, ServerResponse};

use super::GameLogic::game_logic_main::*;

pub fn CallBack_CreateNewPlayer(buffer: &[u8])
{

}

pub fn CallBack_EnterNewPlayerToGame(buffer: &[u8])
{

}

pub fn CallBack_AllowConnectGame(buffer: &[u8])
{

}