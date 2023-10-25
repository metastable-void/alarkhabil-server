
use serde::{Serialize, Deserialize};

use crate::markdown;


/// AuthorInfo is a struct that contains detailed information about an author.
/// It is for example returned by `/api/v1/author/info`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorInfo {
    uuid: String,
    name: String,
    created_date: u64,
    description_text: String,
    description_html: String,
}

impl AuthorInfo {
    pub fn new(uuid: &str, name: &str, created_date: u64, description_text: &str) -> AuthorInfo {
        AuthorInfo {
            uuid: uuid.to_string(),
            name: name.to_string(),
            created_date,
            description_text: description_text.to_string(),
            description_html: markdown::to_html(description_text),
        }
    }

    pub fn uuid(&self) -> &str {
        &self.uuid
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn created_date(&self) -> u64 {
        self.created_date
    }

    pub fn description_text(&self) -> &str {
        &self.description_text
    }

    pub fn description_html(&self) -> &str {
        &self.description_html
    }
}


/// AuthorSummary is a struct that contains summary information about an author.
/// It appears in various places, for example inside another struct.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorSummary {
    uuid: String,
    name: String,
}

impl AuthorSummary {
    pub fn new(uuid: &str, name: &str) -> AuthorSummary {
        AuthorSummary {
            uuid: uuid.to_string(),
            name: name.to_string(),
        }
    }

    pub fn uuid(&self) -> &str {
        &self.uuid
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
