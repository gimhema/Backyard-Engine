use std::collections::HashMap;
use mio::net::{TcpListener, TcpStream};
use mio::Token;



pub struct connection
{
    id: i64,
    tcpStream: TcpStream
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
}
