
use base64::{Engine, engine::general_purpose::STANDARD as base64_engine};
use serde::{Serialize, Deserialize};
use serde::{Deserializer, Serializer};

pub fn encode(data: &[u8]) -> String {
    base64_engine.encode(data)
}

pub fn decode(data: &str) -> Result<Vec<u8>, base64::DecodeError> {
    base64_engine.decode(data)
}

pub fn serialize<S: Serializer>(v: &Vec<u8>, s: S) -> Result<S::Ok, S::Error> {
    let base64 = encode(v);
    String::serialize(&base64, s)
}

pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<u8>, D::Error> {
    let base64 = String::deserialize(d)?;
    decode(&base64)
        .map_err(|e| serde::de::Error::custom(e))
}
