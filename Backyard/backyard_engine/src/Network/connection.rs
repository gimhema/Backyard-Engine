use std::collections::HashMap;
use std::collections::HashSet;
use mio::net::{TcpListener, TcpStream};
use mio::Token;

pub trait connection_handle {
    fn new() -> Self;
    fn get_id_set_clone(&mut self) -> HashSet<i64>;
    fn del_connection(&mut self, token : Token);
    fn get_current_id_sum(&mut self) -> i64;
    fn update_id_sum(&mut self);

//    fn get_connetion_by_token(&mut self, token: Token) -> Option<&mut TcpStream>;
//    fn get_connection_by_id (&mut self, id : i64) -> Option<&mut TcpStream>;
//    fn new_connection(&mut self, _tcpStream : TcpStream, _token: Token);
}


pub struct connection_stream
{
    token: Token,
    id: i64,
    tcpStream: TcpStream
}

impl connection_stream {
    pub fn new(_token: Token, _id: i64, _stream: TcpStream) -> Self {
        connection_stream {
            token : _token,
            id : _id,
            tcpStream : _stream
        }
    }
}

pub struct stream_handler {
    id_sum : i64,
    connections: HashMap<Token, connection_stream>,
    tokenIdMap: HashMap<i64, Token>,
    idSet : HashSet<i64>
}

impl connection_handle for stream_handler {
    fn new() -> Self {
        let mut _connetions = HashMap::new();
        let mut _tokenID = HashMap::new();
        let mut _idSet = HashSet::new();
        stream_handler{
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

    

    
}

impl stream_handler {
    pub fn get_connection_by_id (&mut self, id : i64) -> Option<&mut TcpStream>
    {
        let mut _token = self.tokenIdMap.get(&id);

        if let Some(connection) = self.connections.get_mut(_token.unwrap()) {
            Some(&mut connection.tcpStream)
        } else {
            None
        }   
    }

    pub fn get_connetion_by_token(&mut self, token: Token) -> Option<&mut TcpStream>
    {
        if let Some(connection) = self.connections.get_mut(&token) {
            Some(&mut connection.tcpStream)
        } else {
            None
        }
    }

    pub fn new_connection(&mut self, _tcpStream : TcpStream, _token: Token)
    {
        // Id 처리 로직 필요함
        let _id_top = self.get_current_id_sum();
        let _new_connection = connection_stream::new(_token, _id_top, _tcpStream);

        self.connections.insert(_token, _new_connection);
        self.tokenIdMap.insert(_id_top, _token);
        self.idSet.insert(_id_top);

        self.update_id_sum();
    }
}
