#[macro_use]
extern crate lazy_static;

mod Agent;
mod Core;
mod Event;
mod Network;
mod qsm;
mod manager_messages;
mod ds_registry;


use std::io;
use crate::Network::server::Server;

fn main() -> io::Result<()> {
    let mut server = Server::new("127.0.0.1:8080", "127.0.0.1:8082")?;
    server.start()?;
    Ok(())
}
