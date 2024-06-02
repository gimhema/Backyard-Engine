use super::Network;


use std::sync::Mutex;
use std::collections::HashMap;
use std::time::Duration;
use std::{thread, time};
use std::sync::{RwLock, Arc, RwLockReadGuard};
use super::connection::*;


pub struct server {
    connectionHandler: connection_handler,
    numUser: i64,
    step: i64
}
