use crate::qsm::qsm::handle_quicksot_message;

macro_rules! enum_from_u32 {
    ($name:ident { $($variant:ident = $value:expr),* $(,)? }) => {
        #[repr(u32)]
        #[derive(Debug, Clone, Copy)]
        pub enum $name {
            $($variant = $value),*
        }

        impl From<u32> for $name {
            fn from(value: u32) -> Self {
                match value {
                    $($value => $name::$variant,)*
                    _ => $name::DEFAULT,
                }
            }
        }
    };
}

enum_from_u32! {
    EventHeader {
        DEFAULT = 0,
        END = 1
    }
}

impl EventHeader {
    pub fn action(buffer: &[u8]) {
        // 지금은 QSM 파서로 넘기는 최소 연결만 유지
        handle_quicksot_message(buffer);
    }
}
