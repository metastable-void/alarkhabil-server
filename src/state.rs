
use std::sync::Mutex;

use sha2::Sha256;
use hmac::{Hmac, Mac};


type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, Clone)]
pub struct PrimarySecret {
    secret: String,
}

impl PrimarySecret {
    pub fn new(secret: String) -> PrimarySecret {
        PrimarySecret {
            secret,
        }
    }

    pub fn derive_secret(&self, name: &str) -> Vec<u8> {
        let mut hmac = HmacSha256::new_from_slice(self.secret.as_bytes()).unwrap();
        hmac.update(name.as_bytes());
        hmac.finalize().into_bytes().to_vec()
    }
}

#[derive(Debug)]
pub struct AppState {
    pub db_connection: Mutex<rusqlite::Connection>,
    pub primary_secret: PrimarySecret,
}
