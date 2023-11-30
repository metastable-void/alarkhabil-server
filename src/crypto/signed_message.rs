
use crate::crypto::PrivateKey;
use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer};
use serde::{Serialize, Deserialize};

use sha2::Sha256;
use hmac::{Hmac, Mac};

use digest::CtOutput;
use generic_array::GenericArray;

// Create alias for HMAC-SHA256
type HmacSha256 = Hmac<Sha256>;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedMessage {
    algo: String,

    #[serde(with="crate::base64")]
    pubk: Vec<u8>,

    #[serde(with="crate::base64")]
    sig: Vec<u8>,

    #[serde(with="crate::base64")]
    msg: Vec<u8>,
}

impl SignedMessage {
    pub fn try_new(algo: &str, public_key: &[u8], signature: &[u8], message: &[u8]) -> Result<SignedMessage, anyhow::Error> {
        if algo == "hmac-sha256" && public_key.len() != 0 {
            return Err(anyhow::anyhow!("You must not provide public key for HMAC: {}", algo));
        }

        if algo != "hmac-sha256" && algo != "ed25519" {
            return Err(anyhow::anyhow!("Unsupported algorithm: {}", algo));
        }

        Ok(SignedMessage {
            algo: algo.to_string(),
            pubk: public_key.to_vec(),
            sig: signature.to_vec(),
            msg: message.to_vec(),
        })
    }

    pub fn create(secret_key: PrivateKey, msg: &[u8]) -> Result<SignedMessage, anyhow::Error> {
        let algo = secret_key.algo();
        let secret_key = secret_key.key();

        if algo == "hmac-sha256" {
            let mut hmac = HmacSha256::new_from_slice(secret_key).unwrap();
            hmac.update(msg);
            let signature = hmac.finalize().into_bytes().to_vec();

            return Ok(SignedMessage {
                algo: algo.to_string(),
                pubk: vec![],
                sig: signature,
                msg: msg.to_vec(),
            });
        }

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
        if self.algo == "hmac-sha256" {
            return Err(anyhow::anyhow!("You must provide secret key for HMAC: {}", self.algo));
        }

        if self.algo != "ed25519" {
            return Err(anyhow::anyhow!("Unsupported algorithm: {}", self.algo));
        }

        let public_key_bytes: &[u8; 32] = self.pubk.as_slice().try_into().map_err(|_| anyhow::anyhow!("Invalid public key length"))?;
        let public_key = VerifyingKey::from_bytes(public_key_bytes)?;

        let signature = Signature::from_slice(self.sig.as_slice())?;
        public_key.verify_strict(self.msg.as_slice(), &signature)?;
        Ok(&self.msg)
    }

    pub fn verify_with_secret(&self, secret_key: PrivateKey) -> Result<&[u8], anyhow::Error> {
        if self.algo != "hmac-sha256" {
            return Err(anyhow::anyhow!("Unsupported algorithm: {}", self.algo));
        }

        if secret_key.algo != "hmac-sha256" {
            return Err(anyhow::anyhow!("Unsupported algorithm: {}", secret_key.algo));
        }

        let secret_key = secret_key.key();

        let mut hmac = HmacSha256::new_from_slice(secret_key).unwrap();
        hmac.update(self.msg.as_slice());
        let signature = hmac.finalize();

        if signature != CtOutput::new(GenericArray::from_slice(&self.sig).clone()) {
            return Err(anyhow::anyhow!("Invalid signature"));
        }

        Ok(&self.msg)
    }

    pub fn public_key(&self) -> Result<&[u8], anyhow::Error> {
        if self.algo != "ed25519" {
            return Err(anyhow::anyhow!("Unsupported algorithm: {}", self.algo));
        }

        Ok(&self.pubk)
    }
}
