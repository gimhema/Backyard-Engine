use super::server::*;
use super::connection::*;
use crate::Network::server_datagram::server_datagram;
use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interest, Poll, Registry, Token};
use std::io::{self, Read, Write};


// logic에서 받아와야하나..
pub fn send_message_to_all_conn_TEST(msg : String) {
    println!("Send Game Message TEST {}", msg);

    let id_list;
    {
        println!("Trying to acquire write lock...");
        let server_instance_result = get_tcp_server_instance().write();
        if server_instance_result.is_err() {
            println!("Failed to acquire write lock: {:?}", server_instance_result.err());
            return;
        }
        let mut server_instance = server_instance_result.unwrap();
        println!("Write lock acquired.");
        id_list = server_instance.get_id_list();
    }

    println!("Get Data");

//        for id in get_tcp_server_instance().clone().write().unwrap().get_id_list() {
//            let serialized_msg = msg.as_bytes();
//            if let Some(_targetConn) = get_tcp_server_instance().clone().write().unwrap().get_user_connection_by_id(id) {
//                println!("Send Game Message Step 2");
//                _targetConn.write(serialized_msg);
//            }
//            else {
//                println!("Connection Invalid");
//            }
//        }
}

impl server_stream {

    pub fn send_message_to_all_conn(&mut self, msg : String) {
        println!("Send Game Message {}", msg);
        for id in self.get_id_list() {
            let serialized_msg = msg.as_bytes();
            if let Some(_targetConn) = self.get_user_connection_by_id(id) {
                _targetConn.write(serialized_msg);
            }
            else {
                println!("Connection Invalid");
            }
        }
    }

    pub fn send_message_to_target(&mut self, target : i64, msg : String) {

        let serialized_msg = msg.as_bytes();
        if let Some(_targetConn) = self.get_user_connection_by_id(target) {
            println!("Send Game Message {}", msg);
            _targetConn.write(serialized_msg);
        }
        
    }

    pub fn send_message_to_group(&mut self, target_vec : Vec<i64>, msg : String) {

        for id in target_vec {
            let serialized_msg = msg.as_bytes();
            if let Some(_targetConn) = self.get_user_connection_by_id(id) {
                println!("Send Game Message {}", msg);
                _targetConn.write(serialized_msg);
            }
        }
    }

}

impl server_datagram {
    pub fn send_message_to_all_conn(&mut self, msg : String) {
         for id in self.get_id_list() {
             let serialized_msg = msg.as_bytes();
             if let Some(_targetConn) = self.get_user_connection_by_id(id) {
                 println!("Send Game Message {}", msg);
                 _targetConn.send(serialized_msg);
                 // _targetConn.write(serialized_msg);
             }
         }
    }

    pub fn send_message_to_target(&mut self, target : i64, msg : String) {

        let serialized_msg = msg.as_bytes();
        if let Some(_targetConn) = self.get_user_connection_by_id(target) {
            println!("Send Game Message {}", msg);
            _targetConn.send(serialized_msg);
        }
        
    }

    pub fn send_message_to_group(&mut self, target_vec : Vec<i64>, msg : String) {

        for id in target_vec {
            let serialized_msg = msg.as_bytes();
            if let Some(_targetConn) = self.get_user_connection_by_id(id) {
                println!("Send Game Message {}", msg);
                _targetConn.send(serialized_msg);
            }
        }
    }
}
