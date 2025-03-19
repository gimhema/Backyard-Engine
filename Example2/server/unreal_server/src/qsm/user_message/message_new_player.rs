use std::io;

// 자동 생성된 구조체 및 관련 메서드
#[repr(C)]
#[derive(Debug, Clone)]
pub struct NewPlayer {
   pub pid: u32,
pub name: String,}

impl NewPlayer {
    pub fn new(pid: u32, name: String) -> Self {
        Self {
            pid,
    name,        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.extend(&self.pid.to_le_bytes());
buffer.extend(&self.name.len().to_le_bytes());
buffer.extend(self.name.as_bytes());
        buffer
    }

    pub fn deserialize(buffer: &[u8]) -> io::Result<Self> {
        let mut offset = 0;
        let mut pid_bytes = [0u8; 4];
pid_bytes.copy_from_slice(&buffer[offset..offset + 4]);
let pid = u32::from_le_bytes(pid_bytes);
offset += 4;
let mut name_length_bytes = [0u8; 4];
name_length_bytes.copy_from_slice(&buffer[offset..offset + 4]);
let name_length = u32::from_le_bytes(name_length_bytes);
offset += 4;
let name = String::from_utf8(buffer[offset..offset + name_length as usize].to_vec())
.map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8 string"))?;
offset += name_length as usize;
        Ok(Self {
                        pid,
    name,
        })
    }
}