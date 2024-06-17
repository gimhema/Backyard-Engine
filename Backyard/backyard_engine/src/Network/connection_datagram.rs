// for udp connection
use std::collections::HashMap;
use std::collections::HashSet;
use mio::net::{UdpSocket, TcpStream};
use mio::Token;

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
