
use regex::Regex;

use serde::{Serialize, Deserialize};

use crate::markdown;


pub fn validate_channel_handle(handle: &str) -> Result<(), anyhow::Error> {
    let re = Regex::new(r"^[a-z0-9]+(-[a-z0-9]+)*$").unwrap();
    if !re.is_match(handle) || handle.len() > 64 {
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
    description_html: String,
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
            description_html: markdown::to_html(description_text),
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

    pub fn description_html(&self) -> &str {
        &self.description_html
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
