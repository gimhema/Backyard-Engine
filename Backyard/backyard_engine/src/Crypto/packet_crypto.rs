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
    pub fn new(_mode : CryptMode) -> Self {
        cryption_processor { mode: _mode }
    }

    pub fn encrypt(target : String) -> String {
        let mut result = "";

        return result.to_string();
    }

    pub fn decrypt(target : String) -> String {
        let mut result = "";

        return result.to_string();
    }

}

