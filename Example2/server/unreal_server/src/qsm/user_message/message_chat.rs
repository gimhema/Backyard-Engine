use std::io;

// 자동 생성된 구조체 및 관련 메서드
        #[repr(C)]
        #[derive(Debug, Clone)]
        pub struct Chat {
           pub mid: u32,
   pub id: u32,
   pub content: String,}
    
        impl Chat {
            pub fn new(mid: u32, id: u32, content: String) -> Self {
                Self {
                    mid,
            id,
            content,        }
            }
    
            pub fn serialize(&self) -> Vec<u8> {
                let mut buffer = Vec::new();
                buffer.extend(&self.mid.to_le_bytes());
        buffer.extend(&self.id.to_le_bytes());
        buffer.extend(&self.content.len().to_le_bytes());
buffer.extend(self.content.as_bytes());
                buffer
            }
    
            pub fn deserialize(buffer: &[u8]) -> io::Result<Self> {
                let mut offset = 0;
                let mut mid_bytes = [0u8; 4];
mid_bytes.copy_from_slice(&buffer[offset..offset + 4]);
let mid = u32::from_le_bytes(mid_bytes);
offset += 4;
        let mut id_bytes = [0u8; 4];
id_bytes.copy_from_slice(&buffer[offset..offset + 4]);
let id = u32::from_le_bytes(id_bytes);
offset += 4;
        let mut content_length_bytes = [0u8; 4];
content_length_bytes.copy_from_slice(&buffer[offset..offset + 4]);
let content_length = u32::from_le_bytes(content_length_bytes);
offset += 4;
let content = String::from_utf8(buffer[offset..offset + content_length as usize].to_vec())
.map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8 string"))?;
offset += content_length as usize;
                Ok(Self {
                                mid,
            id,
            content,
                })
            }
        }