use std::result;





pub enum CryptMode {
    DEFAULT,
    AES,
    ChaCha20,
    RSA
}

pub struct cryption_processor {
    mode : CryptMode
}

impl cryption_processor {
    pub fn new() -> Self {
        cryption_processor { mode: CryptMode::DEFAULT }
    }

    pub fn encrypt(target : String) -> String {
        let mut result = "";

        return result.to_string();
    }

    pub fn decrypt(target : String) -> String {
        let mut result = "";

        return result.to_string();
    }

    pub fn init(&mut self) {
        // read file . . .
    }

}

