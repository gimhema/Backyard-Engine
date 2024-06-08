use std::collections::HashMap;
use mio::net::{TcpListener, TcpStream};
use mio::Token;



pub struct connection
{
    token: Token,
    id: i64,
    tcpStream: TcpStream
}

impl connection {
    pub fn new(_token: Token, _id: i64, _stream: TcpStream) -> Self {
        connection {
            token : _token,
            id : _id,
            tcpStream : _stream
        }
    }
}

pub struct connection_handler {
    connections: HashMap<Token, connection>,
    tokenIdMap: HashMap<i64, Token>
}

impl connection_handler {
    pub fn new() -> Self {
        let mut _connetions = HashMap::new();
        let mut _tokenID = HashMap::new();
        connection_handler{
            connections : _connetions,
            tokenIdMap : _tokenID
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
        let _new_connection = connection::new(_token, 0, _tcpStream);

        self.connections.insert(_token, _new_connection);
        self.tokenIdMap.insert(0, _token);
    }
}
