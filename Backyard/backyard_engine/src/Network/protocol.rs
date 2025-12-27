// src/Network/protocol.rs
use std::io;

pub const MAX_FRAME_SIZE: usize = 1024 * 1024; // 1MB (필요시 조정)

/// length-prefixed frame:
/// [u32_le length][payload bytes...]
pub fn encode_frame(payload: &[u8]) -> Vec<u8> {
    let len = payload.len() as u32;
    let mut out = Vec::with_capacity(4 + payload.len());
    out.extend_from_slice(&len.to_le_bytes());
    out.extend_from_slice(payload);
    out
}

/// read buffer에서 "완전한 프레임"들을 최대한 많이 뽑아냄.
/// - buf는 '누적 버퍼'로 사용되며, 소비된 바이트는 제거됨.
/// - 프레임이 완성되지 않았으면 남겨둠.
pub fn drain_frames(buf: &mut Vec<u8>) -> io::Result<Vec<Vec<u8>>> {
    let mut frames = Vec::new();

    loop {
        if buf.len() < 4 {
            break;
        }

        let len = u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]) as usize;
        if len > MAX_FRAME_SIZE {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "frame too large"));
        }

        if buf.len() < 4 + len {
            break;
        }

        // payload 추출
        let payload = buf[4..4 + len].to_vec();
        // consume
        buf.drain(0..4 + len);

        frames.push(payload);
    }

    Ok(frames)
}
