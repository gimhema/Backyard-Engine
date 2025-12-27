// src/manager_messages.rs
use std::io;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MsgId {
    DsRegister  = 1,
    DsHeartbeat = 2,
}

#[derive(Debug, Clone)]
pub struct DsRegister {
    pub ds_id: String,
    pub game_port: u16,
    pub max_players: u16,
    pub build: String,
}

#[derive(Debug, Clone)]
pub struct DsHeartbeat {
    pub ds_id: String,
    pub current_players: u16,
    pub state: u8, // 0=BOOTING,1=READY,2=RUNNING,3=SHUTTING_DOWN...
}

fn read_u16_le(b: &[u8], i: &mut usize) -> io::Result<u16> {
    if *i + 2 > b.len() { return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "u16")); }
    let v = u16::from_le_bytes([b[*i], b[*i+1]]);
    *i += 2;
    Ok(v)
}

fn read_u32_le(b: &[u8], i: &mut usize) -> io::Result<u32> {
    if *i + 4 > b.len() { return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "u32")); }
    let v = u32::from_le_bytes([b[*i], b[*i+1], b[*i+2], b[*i+3]]);
    *i += 4;
    Ok(v)
}

fn read_u8(b: &[u8], i: &mut usize) -> io::Result<u8> {
    if *i + 1 > b.len() { return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "u8")); }
    let v = b[*i];
    *i += 1;
    Ok(v)
}

fn read_string_u16(b: &[u8], i: &mut usize) -> io::Result<String> {
    let n = read_u16_le(b, i)? as usize;
    if *i + n > b.len() { return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "string")); }
    let s = std::str::from_utf8(&b[*i..*i+n])
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "utf8"))?
        .to_string();
    *i += n;
    Ok(s)
}

pub enum ManagerMsg {
    Register(DsRegister),
    Heartbeat(DsHeartbeat),
}

pub fn parse_manager_msg(payload: &[u8]) -> io::Result<ManagerMsg> {
    let mut i = 0usize;
    let msg_id = read_u32_le(payload, &mut i)?;

    match msg_id {
        x if x == MsgId::DsRegister as u32 => {
            let ds_id = read_string_u16(payload, &mut i)?;
            let game_port = read_u16_le(payload, &mut i)?;
            let max_players = read_u16_le(payload, &mut i)?;
            let build = read_string_u16(payload, &mut i)?;
            Ok(ManagerMsg::Register(DsRegister { ds_id, game_port, max_players, build }))
        }
        x if x == MsgId::DsHeartbeat as u32 => {
            let ds_id = read_string_u16(payload, &mut i)?;
            let current_players = read_u16_le(payload, &mut i)?;
            let state = read_u8(payload, &mut i)?;
            Ok(ManagerMsg::Heartbeat(DsHeartbeat { ds_id, current_players, state }))
        }
        _ => Err(io::Error::new(io::ErrorKind::InvalidData, "unknown msg_id")),
    }
}
