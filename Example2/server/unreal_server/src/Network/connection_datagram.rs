// for udp connection
use std::collections::HashMap;
use std::collections::HashSet;
use mio::net::{UdpSocket, TcpStream};
use mio::Token;
use super::connection::*;
use std::sync::{RwLock, Arc, RwLockReadGuard};
lazy_static!{
    static ref G_UDP_CONNECTION_HANDLER: Arc<RwLock<datagram_handler>> = Arc::new(RwLock::new(datagram_handler::new()));
}

pub fn get_udp_connection_instance() -> &'static Arc<RwLock<datagram_handler>> {
    &G_UDP_CONNECTION_HANDLER
}

pub struct connection_datagram
{
    token: Token,
    id: i64,
    udpSocket: UdpSocket
}

impl connection_datagram {
    pub fn new(_token: Token, _id: i64, _sock: UdpSocket) -> Self {
        connection_datagram {
            token : _token,
            id : _id,
            udpSocket : _sock
        }
    }
}

pub struct datagram_handler {
    id_sum : i64,
    connections: HashMap<Token, connection_datagram>,
    tokenIdMap: HashMap<i64, Token>,
    idSet : HashSet<i64>
}

impl connection_handle for datagram_handler {
    fn new() -> Self {
        let mut _connetions = HashMap::new();
        let mut _tokenID = HashMap::new();
        let mut _idSet = HashSet::new();
        datagram_handler{
            id_sum : 0,
            connections : _connetions,
            tokenIdMap : _tokenID,
            idSet : _idSet
        }
    }

    fn get_id_set_clone(&mut self) -> HashSet<i64> {
        self.idSet.clone()
    }

    fn del_connection(&mut self, token : Token) {
        let mut id = self.connections.get(&token).unwrap().id;

        self.connections.remove(&token);
        self.tokenIdMap.remove(&id);
        self.idSet.remove(&id);
    }

    fn get_current_id_sum(&mut self) -> i64 {
        self.id_sum.clone()
    }

    fn update_id_sum(&mut self) {
        self.id_sum += 1;
    }

    fn send(&mut self, _token :Token, _message : String) {
        // Send to udp connection . . .
    }

    fn send_message_byte_to_target(&mut self, target : i64, msg_byte : Vec<u8>) {
        self.tokenIdMap.get(&target).and_then(|token| {
            if let Some(connection) = self.connections.get_mut(token) {
                // 메시지를 전송
                connection.udpSocket.send(&msg_byte).unwrap();
                Some(())
            } else {
                None
            }
        });
    }

    fn send_message_byte_to_all(&mut self, msg_byte : Vec<u8>) {
        for connection in self.connections.values_mut() {
            if let Err(e) = connection.udpSocket.send(&msg_byte) {
                eprintln!("Failed to send message to connection: {:?}", e);
            }
        }
    }

}

impl datagram_handler {

    pub fn get_connection_by_id (&mut self, id : i64) -> Option<&mut UdpSocket>
    {
        let mut _token = self.tokenIdMap.get(&id);

        if let Some(connection) = self.connections.get_mut(_token.unwrap()) {
            Some(&mut connection.udpSocket)
        } else {
            None
        }   
    }
    
    pub fn get_connetion_by_token(&mut self, token: Token) -> Option<&mut UdpSocket>
    {
        if let Some(connection) = self.connections.get_mut(&token) {
            Some(&mut connection.udpSocket)
        } else {
            None
        }
    }

    pub fn new_connection(&mut self, _udpSocket : UdpSocket, _token: Token)
    {
        // Id 처리 로직 필요함
        let _id_top = self.get_current_id_sum();
        let _new_connection = connection_datagram::new(_token, _id_top, _udpSocket);

        self.connections.insert(_token, _new_connection);
        self.tokenIdMap.insert(_id_top, _token);
        self.idSet.insert(_id_top);

        self.update_id_sum();
    }

    
}
