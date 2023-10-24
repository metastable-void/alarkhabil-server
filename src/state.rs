
use std::env;
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

    pub fn new_random() -> PrimarySecret {
        let buf = rand::random::<[u8; 32]>();
        
        PrimarySecret {
            secret: hex::encode(buf),
        }
    }

    pub fn new_from_env() -> PrimarySecret {
        let env = env::var("PRIMARY_SECRET").unwrap_or_else(|_| {
            log::warn!("PRIMARY_SECRET not set, using temporary random value");
            let buf = rand::random::<[u8; 32]>();
            hex::encode(buf)
        });

        let env = if env.is_empty() {
            log::warn!("PRIMARY_SECRET is empty, using temporary random value");
            let buf = rand::random::<[u8; 32]>();
            hex::encode(buf)
        } else {
            env
        };

        PrimarySecret::new(
            env
        )
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
