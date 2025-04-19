use std::io;

// 자동 생성된 구조체 및 관련 메서드
#[repr(C)]
#[derive(Debug, Clone)]
pub struct AllowConnectGame {
   pub sessionId: u32,
pub pid: u32,
pub accountId: String,
pub name: String,
pub connect_info: String,}

impl AllowConnectGame {
    pub fn new(sessionId: u32, pid: u32, accountId: String, name: String, connect_info: String) -> Self {
        Self {
            sessionId,
    pid,
    accountId,
    name,
    connect_info,        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.extend(&self.sessionId.to_le_bytes());
buffer.extend(&self.pid.to_le_bytes());
buffer.extend(&self.accountId.len().to_le_bytes());
buffer.extend(self.accountId.as_bytes());
buffer.extend(&self.name.len().to_le_bytes());
buffer.extend(self.name.as_bytes());
buffer.extend(&self.connect_info.len().to_le_bytes());
buffer.extend(self.connect_info.as_bytes());
        buffer
    }

    pub fn deserialize(buffer: &[u8]) -> io::Result<Self> {
        let mut offset = 0;
        let mut sessionId_bytes = [0u8; 4];
sessionId_bytes.copy_from_slice(&buffer[offset..offset + 4]);
let sessionId = u32::from_le_bytes(sessionId_bytes);
offset += 4;
let mut pid_bytes = [0u8; 4];
pid_bytes.copy_from_slice(&buffer[offset..offset + 4]);
let pid = u32::from_le_bytes(pid_bytes);
offset += 4;
let mut accountId_length_bytes = [0u8; 4];
accountId_length_bytes.copy_from_slice(&buffer[offset..offset + 4]);
let accountId_length = u32::from_le_bytes(accountId_length_bytes);
offset += 4;
let accountId = String::from_utf8(buffer[offset..offset + accountId_length as usize].to_vec())
.map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8 string"))?;
offset += accountId_length as usize;
let mut name_length_bytes = [0u8; 4];
name_length_bytes.copy_from_slice(&buffer[offset..offset + 4]);
let name_length = u32::from_le_bytes(name_length_bytes);
offset += 4;
let name = String::from_utf8(buffer[offset..offset + name_length as usize].to_vec())
.map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8 string"))?;
offset += name_length as usize;
let mut connect_info_length_bytes = [0u8; 4];
connect_info_length_bytes.copy_from_slice(&buffer[offset..offset + 4]);
let connect_info_length = u32::from_le_bytes(connect_info_length_bytes);
offset += 4;
let connect_info = String::from_utf8(buffer[offset..offset + connect_info_length as usize].to_vec())
.map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8 string"))?;
offset += connect_info_length as usize;
        Ok(Self {
                        sessionId,
    pid,
    accountId,
    name,
    connect_info,
        })
    }
}