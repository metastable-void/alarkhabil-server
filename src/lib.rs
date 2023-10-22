
#![feature(impl_trait_in_assoc_type)]

use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer};
use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedMessage {
    algo: String,

    #[serde(with="base64")]
    pubk: Vec<u8>,

    #[serde(with="base64")]
    sig: Vec<u8>,

    #[serde(with="base64")]
    msg: Vec<u8>,
}

pub struct PrivateKey {
    algo: String,
    key: Vec<u8>,
}

impl PrivateKey {
    pub fn new(algo: &str) -> Result<PrivateKey, anyhow::Error> {
        if algo != "ed25519" {
            return Err(anyhow::anyhow!("Unsupported algorithm: {}", algo));
        }

        let secret_key: [u8; 32] = rand::random();

        Ok(PrivateKey {
            algo: algo.to_string(),
            key: secret_key.to_vec(),
        })
    }

    pub fn algo(&self) -> &str {
        &self.algo
    }

    pub fn key(&self) -> &[u8] {
        &self.key
    }
}

impl SignedMessage {
    pub fn create(secret_key: PrivateKey, msg: &[u8]) -> Result<SignedMessage, anyhow::Error> {
        let algo = secret_key.algo();
        let secret_key = secret_key.key();

        if algo != "ed25519" {
            return Err(anyhow::anyhow!("Unsupported algorithm: {}", algo));
        }

        let secret_key: SigningKey = secret_key.try_into().map_err(|_| anyhow::anyhow!("Invalid secret key length"))?;
        let public_key = secret_key.verifying_key();
        let signature = secret_key.sign(msg);

        Ok(SignedMessage {
            algo: algo.to_string(),
            pubk: public_key.to_bytes().to_vec(),
            sig: signature.to_bytes().to_vec(),
            msg: msg.to_vec(),
        })
    }

    pub fn algo(&self) -> &str {
        &self.algo
    }

    pub fn verify(&self) -> Result<&[u8], anyhow::Error> {
        if self.algo != "ed25519" {
            return Err(anyhow::anyhow!("Unsupported algorithm: {}", self.algo));
        }

        let public_key_bytes: &[u8; 32] = self.pubk.as_slice().try_into().map_err(|_| anyhow::anyhow!("Invalid public key length"))?;
        let public_key = VerifyingKey::from_bytes(public_key_bytes)?;

        let signature = Signature::from_slice(self.sig.as_slice())?;
        public_key.verify_strict(self.msg.as_slice(), &signature)?;
        Ok(&self.msg)
    }
}

mod base64 {
    use serde::{Serialize, Deserialize};
    use serde::{Deserializer, Serializer};
    use base64::{Engine, engine::general_purpose::STANDARD as base64_engine};

    pub fn serialize<S: Serializer>(v: &Vec<u8>, s: S) -> Result<S::Ok, S::Error> {
        let base64 = base64_engine.encode(v);
        String::serialize(&base64, s)
    }
    
    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<u8>, D::Error> {
        let base64 = String::deserialize(d)?;
        base64_engine.decode(base64.as_bytes())
            .map_err(|e| serde::de::Error::custom(e))
    }
}

pub fn extract_ed25519_private_key(buf: &[u8]) -> Result<[u8; 32], anyhow::Error> {
    buf.try_into().map_err(|_| anyhow::anyhow!("Invalid length"))
}
