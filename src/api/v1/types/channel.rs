
use serde::{Serialize, Deserialize};

use crate::api::v1::types::is_valid_dns_token;


pub fn validate_channel_handle(handle: &str) -> Result<(), anyhow::Error> {
    if !is_valid_dns_token(handle) {
        return Err(anyhow::anyhow!("Invalid handle"));
    }
    Ok(())
}

/// ChannelInfo is a struct that contains detailed information about a channel.
/// It is for example returned by `/api/v1/channel/info`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelInfo {
    uuid: String,
    handle: String,
    name: String,
    created_date: u64,
    lang: String,
    description_text: String,
}

impl ChannelInfo {
    pub fn new(uuid: &str, handle: &str, name: &str, created_date: u64, lang: &str, description_text: &str) -> ChannelInfo {
        ChannelInfo {
            uuid: uuid.to_string(),
            handle: handle.to_string(),
            name: name.to_string(),
            created_date,
            lang: lang.to_string(),
            description_text: description_text.to_string(),
        }
    }

    pub fn uuid(&self) -> &str {
        &self.uuid
    }

    pub fn handle(&self) -> &str {
        &self.handle
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn created_date(&self) -> u64 {
        self.created_date
    }

    pub fn lang(&self) -> &str {
        &self.lang
    }

    pub fn description_text(&self) -> &str {
        &self.description_text
    }
}


/// ChannelSummary is a struct that contains summary information about a channel.
/// It appears in various places, for example inside another struct.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelSummary {
    uuid: String,
    handle: String,
    name: String,
    lang: String,
}

impl ChannelSummary {
    pub fn new(uuid: &str, handle: &str, name: &str, lang: &str) -> ChannelSummary {
        ChannelSummary {
            uuid: uuid.to_string(),
            handle: handle.to_string(),
            name: name.to_string(),
            lang: lang.to_string(),
        }
    }

    pub fn uuid(&self) -> &str {
        &self.uuid
    }

    pub fn handle(&self) -> &str {
        &self.handle
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn lang(&self) -> &str {
        &self.lang
    }
}
