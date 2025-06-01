// 자동 생성된 구조체 및 관련 메서드
use std::io;

// 자동 생성된 구조체 및 관련 메서드
        #[repr(C)]
        #[derive(Debug, Clone)]
        pub struct CreatePlayer {
           pub mid: u32,
   pub id: u32,
   pub name: String,
   pub connect_info: String,}
    
        impl CreatePlayer {
            pub fn new(mid: u32, id: u32, name: String, connect_info: String) -> Self {
                Self {
                    mid,
            id,
            name,
            connect_info,        }
            }
    
            pub fn serialize(&self) -> Vec<u8> {
                let mut buffer = Vec::new();
                buffer.extend(&self.mid.to_le_bytes());
        buffer.extend(&self.id.to_le_bytes());
        buffer.extend(&self.name.len().to_le_bytes());
buffer.extend(self.name.as_bytes());
        buffer.extend(&self.connect_info.len().to_le_bytes());
buffer.extend(self.connect_info.as_bytes());
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
                                mid,
            id,
            name,
            connect_info,
                })
            }
        }