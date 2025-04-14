use std::io;

// 자동 생성된 구조체 및 관련 메서드
#[repr(C)]
#[derive(Debug, Clone)]
pub struct MakeAccount {
   pub userId: String,
pub userName: String,
pub password: String,}

impl MakeAccount {
    pub fn new(userId: String, userName: String, password: String) -> Self {
        Self {
            userId,
    userName,
    password,        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.extend(&self.userId.len().to_le_bytes());
buffer.extend(self.userId.as_bytes());
buffer.extend(&self.userName.len().to_le_bytes());
buffer.extend(self.userName.as_bytes());
buffer.extend(&self.password.len().to_le_bytes());
buffer.extend(self.password.as_bytes());
        buffer
    }

    pub fn deserialize(buffer: &[u8]) -> io::Result<Self> {
        let mut offset = 0;
        let mut userId_length_bytes = [0u8; 4];
userId_length_bytes.copy_from_slice(&buffer[offset..offset + 4]);
let userId_length = u32::from_le_bytes(userId_length_bytes);
offset += 4;
let userId = String::from_utf8(buffer[offset..offset + userId_length as usize].to_vec())
.map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8 string"))?;
offset += userId_length as usize;
let mut userName_length_bytes = [0u8; 4];
userName_length_bytes.copy_from_slice(&buffer[offset..offset + 4]);
let userName_length = u32::from_le_bytes(userName_length_bytes);
offset += 4;
let userName = String::from_utf8(buffer[offset..offset + userName_length as usize].to_vec())
.map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8 string"))?;
offset += userName_length as usize;
let mut password_length_bytes = [0u8; 4];
password_length_bytes.copy_from_slice(&buffer[offset..offset + 4]);
let password_length = u32::from_le_bytes(password_length_bytes);
offset += 4;
let password = String::from_utf8(buffer[offset..offset + password_length as usize].to_vec())
.map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8 string"))?;
offset += password_length as usize;
        Ok(Self {
                        userId,
    userName,
    password,
        })
    }
}