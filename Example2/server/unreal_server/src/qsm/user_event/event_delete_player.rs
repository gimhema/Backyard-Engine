use crate::qsm::user_message::message_delete_player::{self, DeletePlayer};

use crate::Event::event_handler::EventHeader;

pub fn RequestDeletePlayer(id : i64)
{
    let mut _message_id = EventHeader::DELETE_PLAYER as u32;
    let mut _delete_char_msg = DeletePlayer::new(_message_id, id as u32);

    let mut _send_delete_req = _delete_char_msg.serialize();

//    get_tcp_server_instance().write().unwrap().send_message_byte_to_all(_send_delete_req);
    // get_tcp_server_instance().write().unwrap().send_message_byte_to_target(_pid.clone() as i64, _resp_msg);
}