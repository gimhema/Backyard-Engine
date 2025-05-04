use std::io;

// 자동 생성된 구조체 및 관련 메서드
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ServerResponse {
   pub pId: u32,
pub responseId: u32,
pub responseMessage: String,}

impl ServerResponse {
    pub fn new(pId: u32, responseId: u32, responseMessage: String) -> Self {
        Self {
            pId,
    responseId,
    responseMessage,        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.extend(&self.pId.to_le_bytes());
buffer.extend(&self.responseId.to_le_bytes());
buffer.extend(&self.responseMessage.len().to_le_bytes());
buffer.extend(self.responseMessage.as_bytes());
        buffer
    }

    pub fn deserialize(buffer: &[u8]) -> io::Result<Self> {
        let mut offset = 0;
        let mut pId_bytes = [0u8; 4];
pId_bytes.copy_from_slice(&buffer[offset..offset + 4]);
let pId = u32::from_le_bytes(pId_bytes);
offset += 4;
let mut responseId_bytes = [0u8; 4];
responseId_bytes.copy_from_slice(&buffer[offset..offset + 4]);
let responseId = u32::from_le_bytes(responseId_bytes);
offset += 4;
let mut responseMessage_length_bytes = [0u8; 4];
responseMessage_length_bytes.copy_from_slice(&buffer[offset..offset + 4]);
let responseMessage_length = u32::from_le_bytes(responseMessage_length_bytes);
offset += 4;
let responseMessage = String::from_utf8(buffer[offset..offset + responseMessage_length as usize].to_vec())
.map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8 string"))?;
offset += responseMessage_length as usize;
        Ok(Self {
                        pId,
    responseId,
    responseMessage,
        })
    }
}