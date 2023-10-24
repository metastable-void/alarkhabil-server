
pub struct PrivateKey {
    pub(crate) algo: String,
    pub(crate) key: Vec<u8>,
}

impl PrivateKey {
    pub fn new(algo: &str) -> Result<PrivateKey, anyhow::Error> {
        if algo != "ed25519" && algo != "hmac-sha256" {
            return Err(anyhow::anyhow!("Unsupported algorithm: {}", algo));
        }

        let secret_key: [u8; 32] = rand::random();

        Ok(PrivateKey {
            algo: algo.to_string(),
            key: secret_key.to_vec(),
        })
    }

    pub fn from_bytes(algo: &str, buf: &[u8]) -> Result<PrivateKey, anyhow::Error> {
        if algo != "ed25519" && algo != "hmac-sha256" {
            return Err(anyhow::anyhow!("Unsupported algorithm: {}", algo));
        }

        if algo == "ed25519" && buf.len() != 32 {
            return Err(anyhow::anyhow!("Invalid key length"));
        }

        Ok(PrivateKey {
            algo: algo.to_string(),
            key: buf.to_vec(),
        })
    }

    pub fn algo(&self) -> &str {
        &self.algo
    }

    pub fn key(&self) -> &[u8] {
        &self.key
    }
}
