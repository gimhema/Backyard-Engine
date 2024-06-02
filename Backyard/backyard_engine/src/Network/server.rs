use super::Network;


use std::sync::Mutex;
use std::collections::HashMap;
use std::time::Duration;
use std::{thread, time};
use std::sync::{RwLock, Arc, RwLockReadGuard};
use super::connection::*;
use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interest, Poll, Registry, Token};
use std::io::{self, Read, Write};


const SERVER: Token = Token(0);
const SERVER_TICK: u64 = 1000;

pub struct server {
    connectionHandler: connection_handler,
    numUser: i64,
    step: i64,
    server_address : String,
    port : i64,
}

impl server {
    pub fn new() -> Self {
        let mut _connectionHandler = connection_handler::new();

        server {
            connectionHandler: _connectionHandler,
            numUser: 0,
            step: 0,
            server_address : "".to_string(),
            port : 0
        }
    }

    pub fn run(&mut self) -> io::Result<()> 
    {
        env_logger::init();

        let mut userCount: i64 = 0;
        let mut poll = Poll::new()?;
        let mut events = Events::with_capacity(128);

        let addr = "127.0.0.1:9000".parse().unwrap();
        let mut server = TcpListener::bind(addr)?;
    
        // Register the server with poll we can receive events for it.
        poll.registry().register(&mut server, SERVER, Interest::READABLE | Interest::WRITABLE)?;
    
        // Map of `Token` -> `TcpStream`.
        // let mut connections = HashMap::new();

        let mut unique_token = Token(SERVER.0 + 1);


        loop {
            // println!("Set Poll Step : {}", self.step);
            poll.poll(&mut events, Some(Duration::from_millis(SERVER_TICK)))?;
    
            // println!("Iterate Event For Loop");
            for event in events.iter() {
                if event.token() == Token(0) && event.is_writable() {
                    println!("Writeable Event . . .");
                }
    
                match event.token() {
                    SERVER => loop {
                        // Received an event for the TCP server socket, which
                        // indicates we can accept an connection.
                        let (mut connection, address) = match server.accept() {
                            Ok((connection, address)) =>  (connection, address),
                            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                                // If we get a `WouldBlock` error we know our
                                // listener has no more incoming connections queued,
                                // so we can return to polling and wait for some
                                // more.
                                break;
                            }
                            Err(e) => {
                                // If it was any other kind of error, something went
                                // wrong and we terminate with an error.
                                return Err(e);
                            }
                        };
                        println!("Accepted connection from: {}", address);
    
                        // let token = next(&mut unique_token);
                        // poll.registry().register(
                        //     &mut connection,
                        //     token,
                        //     Interest::READABLE.add(Interest::WRITABLE),
                        // )?;
                        // println!("Add New Player");
                        // let mut sendConnect = connection;
                        
                        // self.AddNewPlayer(sendConnect, token);                        
                    

                        println!("SendGamePacket End");
                    },
                    token => {
                       // let done = if let Some(connection)  = GetGameLogic().write().unwrap().GetUserConnectionsByToken(token) 
                       //  {
                       //      println!("Handle Connection Event");
                       //      // handle_connection_event(poll.registry(), connection, event)?
                       //  } 
                       //  else 
                       //  {
                       //      // println!("User Disconnected . . 2 2");
                       //      // Sporadic events happen, we can safely ignore them.
                       //      false
                       //  };
    // 
                       // if done {
                       //      //  GetGameLogic().write().unwrap()
                       //      // self.clientHandler.GetConnetionByToken(token)
                       //      println!("Disconn search . . .");
                       //      if let Some(mut connection)  = GetGameLogic().write().unwrap().GetUserConnectionsByToken(token)
                       //      {
                       //          println!("User Disconnected . . 1");
                       //          // poll.registry().deregister(connection);
                       //          // let removeID = self.clientHandler.GetIDByConnection(token);
                       //          // 두 과정은 하나의 함수로 표현해야함
                       //          // self.clientHandler.RemoveConnectionByToken(token);
                       //          // self.clientHandler.RemoveTokenPairByID(removeID);
                       //          // self.RemovePlayerByID(removeID);
                       //          // self.DecreaseNumUser();
                       //      }
                       // }
                    }
                }
                // println!("For Loop End");
            }
            // println!("Calling update_logic");
            // update_logic(self);
            // println!("update_logic called");

            // println!("Set Poll End");

            self.step += 1;
    
            // 게임 로직에서 처리후 바로 Send하고있기때문에 필요없을수도있다.
            // if gSendMessageBuffer.GetNumElem() > 0 {
            //     while let Some(item) = gSendMessageBuffer.PopData() {
            //         let mut send_data = gSendMessageBuffer.PopData();
            //         let mut senderID = send_data.as_ref().unwrap().getSenderID();
            //         let mut destination = *send_data.as_ref().unwrap().getTargetID();
            //         // let _targetID = value.get

            //         let _targetToken = *self.clientHandler.GetTokenByID(destination).unwrap();
            //         let _connStream = self.clientHandler.GetConnetionByToken(_targetToken);

            //         if let send_msg = serde_json::to_string(&send_data)? {
            //             let serialized_msg = send_msg.as_bytes();
            //             // value.getTcpStream().write(serialized_msg);
            //             _connStream.unwrap().write(serialized_msg);
            //         }
            //     }
            // }
        }

    }
}
